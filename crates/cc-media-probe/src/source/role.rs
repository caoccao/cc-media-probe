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

//! `SourceRole`, what a source is to the probe (`api.md` section 6.15).

use super::constants::PRIMARY_NAME;
use crate::string_enumeration::string_enumeration;

string_enumeration! {
  /// What a source is to the probe: the primary input or a companion of a role. The companion roles come with
  /// the release that ships companion lookup (`api.md` section 13).
  ///
  /// ```
  /// use cc_media_probe::source::{SourceEntry, SourceRole};
  ///
  /// fn is_primary(entry: &SourceEntry) -> bool {
  ///     matches!(entry.role(), SourceRole::Primary)
  /// }
  ///
  /// assert_eq!(SourceRole::Primary.as_str(), "primary");
  /// ```
  SourceRole {
    /// The input that the caller passed.
    Primary => PRIMARY_NAME,
  }
}

#[cfg(test)]
mod tests {
  use super::SourceRole;
  use crate::model::UnrecognizedValue;

  #[test]
  fn names_each_role() {
    assert_eq!(SourceRole::Primary.as_str(), "primary");
    assert_eq!(
      SourceRole::Unrecognized(UnrecognizedValue::sample("vob_sub_data")).as_str(),
      "vob_sub_data"
    );
  }
}
