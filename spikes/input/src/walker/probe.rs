/*
* Copyright (c) 2026. caoccao.com Sam Cao
* All rights reserved.

* Licensed under the Apache License, Version 2.0 (the "License");
* you may not use this file except in compliance with the License.
* You may obtain a copy of the License at

* http://www.apache.org/licenses/LICENSE-2.0

* Unless required by applicable law or agreed to in writing, software
* distributed under the License is distributed on an "AS IS" BASIS,
* WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
* See the License for the specific language governing permissions and
* limitations under the License.
*/

//! `probe`, one whole probe: detection, the walk of the primary input, then each companion in the order in which
//! the primary input names it, all under one budget (D16). A failing source ends the probe with an error that keeps
//! its cause and the report built so far, its waiting streams with the fields read (R19). The primary input's
//! buffers are released once its walk is over, since nothing reads it again, so a companion's walk has the whole
//! working memory that the scratch buffers leave.

use std::io;
use std::path::PathBuf;

use super::identities::Identities;
use super::walk::walk_source;
use super::{PRIMARY, Walk};
use crate::companion::{CompanionRequest, Companions, FILE_NUMBERS_LENGTH, Resolved, resolved_identity};
use crate::detection::{Detected, detect};
use crate::options::Options;
use crate::report::{DetectionOutcome, ProbeError, Reason, SourceOutcome, ToyReport};
use crate::source::InternalSource;

/// What the probe knows of the primary input besides its bytes.
#[derive(Clone, Debug, Default)]
pub(crate) struct Primary {
  /// The device and file numbers of the primary file where the standard library gives them, a fixed-size value
  /// taken when it was opened, so no buffer holds an identity while the walk runs.
  pub(crate) numbers: Option<[u8; FILE_NUMBERS_LENGTH]>,
  /// Whether the built-in resolver serves the probe: only it compares companions with the primary file's
  /// identity, for `probe_file` alone (`api.md` section 13).
  pub(crate) built_in: bool,
  /// The path that the caller gave to `probe_file`.
  pub(crate) path: Option<PathBuf>,
}

impl Primary {
  /// The primary file's identity in the built-in resolver's form: its file numbers, or else its resolved path.
  fn identity(&self) -> Option<Vec<u8>> {
    if !self.built_in {
      return None;
    }
    match self.numbers {
      Some(numbers) => Some(numbers.to_vec()),
      None => self.path.as_deref().and_then(resolved_identity),
    }
  }
}

/// A source that failed, by its identifier.
struct Failure {
  source: u64,
  cause: io::Error,
}

/// Probes `source` with `options`, resolving companions through `lookup`.
pub(crate) async fn probe<Source: InternalSource, Lookup: Companions>(
  mut source: Source,
  mut lookup: Lookup,
  options: &Options,
  primary: Primary,
) -> Result<ToyReport, ProbeError> {
  let mut walk = Walk::new(options, lookup.enabled());
  let result = run(&mut source, &mut lookup, &mut walk, &primary).await;
  let delivered = source.delivered();
  source.release(&mut walk.budget);
  match result {
    Ok(()) => Ok(walk.into_report(delivered, SourceOutcome::Opened)),
    Err(Failure { source, cause }) => {
      let outcome = if source == PRIMARY {
        SourceOutcome::Failed
      } else {
        SourceOutcome::Opened
      };
      Err(ProbeError::SourceFailed {
        source,
        cause,
        partial: Box::new(walk.into_report(delivered, outcome)),
      })
    }
  }
}

