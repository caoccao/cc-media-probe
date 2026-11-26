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

//! `RawCodecIdentifier` and `CodecNamespace`, a codec identifier as the input wrote it (`api.md` section 6.4).

use super::constants::{
  ISO_SAMPLE_ENTRY_NAME, MATROSKA_CODEC_IDENTIFIER_NAME, MPEG4_OBJECT_TYPE_NAME, QUICKTIME_CODE_NAME,
  RIFF_FOUR_CHARACTER_CODE_NAME, WAVE_FORMAT_TAG_NAME, WAVE_SUBFORMAT_NAME,
};
use crate::form_record::form_record;
use crate::string_enumeration::string_enumeration;

form_record! {
  /// A codec identifier as the input wrote it, in the spelling of its namespace: a text code is its text, a code of
  /// bytes is its bytes as stored in lowercase hexadecimal, two digits a byte, and a coded number is canonical
  /// decimal. Every code has one spelling, so two identifiers name the same code exactly when their namespaces and
  /// values are equal.
  ///
  /// ```
  /// use cc_media_probe::Stream;
  /// use cc_media_probe::model::CodecNamespace;
  ///
  /// fn sample_entry(stream: &Stream) -> Option<&str> {
  ///     stream
  ///         .codec()
  ///         .raw_identifiers()
  ///         .iter()
  ///         .find(|identifier| matches!(identifier.namespace(), CodecNamespace::IsoSampleEntry))
  ///         .map(|identifier| identifier.value())
  /// }
  /// ```
  #[derive(Clone, Debug, PartialEq, Eq, Hash)]
  pub struct RawCodecIdentifier {
    namespace: CodecNamespace,
    value: String,
  }
  check = RawCodecIdentifier::check_form;
}

impl RawCodecIdentifier {
  /// The namespace of the identifier.
  #[must_use]
  pub fn namespace(&self) -> &CodecNamespace {
    &self.namespace
  }

  /// The identifier as written, in the spelling of its namespace: `V_MPEG4/ISO/AVC`, `61766331` for `avc1`, or `85`
  /// for a WAVE format tag.
  #[must_use]
  pub fn value(&self) -> &str {
    &self.value
  }

  /// Whether the value is in the spelling of its namespace (`json-output.md` section 7.3): a Matroska `CodecID` of
  /// printable ASCII and U+FFFD only, the one text that its decoder writes; four bytes for a four-character, sample
  /// entry or QuickTime code; 16 bytes for a WAVE `SubFormat`; and canonical decimal of 16 bits for a WAVE format tag
  /// and of 8 bits for an MPEG-4 object type. A value of a namespace of a newer version is kept as written.
  #[cfg(feature = "serde")]
  fn check_form(&self) -> Result<(), &'static str> {
    use super::constants::{FOUR_CHARACTER_BYTES, PRINTABLE_FIRST, PRINTABLE_LAST, RAW_CODE_MESSAGE, SUBFORMAT_BYTES};
    let spelled = match self.namespace {
      CodecNamespace::MatroskaCodecIdentifier => self.value.chars().all(|character| {
        character == char::REPLACEMENT_CHARACTER || (PRINTABLE_FIRST..=PRINTABLE_LAST).contains(&character)
      }),
      CodecNamespace::RiffFourCharacterCode | CodecNamespace::IsoSampleEntry | CodecNamespace::QuickTimeCode => {
        crate::form::is_bytes(&self.value, FOUR_CHARACTER_BYTES)
      }
      CodecNamespace::WaveSubformat => crate::form::is_bytes(&self.value, SUBFORMAT_BYTES),
      CodecNamespace::WaveFormatTag => crate::form::is_decimal::<u16>(&self.value),
      CodecNamespace::Mpeg4ObjectType => crate::form::is_decimal::<u8>(&self.value),
      CodecNamespace::Unrecognized(_) => true,
    };
    if spelled { Ok(()) } else { Err(RAW_CODE_MESSAGE) }
  }
}

string_enumeration! {
  /// The namespace of a raw codec identifier. A later phase adds its namespaces, spelled by the same rules.
  ///
  /// ```
  /// use cc_media_probe::model::CodecNamespace;
  ///
  /// assert_eq!(CodecNamespace::MatroskaCodecIdentifier.as_str(), "matroska_codec_identifier");
  /// ```
  CodecNamespace {
    /// A Matroska `CodecID`, its text without the null octets that may end an EBML string.
    MatroskaCodecIdentifier => MATROSKA_CODEC_IDENTIFIER_NAME,
    /// A RIFF four-character code, its four bytes as stored.
    RiffFourCharacterCode => RIFF_FOUR_CHARACTER_CODE_NAME,
    /// A WAVE format tag, a 16-bit number in canonical decimal (RFC 2361).
    WaveFormatTag => WAVE_FORMAT_TAG_NAME,
    /// The `SubFormat` GUID of an extensible WAVE format, its 16 bytes as stored.
    WaveSubformat => WAVE_SUBFORMAT_NAME,
    /// An ISO sample entry code, its four bytes as stored.
    IsoSampleEntry => ISO_SAMPLE_ENTRY_NAME,
    /// An MPEG-4 object type, an 8-bit number in canonical decimal.
    Mpeg4ObjectType => MPEG4_OBJECT_TYPE_NAME,
    /// A QuickTime code, its four bytes as stored.
    QuickTimeCode => QUICKTIME_CODE_NAME,
  }
}

#[cfg(test)]
mod tests {
  use super::{CodecNamespace, RawCodecIdentifier};
  use crate::model::UnrecognizedValue;

  impl RawCodecIdentifier {
    /// The identifier `value` in `namespace`.
    pub(crate) fn sample(namespace: CodecNamespace, value: &str) -> RawCodecIdentifier {
      RawCodecIdentifier {
        namespace,
        value: value.to_owned(),
      }
    }
  }

  #[test]
  fn keeps_the_code_in_the_spelling_of_its_namespace() {
    let identifier = RawCodecIdentifier::sample(CodecNamespace::IsoSampleEntry, "61766331");
    assert_eq!(identifier.namespace(), &CodecNamespace::IsoSampleEntry);
    assert_eq!(identifier.value(), "61766331");
  }

  #[test]
  fn names_each_namespace() {
    for (namespace, name) in [
      (CodecNamespace::MatroskaCodecIdentifier, "matroska_codec_identifier"),
      (CodecNamespace::RiffFourCharacterCode, "riff_four_character_code"),
      (CodecNamespace::WaveFormatTag, "wave_format_tag"),
      (CodecNamespace::WaveSubformat, "wave_subformat"),
      (CodecNamespace::IsoSampleEntry, "iso_sample_entry"),
      (CodecNamespace::Mpeg4ObjectType, "mpeg4_object_type"),
      (CodecNamespace::QuickTimeCode, "quicktime_code"),
      (
        CodecNamespace::Unrecognized(UnrecognizedValue::sample("ogg_header")),
        "ogg_header",
      ),
    ] {
      assert_eq!(namespace.as_str(), name);
    }
  }
}
