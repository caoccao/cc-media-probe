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

//! The serialized names of the source table's enums and the digits of a raw path (`json-output.md` sections 5.8
//! and 6.1).

/// The serialized name of `RawPathEncoding::UnixBytes`.
pub const UNIX_BYTES_NAME: &str = "unix_bytes";
/// The serialized name of `RawPathEncoding::WindowsWide`.
pub const WINDOWS_WIDE_NAME: &str = "windows_wide";
/// The hexadecimal digits of one byte of a path on Unix, WASI, Hermit, SOLID or Xous.
#[cfg(any(
  unix,
  target_os = "wasi",
  target_os = "hermit",
  target_os = "solid_asp3",
  target_os = "xous"
))]
pub const BYTE_DIGITS: usize = 2;
/// The hexadecimal digits of one 16-bit unit of a Windows path.
#[cfg(windows)]
pub const UNIT_DIGITS: usize = 4;

/// The serialized name of `SourceRole::Primary`.
pub const PRIMARY_NAME: &str = "primary";

/// The serialized name of `SourceOutcome::Opened`.
pub const OPENED_NAME: &str = "opened";
/// The serialized name of `SourceOutcome::NotFound`.
pub const NOT_FOUND_NAME: &str = "not_found";
/// The serialized name of `SourceOutcome::Declined`.
pub const DECLINED_NAME: &str = "declined";
/// The serialized name of `SourceOutcome::LimitReached`.
pub const LIMIT_REACHED_NAME: &str = "limit_reached";
/// The serialized name of `SourceOutcome::Failed`.
pub const FAILED_NAME: &str = "failed";
