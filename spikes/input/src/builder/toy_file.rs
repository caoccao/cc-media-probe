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

//! `ToyFile`, which writes a toy file element by element.

use super::constants::NO_FLAGS;
use crate::codec::{Framing, Representation};
use crate::toy::constants::{
  CODE_LENGTH, COMPANION_CODE, ELEMENT_HEADER_LENGTH, FILE_HEADER_LENGTH, LIST_CODE, ONE_BYTE_LENGTHS, PACKET_CODE,
  PACKET_FRAMING, SIGNATURE, STREAM_CODE, STREAM_FRAMING, TITLE_CODE, TITLE_REFERENCE_CODE, TOY_CODEC,
  TWO_BYTE_LENGTHS, VERSION, WORD_LENGTH,
};
use crate::utilities::{narrow, widen};

/// A toy file under construction.
#[derive(Clone, Debug)]
pub struct ToyFile {
  bytes: Vec<u8>,
  lists: Vec<usize>,
}

impl Default for ToyFile {
  fn default() -> ToyFile {
    ToyFile::new()
  }
}

impl ToyFile {
  /// A file whose first element follows the file header.
  #[must_use]
  pub fn new() -> ToyFile {
    ToyFile::with_first_element_at(u32::try_from(FILE_HEADER_LENGTH).unwrap_or(u32::MAX))
  }

  /// A file whose first element starts at `offset`, after zero bytes of padding.
  #[must_use]
  pub fn with_first_element_at(offset: u32) -> ToyFile {
    let mut bytes = SIGNATURE.to_vec();
    bytes.push(VERSION);
    bytes.push(NO_FLAGS);
    bytes.extend_from_slice(&offset.to_be_bytes());
    bytes.resize(narrow(u64::from(offset)).max(bytes.len()), 0);
    ToyFile {
      bytes,
      lists: Vec::new(),
    }
  }

  /// The offset at which the next element starts.
  #[must_use]
  pub fn position(&self) -> u64 {
    widen(self.bytes.len())
  }

  /// Any element.
  #[must_use]
  pub fn element(mut self, code: [u8; CODE_LENGTH], body: &[u8]) -> ToyFile {
    self.bytes.extend_from_slice(&code);
    self
      .bytes
      .extend_from_slice(&u32::try_from(body.len()).unwrap_or(u32::MAX).to_be_bytes());
    self.bytes.extend_from_slice(body);
    self
  }

  /// A stream of the toy codec, whose units have one-byte lengths.
  #[must_use]
  pub fn stream(self, number: u8, framing: Framing) -> ToyFile {
    self.declare(number, framing, Representation::OneByteLengths, TOY_CODEC)
  }

  /// A stream of the toy codec in `representation`.
  #[must_use]
  pub fn stream_with_representation(self, number: u8, framing: Framing, representation: Representation) -> ToyFile {
    self.declare(number, framing, representation, TOY_CODEC)
  }

  /// A stream of any codec, with the representation byte of one-byte lengths.
  #[must_use]
  pub fn stream_with_codec(self, number: u8, framing: Framing, codec: [u8; CODE_LENGTH]) -> ToyFile {
    self.declare(number, framing, Representation::OneByteLengths, codec)
  }

  /// A stream declaration with every field given.
  #[must_use]
  pub fn declare(
    self,
    number: u8,
    framing: Framing,
    representation: Representation,
    codec: [u8; CODE_LENGTH],
  ) -> ToyFile {
    let framing = match framing {
      Framing::Packet => PACKET_FRAMING,
      Framing::Stream => STREAM_FRAMING,
    };
    let representation = match representation {
      Representation::OneByteLengths => ONE_BYTE_LENGTHS,
      Representation::TwoByteLengths => TWO_BYTE_LENGTHS,
    };
    let mut body = vec![number, framing, representation];
    body.extend_from_slice(&codec);
    self.element(STREAM_CODE, &body)
  }

  /// A packet of stream `number`.
  #[must_use]
  pub fn packet(self, number: u8, payload: &[u8]) -> ToyFile {
    let mut body = vec![number];
    body.extend_from_slice(payload);
    self.element(PACKET_CODE, &body)
  }

  /// A title.
  #[must_use]
  pub fn title(self, text: &str) -> ToyFile {
    self.element(TITLE_CODE, text.as_bytes())
  }

  /// A reference to the title element at `offset`.
  #[must_use]
  pub fn title_reference(self, offset: u64) -> ToyFile {
    self.element(TITLE_REFERENCE_CODE, &offset.to_be_bytes())
  }

  /// A companion named by `identifier`.
  #[must_use]
  pub fn companion(self, identifier: &str) -> ToyFile {
    self.element(COMPANION_CODE, identifier.as_bytes())
  }

  /// Bytes that belong to no element, such as a list's stray rest that cannot hold a header.
  #[must_use]
  pub fn raw(mut self, bytes: &[u8]) -> ToyFile {
    self.bytes.extend_from_slice(bytes);
    self
  }

  /// Opens a list; its length is written when it is closed.
  #[must_use]
  pub fn open_list(mut self) -> ToyFile {
    self.lists.push(self.bytes.len());
    self.element(LIST_CODE, &[])
  }

  /// Closes the innermost list.
  #[must_use]
  pub fn close_list(mut self) -> ToyFile {
    if let Some(start) = self.lists.pop() {
      let body_start = start.saturating_add(ELEMENT_HEADER_LENGTH);
      let length = u32::try_from(self.bytes.len().saturating_sub(body_start)).unwrap_or(u32::MAX);
      let field = start.saturating_add(CODE_LENGTH);
      if let Some(target) = self.bytes.get_mut(field..field.saturating_add(WORD_LENGTH)) {
        target.copy_from_slice(&length.to_be_bytes());
      }
    }
    self
  }

  /// The bytes of the file.
  #[must_use]
  pub fn build(self) -> Vec<u8> {
    self.bytes
  }
}
