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

//! `Generator`, the program that wrote a document (`api.md` section 12; `json-output.md` section 3).

use crate::form_record::form_record;

form_record! {
  /// The program that wrote a document: its name and its version, both as text. Like every type of the form, it is
  /// read through `Deserialize` alone, which reads it strictly, so a generator written as the array of its values
  /// fails, as the nineteenth review of `api.md` asked.
  ///
  /// ```
  /// use cc_media_probe::document::Generator;
  ///
  /// let generator = Generator::new("media-catalog", "1.2.3");
  /// let text = r#"{"name": "media-catalog", "version": "1.2.3"}"#;
  /// assert_eq!(serde_json::from_str::<Generator>(text).ok(), Some(generator));
  /// assert!(serde_json::from_str::<Generator>(r#"["media-catalog", "1.2.3"]"#).is_err());
  /// ```
  ///
  /// No inherent function of Serde's derive stands beside the trait, so without the trait in scope the name reaches
  /// no `deserialize`:
  ///
  /// ```compile_fail,E0599
  /// use cc_media_probe::document::Generator;
  ///
  /// let mut input = serde_json::Deserializer::from_str(r#"["media-catalog", "1.2.3"]"#);
  /// let _ = Generator::deserialize(&mut input);
  /// ```
  ///
  /// With the trait in scope, the same call reaches it:
  ///
  /// ```
  /// use cc_media_probe::document::Generator;
  /// use serde::Deserialize;
  ///
  /// let mut input = serde_json::Deserializer::from_str(r#"["media-catalog", "1.2.3"]"#);
  /// assert!(Generator::deserialize(&mut input).is_err());
  /// ```
  #[derive(Clone, Debug, PartialEq, Eq, Hash)]
  pub struct Generator {
    name: String,
    version: String,
  }
}

impl Generator {
  /// A generator of that name and version, such as the command-line tool's `cc-media-probe-cli` and `0.1.0`.
  ///
  /// ```
  /// use cc_media_probe::document::Generator;
  ///
  /// let generator = Generator::new("media-catalog", env!("CARGO_PKG_VERSION"));
  /// assert_eq!(generator.name(), "media-catalog");
  /// ```
  #[must_use]
  pub fn new(name: impl Into<String>, version: impl Into<String>) -> Generator {
    Generator {
      name: name.into(),
      version: version.into(),
    }
  }

  /// The program's name.
  ///
  /// ```
  /// use cc_media_probe::document::Generator;
  ///
  /// assert_eq!(Generator::new("media-catalog", "1.2.3").name(), "media-catalog");
  /// ```
  #[must_use]
  pub fn name(&self) -> &str {
    &self.name
  }

  /// The program's version, as the program writes it.
  ///
  /// ```
  /// use cc_media_probe::document::Generator;
  ///
  /// assert_eq!(Generator::new("media-catalog", "1.2.3").version(), "1.2.3");
  /// ```
  #[must_use]
  pub fn version(&self) -> &str {
    &self.version
  }
}
