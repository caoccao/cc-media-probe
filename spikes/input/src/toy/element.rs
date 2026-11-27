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

//! `Element`, an element header of the toy format read at a position: its code and the range of its body.

use super::constants::{CODE_LENGTH, ELEMENT_HEADER_LENGTH, WORD_LENGTH};
use crate::utilities::widen;

/// An element: where it starts, its code, and the range of its body.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Element {
  /// The position of the element header.
  pub(crate) position: u64,
  /// The four-character code.
  pub(crate) code: [u8; CODE_LENGTH],
  /// The position of the body.
  pub(crate) body_start: u64,
  /// The position after the body.
  pub(crate) body_end: u64,
}

impl Element {
  /// Reads an element header at `position`, or `None` when its body would end beyond `u64::MAX`.
  pub(crate) fn parse(position: u64, header: [u8; ELEMENT_HEADER_LENGTH]) -> Option<Element> {
    let (code, rest) = header.split_first_chunk::<CODE_LENGTH>()?;
    let length = u32::from_be_bytes(*rest.first_chunk::<WORD_LENGTH>()?);
    let body_start = position.checked_add(widen(ELEMENT_HEADER_LENGTH))?;
    let body_end = body_start.checked_add(u64::from(length))?;
    Some(Element {
      position,
      code: *code,
      body_start,
      body_end,
    })
  }

  /// The length of the body.
  pub(crate) fn body_length(&self) -> u64 {
    self.body_end.saturating_sub(self.body_start)
  }
}
