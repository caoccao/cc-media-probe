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

//! `TextEncoding` and `EncodingName`, the encoding of a text (`api.md` section 6.7).

use super::constants::{
  ISO8859_PART1_KIND, MAC_ROMAN_KIND, OTHER_KIND, UTF8_KIND, UTF16_BIG_ENDIAN_KIND, UTF16_LITTLE_ENDIAN_KIND,
  UTF32_BIG_ENDIAN_KIND, UTF32_LITTLE_ENDIAN_KIND, WINDOWS1252_KIND,
};
use crate::data_enumeration::data_enumeration;

data_enumeration! {
  /// The encoding of a text, such as a subtitle stream's events. An enum that carries data: serialized, it is a map
  /// of its `kind` and, for `Other`, the member named after it (`json-output.md` section 6.2).
  ///
  /// ```
  /// use cc_media_probe::model::TextEncoding;
  ///
  /// fn name(encoding: &TextEncoding) -> &str {
  ///     match encoding {
  ///         TextEncoding::Other(name) => name.as_str(),
  ///         known => known.as_str(),
  ///     }
  /// }
  ///
  /// assert_eq!(TextEncoding::Utf16LittleEndian.as_str(), "utf16_little_endian");
  /// ```
  TextEncoding {
    /// UTF-8.
    Utf8 => UTF8_KIND,
    /// UTF-16, little-endian.
    Utf16LittleEndian => UTF16_LITTLE_ENDIAN_KIND,
    /// UTF-16, big-endian.
    Utf16BigEndian => UTF16_BIG_ENDIAN_KIND,
    /// UTF-32, little-endian.
    Utf32LittleEndian => UTF32_LITTLE_ENDIAN_KIND,
    /// UTF-32, big-endian.
    Utf32BigEndian => UTF32_BIG_ENDIAN_KIND,
    /// ISO/IEC 8859-1.
    Iso8859Part1 => ISO8859_PART1_KIND,
    /// Windows code page 1252.
    Windows1252 => WINDOWS1252_KIND,
    /// Mac OS Roman.
    MacRoman => MAC_ROMAN_KIND,
    /// An encoding that the file names, kept as written.
    Other(EncodingName) => OTHER_KIND,
  }
}

/// The name of an encoding as the file writes it, such as `KOI8-R`. `TextEncoding` shows an example.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct EncodingName(String);

impl EncodingName {
  /// The name as written.
  #[must_use]
  pub fn as_str(&self) -> &str {
    &self.0
  }
}

#[cfg(test)]
mod tests {
  use super::{EncodingName, TextEncoding};
  use crate::model::UnrecognizedKind;

  impl EncodingName {
    /// The encoding named `name`.
    pub(crate) fn sample(name: &str) -> EncodingName {
      EncodingName(name.to_owned())
    }
  }

  #[test]
  fn names_each_kind() {
    for (encoding, name) in [
      (TextEncoding::Utf8, "utf8"),
      (TextEncoding::Utf16LittleEndian, "utf16_little_endian"),
      (TextEncoding::Utf16BigEndian, "utf16_big_endian"),
      (TextEncoding::Utf32LittleEndian, "utf32_little_endian"),
      (TextEncoding::Utf32BigEndian, "utf32_big_endian"),
      (TextEncoding::Iso8859Part1, "iso8859_part1"),
      (TextEncoding::Windows1252, "windows1252"),
      (TextEncoding::MacRoman, "mac_roman"),
      (TextEncoding::Other(EncodingName::sample("KOI8-R")), "other"),
      (
        TextEncoding::Unrecognized(UnrecognizedKind::sample("shift_jis")),
        "shift_jis",
      ),
    ] {
      assert_eq!(encoding.as_str(), name);
    }
    assert_eq!(EncodingName::sample("KOI8-R").as_str(), "KOI8-R");
  }
}
