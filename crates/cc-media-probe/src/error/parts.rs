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

//! `ErrorParts`, what an error and its record keep alike (`api.md` sections 10.1 and 10.2).

use super::{CandidateCheck, ErrorKind, Observations};
use crate::Format;
use crate::completeness::{LimitRecord, Reason};
use crate::model::{Hint, MediaReport, Usage};
use crate::source::{AttemptedIdentity, SourceIdentifier};

/// What an error and its record keep alike: everything but the I/O error, which only the live error keeps, and the
/// texts that only the record keeps in its place. A record of an error that it takes by value moves these parts,
/// so the partial report is never copied (`api.md` section 10.1).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(super) struct ErrorParts {
  /// Which ending of the ordered procedure the error is.
  pub(super) kind: ErrorKind,
  /// The detected format or family.
  pub(super) format: Option<Format>,
  /// The checks of the candidates, in rank.
  pub(super) candidates: Vec<CandidateCheck>,
  /// For an unknown format with a hint, the format that the hint suggests.
  pub(super) hint: Option<Hint>,
  /// The one reason that stopped the probe.
  pub(super) reason: Option<Reason>,
  /// The source concerned.
  pub(super) source_identifier: Option<SourceIdentifier>,
  /// For a companion that was requested, the identity that the request asked for.
  pub(super) attempted_identity: Option<AttemptedIdentity>,
  /// Where the failure or the stop happened, relative to the start of the source concerned.
  pub(super) offset: Option<u64>,
  /// Where the failure or the stop happened in the stream that several files form.
  pub(super) logical_offset: Option<u64>,
  /// The report established before a source failed, behind its own pointer, so that moving the parts never moves
  /// a large value.
  pub(super) partial_report: Option<Box<MediaReport>>,
  /// What the probe saw before it stopped short of its stream inventory.
  pub(super) observations: Option<Observations>,
  /// What the budget accounted for.
  pub(super) usage: Usage,
  /// The fixed-size record; the partial report's own when there is one.
  pub(super) record: LimitRecord,
}

impl ErrorParts {
  /// The parts of an error of `kind` that holds only what the budget accounted for: no partial data, and a record
  /// that nothing stopped.
  pub(super) fn new(kind: ErrorKind, usage: Usage) -> ErrorParts {
    ErrorParts {
      kind,
      format: None,
      candidates: Vec::new(),
      hint: None,
      reason: None,
      source_identifier: None,
      attempted_identity: None,
      offset: None,
      logical_offset: None,
      partial_report: None,
      observations: None,
      usage,
      record: LimitRecord::empty(),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::ErrorParts;
  use crate::error::ErrorKind;
  use crate::model::Usage;

  impl ErrorParts {
    /// The parts of an error of this kind, without partial data.
    pub(crate) fn sample(kind: ErrorKind) -> ErrorParts {
      ErrorParts::new(kind, Usage::sample())
    }
  }

  #[test]
  fn compares_by_every_part() {
    let parts = ErrorParts::sample(ErrorKind::SourceFailed);
    assert_eq!(parts.clone(), parts);
    assert_ne!(parts, ErrorParts::sample(ErrorKind::UnknownFormat));
  }
}
