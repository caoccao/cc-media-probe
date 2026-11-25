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

//! `Disposition`, a stream's flags (`api.md` section 6.3).

/// A stream's flags, each absent when the format states nothing, never assumed false.
///
/// ```
/// use cc_media_probe::Stream;
///
/// fn flags(stream: &Stream) -> Vec<&'static str> {
///     let disposition = stream.disposition();
///     [
///         (disposition.is_default(), "default"),
///         (disposition.is_enabled(), "enabled"),
///         (disposition.is_forced(), "forced"),
///         (disposition.is_hearing_impaired(), "hearing impaired"),
///         (disposition.is_visually_impaired(), "visually impaired"),
///         (disposition.has_text_descriptions(), "text descriptions"),
///         (disposition.is_original_language(), "original language"),
///         (disposition.is_commentary(), "commentary"),
///     ]
///     .into_iter()
///     .filter_map(|(flag, name)| (flag == Some(true)).then_some(name))
///     .collect()
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Disposition {
  default: Option<bool>,
  enabled: Option<bool>,
  forced: Option<bool>,
  hearing_impaired: Option<bool>,
  visually_impaired: Option<bool>,
  text_descriptions: Option<bool>,
  original_language: Option<bool>,
  commentary: Option<bool>,
}

impl Disposition {
  /// Whether the stream is a default of its kind.
  #[must_use]
  pub fn is_default(&self) -> Option<bool> {
    self.default
  }

  /// Whether the stream is enabled.
  #[must_use]
  pub fn is_enabled(&self) -> Option<bool> {
    self.enabled
  }

  /// Whether a player should show the stream even when the user chose none, as forced subtitles are.
  #[must_use]
  pub fn is_forced(&self) -> Option<bool> {
    self.forced
  }

  /// Whether the stream is meant for the hearing impaired.
  #[must_use]
  pub fn is_hearing_impaired(&self) -> Option<bool> {
    self.hearing_impaired
  }

  /// Whether the stream is meant for the visually impaired.
  #[must_use]
  pub fn is_visually_impaired(&self) -> Option<bool> {
    self.visually_impaired
  }

  /// Whether the stream holds text descriptions of the video.
  #[must_use]
  pub fn has_text_descriptions(&self) -> Option<bool> {
    self.text_descriptions
  }

  /// Whether the stream is in the content's original language.
  #[must_use]
  pub fn is_original_language(&self) -> Option<bool> {
    self.original_language
  }

  /// Whether the stream is commentary.
  #[must_use]
  pub fn is_commentary(&self) -> Option<bool> {
    self.commentary
  }
}

#[cfg(test)]
mod tests {
  use super::Disposition;

  impl Disposition {
    /// A disposition that states nothing.
    pub(crate) fn unstated() -> Disposition {
      Disposition {
        default: None,
        enabled: None,
        forced: None,
        hearing_impaired: None,
        visually_impaired: None,
        text_descriptions: None,
        original_language: None,
        commentary: None,
      }
    }

    /// A disposition that marks the stream as a default.
    pub(crate) fn default_stream() -> Disposition {
      Disposition {
        default: Some(true),
        enabled: Some(true),
        ..Disposition::unstated()
      }
    }
  }

  #[test]
  fn keeps_each_flag_apart() {
    let disposition = Disposition {
      default: Some(true),
      enabled: Some(false),
      forced: Some(true),
      hearing_impaired: Some(false),
      visually_impaired: Some(true),
      text_descriptions: Some(false),
      original_language: Some(true),
      commentary: Some(false),
    };
    assert_eq!(
      [
        disposition.is_default(),
        disposition.is_enabled(),
        disposition.is_forced(),
        disposition.is_hearing_impaired(),
        disposition.is_visually_impaired(),
        disposition.has_text_descriptions(),
        disposition.is_original_language(),
        disposition.is_commentary(),
      ],
      [
        Some(true),
        Some(false),
        Some(true),
        Some(false),
        Some(true),
        Some(false),
        Some(true),
        Some(false)
      ]
    );
    assert_eq!(Disposition::unstated().is_default(), None);
    assert_eq!(Disposition::default_stream().is_default(), Some(true));
  }
}
