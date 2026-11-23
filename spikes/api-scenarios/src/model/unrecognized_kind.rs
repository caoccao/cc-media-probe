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

//! The catch-all of an enum that carries data, `api.md` section 12: a kind that a newer version wrote. Its owning
//! enum is its type parameter, as the maintainer chose at the ninth review, so only a reader of that enum makes one,
//! and the kind it keeps is never one that the enum knows.

use std::fmt;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

#[cfg(feature = "serde")]
use crate::kept::KeptMembers;

/// A kind of the enum `Owner` that a newer version wrote. It keeps the kind's name and, with the `serde` feature, the
/// value's other members as written and unread, which writing it gives back unchanged, as the maintainer chose at
/// the seventh review. It has no serialized form of its own (`api.md` section 3.3).
///
/// A kind goes back into the enum that read it:
///
/// ```
/// use cc_media_probe::model::{TagValue, UnrecognizedKind};
///
/// fn again(kind: UnrecognizedKind<TagValue>) -> TagValue {
///     TagValue::Unrecognized(kind)
/// }
/// ```
///
/// and never into another, as the ninth review's `text`, read as a chroma format, would be a tag value without its
/// text:
///
/// ```compile_fail
/// use cc_media_probe::model::{ChromaFormat, TagValue, UnrecognizedKind};
///
/// fn moved(kind: UnrecognizedKind<ChromaFormat>) -> TagValue {
///     TagValue::Unrecognized(kind)
/// }
/// ```
pub struct UnrecognizedKind<Owner> {
  kind: String,
  #[cfg(feature = "serde")]
  members: KeptMembers,
  owner: PhantomData<fn() -> Owner>,
}

impl<Owner> UnrecognizedKind<Owner> {
  /// The kind's name, as written.
  pub fn as_str(&self) -> &str {
    &self.kind
  }

  /// A kind that the reader of `Owner` does not know, with the members that its value holds besides the kind.
  #[cfg(feature = "serde")]
  pub(crate) fn new(kind: String, members: KeptMembers) -> UnrecognizedKind<Owner> {
    UnrecognizedKind {
      kind,
      members,
      owner: PhantomData,
    }
  }

  /// The members that the value holds besides the kind, in the order written.
  #[cfg(feature = "serde")]
  pub(crate) fn members(&self) -> &KeptMembers {
    &self.members
  }

  /// The kind and its members, which the traits below compare, hash and show.
  #[cfg(feature = "serde")]
  fn parts(&self) -> (&str, &KeptMembers) {
    (&self.kind, &self.members)
  }

  /// The kind, which the traits below compare, hash and show.
  #[cfg(not(feature = "serde"))]
  fn parts(&self) -> &str {
    &self.kind
  }
}

impl<Owner> Clone for UnrecognizedKind<Owner> {
  fn clone(&self) -> UnrecognizedKind<Owner> {
    UnrecognizedKind {
      kind: self.kind.clone(),
      #[cfg(feature = "serde")]
      members: self.members.clone(),
      owner: PhantomData,
    }
  }
}

impl<Owner> PartialEq for UnrecognizedKind<Owner> {
  fn eq(&self, other: &UnrecognizedKind<Owner>) -> bool {
    self.parts() == other.parts()
  }
}

impl<Owner> Eq for UnrecognizedKind<Owner> {}

impl<Owner> Hash for UnrecognizedKind<Owner> {
  fn hash<State: Hasher>(&self, state: &mut State) {
    self.parts().hash(state);
  }
}

impl<Owner> fmt::Debug for UnrecognizedKind<Owner> {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt::Debug::fmt(&self.parts(), formatter)
  }
}
