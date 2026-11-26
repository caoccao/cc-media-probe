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

//! `Format`, one variant per entry of the detection inventory and one per family (`api.md` section 6.4).

use std::fmt;

use super::SupportLevel;
use super::constants::{
  AVI_NAME, AVI_TITLE, F4V_NAME, F4V_TITLE, ISO_BASE_MEDIA_NAME, ISO_BASE_MEDIA_TITLE, MATROSKA_NAME, MATROSKA_TITLE,
  MP4_NAME, MP4_TITLE, QUICKTIME_NAME, QUICKTIME_TITLE, RIFF_NAME, RIFF_TITLE, THREE_GPP_NAME, THREE_GPP_TITLE,
  THREE_GPP2_NAME, THREE_GPP2_TITLE, WEBM_NAME, WEBM_TITLE,
};
use crate::string_enumeration::string_enumeration;

string_enumeration! {
  /// The identity of a container format: one variant per entry of the detection inventory, which G3.5 freezes,
  /// and one per family. A family, `IsoBaseMedia` or `Riff`, is reported when detection is certain of the family
  /// only (plan section 8.6).
  ///
  /// ```
  /// use cc_media_probe::Format;
  /// use cc_media_probe::model::SupportLevel;
  ///
  /// assert_eq!(Format::Mp4.name(), "MP4");
  /// assert_eq!(Format::Mp4.as_str(), "mp4");
  /// assert_eq!(Format::Mp4.family(), Some(Format::IsoBaseMedia));
  /// assert!(Format::IsoBaseMedia.is_family());
  /// assert_eq!(Format::Avi.support(), Some(SupportLevel::Probed));
  /// assert_eq!(Format::WebM.to_string(), "WebM");
  /// ```
  Format (
    /// The library's name for the format, such as "MP4", or for a format that a newer version wrote, the name it
    /// wrote.
    ///
    /// ```
    /// use cc_media_probe::MediaReport;
    ///
    /// fn container(report: &MediaReport) -> &str {
    ///     report.format().name()
    /// }
    /// ```
    pub fn name
  ) {
    /// Matroska (RFC 9559).
    Matroska => MATROSKA_NAME, MATROSKA_TITLE,
    /// WebM, the Matroska profile of the WebM project.
    WebM => WEBM_NAME, WEBM_TITLE,
    /// MP4 (ISO/IEC 14496-14), of the ISO base media family.
    Mp4 => MP4_NAME, MP4_TITLE,
    /// 3GPP (3GPP TS 26.244), of the ISO base media family.
    ThreeGpp => THREE_GPP_NAME, THREE_GPP_TITLE,
    /// 3GPP2 (3GPP2 C.S0050), of the ISO base media family.
    ThreeGpp2 => THREE_GPP2_NAME, THREE_GPP2_TITLE,
    /// F4V, of the ISO base media family.
    F4v => F4V_NAME, F4V_TITLE,
    /// QuickTime, which the walker of the ISO base media family reads.
    QuickTime => QUICKTIME_NAME, QUICKTIME_TITLE,
    /// AVI, of the RIFF family.
    Avi => AVI_NAME, AVI_TITLE,
    /// The ISO base media family (ISO/IEC 14496-12), when no known brand names its type.
    IsoBaseMedia => ISO_BASE_MEDIA_NAME, ISO_BASE_MEDIA_TITLE,
    /// The RIFF family, when its form type is not one that this version lists.
    Riff => RIFF_NAME, RIFF_TITLE,
  }
}

impl Format {
  /// Whether this version probes the format or only detects it, or `None` for a format that a newer version
  /// wrote. The family `Riff` is detect-only: its walker depends on the specific type, so a probe of the family
  /// alone ends with the error of kind `UnsupportedFormat` (`api.md` section 5.1).
  ///
  /// ```
  /// use cc_media_probe::Format;
  /// use cc_media_probe::model::SupportLevel;
  ///
  /// assert_eq!(Format::Riff.support(), Some(SupportLevel::DetectOnly));
  /// ```
  #[must_use]
  pub fn support(&self) -> Option<SupportLevel> {
    match self {
      Format::Riff => Some(SupportLevel::DetectOnly),
      Format::Unrecognized(_) => None,
      _ => Some(SupportLevel::Probed),
    }
  }

