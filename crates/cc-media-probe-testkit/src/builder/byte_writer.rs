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

//! A writer of bytes in the orders that specifications state, which every builder writes through.

use super::constants::{FOUR_CHARACTER_CODE_LENGTH, TWENTY_FOUR_BIT_BYTES, TWENTY_FOUR_BITS};
use super::error::BuildError;

/// Bytes written in order. Each method appends and returns the writer, so calls chain. A value wider than its field
/// is recorded as the writer's error, which `into_bytes` gives back instead of the bytes.
///
/// ```
/// use cc_media_probe_testkit::builder::{BuildError, ByteWriter};
///
/// let mut writer = ByteWriter::new();
/// writer.big_endian_u32(16).four_character_code(*b"ftyp").text("isom").little_endian_u16(0x0201);
/// assert_eq!(writer.into_bytes().unwrap(), b"\0\0\0\x10ftypisom\x01\x02");
///
/// // The chain goes on past a value wider than its field, and `into_bytes` gives back the first such error.
/// let mut wide = ByteWriter::new();
/// wide.byte(1).big_endian_u24(0x0100_0000).little_endian_u24(0x0200_0000).byte(2);
/// assert_eq!(wide.as_bytes(), [1, 2]);
/// assert_eq!(wide.into_bytes(), Err(BuildError::TooWide { value: 0x0100_0000, bits: 24 }));
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ByteWriter {
  /// The bytes written so far.
  bytes: Vec<u8>,
  /// The first error, if any.
  error: Option<BuildError>,
}

impl ByteWriter {
  /// An empty writer.
  #[must_use]
  pub fn new() -> ByteWriter {
    ByteWriter::default()
  }

  /// Appends one byte.
  pub fn byte(&mut self, value: u8) -> &mut ByteWriter {
    self.bytes.push(value);
    self
  }

  /// Appends bytes as they are.
  pub fn bytes(&mut self, value: &[u8]) -> &mut ByteWriter {
    self.bytes.extend_from_slice(value);
    self
  }

  /// Appends the bytes of `value` as they are, its UTF-8 form.
  pub fn text(&mut self, value: &str) -> &mut ByteWriter {
    self.bytes(value.as_bytes())
  }

  /// Appends a four-character code, such as `*b"ftyp"`, as its four bytes.
  pub fn four_character_code(&mut self, code: [u8; FOUR_CHARACTER_CODE_LENGTH]) -> &mut ByteWriter {
    self.bytes(&code)
  }

  /// Appends a 16-bit value, most significant byte first.
  pub fn big_endian_u16(&mut self, value: u16) -> &mut ByteWriter {
    self.bytes(&value.to_be_bytes())
  }

  /// Appends a 24-bit value, most significant byte first. A value of more than 24 bits is the writer's error.
  pub fn big_endian_u24(&mut self, value: u32) -> &mut ByteWriter {
    match value.to_be_bytes().split_first() {
      Some((0, low)) => self.bytes(low),
      _ => self.too_wide(value),
    }
  }

  /// Appends a 32-bit value, most significant byte first.
  pub fn big_endian_u32(&mut self, value: u32) -> &mut ByteWriter {
    self.bytes(&value.to_be_bytes())
  }

  /// Appends a 64-bit value, most significant byte first.
  pub fn big_endian_u64(&mut self, value: u64) -> &mut ByteWriter {
    self.bytes(&value.to_be_bytes())
  }

  /// Appends a 16-bit value, least significant byte first.
  pub fn little_endian_u16(&mut self, value: u16) -> &mut ByteWriter {
    self.bytes(&value.to_le_bytes())
  }

  /// Appends a 24-bit value, least significant byte first. A value of more than 24 bits is the writer's error.
  pub fn little_endian_u24(&mut self, value: u32) -> &mut ByteWriter {
    match value.to_le_bytes().split_at_checked(TWENTY_FOUR_BIT_BYTES) {
      Some((low, [0])) => self.bytes(low),
      _ => self.too_wide(value),
    }
  }

  /// Appends a 32-bit value, least significant byte first.
  pub fn little_endian_u32(&mut self, value: u32) -> &mut ByteWriter {
    self.bytes(&value.to_le_bytes())
  }

  /// Appends a 64-bit value, least significant byte first.
  pub fn little_endian_u64(&mut self, value: u64) -> &mut ByteWriter {
    self.bytes(&value.to_le_bytes())
  }

  /// The bytes written so far, an error or not.
  #[must_use]
  pub fn as_bytes(&self) -> &[u8] {
    &self.bytes
  }

  /// The bytes written.
  ///
  /// # Errors
  ///
  /// The first value that was wider than its field.
  pub fn into_bytes(self) -> Result<Vec<u8>, BuildError> {
    match self.error {
      Some(error) => Err(error),
      None => Ok(self.bytes),
    }
  }

  /// Records a 24-bit field's value that is wider, unless an error came first.
  fn too_wide(&mut self, value: u32) -> &mut ByteWriter {
    self.error.get_or_insert(BuildError::TooWide {
      value: u64::from(value),
      bits: TWENTY_FOUR_BITS,
    });
    self
  }
}
