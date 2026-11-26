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

//! The form of a value that a typed detail stores (`json-output.md` sections 5.5 and 8): an integer that a detail
//! stores at 64 bits, `u64` or `i64`, is a decimal string in canonical form whatever its value, so its JSON type never
//! varies, as the review of the third slice's first part found `StoredValue<u64>` written as a number; a narrower
//! integer is a number, and a flag is a flag. Text is a string, and a floating-point value is its width and its bits
//! (section 5.6), as the second review of that part asked, which found no `StoredValue<String>` and no
//! `StoredValue<FloatingPointValue>`, while the field guide names a default of each kind: a Matroska track's
//! `Language`, `eng`, and its `SamplingFrequency`, 8000 (RFC 9559 sections 5.1.4.1.19 and 5.1.4.1.29.1). RFC 8794
//! lets an element of any type but a master declare a default (section 11.1.6.8), and a later phase that stores a
//! default of another type gives that type its form here.

use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::form::canonical;
use crate::model::FloatingPointValue;

/// A value that a typed detail stores, with its form as written. The trait is public in a private module, so that a
/// public type's traits can require it while no caller can name or implement it.
pub trait DetailValue: Sized {
  /// The value as written.
  type Form: Serialize + DeserializeOwned;

  /// The value as written.
  fn to_form(&self) -> Self::Form;

  /// The value of what was written, or `None` when it is no value of this type.
  fn from_form(form: Self::Form) -> Option<Self>;
}

/// The values written as themselves.
macro_rules! plain_detail_values {
  ($($value:ty),* $(,)?) => {
    $(
      impl DetailValue for $value {
        type Form = $value;

        fn to_form(&self) -> $value {
          *self
        }

        fn from_form(form: $value) -> Option<$value> {
          Some(form)
        }
      }
    )*
  };
}

plain_detail_values!(bool, u8, u16, u32, i8, i16, i32);

/// The integers of 64 bits, written as decimal strings.
macro_rules! wide_detail_values {
  ($($value:ty),* $(,)?) => {
    $(
      impl DetailValue for $value {
        type Form = String;

        fn to_form(&self) -> String {
          self.to_string()
        }

        fn from_form(form: String) -> Option<$value> {
          canonical(&form)
        }
      }
    )*
  };
}

wide_detail_values!(u64, i64);

/// The values that own their data, written in their own forms.
macro_rules! owned_detail_values {
  ($($value:ty),* $(,)?) => {
    $(
      impl DetailValue for $value {
        type Form = $value;

        fn to_form(&self) -> $value {
          self.clone()
        }

        fn from_form(form: $value) -> Option<$value> {
          Some(form)
        }
      }
    )*
  };
}

owned_detail_values!(String, FloatingPointValue);

#[cfg(test)]
mod tests {
  use super::DetailValue;
  use crate::model::{FloatingPointValue, FloatingPointWidth};

  #[test]
  fn writes_an_integer_of_64_bits_as_a_decimal_string_and_a_narrower_one_as_itself() {
    assert_eq!(u64::MAX.to_form(), "18446744073709551615");
    assert_eq!(i64::from_form("-1".to_owned()), Some(-1));
    assert_eq!(u64::from_form("01".to_owned()), None);
    assert_eq!(7_u32.to_form(), 7);
    assert_eq!(bool::from_form(true), Some(true));
  }

  #[test]
  fn writes_text_and_a_floating_point_value_in_their_own_forms() {
    assert_eq!(String::from_form("eng".to_owned()), Some("eng".to_owned()));
    let rate = FloatingPointValue::from_bits(0x40bf_4000_0000_0000, FloatingPointWidth::Double);
    assert_eq!(rate.to_form(), rate);
    assert_eq!(FloatingPointValue::from_form(rate.clone()), Some(rate));
  }
}
