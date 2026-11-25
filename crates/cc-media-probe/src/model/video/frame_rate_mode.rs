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

//! `FrameRateMode`, whether a video's frame rate is constant (`api.md` section 6.5).

use super::constants::{CONSTANT_NAME, UNKNOWN_NAME, VARIABLE_NAME};
use crate::string_enumeration::string_enumeration;

string_enumeration! {
  /// Whether a video's frame rate is constant.
  ///
  /// ```
  /// use cc_media_probe::model::{FrameRateMode, VideoInformation};
  ///
  /// fn frame_rate(video: &VideoInformation) -> String {
  ///     match (video.frame_rate(), video.frame_rate_mode()) {
  ///         (Some(rate), _) => format!("{rate} fps"),
  ///         (None, FrameRateMode::Variable) => "variable frame rate".to_owned(),
  ///         (None, _) => "unknown".to_owned(),
  ///     }
  /// }
  ///
  /// assert_eq!(FrameRateMode::Variable.as_str(), "variable");
  /// ```
  FrameRateMode {
    /// Every frame lasts the same time.
    Constant => CONSTANT_NAME,
    /// The frames' durations differ.
    Variable => VARIABLE_NAME,
    /// Not established, as when the container states a default duration that blocks may override.
    Unknown => UNKNOWN_NAME,
  }
}

#[cfg(test)]
mod tests {
  use super::FrameRateMode;
  use crate::model::UnrecognizedValue;

  #[test]
  fn names_each_mode() {
    for (mode, name) in [
      (FrameRateMode::Constant, "constant"),
      (FrameRateMode::Variable, "variable"),
      (FrameRateMode::Unknown, "unknown"),
      (
        FrameRateMode::Unrecognized(UnrecognizedValue::sample("stepped")),
        "stepped",
      ),
    ] {
      assert_eq!(mode.as_str(), name);
    }
  }
}
