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

//! `CodecInformation`, what a stream says about its codec (`api.md` section 6.4).

use std::fmt;

use super::candidate::codec_text;
use super::constants::{CANDIDATE_SEPARATOR, MISSING_CANDIDATE_TEXT, UNNAMED_CODEC_TEXT};
use super::{CodecCandidate, Level, Profile, RawCodecIdentifier, Tier};
use crate::Codec;
use crate::form_record::form_record;
use crate::model::Provenance;

form_record! {
  /// What a stream says about its codec, from `Stream::codec()`.
  ///
  /// The codec is absent only while it is unresolved, when the description that the fields describe is established
  /// and the probe could not read or keep it, or when the input never named a codec. `Display` writes the codec's
  /// name, or the serialized name of a codec that a newer version wrote, else the first raw identifier as written;
  /// an unresolved codec writes its candidates joined by " or "; and an absent codec without candidates writes "a
  /// description that was not read" or "a codec that was not read".
  ///
  /// ```
  /// use cc_media_probe::Stream;
  ///
  /// fn codec(stream: &Stream) -> String {
  ///     let codec = stream.codec();
  ///     let mut line = codec.to_string();
  ///     if let Some(name) = codec.name() {
  ///         line.push_str(&format!(" ({name}, {:?})", codec.codec().map(|codec| codec.as_str())));
  ///     }
  ///     if !codec.candidates().is_empty() && !codec.has_every_candidate() {
  ///         line.push_str(", unresolved");
  ///     }
  ///     for identifier in codec.raw_identifiers() {
  ///         line.push_str(&format!(" [{}: {}]", identifier.namespace().as_str(), identifier.value()));
  ///     }
  ///     if let (Some(profile), Some(provenance)) = (codec.profile(), codec.profile_provenance()) {
  ///         line.push_str(&format!(", profile {profile} ({})", provenance.as_str()));
  ///     }
  ///     if let Some(level) = codec.level() {
  ///         line.push_str(&format!(", level {level} ({:?})", codec.level_provenance()));
  ///     }
  ///     if let Some(tier) = codec.tier() {
  ///         line.push_str(&format!(", tier {tier} ({:?})", codec.tier_provenance()));
  ///     }
  ///     if let Some(codecs) = codec.codec_string() {
  ///         line.push_str(&format!(", codecs=\"{codecs}\""));
  ///     }
  ///     line
  /// }
  /// ```
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
    profile_provenance: Option<Provenance>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    level: Option<Level>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    level_provenance: Option<Provenance>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    tier: Option<Tier>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    tier_provenance: Option<Provenance>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    codec_string: Option<String>,
  }
  pairs = [
    profile with profile_provenance,
    level with level_provenance,
    tier with tier_provenance,
  ];
  check = super::agreement::check_codec;
}

impl CodecInformation {
  /// The codec's identity. Absent only while the codec is unresolved, when the established description was not
  /// read or kept, or when the input never named a codec. When every candidate is listed and they name one codec,
  /// it is that codec.
  #[must_use]
  pub fn codec(&self) -> Option<&Codec> {
    self.codec.as_ref()
  }

  /// While the first sample's description is unproven, each description that could hold it and that the probe
  /// read and kept, in index order; empty otherwise.
  #[must_use]
  pub fn candidates(&self) -> &[CodecCandidate] {
    &self.candidates
  }

  /// Whether the probe read and kept every description that the fields need: false when one is missing,
  /// including the one that the first sample is known to use and the fallback of a track without samples.
  #[must_use]
  pub fn has_every_candidate(&self) -> bool {
    self.has_every_candidate
  }

  /// The library's name for a known codec; absent for an unknown or unresolved one.
  #[must_use]
  pub fn name(&self) -> Option<&str> {
    self.codec.as_ref().and_then(Codec::name)
  }

  /// The codec identifiers of the reported description, as written, and while it is unproven, every candidate's,
  /// in candidate order.
  #[must_use]
  pub fn raw_identifiers(&self) -> &[RawCodecIdentifier] {
    &self.raw_identifiers
  }

  /// The profile.
  #[must_use]
  pub fn profile(&self) -> Option<&Profile> {
    self.profile.as_ref()
  }

  /// Where the profile came from.
  #[must_use]
  pub fn profile_provenance(&self) -> Option<&Provenance> {
    self.profile_provenance.as_ref()
  }

  /// The level.
  #[must_use]
  pub fn level(&self) -> Option<&Level> {
    self.level.as_ref()
  }

  /// Where the level came from.
  #[must_use]
  pub fn level_provenance(&self) -> Option<&Provenance> {
    self.level_provenance.as_ref()
  }

  /// The tier.
  #[must_use]
  pub fn tier(&self) -> Option<&Tier> {
    self.tier.as_ref()
  }

  /// Where the tier came from.
  #[must_use]
  pub fn tier_provenance(&self) -> Option<&Provenance> {
    self.tier_provenance.as_ref()
  }

  /// The RFC 6381 `codecs` value (F5), absent where a binding's mandatory parameters cannot be established.
  #[must_use]
  pub fn codec_string(&self) -> Option<&str> {
    self.codec_string.as_deref()
  }
}

impl fmt::Display for CodecInformation {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    if let Some(codec) = &self.codec {
      return formatter.write_str(codec_text(codec, &self.raw_identifiers));
    }
    if self.candidates.is_empty() && self.has_every_candidate {
      return formatter.write_str(UNNAMED_CODEC_TEXT);
    }
    for (position, candidate) in self.candidates.iter().enumerate() {
      if position > 0 {
        formatter.write_str(CANDIDATE_SEPARATOR)?;
      }
      write!(formatter, "{candidate}")?;
    }
    if self.has_every_candidate {
      return Ok(());
    }
    if !self.candidates.is_empty() {
      formatter.write_str(CANDIDATE_SEPARATOR)?;
    }
    formatter.write_str(MISSING_CANDIDATE_TEXT)
  }
}

