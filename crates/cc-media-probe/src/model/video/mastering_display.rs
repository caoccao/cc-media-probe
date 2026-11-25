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

//! `MasteringDisplay` and `Chromaticity`, the display a video was mastered on (`api.md` section 6.5).

use crate::Rational;
use crate::model::Provenance;

/// The display that a video was mastered on: its primaries and white point, and its luminance. Each value has its
/// own provenance, since the container and the codec header can each supply some of them (`api.md` section 6.5).
///
/// ```
/// use cc_media_probe::model::{Chromaticity, MasteringDisplay};
///
/// fn point(chromaticity: Option<&Chromaticity>) -> String {
///     chromaticity.map_or_else(String::new, |point| {
///         format!(
///             "({:?} {:?}, {:?} {:?})",
///             point.x_coordinate(),
///             point.x_coordinate_provenance(),
///             point.y_coordinate(),
///             point.y_coordinate_provenance(),
///         )
///     })
/// }
///
/// fn describe(display: &MasteringDisplay) -> String {
///     format!(
///         "red {} green {} blue {} white {}; {:?} to {:?} cd/m2 ({:?}, {:?})",
///         point(display.red()),
///         point(display.green()),
///         point(display.blue()),
///         point(display.white_point()),
///         display.minimum_luminance(),
///         display.maximum_luminance(),
///         display.minimum_luminance_provenance(),
///         display.maximum_luminance_provenance(),
///     )
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MasteringDisplay {
  red: Option<Chromaticity>,
  green: Option<Chromaticity>,
  blue: Option<Chromaticity>,
  white_point: Option<Chromaticity>,
  maximum_luminance: Option<Rational>,
  maximum_luminance_provenance: Option<Provenance>,
  minimum_luminance: Option<Rational>,
  minimum_luminance_provenance: Option<Provenance>,
}

impl MasteringDisplay {
  /// The red primary, absent when the file states neither coordinate.
  #[must_use]
  pub fn red(&self) -> Option<&Chromaticity> {
    self.red.as_ref()
  }

  /// The green primary, absent when the file states neither coordinate.
  #[must_use]
  pub fn green(&self) -> Option<&Chromaticity> {
    self.green.as_ref()
  }

  /// The blue primary, absent when the file states neither coordinate.
  #[must_use]
  pub fn blue(&self) -> Option<&Chromaticity> {
    self.blue.as_ref()
  }

  /// The white point, absent when the file states neither coordinate.
  #[must_use]
  pub fn white_point(&self) -> Option<&Chromaticity> {
    self.white_point.as_ref()
  }

  /// The maximum luminance, in candelas per square meter.
  #[must_use]
  pub fn maximum_luminance(&self) -> Option<Rational> {
    self.maximum_luminance
  }

  /// Where the maximum luminance came from.
  #[must_use]
  pub fn maximum_luminance_provenance(&self) -> Option<&Provenance> {
    self.maximum_luminance_provenance.as_ref()
  }

  /// The minimum luminance, in candelas per square meter.
  #[must_use]
  pub fn minimum_luminance(&self) -> Option<Rational> {
    self.minimum_luminance
  }

  /// Where the minimum luminance came from.
  #[must_use]
  pub fn minimum_luminance_provenance(&self) -> Option<&Provenance> {
    self.minimum_luminance_provenance.as_ref()
  }
}

/// A point of the CIE 1931 chromaticity diagram, each coordinate optional with its own provenance. The coordinates
/// are spelled out, since R32 allows no name of a single letter. `MasteringDisplay` shows an example.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Chromaticity {
  x_coordinate: Option<Rational>,
  x_coordinate_provenance: Option<Provenance>,
  y_coordinate: Option<Rational>,
  y_coordinate_provenance: Option<Provenance>,
}

impl Chromaticity {
  /// The CIE 1931 coordinate x.
  #[must_use]
  pub fn x_coordinate(&self) -> Option<Rational> {
    self.x_coordinate
  }

  /// Where the coordinate x came from.
  #[must_use]
  pub fn x_coordinate_provenance(&self) -> Option<&Provenance> {
    self.x_coordinate_provenance.as_ref()
  }

  /// The CIE 1931 coordinate y.
  #[must_use]
  pub fn y_coordinate(&self) -> Option<Rational> {
    self.y_coordinate
  }

  /// Where the coordinate y came from.
  #[must_use]
  pub fn y_coordinate_provenance(&self) -> Option<&Provenance> {
    self.y_coordinate_provenance.as_ref()
  }
}

#[cfg(test)]
mod tests {
  use super::{Chromaticity, MasteringDisplay};
  use crate::Rational;
  use crate::model::Provenance;

  fn point(x_coordinate: (i64, u64), y_coordinate: (i64, u64)) -> Chromaticity {
    Chromaticity {
      x_coordinate: Rational::new(x_coordinate.0, x_coordinate.1),
      x_coordinate_provenance: Some(Provenance::Declared),
      y_coordinate: Rational::new(y_coordinate.0, y_coordinate.1),
      y_coordinate_provenance: Some(Provenance::Bitstream),
    }
  }

  impl MasteringDisplay {
    /// The primaries and the white point of BT.2020 at up to 1000 candelas per square meter.
    pub(crate) fn sample() -> MasteringDisplay {
      MasteringDisplay {
        red: Some(point((708, 1_000), (292, 1_000))),
        green: Some(point((170, 1_000), (797, 1_000))),
        blue: Some(point((131, 1_000), (46, 1_000))),
        white_point: Some(point((3_127, 10_000), (329, 1_000))),
        maximum_luminance: Rational::new(1_000, 1),
        maximum_luminance_provenance: Some(Provenance::Declared),
        minimum_luminance: Rational::new(1, 200),
        minimum_luminance_provenance: Some(Provenance::Bitstream),
      }
    }
  }

  #[test]
  fn keeps_each_value_with_its_provenance() {
    let display = MasteringDisplay::sample();
    let red = display.red().unwrap();
    assert_eq!(red.x_coordinate(), Rational::new(708, 1_000));
    assert_eq!(red.x_coordinate_provenance(), Some(&Provenance::Declared));
    assert_eq!(red.y_coordinate(), Rational::new(292, 1_000));
    assert_eq!(red.y_coordinate_provenance(), Some(&Provenance::Bitstream));
    assert!(display.green().is_some());
    assert!(display.blue().is_some());
    assert_eq!(
      display.white_point().and_then(Chromaticity::x_coordinate),
      Rational::new(3_127, 10_000)
    );
    assert_eq!(display.maximum_luminance(), Rational::new(1_000, 1));
    assert_eq!(display.maximum_luminance_provenance(), Some(&Provenance::Declared));
    assert_eq!(display.minimum_luminance(), Rational::new(1, 200));
    assert_eq!(display.minimum_luminance_provenance(), Some(&Provenance::Bitstream));
  }
}
