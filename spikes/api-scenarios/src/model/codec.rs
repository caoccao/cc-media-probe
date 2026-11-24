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
  AAC_NAME, AV1_NAME, CANDIDATE_SEPARATOR, H264_NAME, MISSING_CANDIDATE_TEXT, OPUS_NAME, UNKNOWN_CODEC_TEXT,
  UNNAMED_CODEC_TEXT, VP9_NAME, WEBVTT_NAME,
};
use super::unrecognized_value::UnrecognizedValue;
#[cfg(feature = "serde")]
use crate::constants::{FOUR_CHARACTER_CODE_BYTES, GUID_BYTES, PRINTABLE_FIRST, PRINTABLE_LAST, RAW_CODE_MESSAGE};
#[cfg(feature = "serde")]
use crate::decimal::canonical;
#[cfg(feature = "serde")]
use crate::document::InvalidDocument;
#[cfg(feature = "serde")]
use crate::hexadecimal;

crate::form::strict_form! {
  #[form(remote = "Codec")]
  /// A codec's identity.
  #[derive(Clone, Debug, PartialEq, Eq, Hash)]
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
    #[cfg_attr(feature = "serde", serde(with = "crate::model::unrecognized_text"))]
    Unrecognized(UnrecognizedValue<Codec>),
  }
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

  /// What a display writes before the raw identifiers: the library's name, or the serialized name that a newer
  /// version wrote (twelfth review).
  fn display_name(&self) -> Option<&str> {
    match self {
      Codec::Unrecognized(value) => Some(value.as_str()),
      known => known.name(),
    }
  }

  /// The display of a codec that is present: its name, else its first raw identifier, else the text of an unknown
  /// codec. "A codec that was not read" is only for an absent codec.
  fn display_text<'text>(&'text self, raw_identifiers: &'text [RawCodecIdentifier]) -> &'text str {
    let raw = raw_identifiers.first().map(RawCodecIdentifier::value);
    self.display_name().or(raw).unwrap_or(UNKNOWN_CODEC_TEXT)
  }
}

/// One codec identifier as the input wrote it, in the spelling of its namespace, as the maintainer chose at the
/// thirteenth review: a text code is its text, a byte code such as a four-character code is its bytes as stored in
/// lowercase hexadecimal, two digits a byte, and a coded number is canonical decimal. Every value has one spelling,
/// so two identifiers name the same code exactly when their namespaces and values are equal (`api.md` section 6.4).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct RawCodecIdentifier {
  namespace: CodecNamespace,
  value: String,
}

#[cfg(feature = "serde")]
crate::neutral::checked_strict_traits!(RawCodecIdentifier from RawCodecIdentifierData);

/// A raw codec identifier as read, before its spelling is checked.
#[cfg(feature = "serde")]
#[derive(serde::Deserialize)]
struct RawCodecIdentifierData {
  namespace: CodecNamespace,
  value: String,
}

#[cfg(feature = "serde")]
impl TryFrom<RawCodecIdentifierData> for RawCodecIdentifier {
  type Error = InvalidDocument;

  /// The spelling of a known namespace, and any text for a namespace of a newer version, which is kept as written.
  fn try_from(read: RawCodecIdentifierData) -> Result<RawCodecIdentifier, InvalidDocument> {
    let spelled = match read.namespace {
      CodecNamespace::Unrecognized(_) => true,
      // The text that the decoder writes: printable ASCII, and U+FFFD for any other byte, once the terminating null
      // octets are removed (RFC 8794 sections 7.4 and 13; fourteenth review).
      CodecNamespace::MatroskaCodecIdentifier => read.value.chars().all(|character| {
        character == char::REPLACEMENT_CHARACTER || (PRINTABLE_FIRST..=PRINTABLE_LAST).contains(&character)
      }),
      CodecNamespace::IsoSampleEntry => {
        hexadecimal::decode(&read.value).is_some_and(|bytes| bytes.len() == FOUR_CHARACTER_CODE_BYTES)
      }
      CodecNamespace::WaveSubformat => hexadecimal::decode(&read.value).is_some_and(|bytes| bytes.len() == GUID_BYTES),
      CodecNamespace::WaveFormatTag => canonical::<u16>(&read.value).is_some(),
    };
    if !spelled {
      return Err(InvalidDocument::new(RAW_CODE_MESSAGE));
    }
    Ok(RawCodecIdentifier {
      namespace: read.namespace,
      value: read.value,
    })
  }
}

impl RawCodecIdentifier {
  /// The namespace of the identifier.
  pub fn namespace(&self) -> &CodecNamespace {
    &self.namespace
  }

  /// The identifier as written, in the spelling of its namespace: `"V_MPEG4/ISO/AVC"`, `"61766331"` for `avc1`, or
  /// `"85"` for a WAVE format tag.
  pub fn value(&self) -> &str {
    &self.value
  }
}

