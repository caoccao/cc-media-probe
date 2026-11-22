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
use super::values::UnrecognizedValue;

/// What a detection established about one input.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Detection {
  outcome: DetectionOutcome,
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
}

/// The outcome of a detection.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
  Unrecognized(UnrecognizedValue),
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
