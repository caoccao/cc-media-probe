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

//! `LimitRecord`, the fixed-size record of a probe's stops (`api.md` section 5.3; plan section 7.5).

#[cfg(feature = "serde")]
use std::collections::{HashMap, HashSet};

#[cfg(feature = "serde")]
use super::Reason;
#[cfg(feature = "serde")]
use super::constants::{
  DETECTION_LIMIT_MESSAGE, LIMIT_ORDER, LIMIT_RECORD_MESSAGE, OMITTED_REASONS_MESSAGE, OMITTED_REFERENCES_MESSAGE,
  OMITTED_STORAGE_MESSAGE, PART_ORDER,
};
use super::{Part, PartReason};
use crate::Limit;
use crate::form_record::form_record;

form_record! {
  /// The fixed-size record that every limit leaves in place: which counters were exhausted, the part that each
  /// stopped first, the first reason of each part, and the four omitted counts. It exists under any limits and is
  /// never charged to a counter (plan section 7.5).
  ///
  /// ```
  /// use cc_media_probe::Limit;
  /// use cc_media_probe::completeness::{LimitRecord, Part};
  ///
  /// fn summary(record: &LimitRecord) -> Vec<String> {
  ///     let mut lines = Vec::new();
  ///     if record.exhausted(&Limit::BytesRead) {
  ///         let part = record.first_stopped(&Limit::BytesRead).map_or("nothing", |part| part.as_str());
  ///         lines.push(format!("the read limit stopped {part} first"));
  ///     }
  ///     if let Some(first) = record.first_reason(&Part::Discovery) {
  ///         lines.push(format!("discovery: {}", first.reason().kind().as_str()));
  ///     }
  ///     lines.push(format!(
  ///         "omitted: {} diagnostics, {} declarations, {} reasons, {} references",
  ///         record.omitted_diagnostics(),
  ///         record.omitted_declarations(),
  ///         record.omitted_reasons(),
  ///         record.omitted_references(),
  ///     ));
  ///     lines
  /// }
  /// ```
  #[derive(Clone, Debug, PartialEq, Eq, Hash)]
  pub struct LimitRecord {
    exhausted: Vec<Limit>,
    first_stopped: Vec<StoppedLimit>,
    first_reasons: Vec<PartReason>,
    #[cfg_attr(
      feature = "serde",
      serde(serialize_with = "crate::form::write_decimal", deserialize_with = "crate::form::read_decimal")
    )]
    omitted_diagnostics: u64,
    #[cfg_attr(
      feature = "serde",
      serde(serialize_with = "crate::form::write_decimal", deserialize_with = "crate::form::read_decimal")
    )]
    omitted_declarations: u64,
    #[cfg_attr(
      feature = "serde",
      serde(serialize_with = "crate::form::write_decimal", deserialize_with = "crate::form::read_decimal")
    )]
    omitted_reasons: u64,
    #[cfg_attr(
      feature = "serde",
      serde(serialize_with = "crate::form::write_decimal", deserialize_with = "crate::form::read_decimal")
    )]
    omitted_references: u64,
  }
  check = LimitRecord::check_form;
}

form_record! {
  /// A limit and the part that it stopped first, written as a record of `limit` and `part` (`json-output.md` section
  /// 7.2).
  #[derive(Clone, Debug, PartialEq, Eq, Hash)]
  pub(crate) struct StoppedLimit {
    limit: Limit,
    part: Part,
  }
}

impl LimitRecord {
  /// The record of a probe that nothing stopped.
  pub(crate) fn empty() -> LimitRecord {
    LimitRecord {
      exhausted: Vec::new(),
      first_stopped: Vec::new(),
      first_reasons: Vec::new(),
      omitted_diagnostics: 0,
      omitted_declarations: 0,
      omitted_reasons: 0,
      omitted_references: 0,
    }
  }

  /// Whether the counter of `limit` was exhausted.
  #[must_use]
  pub fn exhausted(&self, limit: &Limit) -> bool {
    self.exhausted.contains(limit)
  }

  /// The part that the counter of `limit` stopped first, or `None` when it stopped none.
  #[must_use]
  pub fn first_stopped(&self, limit: &Limit) -> Option<&Part> {
    self
      .first_stopped
      .iter()
      .find_map(|stopped| (stopped.limit == *limit).then_some(&stopped.part))
  }

  /// The first reason recorded against `part`, with its kind, limit, feature, stream and source, and without its
  /// detail text, or `None` when the part did not fall short.
  #[must_use]
  pub fn first_reason(&self, part: &Part) -> Option<&PartReason> {
    self.first_reasons.iter().find(|reason| reason.part() == part)
  }

