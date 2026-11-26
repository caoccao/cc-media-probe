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

//! `Part`, what a reason stopped (`api.md` section 5.3).

use super::constants::{
  ATTACHMENTS_NAME, CHAPTERS_NAME, COMPANIONS_NAME, DETECTION_NAME, DISCOVERY_NAME, HEADERS_NAME, TAGS_NAME,
  TIMING_NAME,
};
use crate::string_enumeration::string_enumeration;

string_enumeration! {
  /// What a reason stopped, in the order that the fixed-size record keeps.
  ///
  /// ```
  /// use cc_media_probe::completeness::{Completeness, Part};
  ///
  /// fn why_tags_fell_short(completeness: &Completeness) -> Option<&str> {
  ///     let first = completeness.record().first_reason(&Part::Tags)?;
  ///     Some(first.reason().kind().as_str())
  /// }
  ///
  /// assert_eq!(Part::Discovery.as_str(), "discovery");
  /// ```
  Part {
    /// Detection, with the validation checks of the candidates.
    Detection => DETECTION_NAME,
    /// Stream discovery, with each reported stream's declaration.
    Discovery => DISCOVERY_NAME,
    /// The streams' headers.
    Headers => HEADERS_NAME,
    /// The tags.
    Tags => TAGS_NAME,
    /// The chapters.
    Chapters => CHAPTERS_NAME,
    /// The attachments of the requested roles.
    Attachments => ATTACHMENTS_NAME,
    /// The timing: durations and start times.
    Timing => TIMING_NAME,
    /// The companions that the input expects.
    Companions => COMPANIONS_NAME,
  }
}

#[cfg(feature = "serde")]
impl Part {
  /// Whether the part finds external references, whose omission it records (`api.md` section 6.12): discovery, the
  /// chapters, the attachments and the companions, and a part of a newer version, which only its version knows.
  pub(crate) fn finds_references(&self) -> bool {
    matches!(
      self,
      Part::Discovery | Part::Chapters | Part::Attachments | Part::Companions | Part::Unrecognized(_)
    )
  }
}

#[cfg(test)]
mod tests {
  use super::Part;
  use crate::model::UnrecognizedValue;

  #[test]
  fn names_each_part() {
    let parts = [
      (Part::Detection, "detection"),
      (Part::Discovery, "discovery"),
      (Part::Headers, "headers"),
      (Part::Tags, "tags"),
      (Part::Chapters, "chapters"),
      (Part::Attachments, "attachments"),
      (Part::Timing, "timing"),
      (Part::Companions, "companions"),
      (Part::Unrecognized(UnrecognizedValue::sample("programs")), "programs"),
    ];
    for (part, name) in parts {
      assert_eq!(part.as_str(), name);
    }
  }
}
