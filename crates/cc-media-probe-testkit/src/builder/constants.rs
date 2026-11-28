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

//! The widths, values and messages of the writers.

/// The bits of a byte.
pub const BITS_PER_BYTE: u32 = 8;
/// The bits of the widest value that `BitWriter::bits` writes at once, a `u64`'s.
pub const MAXIMUM_BITS: u32 = 64;
/// The factor of a signed Exp-Golomb code's magnitude: a positive value `k` has the code `2k − 1`, and any other
/// value `−2k` (ITU-T H.264, section 9.1.1, table 9-3).
pub const SIGNED_CODE_FACTOR: i128 = 2;
/// The bits of a 24-bit field.
pub const TWENTY_FOUR_BITS: u32 = 24;
/// The bytes of a 24-bit field.
pub const TWENTY_FOUR_BIT_BYTES: usize = 3;
/// The bytes of a four-character code.
pub const FOUR_CHARACTER_CODE_LENGTH: usize = 4;

/// The byte that two zero bytes may not be followed by without an escape, at most: the three-byte sequences
/// 0x000000, 0x000001 and 0x000002 may not occur in a NAL unit, nor 0x000003 but as an escape (ITU-T H.264,
/// section 7.4.1).
pub const LARGEST_ESCAPED: u8 = 0x03;
/// `emulation_prevention_three_byte`, the byte inserted after two zero bytes (ITU-T H.264, section 7.4.1).
pub const EMULATION_PREVENTION_THREE_BYTE: u8 = 0x03;
/// The zero bytes in a row after which a byte of at most 0x03 is escaped (ITU-T H.264, section 7.4.1.1).
pub const ZEROS_BEFORE_ESCAPE: usize = 2;

/// The message of a value wider than its field.
pub const TOO_WIDE: &str = "the value does not fit its field of";
/// The unit of a field's width.
pub const BITS: &str = "bits";
/// The message of a field wider than a writer writes at once.
pub const TOO_MANY_BITS: &str = "a field is at most 64 bits wide, not";
/// The message of bits written after the last whole byte.
pub const UNALIGNED_END: &str = "the bits end inside a byte, after this many bits of it:";
