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

//! `StreamLanguage` and `LanguageRole`, one of a stream's languages with its roles (`api.md` section 6.3; F24).

use super::constants::{CLEAN_EFFECTS_NAME, HEARING_IMPAIRED_NAME, VISUALLY_IMPAIRED_COMMENTARY_NAME};
use crate::model::Language;
use crate::string_enumeration::string_enumeration;

/// One of a stream's languages, with the roles that the file gives it (F24).
///
/// ```
/// use cc_media_probe::Stream;
/// use cc_media_probe::model::LanguageRole;
///
/// fn audio_description_languages(stream: &Stream) -> Vec<&str> {
///     stream
///         .languages()
///         .iter()
///         .filter(|entry| entry.roles().iter().any(|role| matches!(role, LanguageRole::VisuallyImpairedCommentary)))
///         .map(|entry| entry.language().as_written())
///         .collect()
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct StreamLanguage {
  language: Language,
  roles: Vec<LanguageRole>,
}

impl StreamLanguage {
  /// The language.
  #[must_use]
  pub fn language(&self) -> &Language {
    &self.language
  }

  /// The roles that the file gives the language, in stored order; empty when it gives none.
  #[must_use]
  pub fn roles(&self) -> &[LanguageRole] {
    &self.roles
  }
}

string_enumeration! {
  /// The role of one of a stream's languages, as a format states it. A format phase adds the roles that its
  /// format defines.
  ///
  /// ```
  /// use cc_media_probe::model::LanguageRole;
  ///
  /// assert_eq!(LanguageRole::HearingImpaired.as_str(), "hearing_impaired");
  /// ```
  LanguageRole {
    /// Audio without speech, such as music and effects.
    CleanEffects => CLEAN_EFFECTS_NAME,
    /// For the hearing impaired.
    HearingImpaired => HEARING_IMPAIRED_NAME,
    /// Commentary for the visually impaired.
    VisuallyImpairedCommentary => VISUALLY_IMPAIRED_COMMENTARY_NAME,
  }
}

#[cfg(test)]
mod tests {
  use super::{LanguageRole, StreamLanguage};
  use crate::model::{Language, LanguageCodeSystem, UnrecognizedValue};

  impl StreamLanguage {
    /// The language with these roles.
    pub(crate) fn sample(language: Language, roles: Vec<LanguageRole>) -> StreamLanguage {
      StreamLanguage { language, roles }
    }
  }

  #[test]
  fn keeps_the_language_and_its_roles() {
    let english = Language::sample("eng", Some("en"), LanguageCodeSystem::Iso639Alpha3);
    let entry = StreamLanguage::sample(english.clone(), vec![LanguageRole::HearingImpaired]);
    assert_eq!(entry.language(), &english);
    assert_eq!(entry.roles(), &[LanguageRole::HearingImpaired]);
  }

  #[test]
  fn names_each_role() {
    for (role, name) in [
      (LanguageRole::CleanEffects, "clean_effects"),
      (LanguageRole::HearingImpaired, "hearing_impaired"),
      (LanguageRole::VisuallyImpairedCommentary, "visually_impaired_commentary"),
      (
        LanguageRole::Unrecognized(UnrecognizedValue::sample("sign_language")),
        "sign_language",
      ),
    ] {
      assert_eq!(role.as_str(), name);
    }
  }
}
