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

//! A JSON number read from its text, for the map that serde_json's `arbitrary_precision` feature hands over in place
//! of a number that is not an integer within 64 bits: one member, named `$serde_json::private::Number`, whose value
//! is the number's text. Cargo turns the feature on for every crate of a build once any crate asks for it, so a
//! reader of the form sees that map whenever it reads a value whose type it does not know yet: in the neutral form,
//! a catch-all and a kept member. As the maintainer chose at the fifteenth review, the map reads as the number that
//! a build without the feature reads: an integer within 64 bits as that integer, any other number as its nearest
//! `f64`, which the form refuses when it is not finite. A map whose value is not the text of a JSON number stays a
//! map.

use crate::constants::{DECIMAL_POINT, EXPONENT_MARKERS, MINUS_SIGN, NEGATIVE_ZERO, PLUS_SIGN, ZERO_DIGIT};

/// A number read from its text, as a build without the feature reads it.
pub(crate) enum NumberText {
  /// An integer that is not negative, within 64 bits.
  Unsigned(u64),
  /// A negative integer within 64 bits.
  Signed(i64),
  /// Any other number, possibly not finite.
  FloatingPoint(f64),
}

/// The number that the text spells, when it is a JSON number (RFC 8259 section 6).
pub(crate) fn parse(text: &str) -> Option<NumberText> {
  let magnitude = text.strip_prefix(MINUS_SIGN).unwrap_or(text);
  let (mantissa, exponent) = match magnitude.split_once(EXPONENT_MARKERS) {
    Some((mantissa, exponent)) => (mantissa, Some(exponent)),
    None => (magnitude, None),
  };
  let (integer, fraction) = match mantissa.split_once(DECIMAL_POINT) {
    Some((integer, fraction)) => (integer, Some(fraction)),
    None => (mantissa, None),
  };
  let digits = |part: &str| !part.is_empty() && part.bytes().all(|byte| byte.is_ascii_digit());
  let integer_valid = digits(integer) && (integer.len() == 1 || !integer.starts_with(ZERO_DIGIT));
  let exponent_valid =
    exponent.is_none_or(|exponent| digits(exponent.strip_prefix([PLUS_SIGN, MINUS_SIGN]).unwrap_or(exponent)));
  if !(integer_valid && fraction.is_none_or(digits) && exponent_valid) {
    return None;
  }
  if fraction.is_none() && exponent.is_none() && text != NEGATIVE_ZERO {
    if let Ok(value) = text.parse::<u64>() {
      return Some(NumberText::Unsigned(value));
    }
    if let Ok(value) = text.parse::<i64>() {
      return Some(NumberText::Signed(value));
    }
  }
  text.parse::<f64>().ok().map(NumberText::FloatingPoint)
}
