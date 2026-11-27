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

/// Bytes written in order. Each method appends and returns the writer, so calls chain.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ByteWriter {
  /// The bytes written so far.
  bytes: Vec<u8>,
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

  /// Appends a 16-bit value, most significant byte first.
  pub fn big_endian_u16(&mut self, value: u16) -> &mut ByteWriter {
    self.bytes(&value.to_be_bytes())
  }

  /// Appends a 32-bit value, most significant byte first.
  pub fn big_endian_u32(&mut self, value: u32) -> &mut ByteWriter {
    self.bytes(&value.to_be_bytes())
  }

  /// Appends bytes as they are.
  pub fn bytes(&mut self, value: &[u8]) -> &mut ByteWriter {
    self.bytes.extend_from_slice(value);
    self
  }

  /// The bytes written so far.
  #[must_use]
  pub fn as_bytes(&self) -> &[u8] {
    &self.bytes
  }

  /// The bytes written.
  #[must_use]
  pub fn into_bytes(self) -> Vec<u8> {
    self.bytes
  }
}
