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

//! Integers within 2^53 in magnitude as the `f64` that holds each of them exactly.
//!
//! The two halves of a 64-bit integer each convert to `f64` without loss, so their weighted sum is exact whenever
//! the integer itself fits the 53 bits of an `f64` significand, as every part of a `Rational` does (R24).

use super::constants::HALF_WORD_SCALE;

/// The `f64` of a signed integer: exact when its magnitude is at most 2^53, and otherwise the sum of its two halves,
/// rounded.
pub(crate) fn signed_to_float(value: i64) -> f64 {
  let [first, second, third, fourth, fifth, sixth, seventh, eighth] = value.to_le_bytes();
  let low = u32::from_le_bytes([first, second, third, fourth]);
  let high = i32::from_le_bytes([fifth, sixth, seventh, eighth]);
  f64::from(high) * HALF_WORD_SCALE + f64::from(low)
}

/// The `f64` of an unsigned integer: exact when it is at most 2^53. A value beyond `i64::MAX` gives the `f64` of
/// `i64::MAX`; no caller passes one.
pub(crate) fn unsigned_to_float(value: u64) -> f64 {
  signed_to_float(i64::try_from(value).unwrap_or(i64::MAX))
}

#[cfg(test)]
mod tests {
  use super::{signed_to_float, unsigned_to_float};

  #[test]
  fn converts_integers_within_two_to_the_fifty_third_exactly() {
    assert_eq!(signed_to_float(0), 0.0);
    assert_eq!(signed_to_float(-1), -1.0);
    assert_eq!(signed_to_float(30_000), 30_000.0);
    assert_eq!(signed_to_float(9_007_199_254_740_991), 9_007_199_254_740_991.0);
    assert_eq!(signed_to_float(-9_007_199_254_740_991), -9_007_199_254_740_991.0);
    assert_eq!(signed_to_float(-4_294_967_297), -4_294_967_297.0);
    assert_eq!(unsigned_to_float(1_001), 1_001.0);
    assert_eq!(unsigned_to_float(9_007_199_254_740_991), 9_007_199_254_740_991.0);
  }

  #[test]
  fn clamps_an_unsigned_value_beyond_the_signed_range() {
    assert_eq!(unsigned_to_float(u64::MAX), signed_to_float(i64::MAX));
  }
}
