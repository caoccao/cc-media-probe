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

//! `ErrorRecord`, the serializable description of an error, a root record (`api.md` section 10.2).

use std::fmt;

use super::input_output_kind::stable_name;
use super::parts::ErrorParts;
use super::{CandidateCheck, Error, ErrorKind, Observations};
use crate::Format;
use crate::completeness::{LimitRecord, Reason};
use crate::model::{Hint, MediaReport, Usage};
use crate::source::{AttemptedIdentity, SourceIdentifier};

/// The serializable description of an error, and a root record. It has every accessor of `Error` except
/// `source_failure()`, `into_partial_report()` and `to_record()`; in their place it keeps the message, the I/O
/// error's kind and code, and the messages of the cause chain. No API turns it back into an `Error`. `Display`
/// writes the message.
///
/// `From<Error>` moves what the error holds, its partial report included, so a caller that gives the error up
/// never holds two copies of the report; `From<&Error>` and `Error::to_record()` copy it.
///
/// ```
/// use cc_media_probe::Error;
/// use cc_media_probe::error::ErrorRecord;
///
/// fn describe(error: &Error) -> String {
///     let record = ErrorRecord::from(error);
///     let mut line = format!("{}: {record}", record.kind().as_str());
///     if let Some(kind) = record.source_failure_kind() {
///         line.push_str(&format!(" ({kind}, code {:?})", record.operating_system_error_code()));
///     }
///     for cause in record.cause_messages() {
///         line.push_str(&format!("; {cause}"));
///     }
///     line.push_str(&format!(
///         "; format {:?}, {} candidates, hint {}, reason {}, source {:?}, identity {}, at {:?} ({:?})",
///         record.format().map(|format| format.name()),
///         record.candidates().len(),
///         record.hint().is_some(),
///         record.reason().is_some(),
///         record.source_identifier(),
///         record.attempted_identity().is_some(),
///         record.offset(),
///         record.logical_offset(),
///     ));
///     line.push_str(&format!(
///         "; partial {}, observations {}, {} bytes read, {} diagnostics omitted",
///         record.partial_report().is_some(),
///         record.observations().is_some(),
///         record.usage().bytes_read(),
///         record.record().omitted_diagnostics(),
///     ));
///     line
/// }
///
/// fn give_up(error: Error) -> ErrorRecord {
///     ErrorRecord::from(error)
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ErrorRecord {
  parts: ErrorParts,
  description: Description,
}

/// What a record keeps in place of the I/O error, which it cannot keep.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Description {
  /// The error's message, for people.
  message: String,
  /// The `std::io::ErrorKind` of a source failure, as a stable string.
  source_failure_kind: Option<String>,
  /// The code that `std::io::Error::raw_os_error()` gave, in canonical decimal.
  operating_system_error_code: Option<String>,
  /// The messages of the cause chain, in order.
  cause_messages: Vec<String>,
}

impl Description {
  /// The description of `error`.
  fn of(error: &Error) -> Description {
    let failure = error.source_failure();
    Description {
      message: error.to_string(),
      source_failure_kind: failure.map(|cause| stable_name(cause.kind()).to_owned()),
      operating_system_error_code: failure
        .and_then(std::io::Error::raw_os_error)
        .map(|code| code.to_string()),
      cause_messages: error.cause_messages(),
    }
  }
}

impl ErrorRecord {
  /// Which ending of the ordered procedure the error is.
  #[must_use]
  pub fn kind(&self) -> &ErrorKind {
    &self.parts.kind
  }

  /// The error's message, for people; its text may change in any release.
  #[must_use]
  pub fn message(&self) -> &str {
    &self.description.message
  }

  /// The detected format or family.
  #[must_use]
  pub fn format(&self) -> Option<&Format> {
    self.parts.format.as_ref()
  }

  /// The checks of the candidates, in rank.
  #[must_use]
  pub fn candidates(&self) -> &[CandidateCheck] {
    &self.parts.candidates
  }

  /// For an unknown format with a hint, the format that the hint suggests.
  #[must_use]
  pub fn hint(&self) -> Option<&Hint> {
    self.parts.hint.as_ref()
  }

  /// The one reason that stopped the probe.
  #[must_use]
  pub fn reason(&self) -> Option<&Reason> {
    self.parts.reason.as_ref()
  }

  /// The source concerned.
  #[must_use]
  pub fn source_identifier(&self) -> Option<SourceIdentifier> {
    self.parts.source_identifier
  }

  /// For a companion that was requested, the identity that the request asked for.
  #[must_use]
  pub fn attempted_identity(&self) -> Option<&AttemptedIdentity> {
    self.parts.attempted_identity.as_ref()
  }