  /// The diagnostics counted but not stored.
  #[must_use]
  pub fn omitted_diagnostics(&self) -> u64 {
    self.omitted_diagnostics
  }

  /// The stream declarations seen but not retained.
  #[must_use]
  pub fn omitted_declarations(&self) -> u64 {
    self.omitted_declarations
  }

  /// The reasons recorded but not kept in `Completeness::reasons()`.
  #[must_use]
  pub fn omitted_reasons(&self) -> u64 {
    self.omitted_reasons
  }

  /// The external references found but not kept in `MediaReport::external_references()`.
  #[must_use]
  pub fn omitted_references(&self) -> u64 {
    self.omitted_references
  }

  /// Whether a part fell short: the record keeps the first reason of every such part, whatever the limits.
  pub(crate) fn has_first_reason(&self) -> bool {
    !self.first_reasons.is_empty()
  }

  /// The first reason of the earliest part that fell short, in the order of the parts.
  pub(crate) fn earliest_reason(&self) -> Option<&PartReason> {
    self.first_reasons.first()
  }
}

#[cfg(feature = "serde")]
impl LimitRecord {
  /// The limits whose counters were exhausted, in the declared order.
  pub(crate) fn exhausted_limits(&self) -> &[Limit] {
    &self.exhausted
  }

  /// The first reason of each part, in the order of the parts.
  pub(crate) fn first_reasons(&self) -> &[PartReason] {
    &self.first_reasons
  }

  /// Whether the record read is true to itself (`json-output.md` section 7.2): each limit once in `exhausted`, and
  /// the pairs naming exactly those limits in their order; each part once among the first reasons; the limits and
  /// parts that this version knows in their declared order; each stopped part with its first reason; no first
  /// reason with a detail, and each limit reason finding its counter; an omitted reason beside a first reason and the
  /// exhausted retained data; and an omitted reference beside the exhausted retained data and the first reason of a
  /// part that finds references. As the fourth review of the third slice's first part asked, which read a declaration
  /// omitted beside complete discovery and no exhausted limit, an omitted diagnostic stands beside the exhausted
  /// diagnostics or retained data, which alone keep one from being stored, and an omitted declaration beside the
  /// first reason of discovery and the exhausted stream limit or retained data, which alone stop the enumeration of
  /// declarations (plan section 7.5). A limit that a first reason of detection names stopped detection first, since
  /// detection comes first, as the maintainer chose at that review.
  fn check_form(&self) -> Result<(), &'static str> {
    let (exhausted_set, parts): (HashSet<&Limit>, Vec<&Part>) = (
      self.exhausted.iter().collect(),
      self.first_reasons.iter().map(PartReason::part).collect(),
    );
    let retained = exhausted_set.contains(&Limit::RetainedData);
    if self.omitted_reasons > 0 && (self.first_reasons.is_empty() || !retained) {
      return Err(OMITTED_REASONS_MESSAGE);
    }
    let references_belong = retained && self.first_reasons.iter().any(|first| first.part().finds_references());
    if self.omitted_references > 0 && !references_belong {
      return Err(OMITTED_REFERENCES_MESSAGE);
    }
    let declarations_belong =
      parts.contains(&&Part::Discovery) && (retained || exhausted_set.contains(&Limit::Streams));
    let diagnostics_belong = retained || exhausted_set.contains(&Limit::Diagnostics);
    if (self.omitted_declarations > 0 && !declarations_belong) || (self.omitted_diagnostics > 0 && !diagnostics_belong)
    {
      return Err(OMITTED_STORAGE_MESSAGE);
    }
    if !self.stopped_detection_first(LimitRecord::of_detection(self.first_reasons.iter())) {
      return Err(DETECTION_LIMIT_MESSAGE);
    }
    let part_set: HashSet<&Part> = parts.iter().copied().collect();
    let pairs_name_the_limits = self.exhausted.len() == self.first_stopped.len()
      && self
        .exhausted
        .iter()
        .zip(&self.first_stopped)
        .all(|(limit, stopped)| *limit == stopped.limit);
    let true_to_itself = pairs_name_the_limits
      && crate::form::each_once(&self.exhausted, |limit| limit)
      && crate::form::each_once(&parts, |part| *part)
      && in_declared_order(&self.exhausted, &LIMIT_ORDER)
      && in_declared_order(&parts, &PART_ORDER.each_ref())
      && self
        .first_stopped
        .iter()
        .all(|stopped| part_set.contains(&stopped.part))
      && self.first_reasons.iter().all(|first| {
        first.reason().detail().is_none() && first.reason().finds_its_counter(|limit| exhausted_set.contains(limit))
      });
    if true_to_itself {
      Ok(())
    } else {
      Err(LIMIT_RECORD_MESSAGE)
    }
  }

  /// The parts that the limits stopped first, in the order of the limits.
  pub(crate) fn stopped_parts(&self) -> impl Iterator<Item = &Part> {
    self.first_stopped.iter().map(|stopped| &stopped.part)
  }

  /// Whether each limit that one of the reasons of detection names as exhausted stopped detection first (`api.md`
  /// section 5.3): the record's first reasons, a completeness's kept reasons, or an error's causes. The limits are
  /// looked up in an index of the pairs, so the check stays linear whatever limits of a newer version a record names.
  pub(crate) fn stopped_detection_first<'reason>(&self, causes: impl Iterator<Item = &'reason Reason>) -> bool {
    let first_stopped: HashMap<&Limit, &Part> = self
      .first_stopped
      .iter()
      .map(|stopped| (&stopped.limit, &stopped.part))
      .collect();
    causes
      .filter_map(Reason::exhausted_limit)
      .all(|limit| first_stopped.get(limit) == Some(&&Part::Detection))
  }

  /// The reasons of detection among `reasons`, whose limits `stopped_detection_first` checks.
  pub(crate) fn of_detection<'reason>(
    reasons: impl Iterator<Item = &'reason PartReason>,
  ) -> impl Iterator<Item = &'reason Reason> {
    reasons
      .filter(|reason| *reason.part() == Part::Detection)
      .map(PartReason::reason)
  }
}