#[cfg(test)]
mod tests {
  use super::CodecInformation;
  use crate::Codec;
  use crate::model::{
    CodecCandidate, CodecNamespace, Level, Profile, Provenance, RawCodecIdentifier, Tier, UnrecognizedValue,
  };

  impl CodecInformation {
    /// The information of an H.264 stream of the profile High at level 4.1.
    pub(crate) fn sample() -> CodecInformation {
      CodecInformation {
        codec: Some(Codec::H264),
        candidates: Vec::new(),
        has_every_candidate: true,
        raw_identifiers: vec![RawCodecIdentifier::sample(
          CodecNamespace::MatroskaCodecIdentifier,
          "V_MPEG4/ISO/AVC",
        )],
        profile: Some(Profile::sample(Some("High"), Some("100"))),
        profile_provenance: Some(Provenance::Bitstream),
        level: Some(Level::sample(Some("4.1"), Some("41"))),
        level_provenance: Some(Provenance::Bitstream),
        tier: None,
        tier_provenance: None,
        codec_string: Some("avc1.640029".to_owned()),
      }
    }

    /// The information of an Opus stream of a Matroska track, which has no profile, level or tier.
    pub(crate) fn opus_sample() -> CodecInformation {
      CodecInformation {
        codec: Some(Codec::Opus),
        raw_identifiers: vec![RawCodecIdentifier::sample(
          CodecNamespace::MatroskaCodecIdentifier,
          "A_OPUS",
        )],
        profile: None,
        profile_provenance: None,
        level: None,
        level_provenance: None,
        codec_string: Some("opus".to_owned()),
        ..CodecInformation::sample()
      }
    }

    /// The information of a WebVTT stream of a Matroska track.
    pub(crate) fn webvtt_sample() -> CodecInformation {
      CodecInformation {
        codec: Some(Codec::WebVtt),
        raw_identifiers: vec![RawCodecIdentifier::sample(
          CodecNamespace::MatroskaCodecIdentifier,
          "S_TEXT/WEBVTT",
        )],
        codec_string: None,
        ..CodecInformation::opus_sample()
      }
    }

    /// The information of a codec that the input never named.
    pub(crate) fn unnamed() -> CodecInformation {
      CodecInformation {
        codec: None,
        raw_identifiers: Vec::new(),
        profile: None,
        profile_provenance: None,
        level: None,
        level_provenance: None,
        codec_string: None,
        ..CodecInformation::sample()
      }
    }
  }

  #[test]
  fn describes_a_known_codec() {
    let information = CodecInformation::sample();
    assert_eq!(information.codec(), Some(&Codec::H264));
    assert_eq!(information.name(), Some("H.264"));
    assert_eq!(information.candidates(), []);
    assert!(information.has_every_candidate());
    assert_eq!(information.raw_identifiers().len(), 1);
    assert_eq!(information.profile().map(ToString::to_string), Some("High".to_owned()));
    assert_eq!(information.profile_provenance(), Some(&Provenance::Bitstream));
    assert_eq!(information.level().and_then(Level::name), Some("4.1"));
    assert_eq!(information.level_provenance(), Some(&Provenance::Bitstream));
    assert_eq!(information.tier().map(Tier::name), None);
    assert_eq!(information.tier_provenance(), None);
    assert_eq!(information.codec_string(), Some("avc1.640029"));
    assert_eq!(information.to_string(), "H.264");
  }

  #[test]
  fn displays_an_unknown_or_newer_codec_by_what_was_written() {
    let unknown = CodecInformation {
      codec: Some(Codec::Unknown),
      raw_identifiers: vec![RawCodecIdentifier::sample(CodecNamespace::IsoSampleEntry, "66757431")],
      ..CodecInformation::sample()
    };
    assert_eq!(unknown.name(), None);
    assert_eq!(unknown.to_string(), "66757431");
    let bare = CodecInformation {
      codec: Some(Codec::Unknown),
      raw_identifiers: Vec::new(),
      ..CodecInformation::sample()
    };
    assert_eq!(bare.to_string(), "an unknown codec");
    let newer = CodecInformation {
      codec: Some(Codec::Unrecognized(UnrecognizedValue::sample("future_codec"))),
      raw_identifiers: Vec::new(),
      ..CodecInformation::sample()
    };
    assert_eq!(newer.to_string(), "future_codec");
  }

  #[test]
  fn displays_an_absent_codec_by_its_cause() {
    assert_eq!(CodecInformation::unnamed().to_string(), "a codec that was not read");
    let lost = CodecInformation {
      has_every_candidate: false,
      ..CodecInformation::unnamed()
    };
    assert_eq!(lost.to_string(), "a description that was not read");
    let unresolved = CodecInformation {
      candidates: vec![
        CodecCandidate::sample(1, Codec::H264, Some("61766331")),
        CodecCandidate::sample(2, Codec::H265, Some("68766331")),
      ],
      ..CodecInformation::unnamed()
    };
    assert_eq!(unresolved.to_string(), "H.264 or H.265");
    let missing = CodecInformation {
      has_every_candidate: false,
      ..unresolved
    };
    assert_eq!(missing.to_string(), "H.264 or H.265 or a description that was not read");
  }
}
