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

//! `ScanType` and `FieldOrder`, how a video's pictures are scanned (`api.md` section 6.5).

use super::constants::{BOTTOM_FIELD_FIRST_NAME, INTERLACED_NAME, PROGRESSIVE_NAME, TOP_FIELD_FIRST_NAME};
use crate::string_enumeration::string_enumeration;

string_enumeration! {
  /// How a video's pictures are scanned.
  ///
  /// ```
  /// use cc_media_probe::model::{FieldOrder, ScanType, VideoInformation};
  ///
  /// fn scan(video: &VideoInformation) -> &str {
  ///     match (video.scan_type(), video.field_order()) {
  ///         (Some(ScanType::Interlaced), Some(FieldOrder::TopFieldFirst)) => "interlaced, top field first",
  ///         (Some(scan), _) => scan.as_str(),
  ///         (None, _) => "unknown",
  ///     }
  /// }
  ///
  /// assert_eq!(ScanType::Progressive.as_str(), "progressive");
  /// ```
  ScanType {
    /// Whole frames.
    Progressive => PROGRESSIVE_NAME,
    /// Two fields per frame.
    Interlaced => INTERLACED_NAME,
  }
}

string_enumeration! {
  /// Which field of an interlaced frame comes first. `ScanType` shows an example.
  ///
  /// ```
  /// use cc_media_probe::model::FieldOrder;
  ///
  /// assert_eq!(FieldOrder::BottomFieldFirst.as_str(), "bottom_field_first");
  /// ```
  FieldOrder {
    /// The top field first.
    TopFieldFirst => TOP_FIELD_FIRST_NAME,
    /// The bottom field first.
    BottomFieldFirst => BOTTOM_FIELD_FIRST_NAME,
  }
}

#[cfg(test)]
mod tests {
  use super::{FieldOrder, ScanType};
  use crate::model::UnrecognizedValue;

  #[test]
  fn names_each_value() {
    assert_eq!(ScanType::Progressive.as_str(), "progressive");
    assert_eq!(ScanType::Interlaced.as_str(), "interlaced");
    assert_eq!(
      ScanType::Unrecognized(UnrecognizedValue::sample("mixed")).as_str(),
      "mixed"
    );
    assert_eq!(FieldOrder::TopFieldFirst.as_str(), "top_field_first");
    assert_eq!(FieldOrder::BottomFieldFirst.as_str(), "bottom_field_first");
  }
}
