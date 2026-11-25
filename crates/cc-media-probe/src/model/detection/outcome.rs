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

//! `DetectionOutcome`, the six outcomes of detection (`api.md` section 8; plan section 8.6).

use super::constants::{
  AMBIGUOUS_NAME, FAMILY_ONLY_NAME, HINT_ONLY_NAME, IDENTIFIED_NAME, INSUFFICIENT_DATA_NAME, UNKNOWN_NAME,
};
use crate::string_enumeration::string_enumeration;

string_enumeration! {
  /// What detection concluded. Each outcome has its own accessors of `Detection`; the others are absent or empty.
  ///
  /// ```
  /// use cc_media_probe::Detection;
  /// use cc_media_probe::model::DetectionOutcome;
  ///
  /// fn summary(detection: &Detection) -> String {
  ///     match detection.outcome() {
  ///         DetectionOutcome::Identified | DetectionOutcome::FamilyOnly => {
  ///             format!("{:?}", detection.format().map(|format| format.name()))
  ///         }
  ///         DetectionOutcome::Ambiguous => format!("{} candidates", detection.candidates().len()),
  ///         DetectionOutcome::HintOnly => "only a hint, not verified".to_owned(),
  ///         DetectionOutcome::InsufficientData => format!("needs {:?} bytes", detection.bytes_needed()),
  ///         other => other.as_str().to_owned(),
  ///     }
  /// }
  ///
  /// assert_eq!(DetectionOutcome::FamilyOnly.as_str(), "family_only");
  /// ```
  DetectionOutcome {
    /// One format, with a confidence.
    Identified => IDENTIFIED_NAME,
    /// The family is certain and the specific type is not.
    FamilyOnly => FAMILY_ONLY_NAME,
    /// Several candidates, in rank.
    Ambiguous => AMBIGUOUS_NAME,
    /// The bytes matched nothing, and a hint suggests a format, which is not verified.
    HintOnly => HINT_ONLY_NAME,
    /// Too few bytes to decide.
    InsufficientData => INSUFFICIENT_DATA_NAME,
    /// Nothing matched, and there is no hint.
    Unknown => UNKNOWN_NAME,
  }
}

#[cfg(test)]
mod tests {
  use super::DetectionOutcome;
  use crate::model::UnrecognizedValue;

  #[test]
  fn names_each_outcome() {
    for (outcome, name) in [
      (DetectionOutcome::Identified, "identified"),
      (DetectionOutcome::FamilyOnly, "family_only"),
      (DetectionOutcome::Ambiguous, "ambiguous"),
      (DetectionOutcome::HintOnly, "hint_only"),
      (DetectionOutcome::InsufficientData, "insufficient_data"),
      (DetectionOutcome::Unknown, "unknown"),
      (
        DetectionOutcome::Unrecognized(UnrecognizedValue::sample("deferred")),
        "deferred",
      ),
    ] {
      assert_eq!(outcome.as_str(), name);
    }
  }
}
