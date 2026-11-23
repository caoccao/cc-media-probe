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

//! The value types of `api.md` section 6.1 that the scenarios use.

use std::fmt;
use std::time::Duration;

use super::constants::{
  BITSTREAM_NAME, CONSTANT_BIT_RATE_OVER_SIZE_NAME, DECLARED_NAME, DELIVERY_CLOCK_SPAN_NAME, DERIVED_NAME,
  ENDPOINT_TIMESTAMPS_NAME, INDEX_SIZE_TOTALS_NAME, LAST_UNIT_START_NAME, SECONDS_SYMBOL, SIZE_SEPARATOR,
};
use super::unrecognized_value::UnrecognizedValue;
use crate::constants::LARGEST_EXACT;
#[cfg(feature = "serde")]
use crate::constants::{RATIONAL_MESSAGE, SIZE_MESSAGE};
#[cfg(feature = "serde")]
use crate::document::InvalidDocument;

/// An exact fraction, always reduced, with a denominator of at least 1 and both parts at most 2^53 - 1 in magnitude.
/// It reads back only in that form (`api.md` section 12).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "RationalData"))]
pub struct Rational {
  numerator: i64,
  denominator: u64,
}

/// A rational as read, before its form is checked.
#[cfg(feature = "serde")]
#[derive(serde::Deserialize)]
struct RationalData {
  numerator: i64,
  denominator: u64,
}

#[cfg(feature = "serde")]
impl TryFrom<RationalData> for Rational {
  type Error = InvalidDocument;

  /// The form that `Rational::new` gives, and no other: a zero denominator, a part out of range or a fraction that
  /// is not reduced is malformed.
  fn try_from(data: RationalData) -> Result<Rational, InvalidDocument> {
    match Rational::new(data.numerator, data.denominator) {
      Some(rational) if (rational.numerator, rational.denominator) == (data.numerator, data.denominator) => {
        Ok(rational)
      }
      _ => Err(InvalidDocument::new(RATIONAL_MESSAGE)),
    }
  }
}

impl Rational {
  /// `numerator / denominator`, reduced; `None` for a zero denominator or a part that is still beyond 2^53 - 1 in
  /// magnitude once reduced.
  pub fn new(numerator: i64, denominator: u64) -> Option<Rational> {
    if denominator == 0 {
      return None;
    }
    let divisor = greatest_common_divisor(numerator.unsigned_abs(), denominator);
    let magnitude = numerator.unsigned_abs() / divisor;
    let denominator = denominator / divisor;
    if magnitude > LARGEST_EXACT || denominator > LARGEST_EXACT {
      return None;
    }
    let reduced = i64::try_from(magnitude).ok()?;
    Some(Rational {
      numerator: if numerator < 0 { reduced.wrapping_neg() } else { reduced },
      denominator,
    })
  }

  /// The numerator, signed.
  pub fn numerator(&self) -> i64 {
    self.numerator
  }

  /// The denominator, at least 1.
  pub fn denominator(&self) -> u64 {
    self.denominator
  }

  /// The nearest `f64`.
  #[allow(clippy::cast_precision_loss)]
  pub fn to_f64(&self) -> f64 {
    self.numerator as f64 / self.denominator as f64
  }
}

/// The greatest common divisor, by Euclid's algorithm; the second number is at least 1.
fn greatest_common_divisor(mut first: u64, mut second: u64) -> u64 {
  while second != 0 {
    (first, second) = (second, first % second);
  }
  first
}

impl fmt::Display for Rational {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    if self.denominator == 1 {
      write!(formatter, "{}", self.numerator)
    } else {
      write!(formatter, "{}/{}", self.numerator, self.denominator)
    }
  }
}

/// A time on a presentation timeline, or a length of time.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Seconds(Rational);

impl Seconds {
  /// The exact value.
  pub fn as_rational(&self) -> Rational {
    self.0
  }

  /// The time as a `Duration`, or `None` when it is negative. The stub keeps whole seconds only.
  pub fn to_duration(&self) -> Option<Duration> {
    let numerator = u64::try_from(self.0.numerator).ok()?;
    numerator.checked_div(self.0.denominator).map(Duration::from_secs)
  }
}

impl fmt::Display for Seconds {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(formatter, "{} {SECONDS_SYMBOL}", self.0)
  }
}

