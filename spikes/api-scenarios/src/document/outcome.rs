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

//! An entry's outcome, `api.md` section 12, and the outcome that a record warrants by the table of plan section 8.9.

use crate::constants::{ERROR_NAME, OK_NAME, PARTIAL_NAME};
use crate::model::{Detection, DetectionOutcome, MediaReport, UnrecognizedValue};

/// What an entry holds. Serialized as its stable name.
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(into = "String", from = "String")]
#[non_exhaustive]
pub enum EntryOutcome {
  /// A complete report, or an identified format.
  Ok,
  /// An incomplete report, or a detection of a family only, an ambiguous result or a hint only.
  Partial,
  /// An error record, or a detection of insufficient data or an unknown format.
  Error,
  /// A value that a newer version wrote.
  Unrecognized(UnrecognizedValue),
}

impl EntryOutcome {
  /// The serialized name, or the text of an unrecognized value.
  pub fn as_str(&self) -> &str {
    match self {
      EntryOutcome::Ok => OK_NAME,
      EntryOutcome::Partial => PARTIAL_NAME,
      EntryOutcome::Error => ERROR_NAME,
      EntryOutcome::Unrecognized(value) => value.as_str(),
    }
  }

  /// The outcome that a report warrants: `ok` when it is complete. `None` for a report whose statuses include one
  /// from a newer version, which a newer table may decide differently.
  pub(super) fn of_report(report: &MediaReport) -> Option<EntryOutcome> {
    let completeness = report.completeness();
    completeness.is_known().then(|| {
      if completeness.is_complete() {
        EntryOutcome::Ok
      } else {
        EntryOutcome::Partial
      }
    })
  }

  /// The outcome that a detection warrants, by the `--detect` rows of plan section 8.9; `None` for a detection
  /// outcome that a newer version wrote.
  pub(super) fn of_detection(detection: &Detection) -> Option<EntryOutcome> {
    match detection.outcome() {
      DetectionOutcome::Identified => Some(EntryOutcome::Ok),
      DetectionOutcome::FamilyOnly | DetectionOutcome::Ambiguous | DetectionOutcome::HintOnly => {
        Some(EntryOutcome::Partial)
      }
      DetectionOutcome::InsufficientData | DetectionOutcome::Unknown => Some(EntryOutcome::Error),
      DetectionOutcome::Unrecognized(_) => None,
    }
  }
}

impl From<String> for EntryOutcome {
  fn from(name: String) -> EntryOutcome {
    match name.as_str() {
      OK_NAME => EntryOutcome::Ok,
      PARTIAL_NAME => EntryOutcome::Partial,
      ERROR_NAME => EntryOutcome::Error,
      _ => EntryOutcome::Unrecognized(UnrecognizedValue::new(name)),
    }
  }
}

impl From<EntryOutcome> for String {
  fn from(outcome: EntryOutcome) -> String {
    outcome.as_str().to_owned()
  }
}
