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

//! `Identifier`, a value whose meaning is identity (`api.md` section 6.1).

use std::fmt;

/// A value whose meaning is identity, such as a track's unique identifier, as text: an integer in canonical
/// decimal, a binary value in lowercase hexadecimal (R24).
///
/// ```
/// use cc_media_probe::Stream;
///
/// fn unique_identifier(stream: &Stream) -> &str {
///     stream.unique_identifier().map_or("none", |identifier| identifier.as_str())
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Identifier(String);

#[cfg(feature = "serde")]
crate::form::text_form!(Identifier);

impl Identifier {
  /// The identifier as text. `Display` writes the same text.
  ///
  /// ```
  /// use cc_media_probe::Stream;
  ///
  /// fn same_identifier(first: &Stream, second: &Stream) -> bool {
  ///     first.unique_identifier().map(|identifier| identifier.as_str())
  ///         == second.unique_identifier().map(|identifier| identifier.as_str())
  /// }
  /// ```
  #[must_use]
  pub fn as_str(&self) -> &str {
    &self.0
  }
}

impl fmt::Display for Identifier {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.0)
  }
}

#[cfg(test)]
mod tests {
  use super::Identifier;

  impl Identifier {
    /// An identifier with this text.
    pub(crate) fn sample(text: &str) -> Identifier {
      Identifier(text.to_owned())
    }
  }

  #[test]
  fn keeps_its_text() {
    let identifier = Identifier::sample("12345");
    assert_eq!(identifier.as_str(), "12345");
    assert_eq!(identifier.to_string(), "12345");
  }
}
