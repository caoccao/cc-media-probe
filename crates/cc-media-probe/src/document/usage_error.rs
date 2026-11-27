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

//! `UsageError`, a command line that could not be run (`api.md` section 12; `json-output.md` section 3; plan
//! section 8.9).

use crate::form_record::form_record;

form_record! {
  /// A command line that could not be run, as the `usage_error` object of the document. It is an object with its
  /// message, so that it can gain members later. The message is unescaped in the document; the command-line tool
  /// writes it escaped on standard error (`cli.md` section 4).
  ///
  /// ```
  /// use cc_media_probe::document::UsageError;
  ///
  /// let error = UsageError::new("standard input (-) was named twice");
  /// let text = r#"{"message": "standard input (-) was named twice"}"#;
  /// assert_eq!(serde_json::from_str::<UsageError>(text).ok(), Some(error));
  /// ```
  #[derive(Clone, Debug, PartialEq, Eq, Hash)]
  pub struct UsageError {
    message: String,
  }
}

impl UsageError {
  /// A usage error with that message.
  ///
  /// ```
  /// use cc_media_probe::document::UsageError;
  ///
  /// let error = UsageError::new("no input was named");
  /// assert_eq!(error.message(), "no input was named");
  /// ```
  #[must_use]
  pub fn new(message: impl Into<String>) -> UsageError {
    UsageError {
      message: message.into(),
    }
  }

  /// The message, for people.
  ///
  /// ```
  /// use cc_media_probe::document::UsageError;
  ///
  /// fn report(error: &UsageError) {
  ///     eprintln!("usage: {}", error.message());
  /// }
  /// # report(&UsageError::new("no input was named"));
  /// ```
  #[must_use]
  pub fn message(&self) -> &str {
    &self.message
  }
}
