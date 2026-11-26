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

//! `Rational`, an exact fraction (`api.md` section 6.1).

use std::cmp::Ordering;
use std::fmt;

use crate::constants::LARGEST_EXACT;
use crate::form_record::form_record;
use crate::utilities::{greatest_common_divisor, signed_to_float, unsigned_to_float};

form_record! {
  /// An exact fraction, the type of every rate, ratio and time of the report.
  ///
  /// A rational is always reduced, its denominator is at least 1, and both parts are at most 2^53 - 1 in magnitude,
  /// so that every JSON reader holds them as exact numbers (R24). Equal values are therefore equal parts, and `Ord`
  /// compares by value.
  ///
  /// ```
  /// use cc_media_probe::Rational;
  ///
  /// # fn main() -> Result<(), &'static str> {
  /// let rate = Rational::new(48_000, 2_002).ok_or("out of range")?;
  /// assert_eq!((rate.numerator(), rate.denominator()), (24_000, 1_001));
  /// assert_eq!(rate.to_string(), "24000/1001");
  /// assert_eq!(rate.to_integer(), None);
  /// assert!((rate.to_f64() - 23.976).abs() < 0.001);
  /// assert!(rate < Rational::new(24, 1).ok_or("out of range")?);
  /// assert_eq!(Rational::new(48_000, 1).ok_or("out of range")?.to_string(), "48000");
  /// assert_eq!(Rational::new(1, 0), None);
  /// # Ok(())
  /// # }
  /// ```
  #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
  pub struct Rational {
    numerator: i64,
    denominator: u64,
  }
  check = Rational::check_form;
}

impl Rational {
  /// `numerator / denominator`, reduced, or `None` for a zero denominator or a part that is still beyond 2^53 - 1
  /// in magnitude once reduced.
  ///
  /// ```
  /// use cc_media_probe::Rational;
  ///
  /// assert_eq!(Rational::new(-6, 4), Rational::new(-3, 2));
  /// assert_eq!(Rational::new(i64::MAX, 1), None);
  /// // A part beyond 2^53 - 1 is reduced first: 18014398509481982 / 2 is 9007199254740991.
  /// assert_eq!(Rational::new(18_014_398_509_481_982, 2).map(|rational| rational.numerator()), Some(9_007_199_254_740_991));
  /// ```
  #[must_use]
  pub fn new(numerator: i64, denominator: u64) -> Option<Rational> {
    if denominator == 0 {
      return None;
    }
    let divisor = greatest_common_divisor(numerator.unsigned_abs(), denominator);
    let magnitude = numerator.unsigned_abs().checked_div(divisor)?;
    let denominator = denominator.checked_div(divisor)?;
    if magnitude > LARGEST_EXACT || denominator > LARGEST_EXACT {
      return None;
    }
    let magnitude = i64::try_from(magnitude).ok()?;
    let numerator = if numerator < 0 {
      magnitude.checked_neg()?
    } else {
      magnitude
    };
    Some(Rational { numerator, denominator })
  }

  /// The numerator, which carries the sign.
  ///
  /// ```
  /// use cc_media_probe::Rational;
  ///
  /// assert_eq!(Rational::new(-3, 6).map(|rational| rational.numerator()), Some(-1));
  /// ```
  #[must_use]
  pub fn numerator(&self) -> i64 {
    self.numerator
  }

  /// The denominator, at least 1.
  ///
  /// ```
  /// use cc_media_probe::Rational;
  ///
  /// assert_eq!(Rational::new(-3, 6).map(|rational| rational.denominator()), Some(2));
  /// ```
  #[must_use]
  pub fn denominator(&self) -> u64 {
    self.denominator
  }

  /// The `f64` nearest to the value. Both parts are exact as `f64`, so the division rounds once.
  ///
  /// ```
  /// use cc_media_probe::Rational;
  ///
  /// assert_eq!(Rational::new(1, 4).map(|rational| rational.to_f64()), Some(0.25));
  /// ```
  #[must_use]
  pub fn to_f64(&self) -> f64 {
    signed_to_float(self.numerator) / unsigned_to_float(self.denominator)
  }

