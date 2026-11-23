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

//! Bytes in the form's one encoding of bytes, lowercase hexadecimal with two digits a byte (`json-output.md`
//! section 5.6): binary tag data, and the byte codes of raw codec identifiers, as the maintainer chose at the
//! thirteenth review.

use crate::constants::{BYTE_HEXADECIMAL_DIGITS, HEXADECIMAL_BASE};

/// The bytes as lowercase hexadecimal text.
pub(crate) fn encode(bytes: &[u8]) -> String {
  bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

/// The bytes of lowercase hexadecimal text, two digits a byte; `None` for any other text, so a sign, an uppercase
/// digit and an odd number of digits fail.
pub(crate) fn decode(text: &str) -> Option<Vec<u8>> {
  let digits = text.as_bytes();
  if !digits.len().is_multiple_of(BYTE_HEXADECIMAL_DIGITS)
    || !digits
      .iter()
      .all(|digit| digit.is_ascii_hexdigit() && !digit.is_ascii_uppercase())
  {
    return None;
  }
  digits
    .chunks(BYTE_HEXADECIMAL_DIGITS)
    .map(|pair| u8::from_str_radix(std::str::from_utf8(pair).ok()?, HEXADECIMAL_BASE).ok())
    .collect()
}
