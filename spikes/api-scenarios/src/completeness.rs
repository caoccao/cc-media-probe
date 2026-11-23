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

//! Completeness and header status, `api.md` sections 5.3 to 5.5, with a subset of their values.

use std::fmt;

use crate::constants::{
  CHAPTERS_NAME, COMPLETE_TEXT, DETECTION_NAME, DISCOVERY_NAME, HEADERS_NAME, INCOMPLETE_TEXT, LIMIT_EXHAUSTED_NAME,
  NOT_APPLICABLE_NAME, NOT_ATTEMPTED_NAME, NOT_FOUND_WITHIN_BUDGET_NAME, NOT_REQUESTED_NAME, PARSED_NAME,
  RANGE_UNAVAILABLE_NAME, TAGS_NAME, TRUNCATED_INPUT_NAME,
};
use crate::limits::Limit;
use crate::model::UnrecognizedValue;
use crate::source::SourceIdentifier;

/// What was established, what was not, and why.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Completeness {
  detection: DetectionStatus,
  discovery: DiscoveryStatus,
  tags: SectionStatus,
  chapters: SectionStatus,
  attachments: SectionStatus,
  reasons: Vec<PartReason>,
  record: LimitRecord,
}

impl Completeness {
  /// The definition of plan section 8.3; the stub checks detection, discovery and the reasons only.
  pub fn is_complete(&self) -> bool {
    self.detection == DetectionStatus::Complete
      && self.discovery == DiscoveryStatus::Complete
      && self.reasons.is_empty()
  }

  /// Whether every candidate's validation check that detection began ran to its end.
  pub fn detection(&self) -> &DetectionStatus {
    &self.detection
  }

  /// Whether every stream was found.
  pub fn discovery(&self) -> &DiscoveryStatus {
    &self.discovery
  }

  /// The tags section.
  pub fn tags(&self) -> &SectionStatus {
    &self.tags
  }

  /// The chapters section.
  pub fn chapters(&self) -> &SectionStatus {
    &self.chapters
  }

  /// The attachments section.
  pub fn attachments(&self) -> &SectionStatus {
    &self.attachments
  }

  /// Each shortfall with the part it stopped.
  pub fn reasons(&self) -> &[PartReason] {
    &self.reasons
  }

  /// The fixed-size record.
  pub fn record(&self) -> &LimitRecord {
    &self.record
  }

  /// Whether a status that this version knows records a shortfall, or a reason names one, which rules out a
  /// complete report whatever a status of a newer version says.
  #[cfg(feature = "serde")]
  pub(crate) fn has_known_shortfall(&self) -> bool {
    self.detection == DetectionStatus::Incomplete
      || self.discovery == DiscoveryStatus::Incomplete
      || [&self.tags, &self.chapters, &self.attachments]
        .into_iter()
        .any(|status| *status == SectionStatus::Incomplete)
      || !self.reasons.is_empty()
      || !self.record.first_reasons.is_empty()
  }

  /// Whether every status is one this version knows, so that `is_complete()` decides as the writer did.
  #[cfg(feature = "serde")]
  pub(crate) fn is_known(&self) -> bool {
    !matches!(self.detection, DetectionStatus::Unrecognized(_))
      && !matches!(self.discovery, DiscoveryStatus::Unrecognized(_))
      && [&self.tags, &self.chapters, &self.attachments]
        .into_iter()
        .all(|status| !matches!(status, SectionStatus::Unrecognized(_)))
  }
}

impl fmt::Display for Completeness {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self.reasons.first() {
      None if self.is_complete() => formatter.write_str(COMPLETE_TEXT),
      None => formatter.write_str(INCOMPLETE_TEXT),
      Some(first) => write!(formatter, "{INCOMPLETE_TEXT}: {}", first.reason.kind.as_str()),
    }
  }
}

/// Whether detection checked every candidate it began to check. A report exists once one candidate validated; a
/// limit that stopped another candidate's check leaves detection incomplete, since that candidate could have been
/// chosen.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum DetectionStatus {
  /// Every check that detection began ran to its end.
  #[default]
  Complete,
  /// A limit stopped a candidate's check.
  Incomplete,
  /// A value that a newer version wrote.
  Unrecognized(UnrecognizedValue),
}

/// Whether every stream was found.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum DiscoveryStatus {
  /// Every stream was found.
  #[default]
  Complete,
  /// Some may be missing.
  Incomplete,
  /// A value that a newer version wrote.
  Unrecognized(UnrecognizedValue),
}

/// The status of the tags, chapters or attachments.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum SectionStatus {
  /// Complete, including proven empty.
  #[default]
  Complete,
  /// Some items may be missing.
  Incomplete,
  /// The options asked to skip it.
  NotRequested,
  /// The format has no such section.
  NotApplicable,
  /// A value that a newer version wrote.
  Unrecognized(UnrecognizedValue),
}

impl SectionStatus {
  /// The serialized name, or the text of an unrecognized value.
  pub fn as_str(&self) -> &str {
    match self {
      SectionStatus::Complete => COMPLETE_TEXT,
      SectionStatus::Incomplete => INCOMPLETE_TEXT,
      SectionStatus::NotRequested => NOT_REQUESTED_NAME,
      SectionStatus::NotApplicable => NOT_APPLICABLE_NAME,
      SectionStatus::Unrecognized(value) => value.as_str(),
    }
  }
}

