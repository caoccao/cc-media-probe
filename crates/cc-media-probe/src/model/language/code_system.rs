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

//! `LanguageCodeSystem`, the system of codes in which a file writes a language (`api.md` section 6.1).

use super::constants::{BCP47_NAME, ISO639_ALPHA3_NAME, MATROSKA_LEGACY_NAME, QUICKTIME_PACKED_NAME};
use crate::string_enumeration::string_enumeration;

string_enumeration! {
  /// The system of codes in which a file writes a language. A format phase adds the systems that its format
  /// defines.
  ///
  /// ```
  /// use cc_media_probe::model::{Language, LanguageCodeSystem};
  ///
  /// fn written_as_bcp47(language: &Language) -> bool {
  ///     matches!(language.code_system(), LanguageCodeSystem::Bcp47)
  /// }
  ///
  /// assert_eq!(LanguageCodeSystem::Iso639Alpha3.as_str(), "iso639_alpha3");
  /// ```
  LanguageCodeSystem {
    /// A BCP 47 language tag (RFC 5646), as Matroska's `LanguageBCP47` stores it.
    Bcp47 => BCP47_NAME,
    /// A three-letter code of ISO 639-2.
    Iso639Alpha3 => ISO639_ALPHA3_NAME,
    /// The code of Matroska's older `Language` element: ISO 639-2, optionally followed by a country code.
    MatroskaLegacy => MATROSKA_LEGACY_NAME,
    /// QuickTime's packed language code: three letters of five bits each, or a Macintosh language code.
    QuickTimePacked => QUICKTIME_PACKED_NAME,
  }
}

#[cfg(test)]
mod tests {
  use super::LanguageCodeSystem;
  use crate::model::UnrecognizedValue;

  #[test]
  fn names_each_system() {
    assert_eq!(LanguageCodeSystem::Bcp47.as_str(), "bcp47");
    assert_eq!(LanguageCodeSystem::Iso639Alpha3.as_str(), "iso639_alpha3");
    assert_eq!(LanguageCodeSystem::MatroskaLegacy.as_str(), "matroska_legacy");
    assert_eq!(LanguageCodeSystem::QuickTimePacked.as_str(), "quicktime_packed");
    assert_eq!(
      LanguageCodeSystem::Unrecognized(UnrecognizedValue::sample("iso639_alpha2")).as_str(),
      "iso639_alpha2"
    );
  }
}
