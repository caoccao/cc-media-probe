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

//! `Error`, what `probe_*` and `detect_*` return when they end without a result (`api.md` section 10.1).

use std::fmt;
use std::io;

use super::constants::{AT_BYTE_TEXT, MAXIMUM_CAUSES, NOT_OPENED_MESSAGE};
use super::parts::ErrorParts;
use super::{CandidateCheck, ErrorKind, ErrorRecord, Observations};
use crate::Format;
use crate::completeness::{LimitRecord, Reason};
use crate::model::{Hint, MediaReport, Usage};
use crate::source::{AttemptedIdentity, SourceIdentifier};

/// What `probe_*` and `detect_*` return when they end without a result: one ending of the ordered procedure of
/// `api.md` section 5.1, with what the probe established on the way.
///
/// It keeps its data behind one pointer, so that returning it never moves a large value. It is not serializable,
/// since an arbitrary cause cannot be rebuilt from text; `to_record()` gives the serializable `ErrorRecord`. Its
/// `source()` is the I/O error with which a source failed, so the cause chain is preserved (R19).
///
/// ```
/// use cc_media_probe::{Error, ErrorKind};
///
/// fn report(error: &Error) {
///     eprintln!("{} ({})", error, error.kind().as_str());
///     if let Some(cause) = error.source_failure() {
///         eprintln!("  caused by {cause} in source {:?}", error.source_identifier());
///     }
///     if let Some(partial) = error.partial_report() {
///         eprintln!("  {} streams were found first", partial.streams().len());
///     }
///     if let Some(observations) = error.observations() {
///         eprintln!("  {} diagnostics before the inventory", observations.diagnostics().len());
///     }
///     eprintln!(
///         "  format {:?}, {} candidates, hint {:?}, reason {:?}, identity {:?}",
///         error.format().map(|format| format.name()),
///         error.candidates().len(),
///         error.hint().map(|hint| hint.format().name()),
///         error.reason().map(|reason| reason.kind().as_str()),
///         error.attempted_identity().and_then(|identity| identity.requested_name()),
///     );
///     eprintln!(
///         "  at byte {:?} (logical {:?}), {} bytes read, {} reasons omitted",
///         error.offset(),
///         error.logical_offset(),
///         error.usage().bytes_read(),
///         error.record().omitted_reasons(),
///     );
///     let record = error.to_record();
///     assert_eq!(record.message(), error.to_string());
///     assert!(matches!(error.kind(), ErrorKind::SourceFailed) || record.source_failure_kind().is_none());
/// }
///
/// fn keep_the_partial_report(error: Error) -> Option<cc_media_probe::MediaReport> {
///     error.into_partial_report()
/// }
/// ```
#[derive(Debug)]
pub struct Error {
  data: Box<ErrorData>,
}

/// The data of an error, behind one pointer.
#[derive(Debug)]
struct ErrorData {
  /// What the error and its record keep alike.
  parts: ErrorParts,
  /// The I/O error with which the source failed.
  source_failure: Option<io::Error>,
  /// Whether the source failed while it was being opened, before any read, as a file that does not exist does.
  opening: bool,
}

impl Error {
  /// Which ending of the ordered procedure the error is.
  #[must_use]
  pub fn kind(&self) -> &ErrorKind {
    &self.data.parts.kind
  }

  /// The detected format or family.
  #[must_use]
  pub fn format(&self) -> Option<&Format> {
    self.data.parts.format.as_ref()
  }

  /// The checks of the candidates, in rank. An error of kind `UnknownFormat` or `FormatNotDetermined` holds no
  /// candidate in the state `Validated` or `Outranked`.
  #[must_use]
  pub fn candidates(&self) -> &[CandidateCheck] {
    &self.data.parts.candidates
  }

  /// For an unknown format with a hint, the format that the hint suggests.
  #[must_use]
  pub fn hint(&self) -> Option<&Hint> {
    self.data.parts.hint.as_ref()
  }

  /// The one reason that stopped the probe, including a named limit or a disabled feature; absent where the causes
  /// lie with the candidates.
  #[must_use]
  pub fn reason(&self) -> Option<&Reason> {
    self.data.parts.reason.as_ref()
  }

  /// The source concerned.
  #[must_use]
  pub fn source_identifier(&self) -> Option<SourceIdentifier> {
    self.data.parts.source_identifier
  }

  /// For a companion that was requested, the identity that the request asked for.
  #[must_use]
  pub fn attempted_identity(&self) -> Option<&AttemptedIdentity> {
    self.data.parts.attempted_identity.as_ref()
  }