/// Detection, the primary input's walk, and the companions.
async fn run<Source: InternalSource, Lookup: Companions>(
  source: &mut Source,
  lookup: &mut Lookup,
  walk: &mut Walk<'_>,
  primary: &Primary,
) -> Result<(), Failure> {
  let primary_failure = |cause| Failure { source: PRIMARY, cause };
  let first_element = match detect(source, &mut walk.budget).await.map_err(primary_failure)? {
    Detected::Identified { first_element } => first_element,
    Detected::Concluded(outcome) => {
      walk.detection = outcome;
      return Ok(());
    }
  };
  walk.detection = DetectionOutcome::Identified;
  let walked = match walk.prepare(source) {
    Ok(()) => walk_source(source, walk, PRIMARY, first_element).await,
    Err(limit) => {
      walk.stop(Reason::Limit(limit));
      Ok(())
    }
  };
  walk.end_streams(PRIMARY, &walked);
  walked.map_err(primary_failure)?;
  source.release(&mut walk.budget);
  resolve_companions(lookup, walk, primary).await
}

/// Requests every companion in order, each within its limits, which are checked just before its call. A found
/// source whose identity an earlier source has is closed again, and its entry records the alias; any other is
/// walked like the primary input, without following its own companions. The identities stay held until the last
/// companion is done; with no companion waiting, none is taken.
async fn resolve_companions<Lookup: Companions>(
  lookup: &mut Lookup,
  walk: &mut Walk<'_>,
  primary: &Primary,
) -> Result<(), Failure> {
  if walk.queued() == 0 {
    return Ok(());
  }
  let mut identities = Identities::default();
  if let Some(identity) = primary.identity() {
    let _ = identities.keep(&mut walk.budget, PRIMARY, &identity);
  }
  let result = request_each(lookup, walk, primary, &mut identities).await;
  identities.release(&mut walk.budget);
  result
}

/// The requests of `resolve_companions`, in order.
async fn request_each<Lookup: Companions>(
  lookup: &mut Lookup,
  walk: &mut Walk<'_>,
  primary: &Primary,
  identities: &mut Identities,
) -> Result<(), Failure> {
  let options = walk.options;
  for index in 0..walk.queued() {
    let Some(request) = walk.pending(index).and_then(|pending| walk.admit(pending)) else {
      continue;
    };
    let asked = CompanionRequest {
      source_identifier: request.source_identifier,
      identifier: request.identifier.clone(),
      name: request.name.clone(),
      primary_path: primary.path.clone(),
    };
    let failure = |cause| Failure {
      source: request.source_identifier,
      cause,
    };
    match lookup.resolve(&asked, options).await {
      Resolved::Found(mut companion, identity) => {
        walk.count_opened();
        let alias = identity.as_deref().and_then(|identity| identities.alias(identity));
        if alias.is_some() {
          walk.record(&request, SourceOutcome::Opened, 0, alias);
          continue;
        }
        if let Some(identity) = identity
          && let Err(limit) = identities.keep(&mut walk.budget, request.source_identifier, &identity)
        {
          walk.record(&request, SourceOutcome::LimitReached(limit), 0, None);
          continue;
        }
        let result = walk_companion(&mut companion, walk, request.source_identifier).await;
        let delivered = companion.delivered();
        companion.release(&mut walk.budget);
        if let Err(cause) = result {
          walk.record(&request, SourceOutcome::Failed, delivered, None);
          return Err(failure(cause));
        }
        walk.record(&request, SourceOutcome::Opened, delivered, None);
      }
      Resolved::Other(outcome) => walk.record(&request, outcome, 0, None),
      Resolved::Failed(cause) => {
        walk.record(&request, SourceOutcome::Failed, 0, None);
        return Err(failure(cause));
      }
    }
  }
  Ok(())
}

/// Detects and walks one companion.
async fn walk_companion<Source: InternalSource>(
  source: &mut Source,
  walk: &mut Walk<'_>,
  identifier: u64,
) -> io::Result<()> {
  let Detected::Identified { first_element } = detect(source, &mut walk.budget).await? else {
    return Ok(());
  };
  let walked = match walk.prepare(source) {
    Ok(()) => walk_source(source, walk, identifier, first_element).await,
    Err(limit) => {
      walk.stop(Reason::Limit(limit));
      Ok(())
    }
  };
  walk.end_streams(identifier, &walked);
  walked
}
