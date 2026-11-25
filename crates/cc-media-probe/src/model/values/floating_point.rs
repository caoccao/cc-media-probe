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

//! `FloatingPointValue` and `FloatingPointWidth`, a floating-point number exactly as stored (`api.md` section 6.1).

use super::Rational;
use super::constants::{
  BINARY_BASE, DOUBLE_EXPONENT_MASK, DOUBLE_EXPONENT_OFFSET, DOUBLE_FRACTION_BITS, DOUBLE_FRACTION_MASK,
  DOUBLE_IMPLICIT_BIT, DOUBLE_NAME, SINGLE_NAME,
};
use crate::string_enumeration::string_enumeration;

/// A floating-point number exactly as a file stores it, by its bits and its width. It is used in the typed details
/// and in tag values; no normalized field holds one (F2).
///
/// ```
/// use cc_media_probe::model::{FloatingPointValue, FloatingPointWidth};
///
/// fn describe(value: &FloatingPointValue) -> String {
///     match (value.width(), value.to_rational()) {
///         (FloatingPointWidth::Single, Some(exact)) => format!("{exact} (single precision)"),
///         (_, Some(exact)) => exact.to_string(),
///         (_, None) => format!("{} (bits {:x})", value.to_f64(), value.bits()),
///     }
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FloatingPointValue {
  bits: u64,
  width: FloatingPointWidth,
}

impl FloatingPointValue {
  /// The bits as stored: the low 32 bits for single precision, all 64 for double precision.
  ///
  /// ```
  /// use cc_media_probe::model::FloatingPointValue;
  ///
  /// fn is_positive_zero(value: &FloatingPointValue) -> bool {
  ///     value.bits() == 0
  /// }
  /// ```
  #[must_use]
  pub fn bits(&self) -> u64 {
    self.bits
  }

  /// The width that the file stored.
  ///
  /// ```
  /// use cc_media_probe::model::{FloatingPointValue, FloatingPointWidth};
  ///
  /// fn is_double(value: &FloatingPointValue) -> bool {
  ///     matches!(value.width(), FloatingPointWidth::Double)
  /// }
  /// ```
  #[must_use]
  pub fn width(&self) -> &FloatingPointWidth {
    &self.width
  }

  /// The value as an `f64`, which holds every single-precision value exactly. A width of a newer version, and bits
  /// that do not fit their width, give NaN.
  ///
  /// ```
  /// use cc_media_probe::model::FloatingPointValue;
  ///
  /// fn is_finite(value: &FloatingPointValue) -> bool {
  ///     value.to_f64().is_finite()
  /// }
  /// ```
  #[must_use]
  pub fn to_f64(&self) -> f64 {
    match self.width {
      FloatingPointWidth::Single => u32::try_from(self.bits).map_or(f64::NAN, |bits| f64::from(f32::from_bits(bits))),
      FloatingPointWidth::Double => f64::from_bits(self.bits),
      FloatingPointWidth::Unrecognized(_) => f64::NAN,
    }
  }

  /// The exact rational of the binary value, or `None` when it is not finite or a part of the fraction is beyond
  /// 2^53 - 1 in magnitude.
  ///
  /// ```
  /// use cc_media_probe::model::FloatingPointValue;
  ///
  /// fn as_fraction(value: &FloatingPointValue) -> String {
  ///     value.to_rational().map_or_else(|| "not exact".to_owned(), |exact| exact.to_string())
  /// }
  /// ```
  #[must_use]
  pub fn to_rational(&self) -> Option<Rational> {
    let value = self.to_f64();
    if !value.is_finite() {
      return None;
    }
    let bits = value.to_bits();
    let biased_exponent = bits.checked_shr(DOUBLE_FRACTION_BITS)? & DOUBLE_EXPONENT_MASK;
    let fraction = bits & DOUBLE_FRACTION_MASK;
    let (significand, exponent) = if biased_exponent == 0 {
      (fraction, 1_i64.checked_sub(DOUBLE_EXPONENT_OFFSET)?)
    } else {
      let biased_exponent = i64::try_from(biased_exponent).ok()?;
      (
        fraction | DOUBLE_IMPLICIT_BIT,
        biased_exponent.checked_sub(DOUBLE_EXPONENT_OFFSET)?,
      )
    };
    if significand == 0 {
      return Rational::new(0, 1);
    }
    let removable = i64::from(significand.trailing_zeros()).min(exponent.checked_neg()?.max(0));
    let significand = significand.checked_shr(u32::try_from(removable).ok()?)?;
    let exponent = exponent.checked_add(removable)?;
    let magnitude = i64::try_from(significand).ok()?;
    let numerator = if value.is_sign_negative() {
      magnitude.checked_neg()?
    } else {
      magnitude
    };
    let power = BINARY_BASE.checked_pow(u32::try_from(exponent.unsigned_abs()).ok()?)?;
    if exponent < 0 {
      Rational::new(numerator, power)
    } else {
      Rational::new(numerator.checked_mul(i64::try_from(power).ok()?)?, 1)
    }
  }
}

