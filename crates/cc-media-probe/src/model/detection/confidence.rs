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

//! `Confidence`, how sure detection is of a format (`api.md` section 8).

use std::fmt;

/// How sure detection is of a format: a number from 0 to 100, which keeps the candidates ordered and leaves room
/// for finer levels later. `Ord` compares by number, and `Display` writes it, as in `100`.
///
/// ```
/// use cc_media_probe::Detection;
///
/// fn is_certain(detection: &Detection) -> bool {
///     detection.confidence().is_some_and(|confidence| confidence.value() == 100)
/// }
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Confidence(u8);

impl Confidence {
  /// The number, from 0 to 100.
  #[must_use]
  pub fn value(&self) -> u8 {
    self.0
  }
}

impl fmt::Display for Confidence {
  /// Writes the number.
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(formatter, "{}", self.0)
  }
}

#[cfg(test)]
mod tests {
  use super::Confidence;

  impl Confidence {
    /// A confidence of this number, or `None` above 100.
    pub(crate) fn new(value: u8) -> Option<Confidence> {
      (value <= 100).then_some(Confidence(value))
    }
  }

  #[test]
  fn holds_a_number_up_to_one_hundred() {
    let certain = Confidence::new(100).unwrap();
    assert_eq!(certain.value(), 100);
    assert_eq!(certain.to_string(), "100");
    assert!(Confidence::new(40).unwrap() < certain);
    assert_eq!(Confidence::new(101), None);
  }
}
