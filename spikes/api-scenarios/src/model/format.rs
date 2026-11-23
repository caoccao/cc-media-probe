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

//! `Format`, `api.md` section 6.4, with a few of its variants.

use std::fmt;

use super::constants::{AVI_NAME, ISO_BASE_MEDIA_NAME, MATROSKA_NAME, MP4_NAME, QUICKTIME_NAME, RIFF_NAME, WEBM_NAME};
use super::unrecognized_value::UnrecognizedValue;

/// A container format or format family.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum Format {
  /// Matroska.
  Matroska,
  /// WebM.
  WebM,
  /// MP4.
  Mp4,
  /// QuickTime.
  QuickTime,
  /// AVI.
  Avi,
  /// The ISO base media family, when detection is certain of the family only.
  IsoBaseMedia,
  /// The RIFF family, when detection is certain of the family only.
  Riff,
  /// A value that a newer version wrote.
  #[cfg_attr(feature = "serde", serde(with = "crate::model::unrecognized_text"))]
  Unrecognized(UnrecognizedValue<Format>),
}

/// Whether the library probes a format or only detects it.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum SupportLevel {
  /// Probed.
  Probed,
  /// Detected only.
  DetectOnly,
  /// A value that a newer version wrote.
  #[cfg_attr(feature = "serde", serde(with = "crate::model::unrecognized_text"))]
  Unrecognized(UnrecognizedValue<SupportLevel>),
}

impl Format {
  /// The format's name, or the text of an unrecognized value.
  pub fn name(&self) -> &str {
    match self {
      Format::Matroska => MATROSKA_NAME,
      Format::WebM => WEBM_NAME,
      Format::Mp4 => MP4_NAME,
      Format::QuickTime => QUICKTIME_NAME,
      Format::Avi => AVI_NAME,
      Format::IsoBaseMedia => ISO_BASE_MEDIA_NAME,
      Format::Riff => RIFF_NAME,
      Format::Unrecognized(value) => value.as_str(),
    }
  }

  /// Whether this version probes the format; `None` for an unrecognized format.
  pub fn support(&self) -> Option<SupportLevel> {
    match self {
      Format::Unrecognized(_) => None,
      Format::IsoBaseMedia | Format::Riff => Some(SupportLevel::DetectOnly),
      _ => Some(SupportLevel::Probed),
    }
  }

  /// The family the format belongs to.
  pub fn family(&self) -> Option<Format> {
    match self {
      Format::Mp4 | Format::QuickTime => Some(Format::IsoBaseMedia),
      Format::Avi => Some(Format::Riff),
      _ => None,
    }
  }
}

impl fmt::Display for Format {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(self.name())
  }
}