  /// Where the failure or the stop happened, relative to the start of the source concerned.
  #[must_use]
  pub fn offset(&self) -> Option<u64> {
    self.parts.offset
  }

  /// Where the failure or the stop happened in the stream that several files form.
  #[must_use]
  pub fn logical_offset(&self) -> Option<u64> {
    self.parts.logical_offset
  }

  /// The `std::io::ErrorKind` of a source failure, as a stable string such as `permission_denied`.
  #[must_use]
  pub fn source_failure_kind(&self) -> Option<&str> {
    self.description.source_failure_kind.as_deref()
  }

  /// The code that `std::io::Error::raw_os_error()` gave, in canonical decimal, such as `"2"`: digits, with a minus
  /// sign for a negative code, no plus sign and no leading zero. It is text because the type of the code differs
  /// by platform: an `i32` on most, and on UEFI a `usize` status, whose error statuses no `i32` holds.
  ///
  /// ```
  /// use cc_media_probe::error::ErrorRecord;
  ///
  /// fn is_missing_on_unix(record: &ErrorRecord) -> bool {
  ///     record.operating_system_error_code() == Some("2")
  /// }
  /// ```
  #[must_use]
  pub fn operating_system_error_code(&self) -> Option<&str> {
    self.description.operating_system_error_code.as_deref()
  }

  /// The messages of the cause chain, in order; empty without a cause. The chain is followed for at most 32
  /// causes, and a cause at the address and with the message of a cause already listed is left out, so the list
  /// always ends and a cyclic chain gives each of its messages once.
  #[must_use]
  pub fn cause_messages(&self) -> &[String] {
    &self.description.cause_messages
  }

  /// The report established before a source failed, when the stream inventory was reached.
  #[must_use]
  pub fn partial_report(&self) -> Option<&MediaReport> {
    self.parts.partial_report.as_deref()
  }

  /// What the probe saw before it stopped short of its stream inventory.
  #[must_use]
  pub fn observations(&self) -> Option<&Observations> {
    self.parts.observations.as_ref()
  }

  /// What the budget accounted for.
  #[must_use]
  pub fn usage(&self) -> &Usage {
    &self.parts.usage
  }

  /// The fixed-size record; the partial report's own when there is one.
  #[must_use]
  pub fn record(&self) -> &LimitRecord {
    &self.parts.record
  }
}

impl From<&Error> for ErrorRecord {
  /// Copies what the error holds.
  fn from(error: &Error) -> ErrorRecord {
    ErrorRecord {
      description: Description::of(error),
      parts: error.parts().clone(),
    }
  }
}

impl From<Error> for ErrorRecord {
  /// Moves what the error holds, after taking its texts.
  fn from(error: Error) -> ErrorRecord {
    ErrorRecord {
      description: Description::of(&error),
      parts: error.into_parts(),
    }
  }
}

impl fmt::Display for ErrorRecord {
  /// Writes the message.
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.description.message)
  }
}

#[cfg(test)]
mod tests {
  use std::ptr;

  use super::ErrorRecord;
  use crate::completeness::{Completeness, LimitRecord, Part, PartReason, Reason, ReasonKind, SectionStatus};
  use crate::error::{CandidateCheck, Error, ErrorKind, Observations};
  use crate::model::{Hint, MediaReport, Usage};
  use crate::source::{AttemptedIdentity, SourceIdentifier};
  use crate::{Format, Limit};

  /// Asserts that `record` keeps what the failure of `Error::failed_source()` cut short: the tags of its partial
  /// report, with the reason `SourceFailed` and the source that failed, in the partial report and in the record.
  fn assert_keeps_the_cut(record: &ErrorRecord) {
    let partial = record.partial_report().map(MediaReport::completeness);
    assert_eq!(partial.map(Completeness::tags), Some(&SectionStatus::Incomplete));
    let kept = partial.and_then(|completeness| completeness.reasons().first());
    for reason in [kept, record.record().first_reason(&Part::Tags)] {
      assert_eq!(reason.map(PartReason::part), Some(&Part::Tags));
      assert_eq!(
        reason.map(|reason| reason.reason().kind()),
        Some(&ReasonKind::SourceFailed)
      );
      assert_eq!(
        reason.and_then(PartReason::source_identifier),
        Some(SourceIdentifier::sample(0))
      );
    }
    assert_eq!(partial.map(Completeness::record), Some(record.record()));
  }

