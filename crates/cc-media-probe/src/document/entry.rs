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

//! `DocumentEntry`, the result for one input (`api.md` section 12; `json-output.md` section 3). An entry has no
//! index of its own: the document writes its position as its index (`entry_form`).

use super::DeriveOutcomeError;
use super::{EntryOutcome, SchemaVersion};
use crate::error::ErrorRecord;
use crate::form::KeptMembers;
use crate::model::{Detection, MediaReport};
use crate::source::PortablePath;

/// The result for one input: its input, its outcome, and the record that it holds. An entry of an outcome that this
/// version knows holds exactly one of a report, a detection and an error record, which allows its outcome. An entry
/// of an outcome that a newer version wrote can hold any of them, and keeps the members that this version does not
/// read, which it writes back after its own. Each record is held behind a pointer, so an entry costs a few words
/// whatever it holds, and a document of many entries holds no space for the records that they do not hold, as the
/// review of the third slice's second part asked.
///
/// ```
/// use std::path::Path;
/// use cc_media_probe::document::{DocumentEntry, EntryOutcome};
/// use cc_media_probe::source::PortablePath;
///
/// let error = cc_media_probe::probe_bytes(b"not a media file").err();
/// let entry = error.map(|error| {
///     DocumentEntry::from_error(PortablePath::from_path(Path::new("notes.txt")), error.to_record())
/// });
/// assert_eq!(entry.as_ref().map(DocumentEntry::outcome), Some(&EntryOutcome::Error));
/// assert!(entry.as_ref().is_some_and(|entry| entry.report().is_none() && entry.error().is_some()));
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct DocumentEntry {
  pub(super) input: PortablePath,
  pub(super) outcome: EntryOutcome,
  pub(super) report: Option<Box<MediaReport>>,
  pub(super) detection: Option<Box<Detection>>,
  pub(super) error: Option<Box<ErrorRecord>>,
  /// For an outcome that a newer version wrote, the members that this version does not read, as written; empty for
  /// an outcome that this version knows.
  pub(super) kept: KeptMembers,
}

impl DocumentEntry {
  /// An entry that holds a report. Its outcome is `ok` when the report is complete and `partial` when a status that
  /// this version knows records a shortfall or the record holds a first reason, whatever a status of a newer version
  /// says. A report of this version's own probe always gives its outcome.
  ///
  /// ```no_run
  /// use std::path::Path;
  /// use cc_media_probe::document::DocumentEntry;
  /// use cc_media_probe::source::PortablePath;
  ///
  /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
  /// let report = cc_media_probe::probe_file("movie.mkv")?;
  /// let entry = DocumentEntry::from_report(PortablePath::from_path(Path::new("movie.mkv")), report)?;
  /// println!("movie.mkv: {}", entry.outcome().as_str());
  /// # Ok(())
  /// # }
  /// ```
  ///
  /// # Errors
  ///
  /// A report read from a newer version that only a status of that version decides, whose known statuses are all
  /// complete and whose record holds no first reason, gives `DeriveOutcomeError`, which gives the report back and
  /// takes the outcome that the caller gives.
  pub fn from_report(input: PortablePath, report: MediaReport) -> Result<DocumentEntry, DeriveOutcomeError> {
    DocumentEntry::derived(input, EntryOutcome::of_report(&report), Some(report), None)
  }

  /// An entry that holds a detection, as the command-line contract specifies for `--detect`: `ok` for an identified
  /// format, `partial` for a family only, an ambiguous result or a hint only, and `error` for insufficient data or
  /// an unknown format.
  ///
  /// ```
  /// use std::path::Path;
  /// use cc_media_probe::document::{DocumentEntry, EntryOutcome};
  /// use cc_media_probe::source::PortablePath;
  ///
  /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
  /// let detection = cc_media_probe::detect_bytes(b"not a media file")?;
  /// let entry = DocumentEntry::from_detection(PortablePath::from_path(Path::new("notes.txt")), detection)?;
  /// assert_eq!(entry.outcome(), &EntryOutcome::Error);
  /// # Ok(())
  /// # }
  /// ```
  ///
  /// # Errors
  ///
  /// A detection whose outcome a newer version wrote gives `DeriveOutcomeError`, which gives the detection back and
  /// takes the outcome that the caller gives.
  pub fn from_detection(input: PortablePath, detection: Detection) -> Result<DocumentEntry, DeriveOutcomeError> {
    DocumentEntry::derived(input, EntryOutcome::of_detection(&detection), None, Some(detection))
  }

