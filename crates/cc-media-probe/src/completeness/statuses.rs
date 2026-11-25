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

//! The status of each part of completeness (`api.md` section 5.3).

use super::constants::{
  COMPLETE_NAME, CUT_SHORT_NAME, ESTABLISHED_NAME, INCOMPLETE_NAME, NOT_APPLICABLE_NAME, NOT_AVAILABLE_BY_DESIGN_NAME,
  NOT_ENABLED_NAME, NOT_REQUESTED_NAME,
};
use crate::string_enumeration::string_enumeration;

string_enumeration! {
  /// Whether detection, with the validation checks of the candidates ranked ahead of the one used, ran in full.
  ///
  /// ```
  /// use cc_media_probe::completeness::{Completeness, DetectionStatus};
  ///
  /// fn detection_is_sure(completeness: &Completeness) -> bool {
  ///     matches!(completeness.detection(), DetectionStatus::Complete)
  /// }
  /// ```
  DetectionStatus {
    /// No candidate ranked ahead of the one used was left unchecked by a limit, an unavailable range or the
    /// input's end.
    Complete => COMPLETE_NAME,
    /// A candidate ranked ahead of the one used could not be checked, and might have been chosen.
    Incomplete => INCOMPLETE_NAME,
  }
}

string_enumeration! {
  /// Whether every stream was found, with each reported stream's declaration in full.
  ///
  /// ```
  /// use cc_media_probe::completeness::{Completeness, DiscoveryStatus};
  ///
  /// fn every_stream_found(completeness: &Completeness) -> bool {
  ///     matches!(completeness.discovery(), DiscoveryStatus::Complete)
  /// }
  /// ```
  DiscoveryStatus {
    /// Every stream was found.
    Complete => COMPLETE_NAME,
    /// A stream may be missing, or a reported stream's declaration fell short.
    Incomplete => INCOMPLETE_NAME,
  }
}

string_enumeration! {
  /// Whether every stream's header state is final (`api.md` section 5.4). `Completeness` keeps it among its
  /// fixed-size data, so `is_complete()` sees the header states without the streams.
  ///
  /// ```
  /// use cc_media_probe::completeness::{Completeness, HeadersStatus};
  ///
  /// fn headers_done(completeness: &Completeness) -> bool {
  ///     matches!(completeness.headers(), HeadersStatus::Complete)
  /// }
  /// ```
  HeadersStatus {
    /// Every stream's header state is final, as in a report of no streams.
    Complete => COMPLETE_NAME,
    /// At least one stream's header state is not final.
    Incomplete => INCOMPLETE_NAME,
  }
}

string_enumeration! {
  /// The status of the tags, the chapters or the attachments.
  ///
  /// ```
  /// use cc_media_probe::MediaReport;
  /// use cc_media_probe::completeness::SectionStatus;
  ///
  /// fn has_no_chapters_for_sure(report: &MediaReport) -> bool {
  ///     report.chapter_editions().is_empty() && matches!(report.completeness().chapters(), SectionStatus::Complete)
  /// }
  ///
  /// assert_eq!(SectionStatus::NotRequested.as_str(), "not_requested");
  /// ```
  SectionStatus {
    /// Every item was found, which includes a section proven empty.
    Complete => COMPLETE_NAME,
    /// An item may be missing.
    Incomplete => INCOMPLETE_NAME,
    /// The options asked to skip the section.
    NotRequested => NOT_REQUESTED_NAME,
    /// The format has no such section.
    NotApplicable => NOT_APPLICABLE_NAME,
  }
}

string_enumeration! {
  /// The status of the timing: durations and start times.
  ///
  /// ```
  /// use cc_media_probe::completeness::{Completeness, TimingStatus};
  ///
  /// fn timing_is_known(completeness: &Completeness) -> bool {
  ///     matches!(completeness.timing(), TimingStatus::Established)
  /// }
  ///
  /// assert_eq!(TimingStatus::NotAvailableByDesign.as_str(), "not_available_by_design");
  /// ```
  TimingStatus {
    /// The timing was established.
    Established => ESTABLISHED_NAME,
    /// The format states no timing that this version could establish.
    NotAvailableByDesign => NOT_AVAILABLE_BY_DESIGN_NAME,
    /// The probe stopped before the timing was established.
    CutShort => CUT_SHORT_NAME,
    /// The input has no timing.
    NotApplicable => NOT_APPLICABLE_NAME,
  }
}