  /// Where the failure or the stop happened, relative to the start of the source that `source_identifier()`
  /// names; a failed range read gives the position it asked for.
  #[must_use]
  pub fn offset(&self) -> Option<u64> {
    self.data.parts.offset
  }

  /// Where the failure or the stop happened in the stream that several files form, where they form one.
  #[must_use]
  pub fn logical_offset(&self) -> Option<u64> {
    self.data.parts.logical_offset
  }

  /// The I/O error with which the source failed, which `source()` of `std::error::Error` also returns.
  #[must_use]
  pub fn source_failure(&self) -> Option<&io::Error> {
    self.data.source_failure.as_ref()
  }

  /// The report established before a source failed, when the stream inventory was reached.
  #[must_use]
  pub fn partial_report(&self) -> Option<&MediaReport> {
    self.data.parts.partial_report.as_deref()
  }

  /// The report established before a source failed, taken out of the error.
  #[must_use]
  pub fn into_partial_report(self) -> Option<MediaReport> {
    self.data.parts.partial_report.map(|report| *report)
  }

  /// What the probe saw before it stopped short of its stream inventory.
  #[must_use]
  pub fn observations(&self) -> Option<&Observations> {
    self.data.parts.observations.as_ref()
  }

  /// What the budget accounted for; always present (plan section 7.5).
  #[must_use]
  pub fn usage(&self) -> &Usage {
    &self.data.parts.usage
  }

  /// The fixed-size record; the partial report's own when there is one (`api.md` section 5.3).
  #[must_use]
  pub fn record(&self) -> &LimitRecord {
    &self.data.parts.record
  }

  /// The serializable description of the error.
  #[must_use]
  pub fn to_record(&self) -> ErrorRecord {
    ErrorRecord::from(self)
  }

  /// The messages of the cause chain, in order, from the I/O error on. A caller's error can name a cause that
  /// came earlier in the chain, and the identity of a value behind `dyn Error` cannot be told for certain, since
  /// Rust may give one type several vtables and several types one. So the chain is followed for at most
  /// `MAXIMUM_CAUSES` causes, and a cause whose address and message equal those of a cause already listed is left
  /// out: a cyclic chain gives each of its messages once, a cause at the start of its parent with a message of its
  /// own is kept, and the work stays bounded. The last cause followed is never asked for its own, so no caller's
  /// code runs beyond the bound.
  pub(super) fn cause_messages(&self) -> Vec<String> {
    let mut listed: Vec<(&(dyn std::error::Error + 'static), String)> = Vec::new();
    let mut next = std::error::Error::source(self);
    // `remaining` counts the causes that may still follow this one.
    for remaining in (0..MAXIMUM_CAUSES).rev() {
      let Some(cause) = next else {
        break;
      };
      let message = cause.to_string();
      let repeated = listed
        .iter()
        .any(|(earlier, earlier_message)| std::ptr::addr_eq(*earlier, cause) && *earlier_message == message);
      if !repeated {
        listed.push((cause, message));
      }
      next = if remaining > 0 { cause.source() } else { None };
    }
    listed.into_iter().map(|(_, message)| message).collect()
  }

  /// The parts that the record of the error copies.
  pub(super) fn parts(&self) -> &ErrorParts {
    &self.data.parts
  }

  /// The parts that the record of the error moves.
  pub(super) fn into_parts(self) -> ErrorParts {
    self.data.parts
  }
}

impl fmt::Display for Error {
  /// Writes the message: the kind's message, and for a source failure, the byte at which it failed, such as "the
  /// source failed at byte 1048576", or "the source could not be opened" for one that failed before any read. It
  /// begins in lowercase and ends without punctuation; the cause is not repeated, since `source()` gives it.
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    match (&self.data.parts.kind, self.data.opening, self.data.parts.offset) {
      (ErrorKind::SourceFailed, true, _) => formatter.write_str(NOT_OPENED_MESSAGE),
      (ErrorKind::SourceFailed, false, Some(offset)) => {
        write!(formatter, "{} {AT_BYTE_TEXT} {offset}", self.data.parts.kind.message())
      }
      (kind, _, _) => formatter.write_str(kind.message()),
    }
  }
}

impl std::error::Error for Error {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    self
      .data
      .source_failure
      .as_ref()
      .map(|cause| cause as &(dyn std::error::Error + 'static))
  }
}

#[cfg(test)]
pub(crate) mod tests {
  use std::fmt;
  use std::io;
  use std::sync::Arc;
  use std::sync::atomic::{AtomicUsize, Ordering};

