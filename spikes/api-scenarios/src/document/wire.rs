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

//! An entry's serialized form, `api.md` section 12: its input is the display string, as in `"input": "movie.mkv"`,
//! with the flag and the raw form beside it for a lossy path. Reading checks that an entry of a known outcome holds
//! exactly one record, and that the record warrants the outcome.

use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::entry::DocumentEntry;
use super::invalid::InvalidDocument;
use super::outcome::EntryOutcome;
use crate::constants::{ENTRY_WITHOUT_ONE_RECORD_MESSAGE, OUTCOME_DISAGREES_MESSAGE};
use crate::error::ErrorRecord;
use crate::model::{Detection, MediaReport};
use crate::source::{PortablePath, RawPath, is_not_lossy};

/// An entry's members, as written.
#[derive(Serialize)]
struct EntryMembers<'entry> {
  index: usize,
  input: &'entry str,
  #[serde(skip_serializing_if = "is_not_lossy")]
  input_lossy: bool,
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

impl Serialize for DocumentEntry {
  fn serialize<Output: Serializer>(&self, serializer: Output) -> Result<Output::Ok, Output::Error> {
    EntryMembers {
      index: self.index,
      input: self.input.display(),
      input_lossy: self.input.is_lossy(),
      input_raw: self.input.raw(),
      outcome: &self.outcome,
      report: self.report.as_ref(),
      detection: self.detection.as_ref(),
      error: self.error.as_ref(),
    }
    .serialize(serializer)
  }
}

/// An entry's members as read, before its rules are checked.
#[derive(Deserialize)]
struct EntryData {
  index: usize,
  input: String,
  #[serde(default)]
  input_lossy: bool,
  input_raw: Option<RawPath>,
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
      (Some(report), None, None) => EntryOutcome::of_report(report),
      (None, Some(detection), None) => EntryOutcome::of_detection(detection),
      (None, None, Some(_)) => Some(EntryOutcome::Error),
      _ => None,
    }
  }
}

impl TryFrom<EntryData> for DocumentEntry {
  type Error = InvalidDocument;

  /// An entry of an outcome this version knows holds exactly one record, and that record warrants the outcome. A
  /// lossy input has its raw form beside it.
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
      input: PortablePath::from_members(data.input, data.input_lossy, data.input_raw)?,
      outcome: data.outcome,
      report: data.report,
      detection: data.detection,
      error: data.error,
    })
  }
}

impl<'input> Deserialize<'input> for DocumentEntry {
  fn deserialize<Input: Deserializer<'input>>(deserializer: Input) -> Result<DocumentEntry, Input::Error> {
    DocumentEntry::try_from(EntryData::deserialize(deserializer)?).map_err(Input::Error::custom)
  }
}
