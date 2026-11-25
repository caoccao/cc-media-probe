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

//! `Rotation`, the clockwise angle that a player applies (`api.md` section 6.5; F7).

use super::constants::{
  CLOCKWISE_90_DEGREES, CLOCKWISE_90_NAME, CLOCKWISE_180_DEGREES, CLOCKWISE_180_NAME, CLOCKWISE_270_DEGREES,
  CLOCKWISE_270_NAME, UPRIGHT_NAME,
};
use crate::string_enumeration::string_enumeration;

string_enumeration! {
  /// The clockwise angle that a player applies to a video's pictures (F7), a multiple of 90 degrees. It is
  /// reported, never applied.
  ///
  /// ```
  /// use cc_media_probe::model::{Rotation, VideoInformation};
  ///
  /// fn is_portrait(video: &VideoInformation) -> Option<bool> {
  ///     let size = video.display_size()?;
  ///     let turned = matches!(video.rotation(), Some(Rotation::Clockwise90 | Rotation::Clockwise270));
  ///     Some((size.height() > size.width()) != turned)
  /// }
  ///
  /// assert_eq!(Rotation::Clockwise90.degrees(), Some(90));
  /// assert_eq!(Rotation::Clockwise270.as_str(), "clockwise_270");
  /// ```
  Rotation {
    /// No rotation.
    Upright => UPRIGHT_NAME,
    /// A quarter turn clockwise.
    Clockwise90 => CLOCKWISE_90_NAME,
    /// A half turn.
    Clockwise180 => CLOCKWISE_180_NAME,
    /// Three quarter turns clockwise.
    Clockwise270 => CLOCKWISE_270_NAME,
  }
}

impl Rotation {
  /// The clockwise angle in degrees, or `None` for a rotation that a newer version wrote.
  ///
  /// ```
  /// use cc_media_probe::model::Rotation;
  ///
  /// assert_eq!(Rotation::Upright.degrees(), Some(0));
  /// ```
  #[must_use]
  pub fn degrees(&self) -> Option<u16> {
    match self {
      Rotation::Upright => Some(0),
      Rotation::Clockwise90 => Some(CLOCKWISE_90_DEGREES),
      Rotation::Clockwise180 => Some(CLOCKWISE_180_DEGREES),
      Rotation::Clockwise270 => Some(CLOCKWISE_270_DEGREES),
      Rotation::Unrecognized(_) => None,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::Rotation;
  use crate::model::UnrecognizedValue;

  #[test]
  fn names_each_rotation_and_its_angle() {
    for (rotation, name, degrees) in [
      (Rotation::Upright, "upright", Some(0)),
      (Rotation::Clockwise90, "clockwise_90", Some(90)),
      (Rotation::Clockwise180, "clockwise_180", Some(180)),
      (Rotation::Clockwise270, "clockwise_270", Some(270)),
      (
        Rotation::Unrecognized(UnrecognizedValue::sample("clockwise_45")),
        "clockwise_45",
        None,
      ),
    ] {
      assert_eq!((rotation.as_str(), rotation.degrees()), (name, degrees));
    }
  }
}