  /// An entry that holds an error record, whose outcome is `error`. A source that fails during detection is such a
  /// record. The example of [`DocumentEntry`] builds one.
  #[must_use]
  pub fn from_error(input: PortablePath, record: ErrorRecord) -> DocumentEntry {
    DocumentEntry::holding(input, EntryOutcome::Error, None, None, Some(record))
  }

  /// The input, as the caller named it.
  ///
  /// ```
  /// use cc_media_probe::document::DocumentEntry;
  ///
  /// fn name(entry: &DocumentEntry) -> &str {
  ///     entry.input().display()
  /// }
  /// ```
  #[must_use]
  pub fn input(&self) -> &PortablePath {
    &self.input
  }

  /// The outcome, which follows from the record that the entry holds, or one that a newer version wrote. The example
  /// of [`DocumentEntry`] reads it.
  #[must_use]
  pub fn outcome(&self) -> &EntryOutcome {
    &self.outcome
  }

  /// The report, for a probe.
  ///
  /// ```
  /// use cc_media_probe::document::DocumentEntry;
  ///
  /// fn stream_count(entry: &DocumentEntry) -> Option<usize> {
  ///     entry.report().map(|report| report.streams().len())
  /// }
  /// ```
  #[must_use]
  pub fn report(&self) -> Option<&MediaReport> {
    self.report.as_deref()
  }

  /// The detection, for an entry of `--detect` in the command-line contract.
  ///
  /// ```
  /// use cc_media_probe::document::DocumentEntry;
  ///
  /// fn detected(entry: &DocumentEntry) -> Option<&str> {
  ///     entry.detection().map(|detection| detection.outcome().as_str())
  /// }
  /// ```
  #[must_use]
  pub fn detection(&self) -> Option<&Detection> {
    self.detection.as_deref()
  }

  /// The error record, for an input that failed. The example of [`DocumentEntry`] reads it.
  #[must_use]
  pub fn error(&self) -> Option<&ErrorRecord> {
    self.error.as_deref()
  }

  /// The entry of the outcome that its report or its detection warrants, or the refusal of one that warrants none
  /// that this version can derive.
  fn derived(
    input: PortablePath,
    warranted: Option<EntryOutcome>,
    report: Option<MediaReport>,
    detection: Option<Detection>,
  ) -> Result<DocumentEntry, DeriveOutcomeError> {
    match warranted {
      Some(outcome) => Ok(DocumentEntry::holding(input, outcome, report, detection, None)),
      None => Err(DeriveOutcomeError::new(input, report, detection)),
    }
  }

  /// An entry of that outcome that holds those records, each behind its pointer, and keeps no member.
  pub(super) fn holding(
    input: PortablePath,
    outcome: EntryOutcome,
    report: Option<MediaReport>,
    detection: Option<Detection>,
    error: Option<ErrorRecord>,
  ) -> DocumentEntry {
    DocumentEntry {
      input,
      outcome,
      report: report.map(Box::new),
      detection: detection.map(Box::new),
      error: error.map(Box::new),
      kept: KeptMembers::default(),
    }
  }

  /// Gives every record that the entry holds that version, down to the partial report of an error record, as a
  /// document gives each record it holds (`api.md` section 12). Each record keeps its place, so no record is moved
  /// or allocated again.
  pub(super) fn set_schema_version(&mut self, version: SchemaVersion) {
    if let Some(report) = &mut self.report {
      report.set_schema_version(version);
    }
    if let Some(detection) = &mut self.detection {
      detection.set_schema_version(version);
    }
    if let Some(record) = &mut self.error {
      record.set_schema_version(version);
    }
  }
}

#[cfg(test)]
mod tests {
  use super::DocumentEntry;
  use crate::error::ErrorRecord;
  use crate::model::{Detection, MediaReport};

  #[test]
  fn an_entry_holds_no_record_in_its_own_space() {
    // The review of the third slice's second part measured 2,096 bytes for an entry that held its three records in
    // place. Behind their pointers, an entry is smaller than the smallest of them.
    let smallest = size_of::<MediaReport>()
      .min(size_of::<Detection>())
      .min(size_of::<ErrorRecord>());
    assert!(size_of::<DocumentEntry>() < smallest, "{}", size_of::<DocumentEntry>());
  }
}
