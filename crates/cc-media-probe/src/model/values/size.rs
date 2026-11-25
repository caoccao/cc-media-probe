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

//! `Size`, a width and a height in whole pixels (`api.md` section 6.1).

use std::fmt;

use super::constants::SIZE_SEPARATOR;
use crate::constants::LARGEST_EXACT;

/// A width and a height in whole pixels, each at most 2^53 - 1, as every normalized dimension is (R24).
///
/// ```
/// use cc_media_probe::model::Size;
///
/// # fn main() -> Result<(), &'static str> {
/// let size = Size::new(1_920, 1_080).ok_or("out of range")?;
/// assert_eq!((size.width(), size.height()), (1_920, 1_080));
/// assert_eq!(size.to_string(), "1920x1080");
/// assert_eq!(Size::new(1 << 53, 1), None);
/// # Ok(())
/// # }
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Size {
  width: u64,
  height: u64,
}

impl Size {
  /// A size, or `None` when the width or the height is beyond 2^53 - 1, which no normalized field holds.
  ///
  /// ```
  /// use cc_media_probe::model::Size;
  ///
  /// assert!(Size::new(3_840, 2_160).is_some());
  /// ```
  #[must_use]
  pub fn new(width: u64, height: u64) -> Option<Size> {
    (width <= LARGEST_EXACT && height <= LARGEST_EXACT).then_some(Size { width, height })
  }

  /// The width, in pixels.
  ///
  /// ```
  /// use cc_media_probe::model::Size;
  ///
  /// assert_eq!(Size::new(720, 576).map(|size| size.width()), Some(720));
  /// ```
  #[must_use]
  pub fn width(&self) -> u64 {
    self.width
  }

  /// The height, in pixels.
  ///
  /// ```
  /// use cc_media_probe::model::Size;
  ///
  /// assert_eq!(Size::new(720, 576).map(|size| size.height()), Some(576));
  /// ```
  #[must_use]
  pub fn height(&self) -> u64 {
    self.height
  }
}

impl fmt::Display for Size {
  /// Writes `1920x1080`.
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(formatter, "{}{SIZE_SEPARATOR}{}", self.width, self.height)
  }
}

#[cfg(test)]
mod tests {
  use super::Size;

  #[test]
  fn holds_whole_pixels_within_the_exact_range() {
    let size = Size::new(1_920, 1_080).unwrap();
    assert_eq!((size.width(), size.height()), (1_920, 1_080));
    assert_eq!(size.to_string(), "1920x1080");
    assert!(Size::new(9_007_199_254_740_991, 9_007_199_254_740_991).is_some());
    assert_eq!(Size::new(9_007_199_254_740_992, 1), None);
    assert_eq!(Size::new(1, 9_007_199_254_740_992), None);
  }
}
