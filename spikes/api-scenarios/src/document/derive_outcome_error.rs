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

//! The refusal of an entry whose outcome this version cannot derive, `api.md` section 12, and the way back.

use std::fmt;

use super::entry::DocumentEntry;
use super::outcome::EntryOutcome;
use crate::constants::DERIVE_OUTCOME_MESSAGE;
use crate::kept::KeptMembers;
use crate::model::{Detection, MediaReport};
use crate::source::PortablePath;

/// A report or detection whose outcome this version cannot derive, because only a value of a newer version decides
/// it. It keeps the entry's parts, behind one pointer, so the outcome can be given, and it gives them back.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct DeriveOutcomeError {
  parts: Box<RefusedParts>,
}

/// The parts of an entry that is still without its outcome.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct RefusedParts {
  input: PortablePath,
  record: RefusedRecord,
}

/// The record of such an entry.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(super) enum RefusedRecord {
  /// A report that only a status of a newer version decides.
  Report(Box<MediaReport>),
  /// A detection whose outcome a newer version wrote.
  Detection(Box<Detection>),
}

impl DeriveOutcomeError {
  /// The refusal of an entry of these parts.
  pub(super) fn new(input: PortablePath, record: RefusedRecord) -> DeriveOutcomeError {
    DeriveOutcomeError {
      parts: Box::new(RefusedParts { input, record }),
    }
  }

  /// The input that the refused entry was given.
  pub fn input(&self) -> &PortablePath {
    &self.parts.input
  }

  /// The report that the refused entry was given, when it was given one.
  pub fn report(&self) -> Option<&MediaReport> {
    match &self.parts.record {
      RefusedRecord::Report(report) => Some(report),
      RefusedRecord::Detection(_) => None,
    }
  }

  /// The detection that the refused entry was given, when it was given one.
  pub fn detection(&self) -> Option<&Detection> {
    match &self.parts.record {
      RefusedRecord::Detection(detection) => Some(detection),
      RefusedRecord::Report(_) => None,
    }
  }

  /// The report, given back without a copy.
  pub fn into_report(self) -> Option<MediaReport> {
    match self.parts.record {
      RefusedRecord::Report(report) => Some(*report),
      RefusedRecord::Detection(_) => None,
    }
  }

  /// The detection, given back without a copy.
  pub fn into_detection(self) -> Option<Detection> {
    match self.parts.record {
      RefusedRecord::Detection(detection) => Some(*detection),
      RefusedRecord::Report(_) => None,
    }
  }

  /// The entry with this outcome, such as the one that a document recorded for the same record. The outcome is one
  /// that this version knows and that the record allows; an outcome of a newer version can need members that only
  /// its version builds, so an entry read with it is forwarded as read (ninth review). Any other outcome is refused,
  /// and the refusal comes back unchanged, so every entry it builds reads back.
  pub fn with_outcome(self, outcome: EntryOutcome) -> Result<DocumentEntry, DeriveOutcomeError> {
    let allowed = !matches!(outcome, EntryOutcome::Unrecognized(_))
      && match &self.parts.record {
        RefusedRecord::Report(report) => outcome.is_allowed_for_report(report),
        RefusedRecord::Detection(detection) => outcome.is_allowed_for_detection(detection),
      };
    if !allowed {
      return Err(self);
    }
    let RefusedParts { input, record } = *self.parts;
    let (report, detection) = match record {
      RefusedRecord::Report(report) => (Some(*report), None),
      RefusedRecord::Detection(detection) => (None, Some(*detection)),
    };
    Ok(DocumentEntry {
      input,
      outcome,
      report,
      detection,
      error: None,
      kept: KeptMembers::default(),
    })
  }
}

impl fmt::Display for DeriveOutcomeError {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(DERIVE_OUTCOME_MESSAGE)
  }
}

impl std::error::Error for DeriveOutcomeError {}
