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

//! `UnrecognizedValue`, the catch-all of an enum whose values are strings (`api.md` section 1, rule 5).

use super::OwnedText;

/// The text that a newer version wrote for a value of the enum `Owner` that this version does not know.
///
/// It has no serialized form of its own (`api.md` section 3.3): the enum writes the text as its own value. Its type
/// parameter binds it to the enum that read it, so its text never becomes a value of another enum, where it could
/// name a value that the other enum knows. A payload goes back into its own enum:
///
/// ```
/// use cc_media_probe::StreamKind;
/// use cc_media_probe::model::UnrecognizedValue;
///
/// fn again(value: UnrecognizedValue<StreamKind>) -> StreamKind {
///     StreamKind::Unrecognized(value)
/// }
/// ```
///
/// and never into another:
///
/// ```compile_fail
/// use cc_media_probe::{Limit, StreamKind};
/// use cc_media_probe::model::UnrecognizedValue;
///
/// fn moved(value: UnrecognizedValue<StreamKind>) -> Limit {
///     Limit::Unrecognized(value)
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct UnrecognizedValue<Owner>(OwnedText<Owner>);

impl<Owner> UnrecognizedValue<Owner> {
  /// The text as written.
  ///
  /// ```
  /// use cc_media_probe::StreamKind;
  ///
  /// fn name(kind: &StreamKind) -> &str {
  ///     match kind {
  ///         StreamKind::Unrecognized(value) => value.as_str(),
  ///         known => known.as_str(),
  ///     }
  /// }
  /// ```
  #[must_use]
  pub fn as_str(&self) -> &str {
    self.0.as_str()
  }

  /// The catch-all of a name of a newer version, as the reader of `Owner` keeps it.
  #[cfg(feature = "serde")]
  pub(crate) fn from_text(text: String) -> UnrecognizedValue<Owner> {
    UnrecognizedValue(OwnedText::new(text))
  }
}

#[cfg(test)]
pub(crate) mod tests {
  use super::UnrecognizedValue;
  use crate::StreamKind;
  use crate::model::unrecognized::OwnedText;

  impl<Owner> UnrecognizedValue<Owner> {
    /// A payload with this text, as the reader of `Owner` would keep it.
    pub(crate) fn sample(text: &str) -> UnrecognizedValue<Owner> {
      UnrecognizedValue(OwnedText::sample(text))
    }
  }

  #[test]
  fn keeps_its_text() {
    let value = UnrecognizedValue::<StreamKind>::sample("hologram");
    assert_eq!(value.as_str(), "hologram");
    assert_eq!(value.clone(), value);
    assert_eq!(format!("{value:?}"), "UnrecognizedValue(\"hologram\")");
  }
}
