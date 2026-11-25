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

//! `Completeness`, what a report established, what it did not, and why (`api.md` section 5.3; R23).

use std::fmt;

use super::constants::{COMPLETE_NAME, DEFINITE_ARTICLE, INCOMPLETE_NAME, STOPPED_BY_TEXT};
use super::{
  CompanionStatus, DetectionStatus, DiscoveryStatus, HeadersStatus, LimitRecord, PartReason, SectionStatus,
  TimingStatus,
};

/// What a report established, what it did not, and why, from `MediaReport::completeness()`. Every shortfall is
/// recorded against the part it stopped, so none can exist without changing `is_complete()`.
///
/// Each status is a stored enum with a catch-all, so its getter returns it by reference. `Display` writes a
/// one-line summary, such as "incomplete: discovery stopped by the stream limit".
///
/// ```
/// use cc_media_probe::MediaReport;
/// use cc_media_probe::completeness::SectionStatus;
///
/// fn describe(report: &MediaReport) {
///     let completeness = report.completeness();
///     if completeness.is_complete() {
///         println!("complete");
///         return;
///     }
///     println!("{completeness}");
///     println!(
///         "detection {}, discovery {}, headers {}, timing {}, companions {}",
///         completeness.detection().as_str(),
///         completeness.discovery().as_str(),
///         completeness.headers().as_str(),
///         completeness.timing().as_str(),
///         completeness.companions().as_str(),
///     );
///     for (name, status) in [
///         ("tags", completeness.tags()),
///         ("chapters", completeness.chapters()),
///         ("attachments", completeness.attachments()),
///     ] {
///         if matches!(status, SectionStatus::Incomplete) {
///             println!("{name} may be missing");
///         }
///     }
///     for reason in completeness.reasons() {
///         println!("{}: {}", reason.part().as_str(), reason.reason().kind().as_str());
///     }
///     println!("{} reasons were not kept", completeness.record().omitted_reasons());
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Completeness {
  detection: DetectionStatus,
  discovery: DiscoveryStatus,
  headers: HeadersStatus,
  tags: SectionStatus,
  chapters: SectionStatus,
  attachments: SectionStatus,
  timing: TimingStatus,
  companions: CompanionStatus,
  reasons: Vec<PartReason>,
  record: LimitRecord,
}

impl Completeness {
  /// Whether no part fell short, as plan section 8.3 defines it and `--strict` uses it: every status is complete,
  /// final or not applicable, no status holds a value of a newer version, and the record holds no first reason. It
  /// reads the statuses and the record, which every limit leaves in place, and never `reasons()`, which a full
  /// budget can leave empty.
  #[must_use]
  pub fn is_complete(&self) -> bool {
    self.detection.is_satisfied()
      && self.discovery.is_satisfied()
      && self.headers.is_satisfied()
      && self.tags.is_satisfied()
      && self.chapters.is_satisfied()
      && self.attachments.is_satisfied()
      && self.timing.is_satisfied()
      && self.companions.is_satisfied()
      && !self.record.has_first_reason()
  }

  /// Whether detection, with the checks of the candidates ranked ahead of the one used, ran in full.
  #[must_use]
  pub fn detection(&self) -> &DetectionStatus {
    &self.detection
  }

  /// Whether every stream was found, with each reported stream's declaration in full.
  #[must_use]
  pub fn discovery(&self) -> &DiscoveryStatus {
    &self.discovery
  }

  /// Whether every stream's header state is final.
  #[must_use]
  pub fn headers(&self) -> &HeadersStatus {
    &self.headers
  }

  /// The status of the tags.
  #[must_use]
  pub fn tags(&self) -> &SectionStatus {
    &self.tags
  }

  /// The status of the chapters.
  #[must_use]
  pub fn chapters(&self) -> &SectionStatus {
    &self.chapters
  }

  /// The status of the attachments of the requested roles.
  #[must_use]
  pub fn attachments(&self) -> &SectionStatus {
    &self.attachments
  }

  /// The status of the timing.
  #[must_use]
  pub fn timing(&self) -> &TimingStatus {
    &self.timing
  }

  /// The status of the companions that the input expects.
  #[must_use]
  pub fn companions(&self) -> &CompanionStatus {
    &self.companions
  }

  /// Each shortfall with the part it stopped, in the order recorded, kept within the retained-data limit. The
  /// record keeps the first reason of each part whatever the limits.
  #[must_use]
  pub fn reasons(&self) -> &[PartReason] {
    &self.reasons
  }

  /// The fixed-size record.
  #[must_use]
  pub fn record(&self) -> &LimitRecord {
    &self.record
  }
}

