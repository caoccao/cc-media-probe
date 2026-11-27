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

//! A builder of toy files (the draft of the toy format, `spikes/input/README.md` section 2): the header with the
//! offset of the first element computed, and elements whose lengths are computed, each of which a test can set to a
//! wrong value on purpose.

use super::byte_writer::ByteWriter;
use super::constants::{CODE_LENGTH, FIRST_ELEMENT, SIGNATURE, VERSION};

/// One element: a four-character code and a body, with the length computed unless a test sets it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToyElement {
  /// The code.
  code: [u8; CODE_LENGTH],
  /// The body.
  body: Vec<u8>,
  /// The length that a test set in place of the computed one.
  length: Option<u32>,
}

/// Why a toy file could not be built.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildError {
  /// A body longer than its 32-bit length field can state.
  TooLong,
}

impl ToyElement {
  /// An element of `code` with `body`.
  #[must_use]
  pub fn new(code: [u8; CODE_LENGTH], body: Vec<u8>) -> ToyElement {
    ToyElement {
      code,
      body,
      length: None,
    }
  }

  /// The same element with `length` written in place of the computed length.
  #[must_use]
  pub fn with_length(mut self, length: u32) -> ToyElement {
    self.length = Some(length);
    self
  }

  /// Writes the element.
  fn write(&self, writer: &mut ByteWriter) -> Result<(), BuildError> {
    let length = match self.length {
      Some(length) => length,
      None => u32::try_from(self.body.len()).map_err(|_| BuildError::TooLong)?,
    };
    writer.bytes(&self.code).big_endian_u32(length).bytes(&self.body);
    Ok(())
  }
}

/// A toy file of version 1 with `elements` right after its header.
///
/// # Errors
///
/// Returns `TooLong` for a body that its length field cannot state.
pub fn toy_file(elements: &[ToyElement]) -> Result<Vec<u8>, BuildError> {
  let mut writer = ByteWriter::new();
  writer
    .bytes(SIGNATURE)
    .byte(VERSION)
    .byte(0)
    .big_endian_u32(FIRST_ELEMENT);
  for element in elements {
    element.write(&mut writer)?;
  }
  Ok(writer.into_bytes())
}
