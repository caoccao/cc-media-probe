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

//! `VideoInformation`, what a video stream says about its pictures (`api.md` section 6.5; field guide section 3.4).

use super::{
  BitDepth, ChromaFormat, ColorDescription, ContentLightLevel, DynamicRange, FieldOrder, FrameRateMode,
  MasteringDisplay, Projection, Rotation, ScanType, StereoMode,
};
use crate::Rational;
use crate::form_record::form_record;
use crate::model::{Provenance, Size};

form_record! {
  /// What a video stream says about its pictures, from `Stream::video()`. Present exactly when the stream's kind is
  /// video; every field inside may still be absent.
  ///
  /// ```
  /// use cc_media_probe::Stream;
  ///
  /// fn video_summary(stream: &Stream) -> Option<String> {
  ///     let video = stream.video()?;
  ///     let mut parts = Vec::new();
  ///     if let Some(size) = video.visible_size().or(video.coded_size()) {
  ///         parts.push(size.to_string());
  ///     }
  ///     if let Some(display) = video.display_size() {
  ///         parts.push(format!("display {display} ({:?})", video.display_size_provenance()));
  ///     }
  ///     if let Some(rate) = video.frame_rate() {
  ///         parts.push(format!("{rate} fps ({}, {:?})", video.frame_rate_mode().as_str(), video.frame_rate_provenance()));
  ///     }
  ///     if let (Some(pixel), Some(display)) = (video.pixel_aspect_ratio(), video.display_aspect_ratio()) {
  ///         parts.push(format!("pixel {pixel}, display {display}"));
  ///     }
  ///     parts.push(format!("bits {:?}", video.bit_depth().and_then(|depth| depth.common())));
  ///     parts.push(format!("chroma {:?}", video.chroma_format().map(|format| format.as_str())));
  ///     parts.push(format!("scan {:?} {:?}", video.scan_type(), video.field_order()));
  ///     parts.push(format!("color {:?}", video.color().and_then(|color| color.primaries())));
  ///     parts.push(format!("mastering {:?}", video.mastering_display().and_then(|display| display.maximum_luminance())));
  ///     parts.push(format!("light {:?}", video.content_light_level().and_then(|level| level.maximum_content())));
  ///     parts.push(format!("range {:?}", video.dynamic_range().and_then(|range| range.transfer())));
  ///     parts.push(format!("rotation {:?}", video.rotation().and_then(|rotation| rotation.degrees())));
  ///     parts.push(format!("stereo {:?}", video.stereo_mode().map(|mode| mode.code())));
  ///     parts.push(format!("projection {:?}", video.projection().map(|projection| projection.kind().as_str())));
  ///     parts.push(format!("alpha {:?} {:?}", video.embedded_alpha(), video.supplemental_alpha()));
  ///     parts.push(format!(
  ///         "provenance {:?} {:?} {:?} {:?}",
  ///         video.coded_size_provenance(),
  ///         video.visible_size_provenance(),
  ///         video.pixel_aspect_ratio_provenance(),
  ///         video.display_aspect_ratio_provenance(),
  ///     ));
  ///     Some(parts.join(", "))
  /// }
  /// ```
  #[derive(Clone, Debug, PartialEq, Eq, Hash)]
  pub struct VideoInformation {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    coded_size: Option<Size>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    coded_size_provenance: Option<Provenance>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    visible_size: Option<Size>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    visible_size_provenance: Option<Provenance>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    display_size: Option<Size>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    display_size_provenance: Option<Provenance>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pixel_aspect_ratio: Option<Rational>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pixel_aspect_ratio_provenance: Option<Provenance>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    display_aspect_ratio: Option<Rational>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    display_aspect_ratio_provenance: Option<Provenance>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    frame_rate: Option<Rational>,
    frame_rate_mode: FrameRateMode,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    frame_rate_provenance: Option<Provenance>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    bit_depth: Option<BitDepth>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    chroma_format: Option<ChromaFormat>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    scan_type: Option<ScanType>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    field_order: Option<FieldOrder>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    color: Option<ColorDescription>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    mastering_display: Option<MasteringDisplay>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    content_light_level: Option<ContentLightLevel>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    dynamic_range: Option<DynamicRange>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    rotation: Option<Rotation>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    stereo_mode: Option<StereoMode>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    projection: Option<Projection>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    embedded_alpha: Option<bool>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    supplemental_alpha: Option<bool>,
  }
  pairs = [
    coded_size with coded_size_provenance,
    visible_size with visible_size_provenance,
    display_size with display_size_provenance,
    pixel_aspect_ratio with pixel_aspect_ratio_provenance,
    display_aspect_ratio with display_aspect_ratio_provenance,
    frame_rate with frame_rate_provenance,
  ];
}