string_enumeration! {
  /// The status of the companions that the input expects.
  ///
  /// ```
  /// use cc_media_probe::completeness::{CompanionStatus, Completeness};
  ///
  /// fn needs_lookup(completeness: &Completeness) -> bool {
  ///     matches!(completeness.companions(), CompanionStatus::NotEnabled)
  /// }
  ///
  /// assert_eq!(CompanionStatus::CutShort.as_str(), "cut_short");
  /// ```
  CompanionStatus {
    /// The input expects no companion.
    NotApplicable => NOT_APPLICABLE_NAME,
    /// Every expected companion was read.
    Complete => COMPLETE_NAME,
    /// The input expects a companion, and companion lookup is off.
    NotEnabled => NOT_ENABLED_NAME,
    /// A companion was not found, declined, refused by a limit or cut short.
    CutShort => CUT_SHORT_NAME,
  }
}

impl DetectionStatus {
  /// Whether the status records no shortfall that this version knows of.
  pub(crate) fn is_satisfied(&self) -> bool {
    matches!(self, DetectionStatus::Complete)
  }
}

impl DiscoveryStatus {
  /// Whether the status records no shortfall that this version knows of.
  pub(crate) fn is_satisfied(&self) -> bool {
    matches!(self, DiscoveryStatus::Complete)
  }
}

impl HeadersStatus {
  /// Whether the status records no shortfall that this version knows of.
  pub(crate) fn is_satisfied(&self) -> bool {
    matches!(self, HeadersStatus::Complete)
  }
}

impl SectionStatus {
  /// Whether the status records no shortfall: complete, not requested or not applicable.
  pub(crate) fn is_satisfied(&self) -> bool {
    !matches!(self, SectionStatus::Incomplete | SectionStatus::Unrecognized(_))
  }
}

impl TimingStatus {
  /// Whether the status records no shortfall: established, not available by design or not applicable.
  pub(crate) fn is_satisfied(&self) -> bool {
    !matches!(self, TimingStatus::CutShort | TimingStatus::Unrecognized(_))
  }
}

impl CompanionStatus {
  /// Whether the status records no shortfall: not applicable or complete.
  pub(crate) fn is_satisfied(&self) -> bool {
    matches!(self, CompanionStatus::NotApplicable | CompanionStatus::Complete)
  }
}

#[cfg(test)]
mod tests {
  use super::{CompanionStatus, DetectionStatus, DiscoveryStatus, HeadersStatus, SectionStatus, TimingStatus};
  use crate::model::UnrecognizedValue;

  #[test]
  fn names_each_status_and_what_it_records() {
    assert_eq!(
      (
        DetectionStatus::Complete.as_str(),
        DetectionStatus::Complete.is_satisfied()
      ),
      ("complete", true)
    );
    assert_eq!(
      (
        DetectionStatus::Incomplete.as_str(),
        DetectionStatus::Incomplete.is_satisfied()
      ),
      ("incomplete", false)
    );
    assert_eq!(
      (
        DiscoveryStatus::Complete.as_str(),
        DiscoveryStatus::Complete.is_satisfied()
      ),
      ("complete", true)
    );
    assert_eq!(
      (
        DiscoveryStatus::Incomplete.as_str(),
        DiscoveryStatus::Incomplete.is_satisfied()
      ),
      ("incomplete", false)
    );
    assert_eq!(
      (HeadersStatus::Complete.as_str(), HeadersStatus::Complete.is_satisfied()),
      ("complete", true)
    );
    assert_eq!(
      (
        HeadersStatus::Incomplete.as_str(),
        HeadersStatus::Incomplete.is_satisfied()
      ),
      ("incomplete", false)
    );
    for (status, name, satisfied) in [
      (SectionStatus::Complete, "complete", true),
      (SectionStatus::Incomplete, "incomplete", false),
      (SectionStatus::NotRequested, "not_requested", true),
      (SectionStatus::NotApplicable, "not_applicable", true),
      (
        SectionStatus::Unrecognized(UnrecognizedValue::sample("partial")),
        "partial",
        false,
      ),
    ] {
      assert_eq!((status.as_str(), status.is_satisfied()), (name, satisfied));
    }
    for (status, name, satisfied) in [
      (TimingStatus::Established, "established", true),
      (TimingStatus::NotAvailableByDesign, "not_available_by_design", true),
      (TimingStatus::CutShort, "cut_short", false),
      (TimingStatus::NotApplicable, "not_applicable", true),
    ] {
      assert_eq!((status.as_str(), status.is_satisfied()), (name, satisfied));
    }
    for (status, name, satisfied) in [
      (CompanionStatus::NotApplicable, "not_applicable", true),
      (CompanionStatus::Complete, "complete", true),
      (CompanionStatus::NotEnabled, "not_enabled", false),
      (CompanionStatus::CutShort, "cut_short", false),
    ] {
      assert_eq!((status.as_str(), status.is_satisfied()), (name, satisfied));
    }
    assert!(!DetectionStatus::Unrecognized(UnrecognizedValue::sample("partial")).is_satisfied());
  }
}
