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

//! `IntegerValue`, any 64-bit signed or unsigned integer of a tag (`api.md` section 6.9).

use std::fmt;

/// Any 64-bit signed or unsigned integer, from -2^63 to 2^64 - 1, as a tag value holds it. It has no serialized
/// form of its own: the tag value that holds it writes it (`api.md` section 3.3).
///
/// ```
/// use cc_media_probe::model::{IntegerValue, TagValue};
///
/// fn as_signed(value: &TagValue) -> Option<i64> {
///     match value {
///         TagValue::Integer(integer) => integer.to_i64(),
///         _ => None,
///     }
/// }
///
/// fn describe(integer: &IntegerValue) -> String {
///     format!("{integer} ({:?} as unsigned)", integer.to_u64())
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct IntegerValue(i128);

impl IntegerValue {
  /// The value as an `i64`, or `None` beyond `i64::MAX`.
  #[must_use]
  pub fn to_i64(&self) -> Option<i64> {
    i64::try_from(self.0).ok()
  }

  /// The value as a `u64`, or `None` below zero.
  #[must_use]
  pub fn to_u64(&self) -> Option<u64> {
    u64::try_from(self.0).ok()
  }
}

impl fmt::Display for IntegerValue {
  /// Writes the value in decimal.
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(formatter, "{}", self.0)
  }
}

#[cfg(test)]
mod tests {
  use super::IntegerValue;

  impl IntegerValue {
    /// The integer `value`, signed.
    pub(crate) fn signed(value: i64) -> IntegerValue {
      IntegerValue(i128::from(value))
    }

    /// The integer `value`, unsigned.
    pub(crate) fn unsigned(value: u64) -> IntegerValue {
      IntegerValue(i128::from(value))
    }
  }

  #[test]
  fn converts_within_each_width() {
    let negative = IntegerValue::signed(-5);
    assert_eq!((negative.to_i64(), negative.to_u64()), (Some(-5), None));
    assert_eq!(negative.to_string(), "-5");
    let large = IntegerValue::unsigned(u64::MAX);
    assert_eq!((large.to_i64(), large.to_u64()), (None, Some(u64::MAX)));
    assert_eq!(large.to_string(), "18446744073709551615");
    assert_eq!(IntegerValue::signed(7), IntegerValue::unsigned(7));
    assert_eq!(IntegerValue::signed(i64::MIN).to_i64(), Some(i64::MIN));
  }
}
