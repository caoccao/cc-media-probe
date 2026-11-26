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

//! `SegmentReference`, a chapter's link to the media of another segment (`api.md` section 6.8).

use crate::form_record::form_record;
use crate::model::Identifier;

form_record! {
  /// A chapter's link to the media of another segment, which the probe never opens.
  ///
  /// ```
  /// use cc_media_probe::model::Chapter;
  ///
  /// fn linked_segment(chapter: &Chapter) -> Option<String> {
  ///     let reference = chapter.segment_reference()?;
  ///     Some(match reference.edition() {
  ///         Some(edition) => format!("segment {} edition {edition}", reference.segment()),
  ///         None => format!("segment {}", reference.segment()),
  ///     })
  /// }
  /// ```
  #[derive(Clone, Debug, PartialEq, Eq, Hash)]
  pub struct SegmentReference {
    segment: Identifier,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    edition: Option<Identifier>,
  }
}

impl SegmentReference {
  /// The linked segment's identifier.
  #[must_use]
  pub fn segment(&self) -> &Identifier {
    &self.segment
  }

  /// The linked edition's identifier, where the chapter names one.
  #[must_use]
  pub fn edition(&self) -> Option<&Identifier> {
    self.edition.as_ref()
  }
}

#[cfg(test)]
mod tests {
  use super::SegmentReference;
  use crate::model::Identifier;

  impl SegmentReference {
    /// A link to this segment.
    pub(crate) fn sample(segment: &str) -> SegmentReference {
      SegmentReference {
        segment: Identifier::sample(segment),
        edition: Some(Identifier::sample("7")),
      }
    }
  }

  #[test]
  fn names_the_segment_and_the_edition() {
    let reference = SegmentReference::sample("00112233445566778899aabbccddeeff");
    assert_eq!(reference.segment().as_str(), "00112233445566778899aabbccddeeff");
    assert_eq!(reference.edition().map(Identifier::as_str), Some("7"));
  }
}
