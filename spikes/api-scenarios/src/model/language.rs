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

//! Languages and localized text, `api.md` section 6.1.

/// A language as the file writes it, and its BCP 47 form.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Language {
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
  tag: Option<String>,
  as_written: String,
}

impl Language {
  /// The BCP 47 tag; absent for `und`.
  pub fn tag(&self) -> Option<&str> {
    self.tag.as_deref()
  }

  /// The language exactly as the file stores it.
  pub fn as_written(&self) -> &str {
    &self.as_written
  }
}

/// A text with the languages and countries it applies to.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LocalizedText {
  text: String,
  languages: Vec<Language>,
  countries: Vec<String>,
}

impl LocalizedText {
  /// The text.
  pub fn text(&self) -> &str {
    &self.text
  }

  /// The languages it applies to; may be empty.
  pub fn languages(&self) -> &[Language] {
    &self.languages
  }

  /// The ISO 3166 codes of the countries it applies to, as written; may be empty.
  pub fn countries(&self) -> &[String] {
    &self.countries
  }
}
