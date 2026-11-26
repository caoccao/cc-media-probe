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

//! Lowercase hexadecimal text, the form of bytes and of 16-bit units in the report (R24; plan section 8.8).

use std::fmt::{LowerHex, Write};

#[cfg(feature = "serde")]
use super::constants::BYTE_DIGITS;
use super::constants::HEXADECIMAL_RADIX;

/// The units as lowercase hexadecimal, `digits` digits a unit: two for a byte, four for a 16-bit unit.
pub(crate) fn encode<Unit: LowerHex>(units: impl IntoIterator<Item = Unit>, digits: usize) -> String {
  units.into_iter().fold(String::new(), |mut text, unit| {
    let _ = write!(text, "{unit:0digits$x}");
    text
  })
}

/// Whether the text is whole units of `digits` lowercase hexadecimal digits each.
pub(crate) fn is_whole_units(data: &str, digits: usize) -> bool {
  digits > 0
    && data.len().is_multiple_of(digits)
    && data
      .bytes()
      .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
}

/// The text read as units of `digits` lowercase hexadecimal digits each, at most four, or `None` for text that is
/// not such units.
pub(crate) fn decode_units(data: &str, digits: usize) -> Option<Vec<u16>> {
  if !is_whole_units(data, digits) {
    return None;
  }
  data
    .as_bytes()
    .chunks(digits)
    .map(|chunk| {
      let text = std::str::from_utf8(chunk).ok()?;
      u16::from_str_radix(text, HEXADECIMAL_RADIX).ok()
    })
    .collect()
}

/// The bytes as lowercase hexadecimal, two digits a byte: the form of every byte string of the report, such as a
/// binary tag value, a GUID and a raw code (`json-output.md` section 5.6).
#[cfg(feature = "serde")]
pub(crate) fn encode_bytes(bytes: &[u8]) -> String {
  encode(bytes.iter().copied(), BYTE_DIGITS)
}

/// The bytes of lowercase hexadecimal text, two digits a byte, or `None` for any other text, so a sign, an uppercase
/// digit and an odd number of digits fail.
#[cfg(feature = "serde")]
pub(crate) fn decode_bytes(data: &str) -> Option<Vec<u8>> {
  decode_units(data, BYTE_DIGITS)?
    .into_iter()
    .map(u8::try_from)
    .collect::<Result<Vec<u8>, _>>()
    .ok()
}

#[cfg(test)]
mod tests {
  use super::{decode_units, encode, is_whole_units};

  #[test]
  fn writes_lowercase_digits() {
    assert_eq!(encode([0x89_u8, 0x50, 0x4e, 0x47], 2), "89504e47");
    assert_eq!(encode(Vec::<u8>::new(), 2), "");
    assert_eq!(encode([0xd800_u16, 0x002e], 4), "d800002e");
  }

  #[test]
  fn reads_whole_units_only() {
    assert!(is_whole_units("ff2e", 2));
    assert!(!is_whole_units("ff2", 2));
    assert!(!is_whole_units("FF2E", 2));
    assert!(!is_whole_units("zz", 2));
    assert!(!is_whole_units("ff", 0));
    assert_eq!(decode_units("ff2e", 2), Some(vec![0xff, 0x2e]));
    assert_eq!(decode_units("d800002e", 4), Some(vec![0xd800, 0x002e]));
    assert_eq!(decode_units("d80", 4), None);
    assert_eq!(decode_units("", 4), Some(Vec::new()));
  }
}
