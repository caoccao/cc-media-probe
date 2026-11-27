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

//! The two representations of the toy codec: units with one-byte and with two-byte lengths.

use super::constants::{
  ONE_BYTE_LENGTHS, ONE_BYTE_LENGTHS_NAME, TWO_BYTE_LENGTH_SIZE, TWO_BYTE_LENGTHS, TWO_BYTE_LENGTHS_NAME,
};

/// How long a unit's length field is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Representation {
  /// One byte.
  OneByteLengths,
  /// Two bytes, big-endian.
  TwoByteLengths,
}

impl Representation {
  /// The representation of a stream declaration's code, or of a seed's first byte.
  pub(crate) fn from_code(code: u8) -> Option<Representation> {
    match code {
      ONE_BYTE_LENGTHS => Some(Representation::OneByteLengths),
      TWO_BYTE_LENGTHS => Some(Representation::TwoByteLengths),
      _ => None,
    }
  }

  /// The representation that a fixture names.
  pub(crate) fn from_name(name: &str) -> Option<Representation> {
    match name {
      ONE_BYTE_LENGTHS_NAME => Some(Representation::OneByteLengths),
      TWO_BYTE_LENGTHS_NAME => Some(Representation::TwoByteLengths),
      _ => None,
    }
  }

  /// The code, as a stream declaration and a seed write it.
  pub(crate) fn code(self) -> u8 {
    match self {
      Representation::OneByteLengths => ONE_BYTE_LENGTHS,
      Representation::TwoByteLengths => TWO_BYTE_LENGTHS,
    }
  }

  /// The size of a unit's length field.
  pub(crate) fn length_size(self) -> usize {
    match self {
      Representation::OneByteLengths => 1,
      Representation::TwoByteLengths => TWO_BYTE_LENGTH_SIZE,
    }
  }
}
