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

//! `DynamicRange`, `DynamicRangeTransfer` and `DynamicRangeMark`, a video's dynamic range (`api.md` section 6.5;
//! F6).

use super::constants::{DOLBY_VISION_NAME, HYBRID_LOG_GAMMA_NAME, PERCEPTUAL_QUANTIZER_NAME, STANDARD_NAME};
use crate::string_enumeration::string_enumeration;

/// A video's dynamic range (F6): its transfer, and the marks of the formats that signal one, whose profile and
/// level stay in the details.
///
/// ```
/// use cc_media_probe::model::{DynamicRange, DynamicRangeMark, DynamicRangeTransfer};
///
/// fn is_high_dynamic_range(range: &DynamicRange) -> bool {
///     matches!(
///         range.transfer(),
///         Some(DynamicRangeTransfer::PerceptualQuantizer | DynamicRangeTransfer::HybridLogGamma)
///     ) || range.marks().iter().any(|mark| matches!(mark, DynamicRangeMark::DolbyVision))
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct DynamicRange {
  transfer: Option<DynamicRangeTransfer>,
  marks: Vec<DynamicRangeMark>,
}

impl DynamicRange {
  /// The transfer of the video's signal.
  #[must_use]
  pub fn transfer(&self) -> Option<&DynamicRangeTransfer> {
    self.transfer.as_ref()
  }

  /// The marks of the formats that signal a dynamic range, in stored order.
  #[must_use]
  pub fn marks(&self) -> &[DynamicRangeMark] {
    &self.marks
  }
}

string_enumeration! {
  /// The transfer of a video's signal. `DynamicRange` shows an example.
  ///
  /// ```
  /// use cc_media_probe::model::DynamicRangeTransfer;
  ///
  /// assert_eq!(DynamicRangeTransfer::HybridLogGamma.as_str(), "hybrid_log_gamma");
  /// ```
  DynamicRangeTransfer {
    /// A standard dynamic range.
    Standard => STANDARD_NAME,
    /// The perceptual quantizer of SMPTE ST 2084.
    PerceptualQuantizer => PERCEPTUAL_QUANTIZER_NAME,
    /// Hybrid log-gamma, of ARIB STD-B67.
    HybridLogGamma => HYBRID_LOG_GAMMA_NAME,
  }
}

string_enumeration! {
  /// A format that signals a dynamic range. `DynamicRange` shows an example.
  ///
  /// ```
  /// use cc_media_probe::model::DynamicRangeMark;
  ///
  /// assert_eq!(DynamicRangeMark::DolbyVision.as_str(), "dolby_vision");
  /// ```
  DynamicRangeMark {
    /// Dolby Vision.
    DolbyVision => DOLBY_VISION_NAME,
  }
}

#[cfg(test)]
mod tests {
  use super::{DynamicRange, DynamicRangeMark, DynamicRangeTransfer};
  use crate::model::UnrecognizedValue;

  impl DynamicRange {
    /// A range of the perceptual quantizer, marked as Dolby Vision.
    pub(crate) fn sample() -> DynamicRange {
      DynamicRange {
        transfer: Some(DynamicRangeTransfer::PerceptualQuantizer),
        marks: vec![DynamicRangeMark::DolbyVision],
      }
    }
  }

  #[test]
  fn keeps_the_transfer_and_the_marks() {
    let range = DynamicRange::sample();
    assert_eq!(range.transfer(), Some(&DynamicRangeTransfer::PerceptualQuantizer));
    assert_eq!(range.marks(), &[DynamicRangeMark::DolbyVision]);
    for (transfer, name) in [
      (DynamicRangeTransfer::Standard, "standard"),
      (DynamicRangeTransfer::PerceptualQuantizer, "perceptual_quantizer"),
      (DynamicRangeTransfer::HybridLogGamma, "hybrid_log_gamma"),
      (
        DynamicRangeTransfer::Unrecognized(UnrecognizedValue::sample("future")),
        "future",
      ),
    ] {
      assert_eq!(transfer.as_str(), name);
    }
    assert_eq!(DynamicRangeMark::DolbyVision.as_str(), "dolby_vision");
  }
}
