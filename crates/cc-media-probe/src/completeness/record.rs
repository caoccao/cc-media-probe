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

use super::{Part, PartReason};
use crate::Limit;

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
  first_stopped: Vec<(Limit, Part)>,
  first_reasons: Vec<PartReason>,
  omitted_diagnostics: u64,
  omitted_declarations: u64,
  omitted_reasons: u64,
  omitted_references: u64,
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
      .find_map(|(stopped, part)| (stopped == limit).then_some(part))
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

#[cfg(test)]
mod tests {
  use super::LimitRecord;
  use crate::Limit;
  use crate::completeness::{Part, PartReason, Reason};

  impl LimitRecord {
    /// The record of a probe whose `limit` stopped `part` first.
    pub(crate) fn stopped(limit: Limit, part: Part) -> LimitRecord {
      LimitRecord {
        exhausted: vec![limit.clone()],
        first_stopped: vec![(limit.clone(), part.clone())],
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
