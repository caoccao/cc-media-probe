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

//! The fixed-size record of `api.md` section 5.3, with the accessors that the scenarios and the tests read. The
//! library's record has a fixed size, one slot per counter and per part; the stub keeps short lists.
//!
//! Serialized, it is the record of `json-output.md` section 7.2: the exhausted limits, the first-stopped pairs as
//! records of `limit` and `part`, and the first reasons. Each array names each limit or part once, and the pairs
//! name exactly the exhausted limits, in their order; two limits may first stop the same part (thirteenth review).

use super::statuses::{Part, PartReason};
#[cfg(feature = "serde")]
use crate::constants::LIMIT_RECORD_MESSAGE;
#[cfg(feature = "serde")]
use crate::document::InvalidDocument;
use crate::limits::Limit;

/// The fixed-size record.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(into = "LimitRecordData"))]
pub struct LimitRecord {
  exhausted: Vec<Limit>,
  first_stopped: Vec<(Limit, Part)>,
  pub(super) first_reasons: Vec<PartReason>,
  omitted_reasons: u64,
  omitted_references: u64,
}

#[cfg(feature = "serde")]
crate::neutral::checked_record_traits!(LimitRecord from LimitRecordData);

/// The record's members as written.
#[cfg(feature = "serde")]
#[derive(serde::Serialize, serde::Deserialize)]
struct LimitRecordData {
  exhausted: Vec<Limit>,
  first_stopped: Vec<FirstStopped>,
  #[serde(default)]
  first_reasons: Vec<PartReason>,
  /// A decimal string, as the other two omitted counts are (`api.md` section 12).
  #[serde(default, with = "crate::decimal")]
  omitted_reasons: u64,
  /// The external references found but not kept within the retained-data limit, a decimal string too.
  #[serde(default, with = "crate::decimal")]
  omitted_references: u64,
}

/// A limit and the part that it stopped first.
#[cfg(feature = "serde")]
#[derive(serde::Serialize, serde::Deserialize)]
struct FirstStopped {
  limit: Limit,
  part: Part,
}

#[cfg(feature = "serde")]
impl TryFrom<LimitRecordData> for LimitRecord {
  type Error = InvalidDocument;

  /// Each limit once in `exhausted`, the pairs naming exactly those limits in that order, so each once too, and each
  /// part once in `first_reasons`.
  fn try_from(read: LimitRecordData) -> Result<LimitRecord, InvalidDocument> {
    let pairs_name_the_limits = read.exhausted.len() == read.first_stopped.len()
      && read
        .exhausted
        .iter()
        .zip(&read.first_stopped)
        .all(|(limit, pair)| *limit == pair.limit);
    let repeats = |position: usize, earlier: &[Limit]| earlier[..position].contains(&earlier[position]);
    let limits_once = !(0..read.exhausted.len()).any(|position| repeats(position, &read.exhausted));
    let parts_once = !(0..read.first_reasons.len()).any(|position| {
      read.first_reasons[..position]
        .iter()
        .any(|earlier| earlier.part() == read.first_reasons[position].part())
    });
    if !(pairs_name_the_limits && limits_once && parts_once) {
      return Err(InvalidDocument::new(LIMIT_RECORD_MESSAGE));
    }
    Ok(LimitRecord {
      exhausted: read.exhausted,
      first_stopped: read
        .first_stopped
        .into_iter()
        .map(|pair| (pair.limit, pair.part))
        .collect(),
      first_reasons: read.first_reasons,
      omitted_reasons: read.omitted_reasons,
      omitted_references: read.omitted_references,
    })
  }
}

#[cfg(feature = "serde")]
impl From<LimitRecord> for LimitRecordData {
  fn from(record: LimitRecord) -> LimitRecordData {
    LimitRecordData {
      exhausted: record.exhausted,
      first_stopped: record
        .first_stopped
        .into_iter()
        .map(|(limit, part)| FirstStopped { limit, part })
        .collect(),
      first_reasons: record.first_reasons,
      omitted_reasons: record.omitted_reasons,
      omitted_references: record.omitted_references,
    }
  }
}

impl LimitRecord {
  /// The first reason recorded against the part, kept whatever the limits, without its detail text.
  pub fn first_reason(&self, part: &Part) -> Option<&PartReason> {
    self.first_reasons.iter().find(|first| first.part() == part)
  }

  /// The reasons recorded but not kept in `Completeness::reasons()`.
  pub fn omitted_reasons(&self) -> u64 {
    self.omitted_reasons
  }

  /// The external references found but not kept in `MediaReport::external_references()`.
  pub fn omitted_references(&self) -> u64 {
    self.omitted_references
  }

  /// Whether the counter ran out.
  pub fn exhausted(&self, limit: &Limit) -> bool {
    self.exhausted.contains(limit)
  }

  /// The part that the counter stopped first, such as `Part::Detection` for a read limit that ended detection.
  pub fn first_stopped(&self, limit: &Limit) -> Option<&Part> {
    self
      .first_stopped
      .iter()
      .find(|(stopped, _)| stopped == limit)
      .map(|(_, part)| part)
  }
}
