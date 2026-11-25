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

//! `ContentLightLevel`, a video's light levels (`api.md` section 6.5).

use crate::model::Provenance;

/// A video's content light levels, in candelas per square meter, each with its own provenance.
///
/// ```
/// use cc_media_probe::model::ContentLightLevel;
///
/// fn describe(level: &ContentLightLevel) -> String {
///     format!(
///         "MaxCLL {:?} ({:?}), MaxFALL {:?} ({:?})",
///         level.maximum_content(),
///         level.maximum_content_provenance(),
///         level.maximum_frame_average(),
///         level.maximum_frame_average_provenance(),
///     )
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ContentLightLevel {
  content: Option<u64>,
  content_provenance: Option<Provenance>,
  frame_average: Option<u64>,
  frame_average_provenance: Option<Provenance>,
}

impl ContentLightLevel {
  /// The maximum content light level.
  #[must_use]
  pub fn maximum_content(&self) -> Option<u64> {
    self.content
  }

  /// Where the maximum content light level came from.
  #[must_use]
  pub fn maximum_content_provenance(&self) -> Option<&Provenance> {
    self.content_provenance.as_ref()
  }

  /// The maximum frame-average light level.
  #[must_use]
  pub fn maximum_frame_average(&self) -> Option<u64> {
    self.frame_average
  }

  /// Where the maximum frame-average light level came from.
  #[must_use]
  pub fn maximum_frame_average_provenance(&self) -> Option<&Provenance> {
    self.frame_average_provenance.as_ref()
  }
}

#[cfg(test)]
mod tests {
  use super::ContentLightLevel;
  use crate::model::Provenance;

  impl ContentLightLevel {
    /// Levels stated by the container and the codec header in turn.
    pub(crate) fn sample() -> ContentLightLevel {
      ContentLightLevel {
        content: Some(1_000),
        content_provenance: Some(Provenance::Declared),
        frame_average: Some(400),
        frame_average_provenance: Some(Provenance::Bitstream),
      }
    }
  }

  #[test]
  fn keeps_each_level_with_its_provenance() {
    let level = ContentLightLevel::sample();
    assert_eq!(level.maximum_content(), Some(1_000));
    assert_eq!(level.maximum_content_provenance(), Some(&Provenance::Declared));
    assert_eq!(level.maximum_frame_average(), Some(400));
    assert_eq!(level.maximum_frame_average_provenance(), Some(&Provenance::Bitstream));
  }
}
