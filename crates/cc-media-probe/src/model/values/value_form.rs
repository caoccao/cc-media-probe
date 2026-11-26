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

//! The serialized form of `Seconds`, `Guid` and `FloatingPointValue` (`json-output.md` section 5.6): seconds as the
//! map of their rational, a GUID as its 16 stored bytes in hexadecimal, and a floating-point value as its width and
//! its bits in hexadecimal, 8 digits for single and 16 for double precision.

use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::constants::{
  DOUBLE_BIT_DIGITS, FLOATING_POINT_MESSAGE, GUID_BYTES, GUID_MESSAGE, HEXADECIMAL_RADIX, SINGLE_BIT_DIGITS,
};
use super::{FloatingPointValue, FloatingPointWidth, Guid, Rational, Seconds};
use crate::form::{ReadAlone, checked_form, read_alone};
use crate::utilities::{decode_bytes, encode_bytes};

impl Serialize for Seconds {
  fn serialize<Output: Serializer>(&self, serializer: Output) -> Result<Output::Ok, Output::Error> {
    self.as_rational().serialize(serializer)
  }
}

impl<'input> Deserialize<'input> for Seconds {
  /// The rational of seconds, which reads through the strict reader itself.
  fn deserialize<Input: Deserializer<'input>>(deserializer: Input) -> Result<Seconds, Input::Error> {
    Rational::deserialize(deserializer).map(Seconds::from_rational)
  }
}

impl Serialize for Guid {
  fn serialize<Output: Serializer>(&self, serializer: Output) -> Result<Output::Ok, Output::Error> {
    serializer.serialize_str(&encode_bytes(&self.bytes()))
  }
}

impl<'input> ReadAlone<'input> for Guid {
  fn read_members<Input: Deserializer<'input>>(deserializer: Input) -> Result<Guid, Input::Error> {
    let text = String::deserialize(deserializer)?;
    let bytes: [u8; GUID_BYTES] = decode_bytes(&text)
      .and_then(|bytes| bytes.try_into().ok())
      .ok_or_else(|| Input::Error::custom(GUID_MESSAGE))?;
    Ok(Guid::from_bytes(bytes))
  }
}

impl<'input> Deserialize<'input> for Guid {
  fn deserialize<Input: Deserializer<'input>>(deserializer: Input) -> Result<Guid, Input::Error> {
    read_alone(deserializer)
  }
}

/// A floating-point value as written: its width and its bits.
#[derive(Serialize, Deserialize)]
struct FloatingPointForm {
  width: FloatingPointWidth,
  bits: String,
}

/// The number of hexadecimal digits of the bits of a width: 8 for single and 16 for double precision, and 16 for a
/// width of a newer version, which keeps all 64 bits.
fn bit_digits(width: &FloatingPointWidth) -> usize {
  match width {
    FloatingPointWidth::Single => SINGLE_BIT_DIGITS,
    _ => DOUBLE_BIT_DIGITS,
  }
}

impl Serialize for FloatingPointValue {
  fn serialize<Output: Serializer>(&self, serializer: Output) -> Result<Output::Ok, Output::Error> {
    let digits = bit_digits(self.width());
    FloatingPointForm {
      width: self.width().clone(),
      bits: format!("{:0digits$x}", self.bits()),
    }
    .serialize(serializer)
  }
}

impl TryFrom<FloatingPointForm> for FloatingPointValue {
  type Error = &'static str;

  /// The bits in exactly the digits of their width, lowercase, so `1.0` in double precision is `3ff0000000000000`.
  fn try_from(form: FloatingPointForm) -> Result<FloatingPointValue, &'static str> {
    let whole = form.bits.len() == bit_digits(&form.width)
      && form
        .bits
        .bytes()
        .all(|digit| digit.is_ascii_hexdigit() && !digit.is_ascii_uppercase());
    let bits = u64::from_str_radix(&form.bits, HEXADECIMAL_RADIX)
      .ok()
      .filter(|_| whole)
      .ok_or(FLOATING_POINT_MESSAGE)?;
    Ok(FloatingPointValue::from_bits(bits, form.width))
  }
}

checked_form!(FloatingPointValue from FloatingPointForm);
