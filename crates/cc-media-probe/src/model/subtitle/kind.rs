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

//! `SubtitleKind`, whether a subtitle stream holds text or pictures (`api.md` section 6.7).

use super::constants::{BITMAP_NAME, TEXT_NAME};
use crate::string_enumeration::string_enumeration;

string_enumeration! {
  /// Whether a subtitle stream holds text or pictures.
  ///
  /// ```
  /// use cc_media_probe::Stream;
  /// use cc_media_probe::model::SubtitleKind;
  ///
  /// fn is_text(stream: &Stream) -> bool {
  ///     stream.subtitle().is_some_and(|subtitle| matches!(subtitle.subtitle_kind(), Some(SubtitleKind::Text)))
  /// }
  ///
  /// assert_eq!(SubtitleKind::Bitmap.as_str(), "bitmap");
  /// ```
  SubtitleKind {
    /// Text events.
    Text => TEXT_NAME,
    /// Picture events.
    Bitmap => BITMAP_NAME,
  }
}

#[cfg(test)]
mod tests {
  use super::SubtitleKind;
  use crate::model::UnrecognizedValue;

  #[test]
  fn names_each_kind() {
    assert_eq!(SubtitleKind::Text.as_str(), "text");
    assert_eq!(SubtitleKind::Bitmap.as_str(), "bitmap");
    assert_eq!(
      SubtitleKind::Unrecognized(UnrecognizedValue::sample("vector")).as_str(),
      "vector"
    );
  }
}
