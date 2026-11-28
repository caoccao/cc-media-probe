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

//! A writer of bits, as the syntax tables of bitstream specifications lay them out (`docs/design/test-kit.md`
//! section 5), in either order of the bits in a byte:
//! - most significant first, as ITU-T H.264 reads its syntax elements with `read_bits` (section 7.2), each value's
//!   most significant bit first, into the most significant unused bit of its byte;
//! - least significant first, as the Vorbis I specification packs its integers (section 2.1.4), each value's least
//!   significant bit first, into the least significant unused bit of its byte.
//!
//! An Exp-Golomb code (ITU-T H.264, section 9.1) is a string of bits, its leading zeros, a one and the suffix, and
//! the writer writes it bit after bit in the writer's own order.

use super::constants::{BITS_PER_BYTE, MAXIMUM_BITS, SIGNED_CODE_FACTOR};
use super::error::BuildError;

/// The order of the bits in a byte.
///
/// ```
/// use cc_media_probe_testkit::builder::{BitOrder, BitWriter};
///
/// let mut writer = BitWriter::new(BitOrder::LeastSignificantFirst);
/// writer.bits(0b101, 3).bits(0b11, 5);
/// assert_eq!(writer.into_bytes().unwrap(), [0b0001_1101]);
/// assert_eq!(BitOrder::default(), BitOrder::MostSignificantFirst);
/// ```
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum BitOrder {
  /// The first bit written is a byte's most significant, and a value is written from its most significant bit.
  #[default]
  MostSignificantFirst,
  /// The first bit written is a byte's least significant, and a value is written from its least significant bit.
  LeastSignificantFirst,
}

/// Bits written in order. Each method appends and returns the writer, so calls chain. A value wider than its field
/// is recorded as the writer's error, which `into_bytes` gives back instead of the bytes.
///
/// ```
/// use cc_media_probe_testkit::builder::{BitWriter, BuildError};
///
/// // ITU-T H.264, section 9.1: a flag of 1, ue(v) of 3 as 00100 and se(v) of -1 as 011, then zeros to the byte.
/// let mut writer = BitWriter::most_significant_first();
/// writer.flag(true).unsigned_exponential_golomb(3).signed_exponential_golomb(-1);
/// assert!(!writer.is_aligned());
/// writer.align_with_zeros();
/// assert_eq!(writer.into_bytes(), Ok(vec![0b1001_0001, 0b1000_0000]));
///
/// let mut wide = BitWriter::most_significant_first();
/// wide.bits(4, 2).bits(1, 6);
/// assert_eq!(wide.into_bytes(), Err(BuildError::TooWide { value: 4, bits: 2 }));
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct BitWriter {
  /// The order of the bits in a byte.
  order: BitOrder,
  /// The whole bytes written so far.
  bytes: Vec<u8>,
  /// The byte being written.
  current: u8,
  /// The bits of `current` written so far, fewer than 8.
  filled: u32,
  /// The first error, if any.
  error: Option<BuildError>,
}

impl BitWriter {
  /// An empty writer whose bits come most significant first, the default order.
  #[must_use]
  pub fn most_significant_first() -> BitWriter {
    BitWriter::default()
  }

  /// An empty writer whose bits come least significant first.
  #[must_use]
  pub fn least_significant_first() -> BitWriter {
    BitWriter::new(BitOrder::LeastSignificantFirst)
  }

  /// An empty writer in `order`.
  #[must_use]
  pub fn new(order: BitOrder) -> BitWriter {
    let empty = BitWriter::default();
    BitWriter { order, ..empty }
  }

  /// Appends the `count` low bits of `value`, at most 64, in the writer's order. A value of more than `count` bits,
  /// or a count beyond 64, is the writer's error.
  pub fn bits(&mut self, value: u64, count: u32) -> &mut BitWriter {
    if count > MAXIMUM_BITS {
      return self.fail(BuildError::TooManyBits(count));
    }
    if value.checked_shr(count).is_some_and(|rest| rest != 0) {
      return self.fail(BuildError::TooWide { value, bits: count });
    }
    for index in 0..count {
      let position = match self.order {
        BitOrder::MostSignificantFirst => count.saturating_sub(1).saturating_sub(index),
        BitOrder::LeastSignificantFirst => index,
      };
      self.bit(value.checked_shr(position).is_some_and(|shifted| shifted & 1 == 1));
    }
    self
  }