  /// The value as an integer, or `None` when it is not whole.
  ///
  /// ```
  /// use cc_media_probe::Rational;
  ///
  /// assert_eq!(Rational::new(96_000, 2).and_then(|rational| rational.to_integer()), Some(48_000));
  /// assert_eq!(Rational::new(1, 2).and_then(|rational| rational.to_integer()), None);
  /// ```
  #[must_use]
  pub fn to_integer(&self) -> Option<i64> {
    (self.denominator == 1).then_some(self.numerator)
  }

  /// Whether the parts read are the form that `Rational::new` gives: reduced, with a denominator of at least 1 and
  /// both parts within 2^53 - 1 (`json-output.md` section 5.6).
  #[cfg(feature = "serde")]
  fn check_form(&self) -> Result<(), &'static str> {
    if Rational::new(self.numerator, self.denominator) == Some(*self) {
      Ok(())
    } else {
      Err(super::constants::RATIONAL_MESSAGE)
    }
  }
}

impl Ord for Rational {
  /// Compares by value: each part is within 2^53, so the cross products are exact in 128 bits.
  fn cmp(&self, other: &Rational) -> Ordering {
    let left = i128::from(self.numerator).checked_mul(i128::from(other.denominator));
    let right = i128::from(other.numerator).checked_mul(i128::from(self.denominator));
    left.cmp(&right)
  }
}

impl PartialOrd for Rational {
  fn partial_cmp(&self, other: &Rational) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl fmt::Display for Rational {
  /// Writes a whole value as its integer, such as `48000`, and any other as its fraction, such as `30000/1001`.
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self.to_integer() {
      Some(integer) => write!(formatter, "{integer}"),
      None => write!(formatter, "{}/{}", self.numerator, self.denominator),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::Rational;

  fn rational(numerator: i64, denominator: u64) -> Rational {
    Rational::new(numerator, denominator).unwrap()
  }

  #[test]
  fn reduces_and_checks_its_range() {
    assert_eq!(rational(48_000, 2_002), rational(24_000, 1_001));
    assert_eq!(rational(0, 7), rational(0, 1));
    assert_eq!(rational(-6, 4).numerator(), -3);
    assert_eq!(rational(-6, 4).denominator(), 2);
    assert_eq!(Rational::new(1, 0), None);
    assert_eq!(Rational::new(9_007_199_254_740_992, 1), None);
    assert_eq!(Rational::new(1, 9_007_199_254_740_992), None);
    assert_eq!(
      Rational::new(-9_007_199_254_740_992, 2).map(|value| value.numerator()),
      Some(-4_503_599_627_370_496)
    );
    assert_eq!(Rational::new(i64::MIN, 1), None);
    assert_eq!(
      Rational::new(i64::MIN, 1 << 11).map(|value| value.numerator()),
      Some(-4_503_599_627_370_496)
    );
    assert_eq!(rational(9_007_199_254_740_991, 1).numerator(), 9_007_199_254_740_991);
  }

  #[test]
  fn converts_to_other_types() {
    assert_eq!(rational(1, 4).to_f64(), 0.25);
    assert_eq!(rational(-30_000, 1_001).to_f64(), -30_000.0 / 1_001.0);
    assert_eq!(rational(7, 1).to_integer(), Some(7));
    assert_eq!(rational(7, 2).to_integer(), None);
  }

  #[test]
  fn orders_by_value() {
    assert!(rational(1, 3) < rational(1, 2));
    assert!(rational(-1, 2) < rational(-1, 3));
    assert!(rational(30_000, 1_001) < rational(30, 1));
    assert_eq!(rational(2, 4).cmp(&rational(1, 2)), std::cmp::Ordering::Equal);
    assert_eq!(
      rational(9_007_199_254_740_991, 3).partial_cmp(&rational(1, 9_007_199_254_740_991)),
      Some(std::cmp::Ordering::Greater)
    );
  }

  #[test]
  fn displays_its_value() {
    assert_eq!(rational(48_000, 1).to_string(), "48000");
    assert_eq!(rational(30_000, 1_001).to_string(), "30000/1001");
    assert_eq!(rational(-1, 2).to_string(), "-1/2");
  }
}
