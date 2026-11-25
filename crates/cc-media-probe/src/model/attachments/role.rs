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

//! `AttachmentRole`, the role of an attachment (`api.md` section 6.10; F8).

use super::constants::{COVER_ART_NAME, OTHER_NAME};
use crate::string_enumeration::string_enumeration;

string_enumeration! {
  /// The role of an attachment (F8), which decides which of the two attachment options applies to it.
  ///
  /// ```
  /// use cc_media_probe::model::{Attachment, AttachmentRole};
  ///
  /// fn is_cover(attachment: &Attachment) -> bool {
  ///     matches!(attachment.role(), Some(AttachmentRole::CoverArt))
  /// }
  ///
  /// assert_eq!(AttachmentRole::CoverArt.as_str(), "cover_art");
  /// ```
  AttachmentRole {
    /// Cover art, whatever its carriage.
    CoverArt => COVER_ART_NAME,
    /// Any other attachment, such as a font.
    Other => OTHER_NAME,
  }
}

#[cfg(test)]
mod tests {
  use super::AttachmentRole;
  use crate::model::UnrecognizedValue;

  #[test]
  fn names_each_role() {
    assert_eq!(AttachmentRole::CoverArt.as_str(), "cover_art");
    assert_eq!(AttachmentRole::Other.as_str(), "other");
    assert_eq!(
      AttachmentRole::Unrecognized(UnrecognizedValue::sample("lyrics")).as_str(),
      "lyrics"
    );
  }
}
