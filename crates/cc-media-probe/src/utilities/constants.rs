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

//! The constants of the utilities.

/// 2^32, the weight of the high half of a 64-bit integer.
pub const HALF_WORD_SCALE: f64 = 4_294_967_296.0;
/// The radix of hexadecimal text.
#[cfg(any(
  feature = "serde",
  unix,
  windows,
  target_os = "wasi",
  target_os = "hermit",
  target_os = "solid_asp3",
  target_os = "xous"
))]
pub const HEXADECIMAL_RADIX: u32 = 16;

/// The hexadecimal digits of a byte, the form of bytes in the report (`json-output.md` section 5.6).
#[cfg(feature = "serde")]
pub const BYTE_DIGITS: usize = 2;
