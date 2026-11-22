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

//! A document's entries, `api.md` section 12: what each holds, and the constructors that derive its outcome.

use super::outcome::EntryOutcome;
use super::underivable::{UnderivableOutcome, UnderivedRecord};
use super::version::SchemaVersion;
use crate::error::ErrorRecord;
use crate::model::{Detection, MediaReport};
use crate::source::PortablePath;

/// The result for one input. Its serialized form is in `wire.rs`.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct DocumentEntry {
  pub(super) index: usize,
  pub(super) input: PortablePath,
  pub(super) outcome: EntryOutcome,
  pub(super) report: Option<MediaReport>,
  pub(super) detection: Option<Detection>,
  pub(super) error: Option<ErrorRecord>,
}

impl DocumentEntry {
  /// An entry that holds a report; its outcome is `ok` or `partial`. A report whose statuses include one from a
  /// newer version is refused, and the refusal can take the outcome that was recorded for it.
  pub fn from_report(
    index: usize,
    input: PortablePath,
    report: MediaReport,
  ) -> Result<DocumentEntry, UnderivableOutcome> {
    match EntryOutcome::of_report(&report) {
      Some(outcome) => Ok(DocumentEntry {
        index,
        input,
        outcome,
        report: Some(report),
        detection: None,
        error: None,
      }),
      None => Err(UnderivableOutcome::new(
        index,
        input,
        UnderivedRecord::Report(Box::new(report)),
      )),
    }
  }

  /// An entry that holds a detection; its outcome follows the `--detect` rows of plan section 8.9. A detection
  /// outcome from a newer version is refused, and the refusal can take the outcome that was recorded for it.
  pub fn from_detection(
    index: usize,
    input: PortablePath,
    detection: Detection,
  ) -> Result<DocumentEntry, UnderivableOutcome> {
    match EntryOutcome::of_detection(&detection) {
      Some(outcome) => Ok(DocumentEntry {
        index,
        input,
        outcome,
        report: None,
        detection: Some(detection),
        error: None,
      }),
      None => Err(UnderivableOutcome::new(
        index,
        input,
        UnderivedRecord::Detection(detection),
      )),
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

  /// The schema version of every nested record, which the document checks against its own. An entry of a newer
  /// outcome can hold more than one.
  pub(super) fn nested_versions(&self) -> impl Iterator<Item = SchemaVersion> + use<> {
    let report = self.report.as_ref().map(MediaReport::schema_version);
    let detection = self.detection.as_ref().map(Detection::schema_version);
    let error = self.error.as_ref().map(ErrorRecord::schema_version);
    [report, detection, error].into_iter().flatten()
  }

  /// The entry with every nested record at the document's version, which a document gives each record it holds.
  pub(super) fn with_schema_version(self, version: SchemaVersion) -> DocumentEntry {
    DocumentEntry {
      report: self.report.map(|report| report.with_schema_version(version)),
      detection: self.detection.map(|detection| detection.with_schema_version(version)),
      error: self.error.map(|record| record.with_schema_version(version)),
      ..self
    }
  }
}
