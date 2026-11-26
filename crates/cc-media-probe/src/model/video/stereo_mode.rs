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

//! `StereoMode` and `StereoLayout`, how a single-track stereo video packs its views (`api.md` section 6.5).

use crate::form_record::form_record;
use crate::string_enumeration::string_enumeration;

form_record! {
  /// How a single-track stereo video packs its views: the layout, and the code as the file writes it.
  ///
  /// ```
  /// use cc_media_probe::model::StereoMode;
  ///
  /// fn describe(mode: &StereoMode) -> String {
  ///     format!("{} (code {})", mode.layout().as_str(), mode.code())
  /// }
  /// ```
  #[derive(Clone, Debug, PartialEq, Eq, Hash)]
  pub struct StereoMode {
    layout: StereoLayout,
    code: String,
  }
}

impl StereoMode {
  /// The layout of the views.
  #[must_use]
  pub fn layout(&self) -> &StereoLayout {
    &self.layout
  }

  /// The code as the file writes it, such as Matroska's `StereoMode` value.
  #[must_use]
  pub fn code(&self) -> &str {
    &self.code
  }
}

string_enumeration! {
  /// The layout of a stereo video's views, one variant per value of the Matroska stereo table. Phase 5 adds the
  /// values; until then a layout is the catch-all. `StereoMode` shows an example.
  StereoLayout {}
}

#[cfg(test)]
mod tests {
  use super::{StereoLayout, StereoMode};
  use crate::model::UnrecognizedValue;

  impl StereoMode {
    /// A mode of this layout and code.
    pub(crate) fn sample(layout: StereoLayout, code: &str) -> StereoMode {
      StereoMode {
        layout,
        code: code.to_owned(),
      }
    }
  }

  #[test]
  fn keeps_the_layout_and_the_code() {
    let mode = StereoMode::sample(
      StereoLayout::Unrecognized(UnrecognizedValue::sample("side_by_side_left_first")),
      "1",
    );
    assert_eq!(mode.layout().as_str(), "side_by_side_left_first");
    assert_eq!(mode.code(), "1");
  }
}