  use super::{Error, ErrorData};
  use crate::completeness::{Completeness, LimitRecord, Part, PartReason, Reason, ReasonKind, SectionStatus};
  use crate::error::parts::ErrorParts;
  use crate::error::{CandidateCheck, CheckState, ErrorKind, Observations};
  use crate::model::{Hint, MediaReport, Usage};
  use crate::source::{AttemptedIdentity, SourceIdentifier};
  use crate::{Format, Limit};

  impl Error {
    /// An error of this kind, without a cause or partial data.
    pub(crate) fn sample(kind: ErrorKind) -> Error {
      Error {
        data: Box::new(ErrorData {
          parts: ErrorParts::sample(kind),
          source_failure: None,
          opening: false,
        }),
      }
    }

    /// A source failure at byte 1040384 of the primary input, a single file, while the probe read its tags after
    /// the inventory. The partial report has its tags incomplete with the reason `SourceFailed`, which names that
    /// source, and its record is the error's. A single file forms no stream of several files, so the error has no
    /// logical offset.
    pub(crate) fn failed_source() -> Error {
      let mut error = Error::sample(ErrorKind::SourceFailed);
      let report = MediaReport::cut_by_a_source_failure();
      let parts = &mut error.data.parts;
      parts.source_identifier = Some(SourceIdentifier::sample(0));
      parts.offset = Some(1_040_384);
      parts.record = report.completeness().record().clone();
      parts.partial_report = Some(Box::new(report));
      error.data.source_failure = Some(io::Error::new(io::ErrorKind::PermissionDenied, "access denied"));
      error
    }

    /// A source failure at byte 2048 of a requested companion, the `.sub` file of a VobSub index, which carries
    /// the identity that the probe asked for.
    pub(crate) fn failed_companion() -> Error {
      let mut error = Error::sample(ErrorKind::SourceFailed);
      let parts = &mut error.data.parts;
      parts.source_identifier = Some(SourceIdentifier::sample(1));
      parts.attempted_identity = Some(AttemptedIdentity::sample(Some("movie"), Some("movie.sub")));
      parts.offset = Some(2_048);
      parts.logical_offset = Some(2_048);
      error.data.source_failure = Some(io::Error::from(io::ErrorKind::UnexpectedEof));
      error
    }

    /// A primary input that does not exist, which the probe could not open.
    pub(crate) fn unopened_source() -> Error {
      let mut error = Error::sample(ErrorKind::SourceFailed);
      error.data.parts.source_identifier = Some(SourceIdentifier::sample(0));
      error.data.source_failure = Some(io::Error::from(io::ErrorKind::NotFound));
      error.data.opening = true;
      error
    }

    /// A source failure whose cause is `cause`.
    pub(crate) fn failed_with(cause: io::Error) -> Error {
      let mut error = Error::sample(ErrorKind::SourceFailed);
      error.data.source_failure = Some(cause);
      error
    }

    /// The error of a probe whose read limit ended detection before it had a candidate: the error's reason names
    /// the limit, and the record names detection as the part that it stopped.
    pub(crate) fn stopped_detection() -> Error {
      let mut error = Error::sample(ErrorKind::FormatNotDetermined);
      let parts = &mut error.data.parts;
      parts.reason = Some(Reason::exhausted(Limit::BytesRead));
      parts.record = LimitRecord::stopped(Limit::BytesRead, Part::Detection);
      error
    }

    /// The error of a detection whose one candidate, MP4, could not be checked, since its check needs bytes outside
    /// the readable range. The candidate carries its own cause, so the error has no reason.
    pub(crate) fn unchecked_candidate() -> Error {
      let mut error = Error::sample(ErrorKind::FormatNotDetermined);
      error.data.parts.candidates = vec![CandidateCheck::sample(
        Format::Mp4,
        CheckState::NotCheckable,
        Some(Reason::sample(ReasonKind::RangeUnavailable)),
      )];
      error
    }

    /// The error of a detection that found nothing but the hint of the file name, which suggests MP4.
    pub(crate) fn hinted_format() -> Error {
      let mut error = Error::sample(ErrorKind::UnknownFormat);
      error.data.parts.hint = Some(Hint::sample(Format::Mp4));
      error
    }

    /// The error of a probe of an ISO media segment without its initialization data, which stopped before the
    /// stream inventory with the family, the reason and what it observed.
    pub(crate) fn stopped_before_the_inventory() -> Error {
      let mut error = Error::sample(ErrorKind::NoStreamInventory);
      let parts = &mut error.data.parts;
      parts.format = Some(Format::IsoBaseMedia);
      parts.reason = Some(Reason::sample(ReasonKind::MediaSegmentWithoutInitializationData));
      parts.observations = Some(Observations::sample());
      error
    }
  }

