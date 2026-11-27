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

//! How a detection is read and written (`json-output.md` section 4.3). Each outcome that this version knows has the
//! members that it needs and none that another outcome needs, as the fourteenth review of `api.md` asked, so an
//! entry's outcome follows from a detection that describes it. The candidates name each format once, rank by
//! confidence that never rises, and an insufficient detection's reason is the input's end or a named limit (fifteenth
//! review). `bytes_needed` beyond 2^53 - 1 is its twin `bytes_needed_exact`. A detection of a newer outcome keeps
//! every member that it does not read and writes it back after its own; the members are held only until the outcome
//! is known (eighteenth review), and a detection of a known outcome passes them by.

use serde::Serialize;

use super::{Candidate, Confidence, Detection, DetectionOutcome, Hint};
use crate::Format;
use crate::completeness::Reason;
use crate::document::SchemaVersion;
use crate::form::{KeptMembers, Outcome, Rooted, WithKept, each_once, unsigned_members, unsigned_value};
use crate::model::Usage;

/// A detection as written: its version first, then its members, then the members that an outcome of a newer version
/// kept.
#[derive(Serialize)]
struct DetectionMembers<'detection> {
  schema_version: SchemaVersion,
  outcome: &'detection DetectionOutcome,
  #[serde(skip_serializing_if = "Option::is_none")]
  format: Option<&'detection Format>,
  #[serde(skip_serializing_if = "Option::is_none")]
  confidence: Option<Confidence>,
  candidates: &'detection [Candidate],
  #[serde(skip_serializing_if = "Option::is_none")]
  hint: Option<&'detection Hint>,
  #[serde(skip_serializing_if = "Option::is_none")]
  bytes_needed: Option<u64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  bytes_needed_exact: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  insufficient_reason: Option<&'detection Reason>,
  usage: &'detection Usage,
  #[serde(flatten)]
  kept: &'detection KeptMembers,
}

impl Serialize for Detection {
  fn serialize<Output: serde::Serializer>(&self, serializer: Output) -> Result<Output::Ok, Output::Error> {
    let (bytes_needed, bytes_needed_exact) = unsigned_members(self.bytes_needed);
    DetectionMembers {
      schema_version: self.schema_version,
      outcome: &self.outcome,
      format: self.format.as_ref(),
      confidence: self.confidence,
      candidates: &self.candidates,
      hint: self.hint.as_ref(),
      bytes_needed,
      bytes_needed_exact,
      insufficient_reason: self.insufficient_reason.as_ref(),
      usage: &self.usage,
      kept: &self.kept,
    }
    .serialize(serializer)
  }
}

/// The members of a detection other than its version, as read.
#[derive(serde::Deserialize)]
pub(super) struct DetectionBody {
  outcome: DetectionOutcome,
  format: Option<Format>,
  confidence: Option<Confidence>,
  candidates: Vec<Candidate>,
  hint: Option<Hint>,
  bytes_needed: Option<u64>,
  bytes_needed_exact: Option<String>,
  insufficient_reason: Option<Reason>,
  usage: Usage,
}

impl Outcome for DetectionBody {
  const MEMBER: &'static str = super::constants::OUTCOME_MEMBER;

  fn is_newer(text: &str) -> bool {
    DetectionOutcome::from_known_text(text).is_none()
  }
}

impl TryFrom<Rooted<WithKept<DetectionBody>>> for Detection {
  type Error = &'static str;

  fn try_from(rooted: Rooted<WithKept<DetectionBody>>) -> Result<Detection, &'static str> {
    let (body, kept) = (rooted.body.data, rooted.body.kept);
    let detection = Detection {
      schema_version: rooted.version,
      outcome: body.outcome,
      format: body.format,
      confidence: body.confidence,
      candidates: body.candidates,
      hint: body.hint,
      bytes_needed: unsigned_value(body.bytes_needed, body.bytes_needed_exact)?,
      insufficient_reason: body.insufficient_reason,
      usage: body.usage,
      kept,
    };
    if !candidates_rank(&detection.candidates) || !members_fit_the_outcome(&detection) {
      return Err(super::constants::DETECTION_MESSAGE);
    }
    check_activities(&detection)?;
    Ok(detection)
  }
}

/// Whether a detection of a known outcome charged only what detection charges (`api.md` sections 5.3 and 8), as the
/// maintainer chose at the seventh review of the third slice's first part, which read an insufficient detection whose
/// reason named the companion requests, a cause that the error of the same detection refuses: detection opens no
/// companion and runs no prober, so its insufficient reason names a limit that can stop detection
/// (`crate::Limit::can_stop`), and its usage counts no companion request, no companion source opened and no payload
/// retained. An outcome of a newer version is not checked.
fn check_activities(detection: &Detection) -> Result<(), &'static str> {
  if matches!(detection.outcome, DetectionOutcome::Unrecognized(_)) {
    return Ok(());
  }
  let stopped_detection = detection
    .insufficient_reason
    .as_ref()
    .is_none_or(|reason| reason.can_stop(&crate::completeness::Part::Detection));
  if !stopped_detection {
    return Err(crate::constants::SCOPED_LIMIT_MESSAGE);
  }
  if detection.usage.charges_no_prober() {
    Ok(())
  } else {
    Err(super::constants::DETECTION_USAGE_MESSAGE)
  }
}

/// Whether the candidates name each format once, with confidences that never rise.
fn candidates_rank(candidates: &[Candidate]) -> bool {
  each_once(candidates, Candidate::format)
    && candidates.is_sorted_by(|earlier, later| earlier.confidence() >= later.confidence())
}

/// Whether a detection of a known outcome holds the members that its outcome needs and none that another needs.
fn members_fit_the_outcome(detection: &Detection) -> bool {
  let family = |wanted: bool| {
    detection
      .format
      .as_ref()
      .is_some_and(|format| matches!(format, Format::Unrecognized(_)) || format.is_family() == wanted)
  };
  let no_format = detection.format.is_none() && detection.confidence.is_none();
  let no_candidates = detection.candidates.is_empty();
  let no_shortage = detection.bytes_needed.is_none() && detection.insufficient_reason.is_none();
  let no_hint = detection.hint.is_none();
  match detection.outcome {
    DetectionOutcome::Identified => {
      family(false) && detection.confidence.is_some() && no_candidates && no_hint && no_shortage
    }
    DetectionOutcome::FamilyOnly => {
      family(true) && detection.confidence.is_none() && no_candidates && no_hint && no_shortage
    }
    DetectionOutcome::Ambiguous => {
      detection.candidates.len() >= super::constants::FEWEST_AMBIGUOUS_CANDIDATES && no_format && no_hint && no_shortage
    }
    DetectionOutcome::HintOnly => detection.hint.is_some() && no_format && no_candidates && no_shortage,
    DetectionOutcome::InsufficientData => {
      no_format
        && no_candidates
        && no_hint
        && detection
          .insufficient_reason
          .as_ref()
          .is_none_or(Reason::tells_too_few_bytes)
    }
    DetectionOutcome::Unknown => no_format && no_candidates && no_hint && no_shortage,
    DetectionOutcome::Unrecognized(_) => true,
  }
}

impl Detection {
  /// Gives the detection that version, as a document gives each detection that it holds (`api.md` section 12).
  pub(crate) fn set_schema_version(&mut self, version: SchemaVersion) {
    self.schema_version = version;
  }
}
