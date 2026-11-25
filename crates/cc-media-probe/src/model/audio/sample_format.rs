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

//! `SampleFormat`, `SampleEncoding` and `ByteOrder`, how an audio stream stores its samples (`api.md` section
//! 6.6).

use super::constants::{BIG_ENDIAN_NAME, FLOATING_POINT_NAME, INTEGER_NAME, LITTLE_ENDIAN_NAME};
use crate::string_enumeration::string_enumeration;

/// How an audio stream stores its samples: their encoding, sign and byte order.
///
/// ```
/// use cc_media_probe::model::{ByteOrder, SampleEncoding, SampleFormat};
///
/// fn describe(format: &SampleFormat) -> String {
///     let sign = match format.is_signed() {
///         Some(true) => "signed ",
///         Some(false) => "unsigned ",
///         None => "",
///     };
///     let order = match format.byte_order() {
///         Some(ByteOrder::LittleEndian) => ", little-endian",
///         Some(ByteOrder::BigEndian) => ", big-endian",
///         _ => "",
///     };
///     let encoding = match format.encoding() {
///         SampleEncoding::Integer => "integer",
///         SampleEncoding::FloatingPoint => "floating point",
///         other => other.as_str(),
///     };
///     format!("{sign}{encoding}{order}")
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SampleFormat {
  encoding: SampleEncoding,
  signed: Option<bool>,
  byte_order: Option<ByteOrder>,
}

impl SampleFormat {
  /// The encoding of the samples.
  #[must_use]
  pub fn encoding(&self) -> &SampleEncoding {
    &self.encoding
  }

  /// Whether the samples are signed.
  #[must_use]
  pub fn is_signed(&self) -> Option<bool> {
    self.signed
  }

  /// The byte order of the samples.
  #[must_use]
  pub fn byte_order(&self) -> Option<&ByteOrder> {
    self.byte_order.as_ref()
  }
}

string_enumeration! {
  /// The encoding of audio samples. `SampleFormat` shows an example.
  ///
  /// ```
  /// use cc_media_probe::model::SampleEncoding;
  ///
  /// assert_eq!(SampleEncoding::FloatingPoint.as_str(), "floating_point");
  /// ```
  SampleEncoding {
    /// Integers.
    Integer => INTEGER_NAME,
    /// IEEE 754 floating-point numbers.
    FloatingPoint => FLOATING_POINT_NAME,
  }
}

string_enumeration! {
  /// The byte order of audio samples. `SampleFormat` shows an example.
  ///
  /// ```
  /// use cc_media_probe::model::ByteOrder;
  ///
  /// assert_eq!(ByteOrder::BigEndian.as_str(), "big_endian");
  /// ```
  ByteOrder {
    /// The least significant byte first.
    LittleEndian => LITTLE_ENDIAN_NAME,
    /// The most significant byte first.
    BigEndian => BIG_ENDIAN_NAME,
  }
}

#[cfg(test)]
mod tests {
  use super::{ByteOrder, SampleEncoding, SampleFormat};
  use crate::model::UnrecognizedValue;

  impl SampleFormat {
    /// Signed little-endian integers.
    pub(crate) fn sample() -> SampleFormat {
      SampleFormat {
        encoding: SampleEncoding::Integer,
        signed: Some(true),
        byte_order: Some(ByteOrder::LittleEndian),
      }
    }
  }

  #[test]
  fn keeps_the_encoding_the_sign_and_the_order() {
    let format = SampleFormat::sample();
    assert_eq!(format.encoding(), &SampleEncoding::Integer);
    assert_eq!(format.is_signed(), Some(true));
    assert_eq!(format.byte_order(), Some(&ByteOrder::LittleEndian));
    assert_eq!(SampleEncoding::FloatingPoint.as_str(), "floating_point");
    assert_eq!(
      SampleEncoding::Unrecognized(UnrecognizedValue::sample("mu_law")).as_str(),
      "mu_law"
    );
    assert_eq!(ByteOrder::BigEndian.as_str(), "big_endian");
    assert_eq!(SampleEncoding::Integer.as_str(), "integer");
    assert_eq!(ByteOrder::LittleEndian.as_str(), "little_endian");
  }
}
