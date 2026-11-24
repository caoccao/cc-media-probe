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

//! `Detection`, `api.md` section 8, as far as a document entry and its tests read it. Since the fourteenth review it
//! has the members of every outcome, which reading checks outcome by outcome (`super::detection_reading`). Since the
//! fifteenth review it has its usage, and a detection of a newer outcome keeps the members that it does not read, as
//! the maintainer chose.

use super::constants::{
  AMBIGUOUS_NAME, FAMILY_ONLY_NAME, HINT_ONLY_NAME, IDENTIFIED_NAME, INSUFFICIENT_DATA_NAME, UNKNOWN_NAME,
};
use super::format::Format;
use super::unrecognized_value::UnrecognizedValue;
use super::usage::Usage;
use crate::completeness::Reason;
#[cfg(feature = "serde")]
use crate::document::SchemaVersion;
#[cfg(feature = "serde")]
use crate::kept::KeptMembers;

/// What a detection established about one input, a root record: serialized, it carries its schema version.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(
  feature = "serde",
  derive(serde::Serialize, serde::Deserialize),
  serde(
    into = "super::detection_reading::DetectionMembers",
    try_from = "crate::neutral::Rooted<super::detection_reading::DetectionData>"
  )
)]
pub struct Detection {
  #[cfg(feature = "serde")]
  pub(super) schema_version: SchemaVersion,
  pub(super) outcome: DetectionOutcome,
  pub(super) format: Option<Format>,
  pub(super) confidence: Option<u8>,
  pub(super) candidates: Vec<Candidate>,
  pub(super) hint: Option<Hint>,
  pub(super) bytes_needed: Option<u64>,
  pub(super) insufficient_reason: Option<Reason>,
  pub(super) usage: Usage,
  /// For an outcome that a newer version wrote, the members that this version does not read, as written, which the
  /// detection writes back after its own; empty for an outcome that this version knows (fifteenth review).
  #[cfg(feature = "serde")]
  pub(super) kept: KeptMembers,
}

impl Detection {
  /// The outcome.
  pub fn outcome(&self) -> &DetectionOutcome {
    &self.outcome
  }

  /// The identified format or the family. A hinted format is under `hint()`, never here.
  pub fn format(&self) -> Option<&Format> {
    self.format.as_ref()
  }

  /// For an identification, from 0 to 100.
  pub fn confidence(&self) -> Option<u8> {
    self.confidence
  }

  /// For an ambiguous outcome, in rank order, which falls with the confidence.
  pub fn candidates(&self) -> &[Candidate] {
    &self.candidates
  }

  /// For a hint only; not verified.
  pub fn hint(&self) -> Option<&Hint> {
    self.hint.as_ref()
  }

  /// For insufficient data, when known: the least length of the input, from its start, that detection needs.
  pub fn bytes_needed(&self) -> Option<u64> {
    self.bytes_needed
  }

  /// For insufficient data: whether the input ended or a named limit was reached.
  pub fn insufficient_reason(&self) -> Option<&Reason> {
    self.insufficient_reason.as_ref()
  }

  /// What the budget accounted for.
  pub fn usage(&self) -> &Usage {
    &self.usage
  }

  /// The schema version that the record was read with, or the current one for a record that this version made. A
  /// document gives each record it holds its own, and checks it against its own (twelfth review).
  #[cfg(feature = "serde")]
  pub fn schema_version(&self) -> SchemaVersion {
    self.schema_version
  }

  /// The record with another schema version: a document gives each record it holds its own.
  #[cfg(feature = "serde")]
  pub(crate) fn with_schema_version(mut self, version: SchemaVersion) -> Detection {
    self.schema_version = version;
    self
  }
}

/// One candidate of an ambiguous detection. Read alone, it checks its confidence as a detection does (fifteenth
/// review).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Candidate {
  pub(super) format: Format,
  pub(super) confidence: u8,
}

#[cfg(feature = "serde")]
crate::neutral::checked_record_traits!(Candidate from CandidateData);

/// A candidate's members as read, before its confidence is checked.
#[cfg(feature = "serde")]
#[derive(serde::Deserialize)]
pub(super) struct CandidateData {
  pub(super) format: Format,
  pub(super) confidence: u8,
}

impl Candidate {
  /// The candidate's format.
  pub fn format(&self) -> &Format {
    &self.format
  }

  /// Its confidence, from 0 to 100.
  pub fn confidence(&self) -> u8 {
    self.confidence
  }
}

/// A format that a hint suggests, not verified.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(
  feature = "serde",
  derive(serde::Serialize, serde::Deserialize),
  serde(remote = "Self")
)]
pub struct Hint {
  format: Format,
  origin: HintOrigin,
}

#[cfg(feature = "serde")]
crate::neutral::record_traits!(Hint);

impl Hint {
  /// The hinted format.
  pub fn format(&self) -> &Format {
    &self.format
  }

  /// Where the hint came from.
  pub fn origin(&self) -> &HintOrigin {
    &self.origin
  }
}

/// Where a hint came from.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum HintOrigin {
  /// The file name.
  FileName,
  /// The format option of the builder.
  FormatOption,
  /// A value that a newer version wrote.
  #[cfg_attr(feature = "serde", serde(with = "crate::model::unrecognized_text"))]
  Unrecognized(UnrecognizedValue<HintOrigin>),
}

/// The outcome of a detection. Serialized as its stable name.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(into = "String", from = "String"))]
#[non_exhaustive]
pub enum DetectionOutcome {
  /// One format, identified.
  Identified,
  /// The family only.
  FamilyOnly,
  /// Several candidates.
  Ambiguous,
  /// Nothing but a hint.
  HintOnly,
  /// Too few bytes to decide.
  InsufficientData,
  /// Nothing matched.
  Unknown,
  /// A value that a newer version wrote.
  Unrecognized(UnrecognizedValue<DetectionOutcome>),
}

impl DetectionOutcome {
  /// The serialized name, or the text of an unrecognized value.
  pub fn as_str(&self) -> &str {
    match self {
      DetectionOutcome::Identified => IDENTIFIED_NAME,
      DetectionOutcome::FamilyOnly => FAMILY_ONLY_NAME,
      DetectionOutcome::Ambiguous => AMBIGUOUS_NAME,
      DetectionOutcome::HintOnly => HINT_ONLY_NAME,
      DetectionOutcome::InsufficientData => INSUFFICIENT_DATA_NAME,
      DetectionOutcome::Unknown => UNKNOWN_NAME,
      DetectionOutcome::Unrecognized(value) => value.as_str(),
    }
  }
}

#[cfg(feature = "serde")]
impl From<String> for DetectionOutcome {
  fn from(name: String) -> DetectionOutcome {
    match name.as_str() {
      IDENTIFIED_NAME => DetectionOutcome::Identified,
      FAMILY_ONLY_NAME => DetectionOutcome::FamilyOnly,
      AMBIGUOUS_NAME => DetectionOutcome::Ambiguous,
      HINT_ONLY_NAME => DetectionOutcome::HintOnly,
      INSUFFICIENT_DATA_NAME => DetectionOutcome::InsufficientData,
      UNKNOWN_NAME => DetectionOutcome::Unknown,
      _ => DetectionOutcome::Unrecognized(UnrecognizedValue::new(name)),
    }
  }
}

#[cfg(feature = "serde")]
impl From<DetectionOutcome> for String {
  fn from(outcome: DetectionOutcome) -> String {
    outcome.as_str().to_owned()
  }
}