  #[test]
  fn describes_a_source_failure() {
    let error = Error::failed_source();
    let record = error.to_record();
    assert_eq!(record.kind(), &ErrorKind::SourceFailed);
    assert_eq!(record.message(), "the source failed at byte 1040384");
    assert_eq!(record.to_string(), record.message());
    assert_eq!(record.source_failure_kind(), Some("permission_denied"));
    assert_eq!(record.operating_system_error_code(), None);
    assert_eq!(record.cause_messages(), ["access denied"]);
    assert_eq!(record.source_identifier(), Some(SourceIdentifier::sample(0)));
    assert_eq!(record.attempted_identity(), None);
    assert_eq!(record.offset(), Some(1_040_384));
    assert_eq!(record.logical_offset(), None);
    assert_eq!(record.observations(), None);
    assert_keeps_the_cut(&record);
    let moved = ErrorRecord::from(error);
    assert_keeps_the_cut(&moved);
    assert_eq!(record, moved);
  }

  #[test]
  fn moves_the_partial_report_of_an_owned_error() {
    let error = Error::failed_source();
    let held = error.partial_report().map(ptr::from_ref);
    let copied = ErrorRecord::from(&error);
    assert_ne!(copied.partial_report().map(ptr::from_ref), held);
    let moved = ErrorRecord::from(error);
    assert_eq!(moved.partial_report().map(ptr::from_ref), held);
    assert_eq!(moved, copied);
  }

  #[test]
  fn names_the_companion_that_failed() {
    let record = ErrorRecord::from(Error::failed_companion());
    assert_eq!(record.source_identifier(), Some(SourceIdentifier::sample(1)));
    assert_eq!(
      record.attempted_identity().and_then(AttemptedIdentity::identifier),
      Some("movie")
    );
    assert_eq!(record.source_failure_kind(), Some("unexpected_eof"));
    assert_eq!(record.partial_report(), None);
  }

  #[test]
  fn describes_a_source_that_could_not_be_opened() {
    let record = ErrorRecord::from(Error::unopened_source());
    assert_eq!(record.message(), "the source could not be opened");
    assert_eq!(record.source_failure_kind(), Some("not_found"));
    assert_eq!(record.offset(), None);
  }

  #[test]
  fn describes_a_stop_before_the_inventory() {
    let record = ErrorRecord::from(&Error::stopped_before_the_inventory());
    assert_eq!(record.kind(), &ErrorKind::NoStreamInventory);
    assert_eq!(record.format(), Some(&Format::IsoBaseMedia));
    assert_eq!(
      record.reason().map(Reason::kind),
      Some(&ReasonKind::MediaSegmentWithoutInitializationData)
    );
    assert_eq!(record.observations(), Some(&Observations::sample()));
    assert_eq!((record.candidates(), record.hint()), (&[] as &[CandidateCheck], None));
    assert_eq!(record.source_failure_kind(), None);
    assert_eq!(record.cause_messages(), &[] as &[String]);
    assert_eq!(record.usage(), &Usage::sample());
  }

  #[test]
  fn describes_each_ending_of_detection() {
    let stopped = ErrorRecord::from(Error::stopped_detection());
    assert_eq!(stopped.kind(), &ErrorKind::FormatNotDetermined);
    assert_eq!(stopped.reason().and_then(Reason::limit), Some(&Limit::BytesRead));
    assert_eq!(
      stopped.record(),
      &LimitRecord::stopped(Limit::BytesRead, Part::Detection)
    );
    assert_eq!((stopped.candidates().len(), stopped.format()), (0, None));
    let unchecked = ErrorRecord::from(&Error::unchecked_candidate());
    assert_eq!(unchecked.kind(), &ErrorKind::FormatNotDetermined);
    assert_eq!(unchecked.reason(), None);
    assert_eq!(
      unchecked
        .candidates()
        .first()
        .and_then(CandidateCheck::reason)
        .map(Reason::kind),
      Some(&ReasonKind::RangeUnavailable)
    );
    assert!(!unchecked.record().has_first_reason());
    let hinted = ErrorRecord::from(Error::hinted_format());
    assert_eq!(hinted.kind(), &ErrorKind::UnknownFormat);
    assert_eq!(hinted.hint().map(Hint::format), Some(&Format::Mp4));
    assert_eq!((hinted.candidates().len(), hinted.reason()), (0, None));
  }

  #[test]
  fn keeps_the_operating_system_code_in_decimal() {
    let without_code = ErrorRecord::from(&Error::sample(ErrorKind::SourceFailed));
    assert_eq!(without_code.operating_system_error_code(), None);
    let missing = ErrorRecord::from(Error::failed_with(std::io::Error::from_raw_os_error(2)));
    assert_eq!(missing.operating_system_error_code(), Some("2"));
    let large = ErrorRecord::from(Error::failed_with(std::io::Error::from_raw_os_error(2_147_024_894)));
    assert_eq!(large.operating_system_error_code(), Some("2147024894"));
  }
}
