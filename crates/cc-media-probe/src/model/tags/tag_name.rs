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

//! `TagName`, a tag's name as the file writes it (`api.md` section 6.9).

use crate::form_record::form_record;

form_record! {
  /// A tag's name as the file writes it: decoded by the rules of its format or namespace, with its namespace, and
  /// where the format defines a key as bytes, the bytes exactly as stored. Two keys that decode alike but differ in
  /// their bytes stay distinct.
  ///
  /// ```
  /// use cc_media_probe::model::Tag;
  ///
  /// fn name(tag: &Tag) -> String {
  ///     let name = tag.name();
  ///     match (name.namespace(), name.bytes()) {
  ///         (Some(namespace), Some(bytes)) => format!("{namespace}:{} ({bytes:02x?})", name.as_written()),
  ///         (Some(namespace), None) => format!("{namespace}:{}", name.as_written()),
  ///         (None, _) => name.as_written().to_owned(),
  ///     }
  /// }
  /// ```
  #[derive(Clone, Debug, PartialEq, Eq, Hash)]
  pub struct TagName {
    as_written: String,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    namespace: Option<String>,
    #[cfg_attr(
      feature = "serde",
      serde(
        default,
        skip_serializing_if = "Option::is_none",
        serialize_with = "crate::form::write_optional_bytes",
        deserialize_with = "crate::form::read_optional_bytes"
      )
    )]
    bytes: Option<Vec<u8>>,
  }
}

impl TagName {
  /// The name decoded by the rules of its format or namespace, with U+FFFD for any byte those rules cannot decode.
  #[must_use]
  pub fn as_written(&self) -> &str {
    &self.as_written
  }

  /// The namespace of a key that has one, such as QuickTime's `mdta`.
  #[must_use]
  pub fn namespace(&self) -> Option<&str> {
    self.namespace.as_deref()
  }

  /// The key exactly as stored, where the format defines a key as bytes that its namespace interprets, as
  /// QuickTime's metadata keys; absent for a name stored as text, as Matroska's.
  #[must_use]
  pub fn bytes(&self) -> Option<&[u8]> {
    self.bytes.as_deref()
  }
}

#[cfg(test)]
mod tests {
  use super::TagName;

  impl TagName {
    /// The name `as_written`, without a namespace or bytes.
    pub(crate) fn sample(as_written: &str) -> TagName {
      TagName {
        as_written: as_written.to_owned(),
        namespace: None,
        bytes: None,
      }
    }
  }

  #[test]
  fn keeps_the_name_and_its_bytes() {
    let key = TagName {
      as_written: "\u{a9}nam".to_owned(),
      namespace: Some("udta".to_owned()),
      bytes: Some(vec![0xa9, b'n', b'a', b'm']),
    };
    assert_eq!(key.as_written(), "\u{a9}nam");
    assert_eq!(key.namespace(), Some("udta"));
    assert_eq!(key.bytes(), Some(&[0xa9, b'n', b'a', b'm'][..]));
    assert_eq!(TagName::sample("TITLE").bytes(), None);
  }
}
