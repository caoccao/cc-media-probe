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

//! `LocalizedText`, a text with the languages and countries it applies to (`api.md` section 6.1).

use super::Language;
use crate::form_record::form_record;

form_record! {
  /// A text, such as a title, with the languages and the countries that it applies to. Both lists may be empty.
  ///
  /// ```
  /// use cc_media_probe::Stream;
  ///
  /// fn titles(stream: &Stream) -> Vec<String> {
  ///     stream
  ///         .titles()
  ///         .iter()
  ///         .map(|title| {
  ///             let languages: Vec<&str> = title.languages().iter().map(|language| language.as_written()).collect();
  ///             format!("{} [{}] {:?}", title.text(), languages.join(", "), title.countries())
  ///         })
  ///         .collect()
  /// }
  /// ```
  #[derive(Clone, Debug, PartialEq, Eq, Hash)]
  pub struct LocalizedText {
    text: String,
    languages: Vec<Language>,
    countries: Vec<String>,
  }
}

impl LocalizedText {
  /// The text.
  ///
  /// ```
  /// use cc_media_probe::model::LocalizedText;
  ///
  /// fn is_blank(text: &LocalizedText) -> bool {
  ///     text.text().trim().is_empty()
  /// }
  /// ```
  #[must_use]
  pub fn text(&self) -> &str {
    &self.text
  }

  /// The languages that the text applies to, in stored order.
  ///
  /// ```
  /// use cc_media_probe::model::LocalizedText;
  ///
  /// fn applies_to_every_language(text: &LocalizedText) -> bool {
  ///     text.languages().is_empty()
  /// }
  /// ```
  #[must_use]
  pub fn languages(&self) -> &[Language] {
    &self.languages
  }

  /// The countries that the text applies to, as ISO 3166 codes as written.
  ///
  /// ```
  /// use cc_media_probe::model::LocalizedText;
  ///
  /// fn for_country(text: &LocalizedText, country: &str) -> bool {
  ///     text.countries().iter().any(|code| code == country)
  /// }
  /// ```
  #[must_use]
  pub fn countries(&self) -> &[String] {
    &self.countries
  }
}

#[cfg(test)]
mod tests {
  use super::LocalizedText;
  use crate::model::{Language, LanguageCodeSystem};

  impl LocalizedText {
    /// A text in these languages and countries.
    pub(crate) fn sample(text: &str, languages: Vec<Language>, countries: &[&str]) -> LocalizedText {
      LocalizedText {
        text: text.to_owned(),
        languages,
        countries: countries.iter().map(|country| (*country).to_owned()).collect(),
      }
    }
  }

  #[test]
  fn keeps_the_text_and_where_it_applies() {
    let english = Language::sample("eng", Some("en"), LanguageCodeSystem::Iso639Alpha3);
    let title = LocalizedText::sample("Prologue", vec![english.clone()], &["US"]);
    assert_eq!(title.text(), "Prologue");
    assert_eq!(title.languages(), &[english]);
    assert_eq!(title.countries(), &["US".to_owned()]);
  }
}
