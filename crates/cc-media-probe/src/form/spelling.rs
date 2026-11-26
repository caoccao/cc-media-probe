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

//! The spellings of a raw code and of a protection scheme's value, which each namespace fixes (`json-output.md`
//! section 7.3, after the thirteenth review of `api.md`): a code of bytes in lowercase hexadecimal, two digits a
//! byte, and a coded number in canonical decimal within its width; and an integer in canonical decimal of any length,
//! as an exact value's parts are (section 8), which the review of the third slice's first part found held to 128
//! bits.

use std::fmt::Display;
use std::str::FromStr;

use super::constants::{MINUS_SIGN, ZERO_DIGIT};
use super::numbers::canonical;
use crate::utilities::decode_bytes;

/// Whether the text is exactly that many bytes in lowercase hexadecimal.
pub(crate) fn is_bytes(text: &str, count: usize) -> bool {
  decode_bytes(text).is_some_and(|bytes| bytes.len() == count)
}

/// Whether the text is a number of that type in canonical decimal.
pub(crate) fn is_decimal<Number: FromStr + Display>(text: &str) -> bool {
  canonical::<Number>(text).is_some()
}

/// Whether the text is an integer in canonical decimal of any length: ASCII digits with no leading zero, after a
/// minus sign for a negative integer, so `0` is the one spelling of zero and `-0` is none.
pub(crate) fn is_whole_decimal(text: &str) -> bool {
  match text.strip_prefix(MINUS_SIGN) {
    Some(magnitude) => is_positive_decimal(magnitude),
    None => text.strip_prefix(ZERO_DIGIT).is_some_and(str::is_empty) || is_positive_decimal(text),
  }
}

/// Whether the text is an integer of at least 1 in canonical decimal of any length: ASCII digits, the first not 0.
pub(crate) fn is_positive_decimal(text: &str) -> bool {
  !text.starts_with(ZERO_DIGIT) && !text.is_empty() && text.bytes().all(|digit| digit.is_ascii_digit())
}

#[cfg(test)]
mod tests {
  use super::{is_positive_decimal, is_whole_decimal};

  #[test]
  fn spells_an_integer_of_any_length_one_way() {
    let wide = "170141183460469231731687303715884105728170141183460469231731687303715884105728";
    for text in ["0", "7", "-7", "10", wide] {
      assert!(is_whole_decimal(text), "{text}");
    }
    for text in ["", "-", "-0", "+7", "07", "-07", "7.0", "1e3", " 7", "٣"] {
      assert!(!is_whole_decimal(text), "{text}");
    }
    assert!(is_positive_decimal(wide));
    assert!(!is_positive_decimal("0"));
    assert!(!is_positive_decimal("-1"));
  }
}
