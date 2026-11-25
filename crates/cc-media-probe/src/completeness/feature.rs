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

//! `Feature`, a Cargo feature that gates parsing (`api.md` section 5.5).

use super::constants::ZLIB_NAME;
use crate::string_enumeration::string_enumeration;

string_enumeration! {
  /// A Cargo feature that gates parsing, named by a reason of the kind `FeatureDisabled`. In `0.1.0` that is only
  /// `Zlib`: the serialization and async features never make information unavailable. A later feature that gates
  /// parsing adds its variant.
  ///
  /// ```
  /// use cc_media_probe::completeness::{Feature, Reason};
  ///
  /// fn needs_zlib(reason: &Reason) -> bool {
  ///     matches!(reason.feature(), Some(Feature::Zlib))
  /// }
  ///
  /// assert_eq!(Feature::Zlib.as_str(), "zlib");
  /// ```
  Feature {
    /// Inflating zlib data: Matroska content compression and the compressed movie box of the ISO family.
    Zlib => ZLIB_NAME,
  }
}

#[cfg(test)]
mod tests {
  use super::Feature;
  use crate::model::UnrecognizedValue;

  #[test]
  fn names_each_feature() {
    assert_eq!(Feature::Zlib.as_str(), "zlib");
    assert_eq!(
      Feature::Unrecognized(UnrecognizedValue::sample("brotli")).as_str(),
      "brotli"
    );
  }
}
