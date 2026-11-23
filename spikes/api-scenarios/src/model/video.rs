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

//! `VideoInformation`, `api.md` section 6.5, with the fields the scenarios read.

use super::chroma_format::ChromaFormat;
use super::constants::{CLOCKWISE_90_DEGREES, CLOCKWISE_180_DEGREES, CLOCKWISE_270_DEGREES, UPRIGHT_DEGREES};
use super::unrecognized_value::UnrecognizedValue;
use super::values::{Provenance, Rational, Size};

/// What a video stream states.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(
  feature = "serde",
  derive(serde::Serialize, serde::Deserialize),
  serde(remote = "Self")
)]
pub struct VideoInformation {
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
  coded_size: Option<Size>,
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
  visible_size: Option<Size>,
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
  display_size: Option<Size>,
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
  pixel_aspect_ratio: Option<Rational>,
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
  frame_rate: Option<Rational>,
  frame_rate_mode: FrameRateMode,
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
  frame_rate_provenance: Option<Provenance>,
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
  rotation: Option<Rotation>,
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
  chroma_format: Option<ChromaFormat>,
}

#[cfg(feature = "serde")]
crate::neutral::record_traits!(VideoInformation);

impl VideoInformation {
  /// The coded size.
  pub fn coded_size(&self) -> Option<Size> {
    self.coded_size
  }

  /// The visible size, after cropping.
  pub fn visible_size(&self) -> Option<Size> {
    self.visible_size
  }

  /// The size as displayed.
  pub fn display_size(&self) -> Option<Size> {
    self.display_size
  }

  /// The shape of a visible pixel as presented.
  pub fn pixel_aspect_ratio(&self) -> Option<Rational> {
    self.pixel_aspect_ratio
  }

  /// Frames per second.
  pub fn frame_rate(&self) -> Option<Rational> {
    self.frame_rate
  }

  /// Whether the intervals between frames are shown to be equal.
  pub fn frame_rate_mode(&self) -> &FrameRateMode {
    &self.frame_rate_mode
  }

  /// Where the frame rate came from.
  pub fn frame_rate_provenance(&self) -> Option<&Provenance> {
    self.frame_rate_provenance.as_ref()
  }

  /// The clockwise rotation a player applies.
  pub fn rotation(&self) -> Option<&Rotation> {
    self.rotation.as_ref()
  }

  /// The chroma format.
  pub fn chroma_format(&self) -> Option<&ChromaFormat> {
    self.chroma_format.as_ref()
  }
}

/// Whether a frame rate is constant.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum FrameRateMode {
  /// Every interval is shown to be equal.
  Constant,
  /// Two intervals are shown to differ.
  Variable,
  /// Neither is shown.
  #[default]
  Unknown,
  /// A value that a newer version wrote.
  #[cfg_attr(feature = "serde", serde(with = "crate::model::unrecognized_text"))]
  Unrecognized(UnrecognizedValue<FrameRateMode>),
}

/// The clockwise rotation a player applies, a multiple of 90 degrees.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum Rotation {
  /// No rotation.
  Upright,
  /// A quarter turn clockwise.
  Clockwise90,
  /// A half turn.
  Clockwise180,
  /// Three quarter turns clockwise.
  Clockwise270,
  /// A value that a newer version wrote.
  #[cfg_attr(feature = "serde", serde(with = "crate::model::unrecognized_text"))]
  Unrecognized(UnrecognizedValue<Rotation>),
}

impl Rotation {
  /// The angle in degrees; `None` for an unrecognized value.
  pub fn degrees(&self) -> Option<u16> {
    match self {
      Rotation::Upright => Some(UPRIGHT_DEGREES),
      Rotation::Clockwise90 => Some(CLOCKWISE_90_DEGREES),
      Rotation::Clockwise180 => Some(CLOCKWISE_180_DEGREES),
      Rotation::Clockwise270 => Some(CLOCKWISE_270_DEGREES),
      Rotation::Unrecognized(_) => None,
    }
  }
}
