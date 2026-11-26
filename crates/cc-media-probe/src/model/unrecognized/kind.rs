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

//! `UnrecognizedKind`, the catch-all of an enum that carries data (`api.md` section 1, rule 5, and section 12).

use super::OwnedText;

/// A kind of the enum `Owner` that a newer version wrote and this version does not know.
///
/// It keeps the kind's name, which `as_str()` returns, and, with the `serde` feature, the value's other members as
/// written and unread, which writing it gives back unchanged. It has no serialized form of its own (`api.md`
/// section 3.3), and its type parameter binds it to the enum that read it. A kind goes back into its own enum:
///
/// ```
/// use cc_media_probe::model::{TagValue, UnrecognizedKind};
///
/// fn again(kind: UnrecognizedKind<TagValue>) -> TagValue {
///     TagValue::Unrecognized(kind)
/// }
/// ```
///
/// and never into another, where its name could be a kind that the other enum knows without that kind's members:
///
/// ```compile_fail
/// use cc_media_probe::model::{ChromaFormat, TagValue, UnrecognizedKind};
///
/// fn moved(kind: UnrecognizedKind<ChromaFormat>) -> TagValue {
///     TagValue::Unrecognized(kind)
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct UnrecognizedKind<Owner> {
  kind: OwnedText<Owner>,
  #[cfg(feature = "serde")]
  members: crate::form::KeptMembers,
}

impl<Owner> UnrecognizedKind<Owner> {
  /// The kind's name, as written.
  ///
  /// ```
  /// use cc_media_probe::model::TagValue;
  ///
  /// fn kind(value: &TagValue) -> &str {
  ///     match value {
  ///         TagValue::Unrecognized(kind) => kind.as_str(),
  ///         known => known.as_str(),
  ///     }
  /// }
  /// ```
  #[must_use]
  pub fn as_str(&self) -> &str {
    self.kind.as_str()
  }

  /// The catch-all of a kind of a newer version, with its other members as written, as the reader of `Owner` keeps
  /// them.
  #[cfg(feature = "serde")]
  pub(crate) fn from_kept(kind: String, members: crate::form::KeptMembers) -> UnrecognizedKind<Owner> {
    UnrecognizedKind {
      kind: OwnedText::new(kind),
      members,
    }
  }

  /// Writes the kind and the members that it kept, in the order read.
  #[cfg(feature = "serde")]
  pub(crate) fn write_kept<Output: serde::Serializer>(&self, serializer: Output) -> Result<Output::Ok, Output::Error> {
    crate::form::write_kept(serializer, self.kind.as_str(), &self.members)
  }
}

#[cfg(test)]
pub(crate) mod tests {
  use super::UnrecognizedKind;
  use crate::model::TagValue;
  use crate::model::unrecognized::OwnedText;

  impl<Owner> UnrecognizedKind<Owner> {
    /// A payload with this kind, as the reader of `Owner` would keep it.
    pub(crate) fn sample(kind: &str) -> UnrecognizedKind<Owner> {
      UnrecognizedKind {
        kind: OwnedText::sample(kind),
        #[cfg(feature = "serde")]
        members: crate::form::KeptMembers::default(),
      }
    }
  }

  #[test]
  fn keeps_its_kind() {
    let kind = UnrecognizedKind::<TagValue>::sample("rating");
    assert_eq!(kind.as_str(), "rating");
    assert_eq!(kind.clone(), kind);
    assert!(format!("{kind:?}").starts_with("UnrecognizedKind { kind: \"rating\""));
  }
}
