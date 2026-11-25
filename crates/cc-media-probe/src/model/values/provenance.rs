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

//! `Provenance`, where a value came from (`api.md` section 6.1; plan section 8.4).

use super::constants::{BITSTREAM_NAME, DECLARED_NAME, DERIVED_NAME};
use crate::string_enumeration::string_enumeration;

string_enumeration! {
  /// Where a value came from (plan section 8.4). A provenance is present exactly when its value is.
  ///
  /// ```
  /// use cc_media_probe::Stream;
  /// use cc_media_probe::model::Provenance;
  ///
  /// fn duration_is_stated(stream: &Stream) -> bool {
  ///     matches!(stream.duration_provenance(), Some(Provenance::Declared))
  /// }
  ///
  /// assert_eq!(Provenance::Bitstream.as_str(), "bitstream");
  /// ```
  Provenance {
    /// Stated by the container, including a default that a specification states for an absent element (F1).
    Declared => DECLARED_NAME,
    /// Read from a codec header.
    Bitstream => BITSTREAM_NAME,
    /// Computed from other values.
    Derived => DERIVED_NAME,
  }
}

#[cfg(test)]
mod tests {
  use super::Provenance;
  use crate::model::UnrecognizedValue;

  #[test]
  fn names_each_value() {
    assert_eq!(Provenance::Declared.as_str(), "declared");
    assert_eq!(Provenance::Bitstream.as_str(), "bitstream");
    assert_eq!(Provenance::Derived.as_str(), "derived");
    assert_eq!(
      Provenance::Unrecognized(UnrecognizedValue::sample("inferred")).as_str(),
      "inferred"
    );
  }
}