  /// Appends one bit, 1 for `true`.
  pub fn flag(&mut self, value: bool) -> &mut BitWriter {
    self.bit(value);
    self
  }

  /// Appends `value` as `ue(v)`, an unsigned Exp-Golomb code (ITU-T H.264, section 9.1, table 9-2): as many zeros
  /// as the bits of `value + 1` less one, then `value + 1` from its most significant bit, which is the one.
  pub fn unsigned_exponential_golomb(&mut self, value: u64) -> &mut BitWriter {
    self.exponential_golomb(u128::from(value))
  }

  /// Appends `value` as `se(v)`, a signed Exp-Golomb code (ITU-T H.264, section 9.1.1, table 9-3): the code of
  /// `2 × value − 1` for a positive value, and of `−2 × value` otherwise, so that 1 is coded as 1 and −1 as 2.
  pub fn signed_exponential_golomb(&mut self, value: i64) -> &mut BitWriter {
    let wide = i128::from(value);
    let code = if wide > 0 {
      wide.saturating_mul(SIGNED_CODE_FACTOR).saturating_sub(1)
    } else {
      wide.saturating_neg().saturating_mul(SIGNED_CODE_FACTOR)
    };
    self.exponential_golomb(code.unsigned_abs())
  }

  /// Appends zeros up to the end of the byte, if a byte is begun.
  pub fn align_with_zeros(&mut self) -> &mut BitWriter {
    self.align(false)
  }

  /// Appends ones up to the end of the byte, if a byte is begun.
  pub fn align_with_ones(&mut self) -> &mut BitWriter {
    self.align(true)
  }

  /// Whether the bits written end at the end of a byte.
  #[must_use]
  pub fn is_aligned(&self) -> bool {
    self.filled == 0
  }

  /// The bytes written.
  ///
  /// # Errors
  ///
  /// The first value that was wider than its field, or bits that end inside a byte.
  pub fn into_bytes(self) -> Result<Vec<u8>, BuildError> {
    if let Some(error) = self.error {
      return Err(error);
    }
    if !self.is_aligned() {
      return Err(BuildError::UnalignedEnd(self.filled));
    }
    Ok(self.bytes)
  }

  /// Appends the Exp-Golomb code of `code` (ITU-T H.264, section 9.1): `code + 1` has a bit length of `n`, written
  /// after `n − 1` zeros.
  fn exponential_golomb(&mut self, code: u128) -> &mut BitWriter {
    let coded = code.saturating_add(1);
    let length = u128::BITS.saturating_sub(coded.leading_zeros());
    for _ in 1..length {
      self.bit(false);
    }
    for position in (0..length).rev() {
      self.bit(coded.checked_shr(position).is_some_and(|shifted| shifted & 1 == 1));
    }
    self
  }

  /// Appends `value` until the byte ends.
  fn align(&mut self, value: bool) -> &mut BitWriter {
    while !self.is_aligned() {
      self.bit(value);
    }
    self
  }

  /// Appends one bit at the next unused position of the byte in the writer's order.
  fn bit(&mut self, value: bool) {
    if value {
      let position = match self.order {
        BitOrder::MostSignificantFirst => BITS_PER_BYTE.saturating_sub(1).saturating_sub(self.filled),
        BitOrder::LeastSignificantFirst => self.filled,
      };
      self.current |= 1_u8.checked_shl(position).unwrap_or_default();
    }
    self.filled = self.filled.saturating_add(1);
    if self.filled == BITS_PER_BYTE {
      self.bytes.push(self.current);
      self.current = 0;
      self.filled = 0;
    }
  }

  /// Records `error`, unless an error came first.
  fn fail(&mut self, error: BuildError) -> &mut BitWriter {
    self.error.get_or_insert(error);
    self
  }
}