impl VideoInformation {
  /// The size of the coded pictures, in pixels.
  #[must_use]
  pub fn coded_size(&self) -> Option<Size> {
    self.coded_size
  }

  /// Where the coded size came from.
  #[must_use]
  pub fn coded_size_provenance(&self) -> Option<&Provenance> {
    self.coded_size_provenance.as_ref()
  }

  /// The size of the visible pictures, after cropping, in pixels.
  #[must_use]
  pub fn visible_size(&self) -> Option<Size> {
    self.visible_size
  }

  /// Where the visible size came from.
  #[must_use]
  pub fn visible_size_provenance(&self) -> Option<&Provenance> {
    self.visible_size_provenance.as_ref()
  }

  /// The size at which the pictures are shown, in pixels. For single-track stereo, it describes one view.
  #[must_use]
  pub fn display_size(&self) -> Option<Size> {
    self.display_size
  }

  /// Where the display size came from.
  #[must_use]
  pub fn display_size_provenance(&self) -> Option<&Provenance> {
    self.display_size_provenance.as_ref()
  }

  /// The shape of a visible pixel as presented (F26).
  #[must_use]
  pub fn pixel_aspect_ratio(&self) -> Option<Rational> {
    self.pixel_aspect_ratio
  }

  /// Where the pixel aspect ratio came from.
  #[must_use]
  pub fn pixel_aspect_ratio_provenance(&self) -> Option<&Provenance> {
    self.pixel_aspect_ratio_provenance.as_ref()
  }

  /// The aspect ratio of the displayed picture, never from a rounded size.
  #[must_use]
  pub fn display_aspect_ratio(&self) -> Option<Rational> {
    self.display_aspect_ratio
  }

  /// Where the display aspect ratio came from.
  #[must_use]
  pub fn display_aspect_ratio_provenance(&self) -> Option<&Provenance> {
    self.display_aspect_ratio_provenance.as_ref()
  }

  /// The frame rate, in frames per second.
  #[must_use]
  pub fn frame_rate(&self) -> Option<Rational> {
    self.frame_rate
  }

  /// Whether the frame rate is constant.
  #[must_use]
  pub fn frame_rate_mode(&self) -> &FrameRateMode {
    &self.frame_rate_mode
  }

  /// Where the frame rate came from.
  #[must_use]
  pub fn frame_rate_provenance(&self) -> Option<&Provenance> {
    self.frame_rate_provenance.as_ref()
  }

  /// The precision of the samples.
  #[must_use]
  pub fn bit_depth(&self) -> Option<&BitDepth> {
    self.bit_depth.as_ref()
  }

  /// How the chroma is subsampled.
  #[must_use]
  pub fn chroma_format(&self) -> Option<&ChromaFormat> {
    self.chroma_format.as_ref()
  }

  /// How the pictures are scanned.
  #[must_use]
  pub fn scan_type(&self) -> Option<&ScanType> {
    self.scan_type.as_ref()
  }

  /// Which field of an interlaced frame comes first.
  #[must_use]
  pub fn field_order(&self) -> Option<&FieldOrder> {
    self.field_order.as_ref()
  }

  /// The color description.
  #[must_use]
  pub fn color(&self) -> Option<&ColorDescription> {
    self.color.as_ref()
  }

  /// The display that the video was mastered on.
  #[must_use]
  pub fn mastering_display(&self) -> Option<&MasteringDisplay> {
    self.mastering_display.as_ref()
  }

  /// The content light levels.
  #[must_use]
  pub fn content_light_level(&self) -> Option<&ContentLightLevel> {
    self.content_light_level.as_ref()
  }

  /// The dynamic range (F6).
  #[must_use]
  pub fn dynamic_range(&self) -> Option<&DynamicRange> {
    self.dynamic_range.as_ref()
  }

  /// The rotation that a player applies (F7), reported and never applied.
  #[must_use]
  pub fn rotation(&self) -> Option<&Rotation> {
    self.rotation.as_ref()
  }

  /// How a single-track stereo video packs its views.
  #[must_use]
  pub fn stereo_mode(&self) -> Option<&StereoMode> {
    self.stereo_mode.as_ref()
  }

  /// How the video maps onto a sphere.
  #[must_use]
  pub fn projection(&self) -> Option<&Projection> {
    self.projection.as_ref()
  }

  /// Whether the pictures carry an alpha channel of their own.
  #[must_use]
  pub fn embedded_alpha(&self) -> Option<bool> {
    self.embedded_alpha
  }

