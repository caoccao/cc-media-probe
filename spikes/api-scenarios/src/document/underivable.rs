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
use crate::constants::UNDERIVABLE_OUTCOME_MESSAGE;
use crate::kept::KeptMembers;
use crate::model::{Detection, MediaReport};
use crate::source::PortablePath;

/// A report or detection whose outcome this version cannot derive, because only a value of a newer version decides
/// it. It keeps the entry's parts, behind one pointer, so the outcome can be given.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct UnderivableOutcome {
  parts: Box<UnderivedParts>,
}

/// The parts of an entry that is still without its outcome.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct UnderivedParts {
  input: PortablePath,
  record: UnderivedRecord,
}

/// The record of such an entry.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(super) enum UnderivedRecord {
  /// A report that only a status of a newer version decides.
  Report(Box<MediaReport>),
  /// A detection whose outcome a newer version wrote.
  Detection(Detection),
}

impl UnderivableOutcome {
  /// The refusal of an entry of these parts.
  pub(super) fn new(input: PortablePath, record: UnderivedRecord) -> UnderivableOutcome {
    UnderivableOutcome {
      parts: Box::new(UnderivedParts { input, record }),
    }
  }

  /// The entry with this outcome, such as the one that a document recorded for the same record. The outcome is one
  /// that this version knows and that the record allows; an outcome of a newer version can need members that only
  /// its version builds, so an entry read with it is forwarded as read (ninth review). Any other outcome is refused,
  /// and the refusal comes back unchanged, so every entry it builds reads back.
  pub fn with_outcome(self, outcome: EntryOutcome) -> Result<DocumentEntry, UnderivableOutcome> {
    let allowed = !matches!(outcome, EntryOutcome::Unrecognized(_))
      && match &self.parts.record {
        UnderivedRecord::Report(report) => outcome.is_allowed_for_report(report),
        UnderivedRecord::Detection(detection) => outcome.is_allowed_for_detection(detection),
      };
    if !allowed {
      return Err(self);
    }
    let UnderivedParts { input, record } = *self.parts;
    let (report, detection) = match record {
      UnderivedRecord::Report(report) => (Some(*report), None),
      UnderivedRecord::Detection(detection) => (None, Some(detection)),
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

impl fmt::Display for UnderivableOutcome {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(UNDERIVABLE_OUTCOME_MESSAGE)
  }
}

impl std::error::Error for UnderivableOutcome {}
