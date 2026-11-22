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

//! `Codec` and `CodecInformation`, `api.md` section 6.4, with a few codecs.

use std::fmt;

use super::constants::{
  AAC_NAME, AV1_NAME, CANDIDATE_SEPARATOR, H264_NAME, OPUS_NAME, UNKNOWN_CODEC_TEXT, VP9_NAME, WEBVTT_NAME,
};
use super::values::UnrecognizedValue;

/// A codec's identity.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum Codec {
  /// H.264.
  H264,
  /// AV1.
  Av1,
  /// VP9.
  Vp9,
  /// AAC.
  Aac,
  /// Opus.
  Opus,
  /// WebVTT.
  WebVtt,
  /// A codec that the file names and this version does not know; the raw identifiers keep its name.
  Unknown,
  /// A value that a newer version wrote.
  Unrecognized(UnrecognizedValue),
}

impl Codec {
  /// The library's name for a known codec; `None` for `Unknown` and `Unrecognized`.
  pub fn name(&self) -> Option<&str> {
    match self {
      Codec::H264 => Some(H264_NAME),
      Codec::Av1 => Some(AV1_NAME),
      Codec::Vp9 => Some(VP9_NAME),
      Codec::Aac => Some(AAC_NAME),
      Codec::Opus => Some(OPUS_NAME),
      Codec::WebVtt => Some(WEBVTT_NAME),
      Codec::Unknown | Codec::Unrecognized(_) => None,
    }
  }
}

/// One codec identifier as the input wrote it.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RawCodecIdentifier {
  namespace: CodecNamespace,
  value: String,
}

impl RawCodecIdentifier {
  /// The namespace of the identifier.
  pub fn namespace(&self) -> &CodecNamespace {
    &self.namespace
  }

  /// The identifier as written.
  pub fn value(&self) -> &str {
    &self.value
  }
}

/// The namespace of a raw codec identifier.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum CodecNamespace {
  /// A Matroska `CodecID`.
  MatroskaCodecIdentifier,
  /// An ISO sample entry code.
  IsoSampleEntry,
  /// A value that a newer version wrote.
  Unrecognized(UnrecognizedValue),
}

/// A profile, level or tier: the specification's name and the coded value.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Profile {
  name: Option<String>,
  code: Option<String>,
}

impl Profile {
  /// The specification's name, such as "High".
  pub fn name(&self) -> Option<&str> {
    self.name.as_deref()
  }

  /// The coded value as written, such as `100`.
  pub fn code(&self) -> Option<&str> {
    self.code.as_deref()
  }
}

impl fmt::Display for Profile {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(self.name.as_deref().or(self.code.as_deref()).unwrap_or_default())
  }
}

/// What a stream says about its codec.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CodecInformation {
  codec: Option<Codec>,
  candidate_codecs: Vec<Codec>,
  raw_identifiers: Vec<RawCodecIdentifier>,
  profile: Option<Profile>,
  level: Option<Profile>,
  codec_string: Option<String>,
}

impl CodecInformation {
  /// The codec's identity; absent while the description of the first sample is unproven and the candidates name
  /// different codecs.
  pub fn codec(&self) -> Option<&Codec> {
    self.codec.as_ref()
  }

  /// While the codec is unresolved, the codecs of the descriptions that could hold the first sample.
  pub fn candidate_codecs(&self) -> &[Codec] {
    &self.candidate_codecs
  }

  /// The library's name for a known codec; absent for an unknown or unresolved one.
  pub fn name(&self) -> Option<&str> {
    self.codec.as_ref().and_then(Codec::name)
  }

  /// Every codec identifier the input carries, as written; for an unresolved codec, each candidate's.
  pub fn raw_identifiers(&self) -> &[RawCodecIdentifier] {
    &self.raw_identifiers
  }

  /// The profile.
  pub fn profile(&self) -> Option<&Profile> {
    self.profile.as_ref()
  }

  /// The level; the stub reuses the profile's shape.
  pub fn level(&self) -> Option<&Profile> {
    self.level.as_ref()
  }

  /// The RFC 6381 `codecs` value.
  pub fn codec_string(&self) -> Option<&str> {
    self.codec_string.as_deref()
  }
}

impl fmt::Display for CodecInformation {
  /// The codec's name; for a codec without one, the first raw identifier as written. An unresolved codec writes
  /// each candidate in the same way, separated by " or ".
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    if self.codec.is_some() || self.candidate_codecs.is_empty() {
      let raw = self.raw_identifiers.first().map(RawCodecIdentifier::value);
      return formatter.write_str(self.name().or(raw).unwrap_or(UNKNOWN_CODEC_TEXT));
    }
    for (position, candidate) in self.candidate_codecs.iter().enumerate() {
      if position > 0 {
        formatter.write_str(CANDIDATE_SEPARATOR)?;
      }
      let raw = self.raw_identifiers.get(position).map(RawCodecIdentifier::value);
      formatter.write_str(candidate.name().or(raw).unwrap_or(UNKNOWN_CODEC_TEXT))?;
    }
    Ok(())
  }
}
