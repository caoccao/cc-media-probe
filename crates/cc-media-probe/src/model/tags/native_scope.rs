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

//! `NativeScope`, the ISO family's scope of a tag set (`api.md` section 6.9).

use crate::form_record::form_record;

form_record! {
  /// The ISO family's scope of a tag set: file, movie, track or media, with its owner and its `meta` or `udta`
  /// source. Phase 6 fixes its members, which a later release adds as accessors (`json-output.md` section 7.8).
  ///
  /// ```
  /// use cc_media_probe::model::TagSet;
  ///
  /// fn has_native_scope(set: &TagSet) -> bool {
  ///     set.native_scope().is_some()
  /// }
  /// ```
  #[derive(Clone, Debug, PartialEq, Eq, Hash)]
  #[non_exhaustive]
  pub struct NativeScope {}
}

#[cfg(test)]
mod tests {
  use super::NativeScope;

  #[test]
  fn has_no_members_before_phase_6() {
    assert_eq!(format!("{:?}", NativeScope {}), "NativeScope");
  }
}