/// A width and a height in whole pixels, each at most 2^53 - 1, so that JSON holds them as exact numbers. It reads
/// back only in that range (`api.md` sections 6.1 and 12).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "SizeData"))]
pub struct Size {
  width: u64,
  height: u64,
}

/// A size as read, before its range is checked.
#[cfg(feature = "serde")]
#[derive(serde::Deserialize)]
struct SizeData {
  width: u64,
  height: u64,
}

#[cfg(feature = "serde")]
impl TryFrom<SizeData> for Size {
  type Error = InvalidDocument;

  fn try_from(data: SizeData) -> Result<Size, InvalidDocument> {
    Size::new(data.width, data.height).ok_or(InvalidDocument::new(SIZE_MESSAGE))
  }
}

impl Size {
  /// A size, or `None` when the width or the height is beyond 2^53 - 1, which no normalized field holds (R24).
  pub fn new(width: u64, height: u64) -> Option<Size> {
    (width <= LARGEST_EXACT && height <= LARGEST_EXACT).then_some(Size { width, height })
  }

  /// The width, in pixels.
  pub fn width(&self) -> u64 {
    self.width
  }

  /// The height, in pixels.
  pub fn height(&self) -> u64 {
    self.height
  }
}

impl fmt::Display for Size {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(formatter, "{}{SIZE_SEPARATOR}{}", self.width, self.height)
  }
}

/// A value whose meaning is identity, as text.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Identifier(String);

impl Identifier {
  /// The identifier: an integer in decimal, a binary value in lowercase hexadecimal.
  pub fn as_str(&self) -> &str {
    &self.0
  }
}

impl fmt::Display for Identifier {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.0)
  }
}

/// A media type, such as `video/mp4`.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MediaType(String);

impl MediaType {
  /// The media type as text.
  pub fn as_str(&self) -> &str {
    &self.0
  }
}

impl fmt::Display for MediaType {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.0)
  }
}

/// Where a value came from (plan section 8.4).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum Provenance {
  /// Stated by the container, including a default that a specification states.
  Declared,
  /// Read from a codec header.
  Bitstream,
  /// Computed from other values.
  Derived,
  /// A value that a newer version wrote.
  #[cfg_attr(feature = "serde", serde(with = "crate::model::unrecognized_text"))]
  Unrecognized(UnrecognizedValue<Provenance>),
}

impl Provenance {
  /// The serialized name, or the text of an unrecognized value.
  pub fn as_str(&self) -> &str {
    match self {
      Provenance::Declared => DECLARED_NAME,
      Provenance::Bitstream => BITSTREAM_NAME,
      Provenance::Derived => DERIVED_NAME,
      Provenance::Unrecognized(value) => value.as_str(),
    }
  }
}

/// An estimated value and how it was made (R25).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Estimate<Value> {
  value: Value,
  method: EstimateMethod,
}

impl<Value: Copy> Estimate<Value> {
  /// The estimated value.
  pub fn value(&self) -> Value {
    self.value
  }

  /// How it was estimated.
  pub fn method(&self) -> &EstimateMethod {
    &self.method
  }
}

/// How an estimate was made (plan section 8.4).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum EstimateMethod {
  /// A constant bit rate over the size.
  ConstantBitRateOverSize,
  /// The span of a delivery clock.
  DeliveryClockSpan,
  /// The first and last timestamps.
  EndpointTimestamps,
  /// The start of the last unit.
  LastUnitStart,
  /// The totals of an index.
  IndexSizeTotals,
  /// A value that a newer version wrote.
  #[cfg_attr(feature = "serde", serde(with = "crate::model::unrecognized_text"))]
  Unrecognized(UnrecognizedValue<EstimateMethod>),
}

impl EstimateMethod {
  /// The serialized name, or the text of an unrecognized value.
  pub fn as_str(&self) -> &str {
    match self {
      EstimateMethod::ConstantBitRateOverSize => CONSTANT_BIT_RATE_OVER_SIZE_NAME,
      EstimateMethod::DeliveryClockSpan => DELIVERY_CLOCK_SPAN_NAME,
      EstimateMethod::EndpointTimestamps => ENDPOINT_TIMESTAMPS_NAME,
      EstimateMethod::LastUnitStart => LAST_UNIT_START_NAME,
      EstimateMethod::IndexSizeTotals => INDEX_SIZE_TOTALS_NAME,
      EstimateMethod::Unrecognized(value) => value.as_str(),
    }
  }
}
