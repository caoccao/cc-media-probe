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

//! Why a writer or a builder could not give its bytes: a value wider than its field, or bits that end inside a
//! byte. A writer records its first error and gives it back instead of its bytes, so that calls still chain.

use std::error::Error;
use std::fmt;

use super::constants::{BITS, TOO_MANY_BITS, TOO_WIDE, UNALIGNED_END};

/// Why a writer or a builder could not give its bytes.
///
/// ```
/// use cc_media_probe_testkit::builder::{BitWriter, BuildError};
///
/// let mut writer = BitWriter::most_significant_first();
/// writer.bits(1, 3);
/// let error = writer.into_bytes().unwrap_err();
/// assert_eq!(error, BuildError::UnalignedEnd(3));
/// assert_eq!(error.to_string(), "the bits end inside a byte, after this many bits of it: 3");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuildError {
  /// A value that does not fit the bits of its field.
  TooWide {
    /// The value.
    value: u64,
    /// The width of its field, in bits.
    bits: u32,
  },
  /// A field wider than a writer writes at once, with its width in bits.
  TooManyBits(u32),
  /// Bits that end inside a byte, with how many bits of it were written.
  UnalignedEnd(u32),
}

impl fmt::Display for BuildError {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      BuildError::TooWide { value, bits } => write!(formatter, "{value}: {TOO_WIDE} {bits} {BITS}"),
      BuildError::TooManyBits(bits) => write!(formatter, "{TOO_MANY_BITS} {bits}"),
      BuildError::UnalignedEnd(bits) => write!(formatter, "{UNALIGNED_END} {bits}"),
    }
  }
}

impl Error for BuildError {}