  /// Whether a supplemental stream of data carries the alpha channel, beside the pictures.
  #[must_use]
  pub fn supplemental_alpha(&self) -> Option<bool> {
    self.supplemental_alpha
  }
}

#[cfg(test)]
mod tests {
  use super::VideoInformation;
  use crate::Rational;
  use crate::model::{
    BitDepth, ChromaFormat, ColorDescription, Component, ContentLightLevel, DynamicRange, FieldOrder, FrameRateMode,
    MasteringDisplay, Projection, Provenance, Rotation, ScanType, Size, StereoLayout, StereoMode, UnrecognizedValue,
  };

  impl VideoInformation {
    /// A 1080p video at 24000/1001 frames per second, with every field stated.
    pub(crate) fn sample() -> VideoInformation {
      VideoInformation {
        coded_size: Size::new(1_920, 1_088),
        coded_size_provenance: Some(Provenance::Bitstream),
        visible_size: Size::new(1_920, 1_080),
        visible_size_provenance: Some(Provenance::Bitstream),
        display_size: Size::new(1_920, 1_080),
        display_size_provenance: Some(Provenance::Declared),
        pixel_aspect_ratio: Rational::new(1, 1),
        pixel_aspect_ratio_provenance: Some(Provenance::Derived),
        display_aspect_ratio: Rational::new(16, 9),
        display_aspect_ratio_provenance: Some(Provenance::Derived),
        frame_rate: Rational::new(24_000, 1_001),
        frame_rate_mode: FrameRateMode::Constant,
        frame_rate_provenance: Some(Provenance::Declared),
        bit_depth: Some(BitDepth::sample(Some(8), &[(Component::Luma, 8)])),
        chroma_format: Some(ChromaFormat::Chroma420),
        scan_type: Some(ScanType::Interlaced),
        field_order: Some(FieldOrder::TopFieldFirst),
        color: Some(ColorDescription::sample()),
        mastering_display: Some(MasteringDisplay::sample()),
        content_light_level: Some(ContentLightLevel::sample()),
        dynamic_range: Some(DynamicRange::sample()),
        rotation: Some(Rotation::Clockwise90),
        stereo_mode: Some(StereoMode::sample(
          StereoLayout::Unrecognized(UnrecognizedValue::sample("mono")),
          "0",
        )),
        projection: Some(Projection::sample()),
        embedded_alpha: Some(false),
        supplemental_alpha: Some(true),
      }
    }
  }

  #[test]
  fn gives_each_field() {
    let video = VideoInformation::sample();
    assert_eq!(video.coded_size(), Size::new(1_920, 1_088));
    assert_eq!(video.coded_size_provenance(), Some(&Provenance::Bitstream));
    assert_eq!(video.visible_size(), Size::new(1_920, 1_080));
    assert_eq!(video.visible_size_provenance(), Some(&Provenance::Bitstream));
    assert_eq!(video.display_size(), Size::new(1_920, 1_080));
    assert_eq!(video.display_size_provenance(), Some(&Provenance::Declared));
    assert_eq!(video.pixel_aspect_ratio(), Rational::new(1, 1));
    assert_eq!(video.pixel_aspect_ratio_provenance(), Some(&Provenance::Derived));
    assert_eq!(video.display_aspect_ratio(), Rational::new(16, 9));
    assert_eq!(video.display_aspect_ratio_provenance(), Some(&Provenance::Derived));
    assert_eq!(video.frame_rate(), Rational::new(24_000, 1_001));
    assert_eq!(video.frame_rate_mode(), &FrameRateMode::Constant);
    assert_eq!(video.frame_rate_provenance(), Some(&Provenance::Declared));
    assert_eq!(video.bit_depth().and_then(BitDepth::common), Some(8));
    assert_eq!(video.chroma_format(), Some(&ChromaFormat::Chroma420));
    assert_eq!(video.scan_type(), Some(&ScanType::Interlaced));
    assert_eq!(video.field_order(), Some(&FieldOrder::TopFieldFirst));
    assert_eq!(video.color(), Some(&ColorDescription::sample()));
    assert_eq!(video.mastering_display(), Some(&MasteringDisplay::sample()));
    assert_eq!(video.content_light_level(), Some(&ContentLightLevel::sample()));
    assert_eq!(video.dynamic_range(), Some(&DynamicRange::sample()));
    assert_eq!(video.rotation(), Some(&Rotation::Clockwise90));
    assert_eq!(video.stereo_mode().map(StereoMode::code), Some("0"));
    assert_eq!(video.projection(), Some(&Projection::sample()));
    assert_eq!(video.embedded_alpha(), Some(false));
    assert_eq!(video.supplemental_alpha(), Some(true));
  }
}
