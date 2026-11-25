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

//! `ReasonKind`, the open set of reasons of plan section 8.3 (`api.md` section 5.5).

use super::constants::{
  COMPANION_DECLINED_NAME, COMPANION_DECLINED_TEXT, COMPANION_LIMITED_NAME, COMPANION_LIMITED_TEXT,
  COMPANION_NAME_UNAVAILABLE_NAME, COMPANION_NAME_UNAVAILABLE_TEXT, COMPANION_NOT_FOUND_NAME, COMPANION_NOT_FOUND_TEXT,
  COMPANIONS_NOT_ENABLED_NAME, COMPANIONS_NOT_ENABLED_TEXT, DAMAGED_STRUCTURE_NAME, DAMAGED_STRUCTURE_TEXT,
  END_OF_INPUT_NOT_OBSERVED_NAME, END_OF_INPUT_NOT_OBSERVED_TEXT, FEATURE_DISABLED_NAME, FEATURE_DISABLED_TEXT,
  FURTHER_SEGMENTS_NOT_EXAMINED_NAME, FURTHER_SEGMENTS_NOT_EXAMINED_TEXT, LIMIT_EXHAUSTED_NAME, LIMIT_EXHAUSTED_TEXT,
  MEDIA_SEGMENT_WITHOUT_INITIALIZATION_DATA_NAME, MEDIA_SEGMENT_WITHOUT_INITIALIZATION_DATA_TEXT,
  RANGE_UNAVAILABLE_NAME, RANGE_UNAVAILABLE_TEXT, SOURCE_FAILED_NAME, SOURCE_FAILED_TEXT, TRUNCATED_INPUT_NAME,
  TRUNCATED_INPUT_TEXT, UNSUPPORTED_STRUCTURE_NAME, UNSUPPORTED_STRUCTURE_TEXT,
};
use crate::string_enumeration::string_enumeration;

string_enumeration! {
  /// Why something fell short: the open set of plan section 8.3. A format phase adds the kinds it needs; adding a
  /// variant is compatible.
  ///
  /// A reason of a known kind has exactly the members that its kind defines: `LimitExhausted` names the limit
  /// exhausted, `CompanionLimited` the first limit that refused the companion, `FeatureDisabled` the feature, and
  /// every other kind neither.
  ///
  /// ```
  /// use cc_media_probe::completeness::{Reason, ReasonKind};
  ///
  /// fn is_cut_short(reason: &Reason) -> bool {
  ///     matches!(reason.kind(), ReasonKind::TruncatedInput | ReasonKind::LimitExhausted)
  /// }
  ///
  /// assert_eq!(ReasonKind::SourceFailed.as_str(), "source_failed");
  /// ```
  ReasonKind (
    /// The kind's text for people, such as "truncated input", or for a kind that a newer version wrote, its
    /// serialized name.
    pub(crate) fn text
  ) {
    /// The bytes needed lie outside the readable range, such as behind the window of a forward-only source.
    RangeUnavailable => RANGE_UNAVAILABLE_NAME, RANGE_UNAVAILABLE_TEXT,
    /// The input ended inside a structure.
    TruncatedInput => TRUNCATED_INPUT_NAME, TRUNCATED_INPUT_TEXT,
    /// A limit was exhausted; the reason names it.
    LimitExhausted => LIMIT_EXHAUSTED_NAME, LIMIT_EXHAUSTED_TEXT,
    /// The end of an input of unknown length was not observed.
    EndOfInputNotObserved => END_OF_INPUT_NOT_OBSERVED_NAME, END_OF_INPUT_NOT_OBSERVED_TEXT,
    /// A disabled Cargo feature would have read the data; the reason names it.
    FeatureDisabled => FEATURE_DISABLED_NAME, FEATURE_DISABLED_TEXT,
    /// The input expects a companion, and companion lookup is off.
    CompanionsNotEnabled => COMPANIONS_NOT_ENABLED_NAME, COMPANIONS_NOT_ENABLED_TEXT,
    /// The resolver found no such companion.
    CompanionNotFound => COMPANION_NOT_FOUND_NAME, COMPANION_NOT_FOUND_TEXT,
    /// The resolver declined the request; the detail holds its reason within the retained-data limit.
    CompanionDeclined => COMPANION_DECLINED_NAME, COMPANION_DECLINED_TEXT,
    /// A limit refused the companion before its request; the reason names it.
    CompanionLimited => COMPANION_LIMITED_NAME, COMPANION_LIMITED_TEXT,
    /// No name could be built for the companion.
    CompanionNameUnavailable => COMPANION_NAME_UNAVAILABLE_NAME, COMPANION_NAME_UNAVAILABLE_TEXT,
    /// The structure is one that this version does not support.
    UnsupportedStructure => UNSUPPORTED_STRUCTURE_NAME, UNSUPPORTED_STRUCTURE_TEXT,
    /// The bytes contradict the structure's syntax.
    DamagedStructure => DAMAGED_STRUCTURE_NAME, DAMAGED_STRUCTURE_TEXT,
    /// The file holds further segments that the probe did not examine.
    FurtherSegmentsNotExamined => FURTHER_SEGMENTS_NOT_EXAMINED_NAME, FURTHER_SEGMENTS_NOT_EXAMINED_TEXT,
    /// A media segment came without the initialization data that describes it.
    MediaSegmentWithoutInitializationData =>
      MEDIA_SEGMENT_WITHOUT_INITIALIZATION_DATA_NAME, MEDIA_SEGMENT_WITHOUT_INITIALIZATION_DATA_TEXT,
    /// A source failed with an I/O error, which the error keeps (R19).
    SourceFailed => SOURCE_FAILED_NAME, SOURCE_FAILED_TEXT,
  }
}

