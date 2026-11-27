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

//! `Language`, a language as the file writes it and its BCP 47 form (`api.md` section 6.1).

use std::fmt;

use super::LanguageCodeSystem;
use crate::form_record::form_record;

form_record! {
  /// A language as the file writes it, with its BCP 47 form.
  ///
  /// The BCP 47 tag is absent for `und`, while `mis`, `mul` and `zxx` stay as written (field guide section 2). A
  /// language displays its tag, or the code as written where it has none (`api.md` section 3.3).
  ///
  /// ```
  /// use cc_media_probe::Stream;
  ///
  /// fn language(stream: &Stream) -> &str {
  ///     stream
  ///         .language()
  ///         .map_or("unknown", |language| language.tag().unwrap_or(language.as_written()))
  /// }
  /// ```
  #[derive(Clone, Debug, PartialEq, Eq, Hash)]
  pub struct Language {
    as_written: String,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    tag: Option<String>,
    code_system: LanguageCodeSystem,
  }
}

impl Language {
  /// The BCP 47 tag, such as `de` for the ISO 639-2 code `ger`, or `None` for `und`.
  ///
  /// ```
  /// use cc_media_probe::model::Language;
  ///
  /// fn is_german(language: &Language) -> bool {
  ///     language.tag().is_some_and(|tag| tag == "de" || tag.starts_with("de-"))
  /// }
  /// ```
  #[must_use]
  pub fn tag(&self) -> Option<&str> {
    self.tag.as_deref()
  }

  /// The code exactly as the file writes it, such as `ger`.
  ///
  /// ```
  /// use cc_media_probe::model::Language;
  ///
  /// fn as_stored(language: &Language) -> &str {
  ///     language.as_written()
  /// }
  /// ```
  #[must_use]
  pub fn as_written(&self) -> &str {
    &self.as_written
  }

  /// The system of codes in which the file writes it.
  ///
  /// ```
  /// use cc_media_probe::model::{Language, LanguageCodeSystem};
  ///
  /// fn from_a_legacy_element(language: &Language) -> bool {
  ///     matches!(language.code_system(), LanguageCodeSystem::MatroskaLegacy)
  /// }
  /// ```
  #[must_use]
  pub fn code_system(&self) -> &LanguageCodeSystem {
    &self.code_system
  }
}

impl fmt::Display for Language {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(self.tag().unwrap_or(&self.as_written))
  }
}

#[cfg(test)]
mod tests {
  use super::Language;
  use crate::model::LanguageCodeSystem;

  impl Language {
    /// A language written as `as_written`, with this tag.
    pub(crate) fn sample(as_written: &str, tag: Option<&str>, code_system: LanguageCodeSystem) -> Language {
      Language {
        as_written: as_written.to_owned(),
        tag: tag.map(str::to_owned),
        code_system,
      }
    }
  }

  #[test]
  fn keeps_the_code_as_written_and_its_tag() {
    let german = Language::sample("ger", Some("de"), LanguageCodeSystem::Iso639Alpha3);
    assert_eq!(german.as_written(), "ger");
    assert_eq!(german.tag(), Some("de"));
    assert_eq!(german.code_system(), &LanguageCodeSystem::Iso639Alpha3);
    let undetermined = Language::sample("und", None, LanguageCodeSystem::Bcp47);
    assert_eq!(undetermined.tag(), None);
  }

  #[test]
  fn displays_its_tag_or_else_the_code_as_written() {
    let german = Language::sample("ger", Some("de"), LanguageCodeSystem::Iso639Alpha3);
    assert_eq!(german.to_string(), "de");
    let undetermined = Language::sample("und", None, LanguageCodeSystem::Iso639Alpha3);
    assert_eq!(undetermined.to_string(), "und");
  }
}