  /// A caller's error that names itself as its cause.
  #[derive(Debug)]
  struct Looping;

  impl fmt::Display for Looping {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
      formatter.write_str("looping")
    }
  }

  impl std::error::Error for Looping {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
      Some(self)
    }
  }

  /// A caller's error whose cause is its only field, which a transparent layout puts at its start, so that the two
  /// share an address.
  #[derive(Debug)]
  #[repr(transparent)]
  struct Outer {
    inner: Inner,
  }

  /// The cause that `Outer` holds at its start.
  #[derive(Debug)]
  struct Inner {
    code: u32,
  }

  impl fmt::Display for Outer {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
      formatter.write_str("outer")
    }
  }

  impl std::error::Error for Outer {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
      Some(&self.inner)
    }
  }

  impl fmt::Display for Inner {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(formatter, "inner {}", self.code)
    }
  }

  impl std::error::Error for Inner {}

  /// An error whose only cause is `Outer`, which the I/O error holds as its payload.
  #[derive(Debug)]
  struct Middle {
    outer: Outer,
  }

  impl fmt::Display for Middle {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
      formatter.write_str("middle")
    }
  }

  impl std::error::Error for Middle {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
      Some(&self.outer)
    }
  }

  /// An error whose only cause is `Wrapper`, so that the wrapper is in the chain.
  #[derive(Debug)]
  struct Holder {
    wrapper: Wrapper,
  }

  /// A caller's error whose cause is its only field, at its start, with the same message as its own.
  #[derive(Debug)]
  #[repr(transparent)]
  struct Wrapper {
    leaf: Leaf,
  }

  /// The cause that `Wrapper` holds at its start, whose own cause lives elsewhere.
  #[derive(Debug)]
  struct Leaf {
    deeper: Box<Inner>,
  }

  impl fmt::Display for Holder {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
      formatter.write_str("holder")
    }
  }

  impl std::error::Error for Holder {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
      Some(&self.wrapper)
    }
  }

  impl fmt::Display for Wrapper {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
      formatter.write_str("same")
    }
  }

  impl std::error::Error for Wrapper {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
      Some(&self.leaf)
    }
  }

  impl fmt::Display for Leaf {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
      formatter.write_str("same")
    }
  }

  impl std::error::Error for Leaf {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
      Some(self.deeper.as_ref())
    }
  }

  /// One link of a long chain of distinct causes, which counts how often a link of the chain is asked for its cause.
  #[derive(Debug)]
  struct Link {
    depth: u32,
    next: Option<Box<Link>>,
    requests: Arc<AtomicUsize>,
  }

  impl fmt::Display for Link {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(formatter, "link {}", self.depth)
    }
  }

  impl std::error::Error for Link {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
      self.requests.fetch_add(1, Ordering::Relaxed);
      self
        .next
        .as_deref()
        .map(|next| next as &(dyn std::error::Error + 'static))
    }
  }

  #[test]
  fn keeps_the_cause_and_the_partial_report() {
    let error = Error::failed_source();
    assert_eq!(error.kind(), &ErrorKind::SourceFailed);
    assert_eq!(error.to_string(), "the source failed at byte 1040384");
    assert_eq!(
      error.source_failure().map(io::Error::kind),
      Some(io::ErrorKind::PermissionDenied)
    );
    assert!(std::error::Error::source(&error).is_some());
    assert_eq!(error.source_identifier(), Some(SourceIdentifier::sample(0)));
    assert_eq!(error.attempted_identity(), None);
    assert_eq!(error.offset(), Some(1_040_384));
    assert_eq!(error.logical_offset(), None);
    let partial = error.partial_report().map(MediaReport::completeness);
    assert_eq!(partial.map(Completeness::tags), Some(&SectionStatus::Incomplete));
    assert_eq!(partial.map(Completeness::record), Some(error.record()));
    let cut = error.record().first_reason(&Part::Tags);
    assert_eq!(
      cut.map(|reason| reason.reason().kind()),
      Some(&ReasonKind::SourceFailed)
    );
    assert_eq!(
      cut.and_then(PartReason::source_identifier),
      Some(SourceIdentifier::sample(0))
    );
    assert_eq!(error.cause_messages(), ["access denied"]);
    assert!(error.into_partial_report().is_some());
  }

  #[test]
  fn names_the_companion_that_failed() {
    let error = Error::failed_companion();
    assert_eq!(error.to_string(), "the source failed at byte 2048");
    assert_eq!(error.source_identifier(), Some(SourceIdentifier::sample(1)));
    assert_eq!(
      error.attempted_identity().and_then(AttemptedIdentity::requested_name),
      Some("movie.sub")
    );
    assert_eq!(error.partial_report(), None);
  }

  #[test]
  fn says_that_a_source_could_not_be_opened() {
    let error = Error::unopened_source();
    assert_eq!(error.to_string(), "the source could not be opened");
    assert_eq!(error.offset(), None);
    assert_eq!(Error::sample(ErrorKind::SourceFailed).to_string(), "the source failed");
  }

  #[test]
  fn gives_each_message_of_a_cycle_once() {
    let looping = Error::failed_with(io::Error::other(Looping));
    assert_eq!(looping.cause_messages(), ["looping", "looping"]);
    let nested = Error::failed_with(io::Error::other(Middle {
      outer: Outer {
        inner: Inner { code: 7 },
      },
    }));
    assert_eq!(nested.cause_messages(), ["middle", "outer", "inner 7"]);
  }

  #[test]
  fn leaves_out_a_repeated_message_and_follows_its_causes() {
    let echoing = Error::failed_with(io::Error::other(Holder {
      wrapper: Wrapper {
        leaf: Leaf {
          deeper: Box::new(Inner { code: 9 }),
        },
      },
    }));
    assert_eq!(echoing.cause_messages(), ["holder", "same", "inner 9"]);
  }

  #[test]
  fn follows_at_most_thirty_two_causes() {
    let requests = Arc::new(AtomicUsize::new(0));
    let rest = (1..40).rev().fold(None, |next, depth| {
      Some(Box::new(Link {
        depth,
        next,
        requests: Arc::clone(&requests),
      }))
    });
    let long = Error::failed_with(io::Error::other(Link {
      depth: 0,
      next: rest,
      requests: Arc::clone(&requests),
    }));
    let messages = long.cause_messages();
    assert_eq!(messages.len(), 32);
    assert_eq!(messages.first().map(String::as_str), Some("link 0"));
    assert_eq!(messages.last().map(String::as_str), Some("link 31"));
    // The I/O error asks link 0 for its cause, and links 1 to 30 are asked in turn; link 31, the 32nd cause, never is.
    assert_eq!(requests.load(Ordering::Relaxed), 31);
  }

  #[test]
  fn says_what_stopped_detection() {
    let error = Error::stopped_detection();
    assert_eq!(error.to_string(), "the format could not be determined");
    assert_eq!(error.reason().and_then(Reason::limit), Some(&Limit::BytesRead));
    assert_eq!(error.record().first_stopped(&Limit::BytesRead), Some(&Part::Detection));
    assert_eq!(error.candidates(), []);
    assert_eq!((error.hint(), error.format(), error.observations()), (None, None, None));
    assert_eq!(error.usage(), &Usage::sample());
    assert!(std::error::Error::source(&error).is_none());
    assert_eq!(error.cause_messages(), Vec::<String>::new());
    assert!(format!("{error:?}").starts_with("Error { data: ErrorData"));
  }

  #[test]
  fn leaves_each_unchecked_candidate_its_own_cause() {
    let error = Error::unchecked_candidate();
    assert_eq!(error.kind(), &ErrorKind::FormatNotDetermined);
    assert_eq!(error.reason(), None);
    let candidate = error.candidates().first();
    assert_eq!(candidate.map(CandidateCheck::state), Some(&CheckState::NotCheckable));
    assert_eq!(
      candidate.and_then(CandidateCheck::reason).map(Reason::kind),
      Some(&ReasonKind::RangeUnavailable)
    );
    assert_eq!(error.record(), &LimitRecord::empty());
    assert_eq!((error.hint(), error.format()), (None, None));
  }

  #[test]
  fn keeps_the_hint_of_an_unknown_format() {
    let error = Error::hinted_format();
    assert_eq!(error.to_string(), "the format is unknown");
    assert_eq!(error.hint().map(Hint::format), Some(&Format::Mp4));
    assert_eq!(error.candidates(), []);
    assert_eq!((error.reason(), error.format()), (None, None));
  }

  #[test]
  fn keeps_what_it_observed_before_the_inventory() {
    let error = Error::stopped_before_the_inventory();
    assert_eq!(error.to_string(), "the probe stopped before the stream inventory");
    assert_eq!(error.format(), Some(&Format::IsoBaseMedia));
    assert_eq!(
      error.reason().map(Reason::kind),
      Some(&ReasonKind::MediaSegmentWithoutInitializationData)
    );
    assert_eq!(error.observations(), Some(&Observations::sample()));
    assert_eq!(error.partial_report(), None);
  }
}