/// One shortfall and the part it stopped.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PartReason {
  part: Part,
  stream: Option<usize>,
  source_identifier: Option<SourceIdentifier>,
  reason: Reason,
}

impl PartReason {
  /// The part the reason stopped.
  pub fn part(&self) -> &Part {
    &self.part
  }

  /// The stream, for a stream's headers.
  pub fn stream(&self) -> Option<usize> {
    self.stream
  }

  /// The source table's entry of the request, for a companion's reason.
  pub fn source_identifier(&self) -> Option<SourceIdentifier> {
    self.source_identifier
  }

  /// Why.
  pub fn reason(&self) -> &Reason {
    &self.reason
  }
}

/// What a reason stopped.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum Part {
  /// Detection, with the validation checks of the candidates. An error names it when a limit ended detection, and a
  /// report when a limit stopped one candidate's check while another validated.
  Detection,
  /// Stream discovery.
  Discovery,
  /// A stream's headers.
  Headers,
  /// The tags.
  Tags,
  /// The chapters.
  Chapters,
  /// A value that a newer version wrote.
  Unrecognized(UnrecognizedValue),
}

impl Part {
  /// The serialized name, or the text of an unrecognized value.
  pub fn as_str(&self) -> &str {
    match self {
      Part::Detection => DETECTION_NAME,
      Part::Discovery => DISCOVERY_NAME,
      Part::Headers => HEADERS_NAME,
      Part::Tags => TAGS_NAME,
      Part::Chapters => CHAPTERS_NAME,
      Part::Unrecognized(value) => value.as_str(),
    }
  }
}

/// The fixed-size record of `api.md` section 5.3, with the accessors that the scenarios and the tests read. The
/// library's record has a fixed size, one slot per counter and per part; the stub keeps short lists.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LimitRecord {
  exhausted: Vec<Limit>,
  first_stopped: Vec<(Limit, Part)>,
  #[cfg_attr(feature = "serde", serde(default))]
  first_reasons: Vec<PartReason>,
  #[cfg_attr(feature = "serde", serde(default))]
  omitted_reasons: u64,
}

impl LimitRecord {
  /// The first reason recorded against the part, kept whatever the limits, without its detail text.
  pub fn first_reason(&self, part: &Part) -> Option<&PartReason> {
    self.first_reasons.iter().find(|first| first.part == *part)
  }

  /// The reasons recorded but not kept in `Completeness::reasons()`.
  pub fn omitted_reasons(&self) -> u64 {
    self.omitted_reasons
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

/// Why something was not established.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Reason {
  kind: ReasonKind,
  limit: Option<Limit>,
  detail: Option<String>,
}

impl Reason {
  /// The kind of reason.
  pub fn kind(&self) -> &ReasonKind {
    &self.kind
  }

  /// The limit, for an exhausted one.
  pub fn limit(&self) -> Option<&Limit> {
    self.limit.as_ref()
  }

  /// More detail, for people.
  pub fn detail(&self) -> Option<&str> {
    self.detail.as_deref()
  }
}

/// The kinds of reason, plan section 8.3.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum ReasonKind {
  /// On a forward-only source, the bytes fell behind the read window before they were needed.
  RangeUnavailable,
  /// The input ended early.
  TruncatedInput,
  /// A limit was reached.
  LimitExhausted,
  /// A value that a newer version wrote.
  Unrecognized(UnrecognizedValue),
}

impl ReasonKind {
  /// The serialized name, or the text of an unrecognized value.
  pub fn as_str(&self) -> &str {
    match self {
      ReasonKind::RangeUnavailable => RANGE_UNAVAILABLE_NAME,
      ReasonKind::TruncatedInput => TRUNCATED_INPUT_NAME,
      ReasonKind::LimitExhausted => LIMIT_EXHAUSTED_NAME,
      ReasonKind::Unrecognized(value) => value.as_str(),
    }
  }
}

/// The outcome of header parsing for one stream.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HeaderStatus {
  state: HeaderState,
  reason: Option<Reason>,
}

impl HeaderStatus {
  /// The state.
  pub fn state(&self) -> &HeaderState {
    &self.state
  }

  /// Why the headers were not parsed, where a reason applies.
  pub fn reason(&self) -> Option<&Reason> {
    self.reason.as_ref()
  }

  /// Whether every header field was read.
  pub fn is_parsed(&self) -> bool {
    self.state == HeaderState::Parsed
  }
}

impl fmt::Display for HeaderStatus {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    match &self.reason {
      Some(reason) => write!(formatter, "{} ({})", self.state.as_str(), reason.kind.as_str()),
      None => formatter.write_str(self.state.as_str()),
    }
  }
}

/// The states of header parsing, a subset of `api.md` section 5.4.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum HeaderState {
  /// Every header field read.
  Parsed,
  /// No header field was read before the input ended, a limit ended the search, or the range fell behind the
  /// forward-only window.
  NotFoundWithinBudget,
  /// The probe stopped before the stream's turn.
  NotAttempted,
  /// A value that a newer version wrote; not final.
  Unrecognized(UnrecognizedValue),
}

impl HeaderState {
  /// The serialized name, or the text of an unrecognized value.
  pub fn as_str(&self) -> &str {
    match self {
      HeaderState::Parsed => PARSED_NAME,
      HeaderState::NotFoundWithinBudget => NOT_FOUND_WITHIN_BUDGET_NAME,
      HeaderState::NotAttempted => NOT_ATTEMPTED_NAME,
      HeaderState::Unrecognized(value) => value.as_str(),
    }
  }
}