string_enumeration! {
  /// The width of a stored floating-point number.
  ///
  /// ```
  /// use cc_media_probe::model::FloatingPointWidth;
  ///
  /// assert_eq!(FloatingPointWidth::Double.as_str(), "double");
  /// ```
  FloatingPointWidth {
    /// IEEE 754 single precision, 32 bits.
    Single => SINGLE_NAME,
    /// IEEE 754 double precision, 64 bits.
    Double => DOUBLE_NAME,
  }
}

#[cfg(test)]
mod tests {
  use super::{FloatingPointValue, FloatingPointWidth};
  use crate::Rational;
  use crate::model::UnrecognizedValue;

  impl FloatingPointValue {
    /// A double-precision value.
    pub(crate) fn double(value: f64) -> FloatingPointValue {
      FloatingPointValue {
        bits: value.to_bits(),
        width: FloatingPointWidth::Double,
      }
    }

    /// A single-precision value.
    pub(crate) fn single(value: f32) -> FloatingPointValue {
      FloatingPointValue {
        bits: u64::from(value.to_bits()),
        width: FloatingPointWidth::Single,
      }
    }
  }

  fn rational(numerator: i64, denominator: u64) -> Option<Rational> {
    Rational::new(numerator, denominator)
  }

  #[test]
  fn converts_each_width() {
    assert_eq!(FloatingPointValue::double(1.5).to_f64(), 1.5);
    assert_eq!(FloatingPointValue::double(1.5).bits(), 0x3ff8_0000_0000_0000);
    assert_eq!(FloatingPointValue::single(0.1).to_f64(), f64::from(0.1_f32));
    assert_eq!(FloatingPointValue::single(0.1).width(), &FloatingPointWidth::Single);
    let wide = FloatingPointValue {
      bits: u64::MAX,
      width: FloatingPointWidth::Single,
    };
    assert!(wide.to_f64().is_nan());
    let newer = FloatingPointValue {
      bits: 0,
      width: FloatingPointWidth::Unrecognized(UnrecognizedValue::sample("half")),
    };
    assert!(newer.to_f64().is_nan());
    assert_eq!(newer.to_rational(), None);
  }

  #[test]
  fn gives_the_exact_rational_of_the_binary_value() {
    assert_eq!(FloatingPointValue::double(0.0).to_rational(), rational(0, 1));
    assert_eq!(FloatingPointValue::double(-0.0).to_rational(), rational(0, 1));
    assert_eq!(FloatingPointValue::double(1.0).to_rational(), rational(1, 1));
    assert_eq!(FloatingPointValue::double(1.5).to_rational(), rational(3, 2));
    assert_eq!(FloatingPointValue::double(-0.25).to_rational(), rational(-1, 4));
    assert_eq!(
      FloatingPointValue::double(23.976).to_rational(),
      rational(210_895_126_300_459, 8_796_093_022_208)
    );
    assert_eq!(FloatingPointValue::double(48_000.0).to_rational(), rational(48_000, 1));
    assert_eq!(
      FloatingPointValue::double(9_007_199_254_740_991.0).to_rational(),
      rational(9_007_199_254_740_991, 1)
    );
    assert_eq!(FloatingPointValue::double(9_007_199_254_740_992.0).to_rational(), None);
    assert_eq!(FloatingPointValue::double(0.1).to_rational(), None);
    assert_eq!(
      FloatingPointValue::single(0.1).to_rational(),
      rational(13_421_773, 134_217_728)
    );
    assert_eq!(FloatingPointValue::double(f64::MIN_POSITIVE).to_rational(), None);
    assert_eq!(FloatingPointValue::double(5e-324).to_rational(), None);
    assert_eq!(FloatingPointValue::double(f64::INFINITY).to_rational(), None);
    assert_eq!(FloatingPointValue::double(f64::NAN).to_rational(), None);
    assert_eq!(FloatingPointValue::double(1e300).to_rational(), None);
    assert_eq!(
      FloatingPointValue::double(1.0 + f64::EPSILON).to_rational(),
      rational(4_503_599_627_370_497, 4_503_599_627_370_496)
    );
    assert_eq!(FloatingPointValue::double(-1_024.0).to_rational(), rational(-1_024, 1));
  }

  #[test]
  fn names_each_width() {
    assert_eq!(FloatingPointWidth::Single.as_str(), "single");
    assert_eq!(FloatingPointWidth::Double.as_str(), "double");
  }
}