impl fmt::Display for Completeness {
  /// Writes "complete", or "incomplete" with the first reason, from `reasons()` or else from the record.
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    if self.is_complete() {
      return formatter.write_str(COMPLETE_NAME);
    }
    let Some(first) = self.reasons.first().or(self.record.earliest_reason()) else {
      return formatter.write_str(INCOMPLETE_NAME);
    };
    let part = first.part().as_str();
    match first.reason().limit() {
      Some(_) => write!(
        formatter,
        "{INCOMPLETE_NAME}: {part} {STOPPED_BY_TEXT} {DEFINITE_ARTICLE} {}",
        first.reason().cause()
      ),
      None => write!(
        formatter,
        "{INCOMPLETE_NAME}: {part} {STOPPED_BY_TEXT} {}",
        first.reason().cause()
      ),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::Completeness;
  use crate::Limit;
  use crate::completeness::{
    CompanionStatus, DetectionStatus, DiscoveryStatus, HeadersStatus, LimitRecord, Part, PartReason, Reason,
    ReasonKind, SectionStatus, TimingStatus,
  };
  use crate::model::UnrecognizedValue;

  impl Completeness {
    /// The completeness of a report that nothing stopped.
    pub(crate) fn complete() -> Completeness {
      Completeness {
        detection: DetectionStatus::Complete,
        discovery: DiscoveryStatus::Complete,
        headers: HeadersStatus::Complete,
        tags: SectionStatus::Complete,
        chapters: SectionStatus::Complete,
        attachments: SectionStatus::NotRequested,
        timing: TimingStatus::Established,
        companions: CompanionStatus::NotApplicable,
        reasons: Vec::new(),
        record: LimitRecord::empty(),
      }
    }

    /// The completeness of a report whose discovery the stream limit stopped.
    pub(crate) fn stopped_by_the_stream_limit() -> Completeness {
      Completeness {
        discovery: DiscoveryStatus::Incomplete,
        reasons: vec![PartReason::sample(Part::Discovery, Reason::exhausted(Limit::Streams))],
        record: LimitRecord::stopped(Limit::Streams, Part::Discovery),
        ..Completeness::complete()
      }
    }

    /// The completeness of a report whose tags a failure of the primary input cut short.
    pub(crate) fn cut_by_a_source_failure() -> Completeness {
      let reason = PartReason::sample(Part::Tags, Reason::sample(ReasonKind::SourceFailed)).of_source(0);
      Completeness {
        tags: SectionStatus::Incomplete,
        reasons: vec![reason.clone()],
        record: LimitRecord::first_reason_only(reason),
        ..Completeness::complete()
      }
    }
  }

  #[test]
  fn is_complete_when_nothing_fell_short() {
    let complete = Completeness::complete();
    assert!(complete.is_complete());
    assert_eq!(complete.to_string(), "complete");
    assert_eq!(complete.detection(), &DetectionStatus::Complete);
    assert_eq!(complete.discovery(), &DiscoveryStatus::Complete);
    assert_eq!(complete.headers(), &HeadersStatus::Complete);
    assert_eq!(complete.tags(), &SectionStatus::Complete);
    assert_eq!(complete.chapters(), &SectionStatus::Complete);
    assert_eq!(complete.attachments(), &SectionStatus::NotRequested);
    assert_eq!(complete.timing(), &TimingStatus::Established);
    assert_eq!(complete.companions(), &CompanionStatus::NotApplicable);
    assert_eq!(complete.reasons(), []);
    assert_eq!(complete.record(), &LimitRecord::empty());
  }

  #[test]
  fn names_the_first_reason_when_a_part_fell_short() {
    let stopped = Completeness::stopped_by_the_stream_limit();
    assert!(!stopped.is_complete());
    assert_eq!(stopped.to_string(), "incomplete: discovery stopped by the stream limit");
    let full_budget = Completeness {
      reasons: Vec::new(),
      ..Completeness::stopped_by_the_stream_limit()
    };
    assert_eq!(
      full_budget.to_string(),
      "incomplete: discovery stopped by the stream limit"
    );
    let truncated = Completeness {
      tags: SectionStatus::Incomplete,
      reasons: vec![PartReason::sample(
        Part::Tags,
        Reason::sample(ReasonKind::TruncatedInput),
      )],
      ..Completeness::complete()
    };
    assert_eq!(truncated.to_string(), "incomplete: tags stopped by truncated input");
    let failed = Completeness::cut_by_a_source_failure();
    assert!(!failed.is_complete());
    assert_eq!(failed.to_string(), "incomplete: tags stopped by a source that failed");
  }

  #[test]
  fn is_not_complete_beside_a_shortfall_or_a_newer_status() {
    for incomplete in [
      Completeness {
        detection: DetectionStatus::Incomplete,
        ..Completeness::complete()
      },
      Completeness {
        headers: HeadersStatus::Incomplete,
        ..Completeness::complete()
      },
      Completeness {
        chapters: SectionStatus::Incomplete,
        ..Completeness::complete()
      },
      Completeness {
        attachments: SectionStatus::Incomplete,
        ..Completeness::complete()
      },
      Completeness {
        timing: TimingStatus::CutShort,
        ..Completeness::complete()
      },
      Completeness {
        companions: CompanionStatus::NotEnabled,
        ..Completeness::complete()
      },
      Completeness {
        tags: SectionStatus::Unrecognized(UnrecognizedValue::sample("partial")),
        ..Completeness::complete()
      },
      Completeness {
        record: LimitRecord::stopped(
          Limit::RetainedData,
          Part::Unrecognized(UnrecognizedValue::sample("programs")),
        ),
        ..Completeness::complete()
      },
    ] {
      assert!(!incomplete.is_complete());
    }
    let newer = Completeness {
      discovery: DiscoveryStatus::Unrecognized(UnrecognizedValue::sample("partial")),
      ..Completeness::complete()
    };
    assert_eq!(newer.to_string(), "incomplete");
  }
}
