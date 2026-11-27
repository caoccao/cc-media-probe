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

//! `DeriveOutcomeError`, the refusal of an entry whose outcome this version cannot derive, and the way back
//! (`api.md` section 12).

use std::fmt;

use super::constants::DERIVE_OUTCOME_MESSAGE;
use super::{DocumentEntry, EntryOutcome};
use crate::model::{Detection, MediaReport};
use crate::source::PortablePath;

/// A report or a detection whose outcome this version cannot derive, because only a value of a newer version decides
/// it: a report whose known statuses are all complete and whose record holds no first reason, beside a status of a
/// newer version, or a detection outcome of a newer version. A newer table may decide the outcome differently, so
/// this version does not guess. The refusal keeps the entry's parts behind one pointer, gives them back, and builds
/// the entry with an outcome that the caller gives, such as the one that a document recorded for the same record.
///
/// ```
/// use std::path::Path;
/// use cc_media_probe::Detection;
/// use cc_media_probe::document::{DeriveOutcomeError, DocumentEntry, EntryOutcome};
/// use cc_media_probe::source::PortablePath;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let usage = r#"{"bytes_read": 0, "peak_working_memory": 0, "retained_data": 0, "retained_payloads": 0,
///     "decompressed_output": 0, "work_units": 0, "companion_requests": 0, "sources_opened": 0}"#;
/// let text = format!(r#"{{"schema_version": "1.1", "outcome": "verified", "candidates": [], "usage": {usage}}}"#);
/// let detection: Detection = serde_json::from_str(&text)?;
/// let input = PortablePath::from_path(Path::new("clip.mp4"));
/// let refused: DeriveOutcomeError = DocumentEntry::from_detection(input, detection).err().ok_or("derived")?;
/// assert_eq!(refused.detection().map(|detection| detection.outcome().as_str()), Some("verified"));
/// let entry = refused.with_outcome(EntryOutcome::Partial)?;
/// assert_eq!(entry.outcome(), &EntryOutcome::Partial);
/// # Ok(())
/// # }
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct DeriveOutcomeError {
  parts: Box<RefusedParts>,
}

/// The parts of an entry that is still without its outcome: its input, and either the report that only a status of
/// a newer version decides or the detection whose outcome a newer version wrote, never both.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct RefusedParts {
  input: PortablePath,
  report: Option<MediaReport>,
  detection: Option<Detection>,
}

impl DeriveOutcomeError {
  /// The refusal of an entry of that input and that record, a report or a detection.
  pub(super) fn new(
    input: PortablePath,
    report: Option<MediaReport>,
    detection: Option<Detection>,
  ) -> DeriveOutcomeError {
    DeriveOutcomeError {
      parts: Box::new(RefusedParts {
        input,
        report,
        detection,
      }),
    }
  }

  /// The input that the refused entry was given.
  ///
  /// ```
  /// use cc_media_probe::document::DeriveOutcomeError;
  ///
  /// fn report(refused: &DeriveOutcomeError) {
  ///     eprintln!("{}: {refused}", refused.input().display());
  /// }
  /// ```
  #[must_use]
  pub fn input(&self) -> &PortablePath {
    &self.parts.input
  }

  /// The report that the refused entry was given, when it was given one.
  ///
  /// ```
  /// use cc_media_probe::document::DeriveOutcomeError;
  ///
  /// fn format_name(refused: &DeriveOutcomeError) -> Option<&str> {
  ///     refused.report().map(|report| report.format().name())
  /// }
  /// ```
  #[must_use]
  pub fn report(&self) -> Option<&MediaReport> {
    self.parts.report.as_ref()
  }

  /// The detection that the refused entry was given, when it was given one. The example of
  /// [`DeriveOutcomeError`] reads it.
  #[must_use]
  pub fn detection(&self) -> Option<&Detection> {
    self.parts.detection.as_ref()
  }

  /// The report, given back without a copy, as `String::from_utf8` gives back its bytes.
  ///
  /// ```
  /// use cc_media_probe::MediaReport;
  /// use cc_media_probe::document::DeriveOutcomeError;
  ///
  /// fn keep(refused: DeriveOutcomeError) -> Option<MediaReport> {
  ///     refused.into_report()
  /// }
  /// ```
  #[must_use]
  pub fn into_report(self) -> Option<MediaReport> {
    self.parts.report
  }

  /// The detection, given back without a copy.
  ///
  /// ```
  /// use cc_media_probe::Detection;
  /// use cc_media_probe::document::DeriveOutcomeError;
  ///
  /// fn keep(refused: DeriveOutcomeError) -> Option<Detection> {
  ///     refused.into_detection()
  /// }
  /// ```
  #[must_use]
  pub fn into_detection(self) -> Option<Detection> {
    self.parts.detection
  }

  /// The entry with this outcome, such as the one that a document recorded for the same record. The example of
  /// [`DeriveOutcomeError`] builds one.
  ///
  /// # Errors
  ///
  /// The outcome must be one that this version knows and that the record allows: `ok` or `partial` for a report,
  /// never `error`, and any of the three for a detection. An outcome of a newer version can need members that only
  /// its version builds, as `skipped` would need its `skip_reason`, so it is refused too; to pass on an entry of a
  /// newer outcome, a caller keeps the `DocumentEntry` that was read. A refusal gives this refusal back unchanged,
  /// so every entry that this function builds reads back.
  pub fn with_outcome(self, outcome: EntryOutcome) -> Result<DocumentEntry, DeriveOutcomeError> {
    let allowed = match (&self.parts.report, &self.parts.detection) {
      (Some(report), _) => outcome.allows_report(report),
      (None, Some(detection)) => outcome.allows_detection(detection),
      (None, None) => false,
    };
    if !allowed {
      return Err(self);
    }
    let RefusedParts {
      input,
      report,
      detection,
    } = *self.parts;
    Ok(DocumentEntry::holding(input, outcome, report, detection, None))
  }
}

impl fmt::Display for DeriveOutcomeError {
  /// Writes why the outcome must be given.
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(DERIVE_OUTCOME_MESSAGE)
  }
}

impl std::error::Error for DeriveOutcomeError {}
