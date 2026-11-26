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

//! `UnrecognizedDetails`, the catch-all of a typed detail (`api.md` section 11.1).

use super::enumeration::SharedDetails;
use super::{Conflict, ExactValue, ObservedBound};
use crate::model::UnrecognizedKind;

/// The details of a `kind` that a newer version wrote, for the enum `Owner` that read them. It keeps the kind's
/// name and, with the `serde` feature, the detail's other members as written and unread, which writing the
/// enclosing detail gives back unchanged. Like `UnrecognizedKind`, it has no serialized form of its own, and its
/// type parameter binds it to the enum that read it, so a format's catch-all never becomes a codec's. The library
/// never produces it while probing.
///
/// ```
/// use cc_media_probe::details::FormatDetails;
///
/// fn kind(details: &FormatDetails) -> &str {
///     match details {
///         FormatDetails::Unrecognized(unrecognized) => unrecognized.kind(),
///         _ => "a kind that this version knows",
///     }
/// }
/// ```
///
/// A payload never moves into the other enum:
///
/// ```compile_fail
/// use cc_media_probe::details::{CodecDetails, FormatDetails, UnrecognizedDetails};
///
/// fn moved(details: UnrecognizedDetails<FormatDetails>) -> CodecDetails {
///     CodecDetails::Unrecognized(details)
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct UnrecognizedDetails<Owner>(UnrecognizedKind<Owner>);

impl<Owner> UnrecognizedDetails<Owner> {
  /// The kind's name, as written.
  #[must_use]
  pub fn kind(&self) -> &str {
    self.0.as_str()
  }

  /// The catch-all of a kind of a newer version, with its other members as written, as the reader of `Owner` keeps
  /// them.
  #[cfg(feature = "serde")]
  pub(crate) fn from_kept(kind: String, members: crate::form::KeptMembers) -> UnrecognizedDetails<Owner> {
    UnrecognizedDetails(UnrecognizedKind::from_kept(kind, members))
  }

  /// Writes the kind and the members that it kept, in the order read.
  #[cfg(feature = "serde")]
  pub(crate) fn write_kept<Output: serde::Serializer>(&self, serializer: Output) -> Result<Output::Ok, Output::Error> {
    self.0.write_kept(serializer)
  }
}

impl<Owner> SharedDetails for UnrecognizedDetails<Owner> {
  /// None: the members of a newer kind stay unread.
  fn conflicts(&self) -> &[Conflict] {
    &[]
  }

  /// None: the members of a newer kind stay unread.
  fn observed_lower_bounds(&self) -> &[ObservedBound] {
    &[]
  }

  /// None: the members of a newer kind stay unread.
  fn exact_values(&self) -> &[ExactValue] {
    &[]
  }
}

#[cfg(test)]
mod tests {
  use super::UnrecognizedDetails;
  use crate::details::FormatDetails;
  use crate::model::UnrecognizedKind;

  impl<Owner> UnrecognizedDetails<Owner> {
    /// The details of this kind.
    pub(crate) fn sample(kind: &str) -> UnrecognizedDetails<Owner> {
      UnrecognizedDetails(UnrecognizedKind::sample(kind))
    }
  }

  #[test]
  fn keeps_its_kind() {
    let details = UnrecognizedDetails::<FormatDetails>::sample("matroska");
    assert_eq!(details.kind(), "matroska");
    assert_eq!(details.clone(), details);
  }
}
