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

//! A document's entries, `api.md` section 12: what each holds and the outcome it warrants.

use super::invalid::InvalidDocument;
use super::version::SchemaVersion;
use crate::constants::{
  ENTRY_WITHOUT_ONE_RECORD_MESSAGE, ERROR_NAME, OK_NAME, OUTCOME_DISAGREES_MESSAGE, PARTIAL_NAME,
};
use crate::error::ErrorRecord;
use crate::model::{Detection, DetectionOutcome, MediaReport, UnrecognizedValue};
use crate::source::PortablePath;

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

  /// The outcome that a report warrants: `ok` when it is complete.
  fn of_report(report: &MediaReport) -> EntryOutcome {
    if report.completeness().is_complete() {
      EntryOutcome::Ok
    } else {
      EntryOutcome::Partial
    }
  }

  /// The outcome that a detection warrants, by the `--detect` rows of plan section 8.9; `None` for a detection
  /// outcome that a newer version wrote.
  fn of_detection(detection: &Detection) -> Option<EntryOutcome> {
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

/// The result for one input.
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(try_from = "EntryData")]
pub struct DocumentEntry {
  index: usize,
  input: PortablePath,
  outcome: EntryOutcome,
  #[serde(skip_serializing_if = "Option::is_none")]
  report: Option<MediaReport>,
  #[serde(skip_serializing_if = "Option::is_none")]
  detection: Option<Detection>,
  #[serde(skip_serializing_if = "Option::is_none")]
  error: Option<ErrorRecord>,
}

/// An entry as read, before its rules are checked.
#[derive(serde::Deserialize)]
struct EntryData {
  index: usize,
  input: PortablePath,
  outcome: EntryOutcome,
  report: Option<MediaReport>,
  detection: Option<Detection>,
  error: Option<ErrorRecord>,
}

impl EntryData {
  /// The outcome that the entry's one record warrants, where this version can tell. A report whose statuses include
  /// one from a newer version, and a detection outcome from a newer version, keep the outcome that was written.
  fn warranted_outcome(&self) -> Option<EntryOutcome> {
    match (&self.report, &self.detection, &self.error) {
      (Some(report), None, None) => report
        .completeness()
        .is_known()
        .then(|| EntryOutcome::of_report(report)),
      (None, Some(detection), None) => EntryOutcome::of_detection(detection),
      (None, None, Some(_)) => Some(EntryOutcome::Error),
      _ => None,
    }
  }
}

impl TryFrom<EntryData> for DocumentEntry {
  type Error = InvalidDocument;

  /// An entry of an outcome this version knows holds exactly one record, and that record warrants the outcome.
  fn try_from(data: EntryData) -> Result<DocumentEntry, InvalidDocument> {
    if !matches!(data.outcome, EntryOutcome::Unrecognized(_)) {
      let records = [data.report.is_some(), data.detection.is_some(), data.error.is_some()];
      if records.into_iter().filter(|present| *present).count() != 1 {
        return Err(InvalidDocument::new(ENTRY_WITHOUT_ONE_RECORD_MESSAGE));
      }
      if data
        .warranted_outcome()
        .is_some_and(|warranted| warranted != data.outcome)
      {
        return Err(InvalidDocument::new(OUTCOME_DISAGREES_MESSAGE));
      }
    }
    Ok(DocumentEntry {
      index: data.index,
      input: data.input,
      outcome: data.outcome,
      report: data.report,
      detection: data.detection,
      error: data.error,
    })
  }
}

impl DocumentEntry {
  /// An entry that holds a report; its outcome is `ok` or `partial`.
  pub fn from_report(index: usize, input: PortablePath, report: MediaReport) -> DocumentEntry {
    DocumentEntry {
      index,
      input,
      outcome: EntryOutcome::of_report(&report),
      report: Some(report),
      detection: None,
      error: None,
    }
  }

  /// An entry that holds a detection; its outcome follows the `--detect` rows of plan section 8.9.
  pub fn from_detection(index: usize, input: PortablePath, detection: Detection) -> DocumentEntry {
    DocumentEntry {
      index,
      input,
      outcome: EntryOutcome::of_detection(&detection).unwrap_or(EntryOutcome::Error),
      report: None,
      detection: Some(detection),
      error: None,
    }
  }

  /// An entry that holds an error record; its outcome is `error`.
  pub fn from_error(index: usize, input: PortablePath, record: ErrorRecord) -> DocumentEntry {
    DocumentEntry {
      index,
      input,
      outcome: EntryOutcome::Error,
      report: None,
      detection: None,
      error: Some(record),
    }
  }

  /// The input's position.
  pub fn index(&self) -> usize {
    self.index
  }

  /// The input.
  pub fn input(&self) -> &PortablePath {
    &self.input
  }

  /// What the entry holds.
  pub fn outcome(&self) -> &EntryOutcome {
    &self.outcome
  }

  /// The report, for a probe.
  pub fn report(&self) -> Option<&MediaReport> {
    self.report.as_ref()
  }

  /// The detection, for `--detect`.
  pub fn detection(&self) -> Option<&Detection> {
    self.detection.as_ref()
  }

  /// The error record.
  pub fn error(&self) -> Option<&ErrorRecord> {
    self.error.as_ref()
  }

  /// The schema version of the nested record, which the document checks against its own.
  pub(super) fn nested_version(&self) -> Option<SchemaVersion> {
    let report = self.report.as_ref().map(MediaReport::schema_version);
    let detection = self.detection.as_ref().map(Detection::schema_version);
    report
      .or(detection)
      .or(self.error.as_ref().map(ErrorRecord::schema_version))
  }
}
