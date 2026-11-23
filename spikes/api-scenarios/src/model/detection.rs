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

//! `Detection`, `api.md` section 8, as far as a document entry reads it.

use super::constants::{
  AMBIGUOUS_NAME, FAMILY_ONLY_NAME, HINT_ONLY_NAME, IDENTIFIED_NAME, INSUFFICIENT_DATA_NAME, UNKNOWN_NAME,
};
use super::format::Format;
use super::unrecognized_value::UnrecognizedValue;
#[cfg(feature = "serde")]
use crate::document::SchemaVersion;

/// What a detection established about one input, a root record: serialized, it carries its schema version.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Detection {
  #[cfg(feature = "serde")]
  #[cfg_attr(feature = "serde", serde(deserialize_with = "crate::document::read_current_major"))]
  schema_version: SchemaVersion,
  outcome: DetectionOutcome,
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
  format: Option<Format>,
}

impl Detection {
  /// The outcome.
  pub fn outcome(&self) -> &DetectionOutcome {
    &self.outcome
  }

  /// The identified format or the family.
  pub fn format(&self) -> Option<&Format> {
    self.format.as_ref()
  }

  /// The schema version the record was written with, which a document checks against its own.
  #[cfg(feature = "serde")]
  pub(crate) fn schema_version(&self) -> SchemaVersion {
    self.schema_version
  }

  /// The record with another schema version: a document gives each record it holds its own.
  #[cfg(feature = "serde")]
  pub(crate) fn with_schema_version(mut self, version: SchemaVersion) -> Detection {
    self.schema_version = version;
    self
  }
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
