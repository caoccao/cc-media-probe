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

//! The constants of the source table and its portable paths.

/// The serialized names of `RawPathEncoding` (plan section 8.8).
pub const UNIX_BYTES_NAME: &str = "unix_bytes";
/// The serialized names of `RawPathEncoding` (plan section 8.8).
pub const WINDOWS_WIDE_NAME: &str = "windows_wide";

/// The radix of a raw path's data.
#[cfg(any(unix, windows))]
pub const HEXADECIMAL_RADIX: u32 = 16;
/// The hexadecimal digits of one byte of a Unix path.
#[cfg(any(unix, feature = "serde"))]
pub const BYTE_DIGITS: usize = 2;
/// The hexadecimal digits of one 16-bit unit of a Windows path.
#[cfg(any(windows, feature = "serde"))]
pub const UNIT_DIGITS: usize = 4;
