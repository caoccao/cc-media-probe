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

//! `MediaType`, a media type such as `video/mp4` (`api.md` section 6.1).

use std::fmt;

/// A media type, such as `video/mp4`, reported where the IANA registry holds it or an approved specification of
/// the format names it (F3).
///
/// ```
/// use cc_media_probe::MediaReport;
///
/// fn is_video(report: &MediaReport) -> bool {
///     report.media_type().is_some_and(|media_type| media_type.as_str().starts_with("video/"))
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MediaType(String);

#[cfg(feature = "serde")]
crate::form::text_form!(MediaType);

impl MediaType {
  /// The media type as text. `Display` writes the same text.
  ///
  /// ```
  /// use cc_media_probe::MediaReport;
  ///
  /// fn media_type(report: &MediaReport) -> &str {
  ///     report.media_type().map_or("unknown", |media_type| media_type.as_str())
  /// }
  /// ```
  #[must_use]
  pub fn as_str(&self) -> &str {
    &self.0
  }
}

impl fmt::Display for MediaType {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.0)
  }
}

#[cfg(test)]
mod tests {
  use super::MediaType;

  impl MediaType {
    /// A media type with this text.
    pub(crate) fn sample(text: &str) -> MediaType {
      MediaType(text.to_owned())
    }
  }

  #[test]
  fn keeps_its_text() {
    let media_type = MediaType::sample("video/mp4");
    assert_eq!(media_type.as_str(), "video/mp4");
    assert_eq!(media_type.to_string(), "video/mp4");
  }
}
