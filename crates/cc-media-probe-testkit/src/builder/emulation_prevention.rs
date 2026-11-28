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

//! The escape of a NAL unit's payload against start-code emulation (ITU-T H.264, section 7.4.1, with the encoder's
//! procedure of section 7.4.1.1): an `emulation_prevention_three_byte`, 0x03, is inserted after each two zero bytes
//! in a row that a byte of at most 0x03 follows, and after a payload that ends with a zero byte. The zero byte that
//! follows an inserted 0x03 counts as the first of the next two, so five zero bytes become
//! `00 00 03 00 00 03 00 03`. A payload can end with a zero byte only after the pairs of zero bytes of its
//! `cabac_zero_word`s (section 7.4.2.10), each of which the NAL unit then holds as `00 00 03`; the NAL unit syntax
//! of section 7.3.1 reads any other payload that ends with a zero byte back with the final 0x03.

use super::constants::{EMULATION_PREVENTION_THREE_BYTE, LARGEST_ESCAPED, ZEROS_BEFORE_ESCAPE};

/// The bytes of a NAL unit's payload, the bytes after its header, with each `emulation_prevention_three_byte` that
/// the payload needs inserted (ITU-T H.264, section 7.4.1.1, step 1).
///
/// ```
/// use cc_media_probe_testkit::builder::insert_emulation_prevention;
///
/// assert_eq!(insert_emulation_prevention(&[0, 0, 1, 0, 0, 4]), [0, 0, 3, 1, 0, 0, 4]);
/// assert_eq!(insert_emulation_prevention(&[0, 0, 0, 0, 0]), [0, 0, 3, 0, 0, 3, 0, 3]);
/// ```
#[must_use]
pub fn insert_emulation_prevention(payload: &[u8]) -> Vec<u8> {
  let mut escaped = Vec::with_capacity(payload.len());
  let mut zeros = 0_usize;
  for byte in payload {
    if zeros >= ZEROS_BEFORE_ESCAPE && *byte <= LARGEST_ESCAPED {
      escaped.push(EMULATION_PREVENTION_THREE_BYTE);
      zeros = 0;
    }
    escaped.push(*byte);
    zeros = if *byte == 0 { zeros.saturating_add(1) } else { 0 };
  }
  if payload.last() == Some(&0) {
    escaped.push(EMULATION_PREVENTION_THREE_BYTE);
  }
  escaped
}