/// Whether the items that the declared order knows stand in that order; one of a newer version may stand anywhere.
#[cfg(feature = "serde")]
fn in_declared_order<Item: PartialEq>(items: &[Item], order: &[Item]) -> bool {
  let ranks: Vec<usize> = items
    .iter()
    .filter_map(|item| order.iter().position(|known| known == item))
    .collect();
  ranks.is_sorted()
}

#[cfg(test)]
mod tests {
  use super::{LimitRecord, StoppedLimit};
  use crate::Limit;
  use crate::completeness::{Part, PartReason, Reason};

  impl LimitRecord {
    /// The record of a probe whose `limit` stopped `part` first.
    pub(crate) fn stopped(limit: Limit, part: Part) -> LimitRecord {
      LimitRecord {
        exhausted: vec![limit.clone()],
        first_stopped: vec![StoppedLimit {
          limit: limit.clone(),
          part: part.clone(),
        }],
        first_reasons: vec![PartReason::sample(part, Reason::exhausted(limit))],
        ..LimitRecord::empty()
      }
    }

    /// The record of a probe whose one first reason, which no limit gave, is `reason`.
    pub(crate) fn first_reason_only(reason: PartReason) -> LimitRecord {
      LimitRecord {
        first_reasons: vec![reason],
        ..LimitRecord::empty()
      }
    }

    /// The same record with these omitted counts.
    pub(crate) fn with_omitted(
      self,
      diagnostics: u64,
      declarations: u64,
      reasons: u64,
      references: u64,
    ) -> LimitRecord {
      LimitRecord {
        omitted_diagnostics: diagnostics,
        omitted_declarations: declarations,
        omitted_reasons: reasons,
        omitted_references: references,
        ..self
      }
    }
  }

  #[test]
  fn records_what_stopped_the_probe() {
    let record = LimitRecord::stopped(Limit::BytesRead, Part::Detection).with_omitted(1, 2, 3, 4);
    assert!(record.exhausted(&Limit::BytesRead));
    assert!(!record.exhausted(&Limit::Streams));
    assert_eq!(record.first_stopped(&Limit::BytesRead), Some(&Part::Detection));
    assert_eq!(record.first_stopped(&Limit::Streams), None);
    assert_eq!(
      record.first_reason(&Part::Detection).map(PartReason::part),
      Some(&Part::Detection)
    );
    assert_eq!(record.first_reason(&Part::Tags), None);
    assert_eq!(
      (
        record.omitted_diagnostics(),
        record.omitted_declarations(),
        record.omitted_reasons(),
        record.omitted_references()
      ),
      (1, 2, 3, 4)
    );
    assert!(record.has_first_reason());
    assert_eq!(record.earliest_reason().map(PartReason::part), Some(&Part::Detection));
    assert!(!LimitRecord::empty().has_first_reason());
  }
}
