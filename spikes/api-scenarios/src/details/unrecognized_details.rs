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

//! The catch-all of a typed detail, `api.md` section 11.1: a kind that a newer version wrote. Its owning enum,
//! `FormatDetails` or `CodecDetails`, is its type parameter, as the maintainer chose at the ninth review, so a
//! format's detail never moves into the codec details. The stub keeps only the kind; task 5 keeps its members too.

use std::fmt;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

/// The details of a kind of the enum `Owner` that a newer version wrote. It has no serialized form of its own.
///
/// A format's catch-all never becomes a codec's, where its kind could name a codec that this version knows:
///
/// ```compile_fail
/// use cc_media_probe::details::{CodecDetails, FormatDetails, UnrecognizedDetails};
///
/// fn moved(details: UnrecognizedDetails<FormatDetails>) -> CodecDetails {
///     CodecDetails::Unrecognized(details)
/// }
/// ```
pub struct UnrecognizedDetails<Owner> {
  kind: String,
  owner: PhantomData<fn() -> Owner>,
}

impl<Owner> UnrecognizedDetails<Owner> {
  /// The kind name as written.
  pub fn kind(&self) -> &str {
    &self.kind
  }

  /// The details of a kind that the reader of `Owner` does not know.
  #[cfg(feature = "serde")]
  pub(super) fn new(kind: String) -> UnrecognizedDetails<Owner> {
    UnrecognizedDetails {
      kind,
      owner: PhantomData,
    }
  }
}

impl<Owner> Clone for UnrecognizedDetails<Owner> {
  fn clone(&self) -> UnrecognizedDetails<Owner> {
    UnrecognizedDetails {
      kind: self.kind.clone(),
      owner: PhantomData,
    }
  }
}

impl<Owner> PartialEq for UnrecognizedDetails<Owner> {
  fn eq(&self, other: &UnrecognizedDetails<Owner>) -> bool {
    self.kind == other.kind
  }
}

impl<Owner> Eq for UnrecognizedDetails<Owner> {}

impl<Owner> Hash for UnrecognizedDetails<Owner> {
  fn hash<State: Hasher>(&self, state: &mut State) {
    self.kind.hash(state);
  }
}

impl<Owner> fmt::Debug for UnrecognizedDetails<Owner> {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt::Debug::fmt(&self.kind, formatter)
  }
}
