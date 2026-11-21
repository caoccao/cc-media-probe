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

//! The numbers that may stay inline: 0 and 1 (R33).

use crate::constants::punctuation::UNDERSCORE;
use crate::lexer::LiteralKind;
use crate::literals::constants::{
  BINARY_PREFIX, BINARY_RADIX, DECIMAL_RADIX, FLOATING_POINT_ONE, FLOATING_POINT_SUFFIX_START, HEXADECIMAL_PREFIX,
  HEXADECIMAL_RADIX, OCTAL_PREFIX, OCTAL_RADIX, SIGNED_SUFFIX_START, UNSIGNED_SUFFIX_START,
};

/// Whether a numeric literal has the value 0 or 1, in any notation and with
/// any suffix, such as `0`, `1_u8`, `0x1` or `1.0`.
#[must_use]
pub fn is_zero_or_one(text: &str, kind: LiteralKind) -> bool {
  let cleaned: String = text.chars().filter(|character| *character != UNDERSCORE).collect();
  match kind {
    LiteralKind::Integer => integer_value(&cleaned).is_some_and(|value| value <= 1),
    LiteralKind::Float => float_value(&cleaned)
      .is_some_and(|value| value.total_cmp(&0.0).is_eq() || value.total_cmp(&FLOATING_POINT_ONE).is_eq()),
    _ => false,
  }
}

fn integer_value(text: &str) -> Option<u128> {
  let (digits, radix) = [
    (HEXADECIMAL_PREFIX, HEXADECIMAL_RADIX),
    (OCTAL_PREFIX, OCTAL_RADIX),
    (BINARY_PREFIX, BINARY_RADIX),
  ]
  .iter()
  .find_map(|(prefix, radix)| text.strip_prefix(prefix).map(|rest| (rest, *radix)))
  .unwrap_or((text, DECIMAL_RADIX));
  let is_suffix_start = |character: char| {
    if radix == HEXADECIMAL_RADIX {
      character == UNSIGNED_SUFFIX_START || character == SIGNED_SUFFIX_START
    } else {
      character.is_alphabetic()
    }
  };
  let end = digits.find(is_suffix_start).unwrap_or(digits.len());
  u128::from_str_radix(digits.get(..end)?, radix).ok()
}

fn float_value(text: &str) -> Option<f64> {
  let end = text.find(FLOATING_POINT_SUFFIX_START).unwrap_or(text.len());
  text.get(..end)?.parse().ok()
}

#[cfg(test)]
mod tests {
  use super::is_zero_or_one;
  use crate::lexer::LiteralKind;

  #[test]
  fn accepts_zero_and_one_in_every_notation() {
    for text in ["0", "1", "0_u8", "1usize", "0x0", "0x1", "0b1", "0o0", "0x1_i32"] {
      assert!(is_zero_or_one(text, LiteralKind::Integer), "{text}");
    }
    for text in ["0.0", "1.0", "1.", "1e0", "0f32", "1.0_f64"] {
      assert!(is_zero_or_one(text, LiteralKind::Float), "{text}");
    }
  }

  #[test]
  fn rejects_other_numbers_and_kinds() {
    for text in ["2", "10", "0x10", "0b10", "0xff", "500_usize"] {
      assert!(!is_zero_or_one(text, LiteralKind::Integer), "{text}");
    }
    for text in ["0.5", "2.0", "1e3"] {
      assert!(!is_zero_or_one(text, LiteralKind::Float), "{text}");
    }
    assert!(!is_zero_or_one("'1'", LiteralKind::Character));
    assert!(!is_zero_or_one("0x", LiteralKind::Integer));
  }
}