#[cfg(test)]
mod tests {
  use super::ReasonKind;
  use crate::model::UnrecognizedValue;

  #[test]
  fn names_each_kind() {
    let kinds = [
      (
        ReasonKind::RangeUnavailable,
        "range_unavailable",
        "bytes outside the readable range",
      ),
      (ReasonKind::TruncatedInput, "truncated_input", "truncated input"),
      (ReasonKind::LimitExhausted, "limit_exhausted", "an exhausted limit"),
      (
        ReasonKind::EndOfInputNotObserved,
        "end_of_input_not_observed",
        "an end of input that was not observed",
      ),
      (ReasonKind::FeatureDisabled, "feature_disabled", "a disabled feature"),
      (
        ReasonKind::CompanionsNotEnabled,
        "companions_not_enabled",
        "companion lookup, which is off",
      ),
      (
        ReasonKind::CompanionNotFound,
        "companion_not_found",
        "a companion that was not found",
      ),
      (
        ReasonKind::CompanionDeclined,
        "companion_declined",
        "a companion that the resolver declined",
      ),
      (
        ReasonKind::CompanionLimited,
        "companion_limited",
        "a companion that a limit refused",
      ),
      (
        ReasonKind::CompanionNameUnavailable,
        "companion_name_unavailable",
        "a companion name that could not be built",
      ),
      (
        ReasonKind::UnsupportedStructure,
        "unsupported_structure",
        "a structure that this version does not support",
      ),
      (ReasonKind::DamagedStructure, "damaged_structure", "a damaged structure"),
      (
        ReasonKind::FurtherSegmentsNotExamined,
        "further_segments_not_examined",
        "further segments that were not examined",
      ),
      (
        ReasonKind::MediaSegmentWithoutInitializationData,
        "media_segment_without_initialization_data",
        "a media segment without its initialization data",
      ),
      (ReasonKind::SourceFailed, "source_failed", "a source that failed"),
      (
        ReasonKind::Unrecognized(UnrecognizedValue::sample("deferred")),
        "deferred",
        "deferred",
      ),
    ];
    for (kind, name, text) in kinds {
      assert_eq!(kind.as_str(), name);
      assert_eq!(kind.text(), text);
    }
  }
}