  /// Whether the value names a family rather than a specific format; false for a format that a newer version
  /// wrote.
  ///
  /// ```
  /// use cc_media_probe::Format;
  ///
  /// assert!(Format::Riff.is_family());
  /// assert!(!Format::Avi.is_family());
  /// ```
  #[must_use]
  pub fn is_family(&self) -> bool {
    matches!(self, Format::IsoBaseMedia | Format::Riff)
  }

  /// The family that the format belongs to, such as `IsoBaseMedia` for `Mp4`, or `None` for a family, a format of
  /// no family and a format that a newer version wrote.
  ///
  /// ```
  /// use cc_media_probe::Format;
  ///
  /// assert_eq!(Format::Avi.family(), Some(Format::Riff));
  /// assert_eq!(Format::Matroska.family(), None);
  /// ```
  #[must_use]
  pub fn family(&self) -> Option<Format> {
    match self {
      Format::Mp4 | Format::ThreeGpp | Format::ThreeGpp2 | Format::F4v | Format::QuickTime => {
        Some(Format::IsoBaseMedia)
      }
      Format::Avi => Some(Format::Riff),
      _ => None,
    }
  }

  /// Whether the format has sample descriptions, as the ISO base media family has, `IsoBaseMedia` itself included,
  /// and no other format of this version (`api.md` section 6.3); `None` for a format of a newer version, which only
  /// its version knows.
  #[cfg(feature = "serde")]
  pub(crate) fn has_sample_descriptions(&self) -> Option<bool> {
    match self {
      Format::Unrecognized(_) => None,
      _ => Some(*self == Format::IsoBaseMedia || self.family() == Some(Format::IsoBaseMedia)),
    }
  }
}

impl fmt::Display for Format {
  /// Writes the library's name, as `name()` returns it.
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(self.name())
  }
}

#[cfg(test)]
mod tests {
  use super::Format;
  use crate::model::{SupportLevel, UnrecognizedValue};

  #[test]
  fn names_each_format() {
    let formats = [
      (Format::Matroska, "matroska", "Matroska", None),
      (Format::WebM, "webm", "WebM", None),
      (Format::Mp4, "mp4", "MP4", Some(Format::IsoBaseMedia)),
      (Format::ThreeGpp, "three_gpp", "3GPP", Some(Format::IsoBaseMedia)),
      (Format::ThreeGpp2, "three_gpp2", "3GPP2", Some(Format::IsoBaseMedia)),
      (Format::F4v, "f4v", "F4V", Some(Format::IsoBaseMedia)),
      (Format::QuickTime, "quicktime", "QuickTime", Some(Format::IsoBaseMedia)),
      (Format::Avi, "avi", "AVI", Some(Format::Riff)),
      (Format::IsoBaseMedia, "iso_base_media", "ISO base media", None),
      (Format::Riff, "riff", "RIFF", None),
    ];
    for (format, serialized, name, family) in formats {
      assert_eq!(format.as_str(), serialized);
      assert_eq!(format.name(), name);
      assert_eq!(format.to_string(), name);
      assert_eq!(format.family(), family);
      assert_eq!(
        format.is_family(),
        matches!(format, Format::IsoBaseMedia | Format::Riff)
      );
    }
  }

  #[test]
  fn tells_probed_formats_from_detected_ones() {
    assert_eq!(Format::Matroska.support(), Some(SupportLevel::Probed));
    assert_eq!(Format::IsoBaseMedia.support(), Some(SupportLevel::Probed));
    assert_eq!(Format::Riff.support(), Some(SupportLevel::DetectOnly));
  }

  #[test]
  fn keeps_a_format_of_a_newer_version() {
    let newer = Format::Unrecognized(UnrecognizedValue::sample("holographic"));
    assert_eq!(newer.as_str(), "holographic");
    assert_eq!(newer.name(), "holographic");
    assert_eq!(newer.support(), None);
    assert!(!newer.is_family());
    assert_eq!(newer.family(), None);
  }
}
