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

//! An entry's serialized form, `api.md` section 12. A document writes each entry with its position as its `index`,
//! and its input as the display string, as in `"input": "movie.mkv"`, with the flag and the raw form beside it for a
//! lossy path. Reading checks that each index is its entry's position, that an entry of a known outcome holds
//! exactly one record, and that the record allows the outcome.

use serde::{Deserialize, Serialize, Serializer};

use super::entry::DocumentEntry;
use super::invalid::InvalidDocument;
use super::outcome::EntryOutcome;
use crate::constants::{ENTRY_WITHOUT_ONE_RECORD_MESSAGE, OUTCOME_DISAGREES_MESSAGE, POSITION_MESSAGE};
use crate::error::ErrorRecord;
use crate::model::{Detection, MediaReport};
use crate::source::{PortablePath, RawPath};

/// An entry's members, as written.
#[derive(Serialize)]
struct EntryMembers<'entry> {
  index: usize,
  input: &'entry str,
  #[serde(skip_serializing_if = "Option::is_none")]
  input_lossy: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  input_raw: Option<&'entry RawPath>,
  outcome: &'entry EntryOutcome,
  #[serde(skip_serializing_if = "Option::is_none")]
  report: Option<&'entry MediaReport>,
  #[serde(skip_serializing_if = "Option::is_none")]
  detection: Option<&'entry Detection>,
  #[serde(skip_serializing_if = "Option::is_none")]
  error: Option<&'entry ErrorRecord>,
}

/// An entry with its position in the document, which is written as its index.
pub(super) struct PositionedEntry<'entry> {
  pub(super) index: usize,
  pub(super) entry: &'entry DocumentEntry,
}

impl Serialize for PositionedEntry<'_> {
  fn serialize<Output: Serializer>(&self, serializer: Output) -> Result<Output::Ok, Output::Error> {
    let entry = self.entry;
    EntryMembers {
      index: self.index,
      input: entry.input.display(),
      input_lossy: entry.input.is_lossy().then_some(true),
      input_raw: entry.input.raw(),
      outcome: &entry.outcome,
      report: entry.report.as_ref(),
      detection: entry.detection.as_ref(),
      error: entry.error.as_ref(),
    }
    .serialize(serializer)
  }
}

/// The results of a document held in memory, each entry with its position as its index.
pub(super) fn serialize_results<Output: Serializer>(
  results: &[DocumentEntry],
  serializer: Output,
) -> Result<Output::Ok, Output::Error> {
  serializer.collect_seq(
    results
      .iter()
      .enumerate()
      .map(|(index, entry)| PositionedEntry { index, entry }),
  )
}

/// An entry's members as read, before its rules are checked.
#[derive(Deserialize)]
pub(super) struct EntryData {
  index: usize,
  input: String,
  input_lossy: Option<bool>,
  input_raw: Option<RawPath>,
  outcome: EntryOutcome,
  report: Option<MediaReport>,
  detection: Option<Detection>,
  error: Option<ErrorRecord>,
}

impl EntryData {
  /// Whether the entry's one record allows its outcome, by the table of plan section 8.9.
  fn allows_outcome(&self) -> bool {
    match (&self.report, &self.detection, &self.error) {
      (Some(report), None, None) => self.outcome.is_allowed_for_report(report),
      (None, Some(detection), None) => self.outcome.is_allowed_for_detection(detection),
      (None, None, Some(_)) => self.outcome == EntryOutcome::Error,
      _ => false,
    }
  }

  /// The entry at that position of the results. Its index is the position; an entry of an outcome this version
  /// knows holds exactly one record, which allows the outcome; and its input describes one path.
  pub(super) fn into_entry(self, position: usize) -> Result<DocumentEntry, InvalidDocument> {
    if self.index != position {
      return Err(InvalidDocument::new(POSITION_MESSAGE));
    }
    if !matches!(self.outcome, EntryOutcome::Unrecognized(_)) {
      let records = [self.report.is_some(), self.detection.is_some(), self.error.is_some()];
      if records.into_iter().filter(|present| *present).count() != 1 {
        return Err(InvalidDocument::new(ENTRY_WITHOUT_ONE_RECORD_MESSAGE));
      }
      if !self.allows_outcome() {
        return Err(InvalidDocument::new(OUTCOME_DISAGREES_MESSAGE));
      }
    }
    Ok(DocumentEntry {
      input: PortablePath::from_members(self.input, self.input_lossy, self.input_raw)?,
      outcome: self.outcome,
      report: self.report,
      detection: self.detection,
      error: self.error,
    })
  }
}
