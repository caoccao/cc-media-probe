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

//! `Stream`, `StreamKind` and `Disposition`, `api.md` section 6.3.

use super::audio::AudioInformation;
use super::codec::CodecInformation;
use super::constants::{AUDIO_NAME, DATA_NAME, SUBTITLE_NAME, UNKNOWN_NAME, VIDEO_NAME};
use super::language::{Language, LocalizedText};
use super::values::{Estimate, Provenance, Rational, Seconds, UnrecognizedValue};
use super::video::VideoInformation;
use crate::completeness::HeaderStatus;
use crate::details::CodecDetails;

/// One stream of a report.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Stream {
  index: usize,
  kind: StreamKind,
  codec: CodecInformation,
  header_status: HeaderStatus,
  language: Option<Language>,
  title: Option<String>,
  titles: Vec<LocalizedText>,
  duration: Option<Seconds>,
  duration_provenance: Option<Provenance>,
  estimated_duration: Option<Estimate<Seconds>>,
  bit_rate: Option<Rational>,
  disposition: Disposition,
  video: Option<VideoInformation>,
  audio: Option<AudioInformation>,
  codec_details: Option<CodecDetails>,
}

impl Stream {
  /// The stream's index in the report.
  pub fn index(&self) -> usize {
    self.index
  }

  /// What the stream carries.
  pub fn kind(&self) -> &StreamKind {
    &self.kind
  }

  /// What the stream says about its codec.
  pub fn codec(&self) -> &CodecInformation {
    &self.codec
  }

  /// The outcome of header parsing.
  pub fn header_status(&self) -> &HeaderStatus {
    &self.header_status
  }

  /// The primary language.
  pub fn language(&self) -> Option<&Language> {
    self.language.as_ref()
  }

  /// The title.
  pub fn title(&self) -> Option<&str> {
    self.title.as_deref()
  }

  /// Every title with its languages.
  pub fn titles(&self) -> &[LocalizedText] {
    &self.titles
  }

  /// The duration.
  pub fn duration(&self) -> Option<Seconds> {
    self.duration
  }

  /// Where the duration came from.
  pub fn duration_provenance(&self) -> Option<&Provenance> {
    self.duration_provenance.as_ref()
  }

  /// An estimate of the duration, separate from `duration()`.
  pub fn estimated_duration(&self) -> Option<&Estimate<Seconds>> {
    self.estimated_duration.as_ref()
  }

  /// The bit rate, in bits per second.
  pub fn bit_rate(&self) -> Option<Rational> {
    self.bit_rate
  }

  /// The stream's flags.
  pub fn disposition(&self) -> &Disposition {
    &self.disposition
  }

  /// Present exactly when the kind is video.
  pub fn video(&self) -> Option<&VideoInformation> {
    self.video.as_ref()
  }

  /// Present exactly when the kind is audio.
  pub fn audio(&self) -> Option<&AudioInformation> {
    self.audio.as_ref()
  }

  /// The codec header's details.
  pub fn codec_details(&self) -> Option<&CodecDetails> {
    self.codec_details.as_ref()
  }
}

/// What a stream carries.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum StreamKind {
  /// Video.
  Video,
  /// Audio.
  Audio,
  /// Subtitles.
  Subtitle,
  /// Data.
  Data,
  /// Not known.
  Unknown,
  /// A value that a newer version wrote.
  Unrecognized(UnrecognizedValue),
}

impl StreamKind {
  /// The serialized name, or the text of an unrecognized value.
  pub fn as_str(&self) -> &str {
    match self {
      StreamKind::Video => VIDEO_NAME,
      StreamKind::Audio => AUDIO_NAME,
      StreamKind::Subtitle => SUBTITLE_NAME,
      StreamKind::Data => DATA_NAME,
      StreamKind::Unknown => UNKNOWN_NAME,
      StreamKind::Unrecognized(value) => value.as_str(),
    }
  }
}

/// A stream's flags, each absent when the format states nothing.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Disposition {
  default: Option<bool>,
  forced: Option<bool>,
  commentary: Option<bool>,
}

impl Disposition {
  /// Whether the stream is a default.
  pub fn is_default(&self) -> Option<bool> {
    self.default
  }

  /// Whether the stream is forced.
  pub fn is_forced(&self) -> Option<bool> {
    self.forced
  }

  /// Whether the stream is commentary.
  pub fn is_commentary(&self) -> Option<bool> {
    self.commentary
  }
}
