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
  COMPLETE_TEXT, INCOMPLETE_TEXT, LIMIT_EXHAUSTED_NAME, NOT_APPLICABLE_NAME, NOT_ATTEMPTED_NAME,
  NOT_FOUND_WITHIN_BUDGET_NAME, NOT_REQUESTED_NAME, PARSED_NAME, TRUNCATED_INPUT_NAME,
};
use crate::limits::Limit;
use crate::model::UnrecognizedValue;

/// What was established, what was not, and why.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Completeness {
  discovery: DiscoveryStatus,
  tags: SectionStatus,
  chapters: SectionStatus,
  attachments: SectionStatus,
  reasons: Vec<PartReason>,
}

impl Completeness {
  /// The definition of plan section 8.3; the stub checks discovery and the reasons only.
  pub fn is_complete(&self) -> bool {
    self.discovery == DiscoveryStatus::Complete && self.reasons.is_empty()
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
  /// A limit ended the search.
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
