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

//! Helpers that several modules of the library use: checked arithmetic, exact conversions and hexadecimal text
//! (plan section 7.2).
//!
//! Hexadecimal text is the form of bytes in the report (`json-output.md` section 5.6), so it is compiled with the
//! `serde` feature, and the exact form of a path uses it too, on a platform that has one (`api.md` section 6.15).

mod constants;
mod exact_float;
mod greatest_common_divisor;
#[cfg(any(
  feature = "serde",
  unix,
  windows,
  target_os = "wasi",
  target_os = "hermit",
  target_os = "solid_asp3",
  target_os = "xous"
))]
mod hexadecimal;

pub(crate) use exact_float::{signed_to_float, unsigned_to_float};
pub(crate) use greatest_common_divisor::greatest_common_divisor;
#[cfg(feature = "serde")]
pub(crate) use hexadecimal::{decode_bytes, encode_bytes};
#[cfg(any(
  feature = "serde",
  unix,
  windows,
  target_os = "wasi",
  target_os = "hermit",
  target_os = "solid_asp3",
  target_os = "xous"
))]
pub(crate) use hexadecimal::{decode_units, encode};
