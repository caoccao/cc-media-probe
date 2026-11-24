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
//! The limits and the parts that this version knows keep their declared order, a first reason has no detail, a part
//! that a limit stopped first has a first reason, and a first reason that names an exhausted limit finds it among
//! the exhausted ones (fourteenth review). A record that counts an omitted reason has a first reason for it to belong
//! to: both members are the record's own, so it checks them wherever it is read, alone, in a completeness, and in an
//! error record without a partial report, which the seventeenth review read with one reason omitted and none first.
//! A record that counts an omitted reference has exhausted the retained data and holds the first reason of a part
//! that finds references, which that reference left incomplete (`api.md` section 6.12): the eighteenth review read
//! one reference omitted beside an empty record in an `ok` document. The stub has no companions part, so its parts
//! that find references are discovery, the chapters and the attachments, and a part of a newer version may too. That
//! the part has the reason that names the retained data needs the kept reasons, so the completeness checks it
//! (`super::agreement`, nineteenth review).

#[cfg(feature = "serde")]
use super::constants::{LIMIT_ORDER, PART_ORDER};
use super::statuses::{Part, PartReason};
#[cfg(feature = "serde")]
use crate::constants::{LIMIT_RECORD_MESSAGE, OMITTED_REASONS_MESSAGE, OMITTED_REFERENCES_MESSAGE};
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
crate::neutral::checked_strict_traits!(LimitRecord from LimitRecordData);

/// The record's members as written.
#[cfg(feature = "serde")]
#[derive(serde::Serialize, serde::Deserialize)]
struct LimitRecordData {
  exhausted: Vec<Limit>,
  first_stopped: Vec<FirstStopped>,
  first_reasons: Vec<PartReason>,
  /// A decimal string, as the other two omitted counts are (`api.md` section 12). Required, as every member of the
  /// record is: the nineteenth review read a record of `exhausted` and `first_stopped` alone, written back with an
  /// empty `first_reasons` and both counts at `"0"`.
  #[serde(with = "crate::decimal")]
  omitted_reasons: u64,
  /// The external references found but not kept within the retained-data limit, a decimal string too.
  #[serde(with = "crate::decimal")]
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

  /// Each limit once in `exhausted`, the pairs naming exactly those limits in that order, so each once too, each
  /// part once in `first_reasons`, a first reason for an omitted count above 0 (seventeenth review), and the retained
  /// data with a part that finds references for an omitted reference (eighteenth review).
  fn try_from(read: LimitRecordData) -> Result<LimitRecord, InvalidDocument> {
    if read.omitted_reasons > 0 && read.first_reasons.is_empty() {
      return Err(InvalidDocument::new(OMITTED_REASONS_MESSAGE));
    }
    if read.omitted_references > 0 && !references_belong(&read) {
      return Err(InvalidDocument::new(OMITTED_REFERENCES_MESSAGE));
    }
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
    let parts: Vec<&Part> = read.first_reasons.iter().map(PartReason::part).collect();
    let ordered = in_declared_order(&read.exhausted, &LIMIT_ORDER) && in_declared_order(&parts, &PART_ORDER.each_ref());
    let stopped_parts_have_reasons = read.first_stopped.iter().all(|pair| parts.contains(&&pair.part));
    let reasons_agree = read
      .first_reasons
      .iter()
      .all(|first| first.reason().detail().is_none() && first.reason().finds_its_counter(&read.exhausted));
    if !(pairs_name_the_limits && limits_once && parts_once && ordered && stopped_parts_have_reasons && reasons_agree) {
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

/// Whether an omitted reference has a part to belong to: the retained data exhausted, and the first reason of a part
/// that finds references (eighteenth review). Which reason that part has, the completeness checks (nineteenth review).
#[cfg(feature = "serde")]
fn references_belong(read: &LimitRecordData) -> bool {
  read.exhausted.contains(&Limit::RetainedData) && read.first_reasons.iter().any(|first| finds_references(first.part()))
}

/// Whether the part finds external references, as `api.md` section 6.12 lists them: discovery, the chapters and the
/// attachments, the stub having no companions part, or a part of a newer version, which only that version knows.
#[cfg(feature = "serde")]
pub(super) fn finds_references(part: &Part) -> bool {
  matches!(
    part,
    Part::Discovery | Part::Chapters | Part::Attachments | Part::Unrecognized(_)
  )
}

/// Whether the values that this version knows come in the declared order, each after the one before it; a value of a
/// newer version may stand anywhere, since only its version knows its place.
#[cfg(feature = "serde")]
fn in_declared_order<Item: PartialEq>(items: &[Item], order: &[Item]) -> bool {
  let ranks: Vec<usize> = items
    .iter()
    .filter_map(|item| order.iter().position(|known| known == item))
    .collect();
  ranks.is_sorted_by(|earlier, later| earlier < later)
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

  /// The limits whose counters ran out, in the declared order.
  #[cfg(feature = "serde")]
  pub(crate) fn exhausted_limits(&self) -> &[Limit] {
    &self.exhausted
  }

  /// The first reason of each part, in the order of the parts.
  #[cfg(feature = "serde")]
  pub(crate) fn first_reasons(&self) -> &[PartReason] {
    &self.first_reasons
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
