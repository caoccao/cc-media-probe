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

//! `CodecCandidate`, a sample description that could hold the first sample (`api.md` section 6.4).

use std::fmt;

use super::RawCodecIdentifier;
use super::constants::UNKNOWN_CODEC_TEXT;
use crate::Codec;
use crate::form_record::form_record;

form_record! {
  /// One sample description that could hold the first sample, while the first sample's description is unproven,
  /// with its own codec and identifiers. `Display` writes its codec's name, or the serialized name of a codec that a
  /// newer version wrote, or else its first raw identifier as written.
  ///
  /// ```
  /// use cc_media_probe::Stream;
  ///
  /// fn candidates(stream: &Stream) -> Vec<String> {
  ///     stream
  ///         .codec()
  ///         .candidates()
  ///         .iter()
  ///         .map(|candidate| {
  ///             let codes: Vec<&str> = candidate.raw_identifiers().iter().map(|code| code.value()).collect();
  ///             format!(
  ///                 "description {}: {candidate} ({}) {codes:?}",
  ///                 candidate.sample_description_index(),
  ///                 candidate.codec().as_str(),
  ///             )
  ///         })
  ///         .collect()
  /// }
  /// ```
  #[derive(Clone, Debug, PartialEq, Eq, Hash)]
  pub struct CodecCandidate {
    sample_description_index: u64,
    codec: Codec,
    raw_identifiers: Vec<RawCodecIdentifier>,
  }
  check = CodecCandidate::check_form;
}

impl CodecCandidate {
  /// The description's index as the file numbers it, from 1 in the ISO family.
  #[must_use]
  pub fn sample_description_index(&self) -> u64 {
    self.sample_description_index
  }

  /// The codec that the description names.
  #[must_use]
  pub fn codec(&self) -> &Codec {
    &self.codec
  }

  /// The description's own codec identifiers, as written, such as a protected entry's code and the code of its
  /// original format.
  #[must_use]
  pub fn raw_identifiers(&self) -> &[RawCodecIdentifier] {
    &self.raw_identifiers
  }

  /// Whether the index counts from 1, as a sample description's does, which needs the candidate alone (review of
  /// the third slice's first part).
  #[cfg(feature = "serde")]
  fn check_form(&self) -> Result<(), &'static str> {
    (self.sample_description_index >= 1)
      .then_some(())
      .ok_or(super::constants::CODEC_AGREEMENT_MESSAGE)
  }
}

/// The display of a present codec: its name or the serialized name that a newer version wrote, else the first raw
/// identifier, else "an unknown codec".
pub(super) fn codec_text<'text>(codec: &'text Codec, raw_identifiers: &'text [RawCodecIdentifier]) -> &'text str {
  let raw = raw_identifiers.first().map(RawCodecIdentifier::value);
  codec.display_name().or(raw).unwrap_or(UNKNOWN_CODEC_TEXT)
}

impl fmt::Display for CodecCandidate {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(codec_text(&self.codec, &self.raw_identifiers))
  }
}

#[cfg(test)]
mod tests {
  use super::CodecCandidate;
  use crate::Codec;
  use crate::model::{CodecNamespace, RawCodecIdentifier, UnrecognizedValue};

  impl CodecCandidate {
    /// A candidate of this description, codec and sample entry code.
    pub(crate) fn sample(sample_description_index: u64, codec: Codec, code: Option<&str>) -> CodecCandidate {
      CodecCandidate {
        sample_description_index,
        codec,
        raw_identifiers: code
          .map(|code| RawCodecIdentifier::sample(CodecNamespace::IsoSampleEntry, code))
          .into_iter()
          .collect(),
      }
    }
  }

  #[test]
  fn displays_its_codec_or_its_code() {
    let known = CodecCandidate::sample(1, Codec::H264, Some("61766331"));
    assert_eq!(known.sample_description_index(), 1);
    assert_eq!(known.codec(), &Codec::H264);
    assert_eq!(known.raw_identifiers().len(), 1);
    assert_eq!(known.to_string(), "H.264");
    assert_eq!(
      CodecCandidate::sample(2, Codec::Unknown, Some("66757431")).to_string(),
      "66757431"
    );
    assert_eq!(
      CodecCandidate::sample(3, Codec::Unknown, None).to_string(),
      "an unknown codec"
    );
    let newer = Codec::Unrecognized(UnrecognizedValue::sample("future_codec"));
    assert_eq!(CodecCandidate::sample(4, newer, None).to_string(), "future_codec");
  }
}
