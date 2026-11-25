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

//! `SupportLevel`, whether this version probes a format or only detects it (`api.md` section 6.4; plan section
//! 8.6).

use super::constants::{DETECT_ONLY_NAME, PROBED_NAME};
use crate::string_enumeration::string_enumeration;

string_enumeration! {
  /// Whether this version probes a format or only detects it, as `Format::support()` computes it. It is a model
  /// type, serialized like every other (`api.md` section 3.3), so a value that a newer version wrote stays in its
  /// catch-all; `Format::support()` never gives one.
  ///
  /// ```
  /// use cc_media_probe::Format;
  /// use cc_media_probe::model::SupportLevel;
  ///
  /// fn is_probed(format: &Format) -> bool {
  ///     matches!(format.support(), Some(SupportLevel::Probed))
  /// }
  ///
  /// assert!(is_probed(&Format::Matroska));
  /// assert_eq!(Format::Riff.support().as_ref().map(SupportLevel::as_str), Some("detect_only"));
  /// ```
  SupportLevel {
    /// The library walks the format and reports its streams.
    Probed => PROBED_NAME,
    /// The library recognizes the format, and a probe ends with the error of kind `UnsupportedFormat`.
    DetectOnly => DETECT_ONLY_NAME,
  }
}

#[cfg(test)]
mod tests {
  use super::SupportLevel;
  use crate::model::UnrecognizedValue;

  #[test]
  fn names_each_level() {
    assert_eq!(SupportLevel::Probed.as_str(), "probed");
    assert_eq!(SupportLevel::DetectOnly.as_str(), "detect_only");
    assert_eq!(
      SupportLevel::Unrecognized(UnrecognizedValue::sample("planned")).as_str(),
      "planned"
    );
  }
}