crate::form::strict_form! {
  #[form(remote = "CodecNamespace")]
  /// The namespace of a raw codec identifier.
  #[derive(Clone, Debug, PartialEq, Eq, Hash)]
  #[non_exhaustive]
  pub enum CodecNamespace {
    /// A Matroska `CodecID`.
    MatroskaCodecIdentifier,
    /// An ISO sample entry code, four bytes.
    IsoSampleEntry,
    /// A WAVE format tag, a 16-bit number (RFC 2361).
    WaveFormatTag,
    /// The `SubFormat` GUID of an extensible WAVE format, its 16 bytes as stored (fourteenth review, as the maintainer
    /// chose).
    WaveSubformat,
    /// A value that a newer version wrote.
    #[cfg_attr(feature = "serde", serde(with = "crate::model::unrecognized_text"))]
    Unrecognized(UnrecognizedValue<CodecNamespace>),
  }
}

crate::form::strict_form! {
  #[form(remote = "Profile")]
  /// A profile, level or tier: the specification's name and the coded value.
  #[derive(Clone, Debug, PartialEq, Eq, Hash)]
  pub struct Profile {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    name: Option<String>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    code: Option<String>,
  }
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

crate::form::strict_form! {
  #[form(remote = "CodecCandidate")]
  /// One sample description that could hold the first sample, with its own codec and identifiers.
  #[derive(Clone, Debug, PartialEq, Eq, Hash)]
  pub struct CodecCandidate {
    sample_description_index: u64,
    codec: Codec,
    raw_identifiers: Vec<RawCodecIdentifier>,
  }
}

impl CodecCandidate {
  /// The description's index as the file numbers it, from 1 in the ISO family.
  pub fn sample_description_index(&self) -> u64 {
    self.sample_description_index
  }

  /// The codec that the description names.
  pub fn codec(&self) -> &Codec {
    &self.codec
  }

  /// The description's own codec identifiers, as written, such as a protected entry's code and its original format.
  pub fn raw_identifiers(&self) -> &[RawCodecIdentifier] {
    &self.raw_identifiers
  }
}

impl fmt::Display for CodecCandidate {
  /// The codec's name, or the serialized name of a codec that a newer version wrote; else the description's first
  /// raw identifier as written; else the text of an unknown codec.
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(self.codec.display_text(&self.raw_identifiers))
  }
}

crate::form::strict_form! {
  #[form(remote = "CodecInformation")]
  /// What a stream says about its codec.
  #[derive(Clone, Debug, PartialEq, Eq, Hash)]
  pub struct CodecInformation {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    codec: Option<Codec>,
    candidates: Vec<CodecCandidate>,
    has_every_candidate: bool,
    raw_identifiers: Vec<RawCodecIdentifier>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    profile: Option<Profile>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    level: Option<Profile>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    codec_string: Option<String>,
  }
}

impl CodecInformation {
  /// The codec's identity; absent while the description of the first sample is unproven, unless every candidate
  /// is listed and they name one codec, and absent when that description is established but was not read or kept.
  pub fn codec(&self) -> Option<&Codec> {
    self.codec.as_ref()
  }

  /// While the description of the first sample is unproven, each description that could hold it and that the probe
  /// read and kept, in index order. Empty once the description is established: the details keep its index.
  pub fn candidates(&self) -> &[CodecCandidate] {
    &self.candidates
  }

  /// Whether no candidate is missing: true while nothing is unresolved, and false when the probe could not read or
  /// keep a description that could hold the first sample, including the one that the first sample is known to use.
  pub fn has_every_candidate(&self) -> bool {
    self.has_every_candidate
  }

  /// The library's name for a known codec; absent for an unknown or unresolved one.
  pub fn name(&self) -> Option<&str> {
    self.codec.as_ref().and_then(Codec::name)
  }

  /// The codec identifiers of the reported description, as written; while it is unproven, every candidate's, in
  /// candidate order.
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
  /// The codec's name, or the serialized name of a codec that a newer version wrote (twelfth review); for a codec
  /// without either, the first raw identifier as written, and "an unknown codec" without one. An unresolved codec
  /// writes each candidate in the same way, from the candidate's own identifiers, separated by " or ", and ends with
  /// a description that was not read when a candidate is missing. With no candidate kept, that phrase stands alone,
  /// as for an ISO description cut off before its format code. An absent codec that a stream without sample
  /// descriptions never named, with no raw identifier and no candidate missing, is a codec that was not read.
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    if let Some(codec) = &self.codec {
      return formatter.write_str(codec.display_text(&self.raw_identifiers));
    }
    if self.candidates.is_empty() && self.has_every_candidate {
      let raw = self.raw_identifiers.first().map(RawCodecIdentifier::value);
      return formatter.write_str(raw.unwrap_or(UNNAMED_CODEC_TEXT));
    }
    for (position, candidate) in self.candidates.iter().enumerate() {
      if position > 0 {
        formatter.write_str(CANDIDATE_SEPARATOR)?;
      }
      write!(formatter, "{candidate}")?;
    }
    if !self.has_every_candidate {
      if !self.candidates.is_empty() {
        formatter.write_str(CANDIDATE_SEPARATOR)?;
      }
      formatter.write_str(MISSING_CANDIDATE_TEXT)?;
    }
    Ok(())
  }
}
