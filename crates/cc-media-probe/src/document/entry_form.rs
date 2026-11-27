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

//! An entry's serialized form (`api.md` section 12; `json-output.md` section 3). A document writes each entry with
//! its position as its `index`, and its input as the display string, as in `"input": "movie.mkv"`, with the flag and
//! the raw form beside it for a lossy path. Reading checks the entry as soon as it is read, as the maintainer chose
//! for the second part of the third slice: its index is its position, an entry of a known outcome holds exactly one
//! record, that record allows the outcome, and the input's three members describe one path. An entry of a newer
//! outcome keeps the members that it does not read, held until its outcome is known, and an entry of a known outcome
//! passes them by as they come (`crate::form::WithKept`).

use serde::{Deserialize, Serialize, Serializer};

use super::constants::{ENTRY_WITHOUT_ONE_RECORD_MESSAGE, OUTCOME_DISAGREES_MESSAGE, OUTCOME_MEMBER, POSITION_MESSAGE};
use super::{DocumentEntry, EntryOutcome};
use crate::error::ErrorRecord;
use crate::form::{KeptMembers, Outcome, WithKept};
use crate::model::{Detection, MediaReport};
use crate::source::{PortablePath, RawPath};

/// An entry's members, as written. The index is a `u64` on every platform, so a writer on a 32-bit target counts
/// beyond 2^32 entries as far as 2^53 - 1.
#[derive(Serialize)]
struct EntryMembers<'entry> {
  index: u64,
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
  /// The members of an entry of a newer outcome that this version does not read, written back as read.
  #[serde(flatten)]
  kept: &'entry KeptMembers,
}

/// An entry with its position in the document, which is written as its index.
pub(super) struct PositionedEntry<'entry> {
  pub(super) index: u64,
  pub(super) entry: &'entry DocumentEntry,
}

impl Serialize for PositionedEntry<'_> {
  fn serialize<Output: Serializer>(&self, serializer: Output) -> Result<Output::Ok, Output::Error> {
    let entry = self.entry;
    let (input, input_lossy, input_raw) = entry.input.members();
    EntryMembers {
      index: self.index,
      input,
      input_lossy,
      input_raw,
      outcome: &entry.outcome,
      report: entry.report.as_deref(),
      detection: entry.detection.as_deref(),
      error: entry.error.as_deref(),
      kept: &entry.kept,
    }
    .serialize(serializer)
  }
}

/// An entry's members as read, before its rules are checked. Each record that it holds is a root record nested in
/// the document, which compares its version with the document's before it decodes its body.
#[derive(Deserialize)]
pub(super) struct EntryBody {
  index: u64,
  input: String,
  input_lossy: Option<bool>,
  input_raw: Option<RawPath>,
  outcome: EntryOutcome,
  report: Option<MediaReport>,
  detection: Option<Detection>,
  error: Option<ErrorRecord>,
}

/// An entry of a newer outcome keeps every member that it does not read, since its outcome can need a member that
/// only the newer version knows; an entry of a known outcome passes them by.
impl Outcome for EntryBody {
  const MEMBER: &'static str = OUTCOME_MEMBER;

  fn is_newer(text: &str) -> bool {
    EntryOutcome::from_known_text(text).is_none()
  }
}

impl WithKept<EntryBody> {
  /// The entry at that position of the results, or the message of the rule that it breaks.
  pub(super) fn into_entry(self, position: u64) -> Result<DocumentEntry, &'static str> {
    let WithKept { data: body, kept } = self;
    if body.index != position {
      return Err(POSITION_MESSAGE);
    }
    if !matches!(body.outcome, EntryOutcome::Unrecognized(_)) {
      let allowed = match (&body.report, &body.detection, &body.error) {
        (Some(report), None, None) => body.outcome.allows_report(report),
        (None, Some(detection), None) => body.outcome.allows_detection(detection),
        (None, None, Some(_)) => body.outcome == EntryOutcome::Error,
        _ => return Err(ENTRY_WITHOUT_ONE_RECORD_MESSAGE),
      };
      if !allowed {
        return Err(OUTCOME_DISAGREES_MESSAGE);
      }
    }
    let input = PortablePath::from_members(body.input, body.input_lossy, body.input_raw)?;
    Ok(DocumentEntry {
      kept,
      ..DocumentEntry::holding(input, body.outcome, body.report, body.detection, body.error)
    })
  }
}
