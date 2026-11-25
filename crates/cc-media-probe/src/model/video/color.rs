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

//! `ColorDescription` and `ColorRange`, a video's color description (`api.md` section 6.5).

use super::constants::{FULL_NAME, LIMITED_NAME};
use crate::model::Provenance;
use crate::string_enumeration::string_enumeration;

/// A video's color description: ITU-T H.273 code points and the range, each with its own provenance, since an
/// override supplies only the components it states (field guide section 2).
///
/// ```
/// use cc_media_probe::model::{ColorDescription, ColorRange};
///
/// fn describe(color: &ColorDescription) -> String {
///     format!(
///         "primaries {:?} ({:?}), transfer {:?} ({:?}), matrix {:?} ({:?}), {} range ({:?})",
///         color.primaries(),
///         color.primaries_provenance(),
///         color.transfer_characteristics(),
///         color.transfer_characteristics_provenance(),
///         color.matrix_coefficients(),
///         color.matrix_coefficients_provenance(),
///         match color.range() {
///             Some(ColorRange::Full) => "full",
///             Some(ColorRange::Limited) => "limited",
///             _ => "unknown",
///         },
///         color.range_provenance(),
///     )
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ColorDescription {
  primaries: Option<u8>,
  primaries_provenance: Option<Provenance>,
  transfer_characteristics: Option<u8>,
  transfer_characteristics_provenance: Option<Provenance>,
  matrix_coefficients: Option<u8>,
  matrix_coefficients_provenance: Option<Provenance>,
  range: Option<ColorRange>,
  range_provenance: Option<Provenance>,
}

impl ColorDescription {
  /// The color primaries, an ITU-T H.273 code point.
  #[must_use]
  pub fn primaries(&self) -> Option<u8> {
    self.primaries
  }

  /// Where the primaries came from.
  #[must_use]
  pub fn primaries_provenance(&self) -> Option<&Provenance> {
    self.primaries_provenance.as_ref()
  }

  /// The transfer characteristics, an ITU-T H.273 code point.
  #[must_use]
  pub fn transfer_characteristics(&self) -> Option<u8> {
    self.transfer_characteristics
  }

  /// Where the transfer characteristics came from.
  #[must_use]
  pub fn transfer_characteristics_provenance(&self) -> Option<&Provenance> {
    self.transfer_characteristics_provenance.as_ref()
  }

  /// The matrix coefficients, an ITU-T H.273 code point.
  #[must_use]
  pub fn matrix_coefficients(&self) -> Option<u8> {
    self.matrix_coefficients
  }

  /// Where the matrix coefficients came from.
  #[must_use]
  pub fn matrix_coefficients_provenance(&self) -> Option<&Provenance> {
    self.matrix_coefficients_provenance.as_ref()
  }

  /// The range of the sample values.
  #[must_use]
  pub fn range(&self) -> Option<&ColorRange> {
    self.range.as_ref()
  }

  /// Where the range came from.
  #[must_use]
  pub fn range_provenance(&self) -> Option<&Provenance> {
    self.range_provenance.as_ref()
  }
}

string_enumeration! {
  /// The range of a video's sample values. `ColorDescription` shows an example.
  ///
  /// ```
  /// use cc_media_probe::model::ColorRange;
  ///
  /// assert_eq!(ColorRange::Limited.as_str(), "limited");
  /// ```
  ColorRange {
    /// The broadcast range, as H.273's `video_full_range_flag` 0.
    Limited => LIMITED_NAME,
    /// The full range, as H.273's `video_full_range_flag` 1.
    Full => FULL_NAME,
  }
}

#[cfg(test)]
mod tests {
  use super::{ColorDescription, ColorRange};
  use crate::model::Provenance;

  impl ColorDescription {
    /// The description of BT.709 in the limited range, declared by the container except for the range.
    pub(crate) fn sample() -> ColorDescription {
      ColorDescription {
        primaries: Some(1),
        primaries_provenance: Some(Provenance::Declared),
        transfer_characteristics: Some(1),
        transfer_characteristics_provenance: Some(Provenance::Declared),
        matrix_coefficients: Some(1),
        matrix_coefficients_provenance: Some(Provenance::Declared),
        range: Some(ColorRange::Limited),
        range_provenance: Some(Provenance::Bitstream),
      }
    }
  }

  #[test]
  fn keeps_each_component_with_its_provenance() {
    let color = ColorDescription::sample();
    assert_eq!(color.primaries(), Some(1));
    assert_eq!(color.primaries_provenance(), Some(&Provenance::Declared));
    assert_eq!(color.transfer_characteristics(), Some(1));
    assert_eq!(color.transfer_characteristics_provenance(), Some(&Provenance::Declared));
    assert_eq!(color.matrix_coefficients(), Some(1));
    assert_eq!(color.matrix_coefficients_provenance(), Some(&Provenance::Declared));
    assert_eq!(color.range(), Some(&ColorRange::Limited));
    assert_eq!(color.range_provenance(), Some(&Provenance::Bitstream));
    assert_eq!(ColorRange::Full.as_str(), "full");
  }
}
