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

//! The serialized names of the source table's enums, the digits of a raw path, the outcomes of a refused request
//! and the messages of a table that contradicts itself (`json-output.md` sections 5.8 and 6.1).

/// The serialized name of `RawPathEncoding::UnixBytes`.
pub const UNIX_BYTES_NAME: &str = "unix_bytes";
/// The serialized name of `RawPathEncoding::WindowsWide`.
pub const WINDOWS_WIDE_NAME: &str = "windows_wide";
/// The hexadecimal digits of one byte of a path on Unix, WASI, Hermit, SOLID or Xous.
#[cfg(any(
  feature = "serde",
  unix,
  target_os = "wasi",
  target_os = "hermit",
  target_os = "solid_asp3",
  target_os = "xous"
))]
pub const BYTE_DIGITS: usize = 2;
/// The hexadecimal digits of one 16-bit unit of a Windows path.
#[cfg(any(feature = "serde", windows))]
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

/// The message of raw path data of a known encoding that is not whole units of lowercase hexadecimal digits
/// (`json-output.md` section 5.8).
#[cfg(feature = "serde")]
pub const RAW_PATH_DATA_MESSAGE: &str = "raw path data is not whole units of lowercase hexadecimal digits";

/// The message of a raw form without the lossy flag, or a flag of `false` (`json-output.md` section 5.8).
#[cfg(feature = "serde")]
pub const LOSSY_FLAG_MESSAGE: &str =
  "a path's raw form stands only beside its lossy flag, which is written only as true";

/// The message of a lossy path whose display string holds no U+FFFD.
#[cfg(feature = "serde")]
pub const LOSSY_DISPLAY_MESSAGE: &str = "a lossy path's display string holds no U+FFFD";

/// The message of raw data of a known encoding that is valid Unicode, which no path that is lossy has.
#[cfg(feature = "serde")]
pub const RAW_PATH_UNICODE_MESSAGE: &str = "raw path data of a known encoding is valid Unicode";

/// The message of raw data of a known encoding whose decoding is not the display string.
#[cfg(feature = "serde")]
pub const PATH_DISAGREES_MESSAGE: &str = "a lossy path's raw data does not decode to its display string";

/// The message of a path's flag or raw form without the path's display string.
#[cfg(feature = "serde")]
pub const PATH_MEMBERS_MESSAGE: &str = "a path's flag or raw form stands without its display string";

/// The message of a source entry whose identifier is 0 while it is no primary input, or the reverse, or that is an
/// alias of itself (`api.md` section 6.15).
#[cfg(feature = "serde")]
pub const SOURCE_IDENTITY_MESSAGE: &str = "a source entry's identifier does not fit its role";

/// The message of a source entry whose reason does not fit its role and its outcome (`api.md` section 6.15).
#[cfg(feature = "serde")]
pub const SOURCE_REASON_MESSAGE: &str = "a source entry's reason does not fit its role and its outcome";

/// The message of a source table that names a source twice, lacks the primary input, or holds an alias of a source
/// that it does not name, that is not earlier, that is an alias itself or whose outcome no alias has (`api.md`
/// section 6.15).
#[cfg(feature = "serde")]
pub const SOURCE_TABLE_MESSAGE: &str = "a source table names a source twice, lacks the primary input or holds an alias \
                                        that does not name an earlier source that it was opened again for";

/// The message of a member that names a source that its source table does not hold (`json-output.md` section 10).
#[cfg(feature = "serde")]
pub const UNKNOWN_SOURCE_MESSAGE: &str = "a member names a source that its source table does not hold";

/// The message of source entries that read more bytes than the usage beside them counts (plan section 7.5).
#[cfg(feature = "serde")]
pub const BYTES_READ_MESSAGE: &str = "the sources of a table read more bytes than the usage beside them counts";

/// The message of a reason of a companion's request that its source entry does not keep (`api.md` section 6.15).
#[cfg(feature = "serde")]
pub const REQUEST_REASON_MESSAGE: &str = "a reason of a companion's request disagrees with its source entry";

/// The outcomes of a request that was made and whose companion was not opened: not found, declined or refused by a
/// limit (`api.md` section 13).
#[cfg(feature = "serde")]
pub const REFUSED_OUTCOMES: [super::SourceOutcome; 3] = [
  super::SourceOutcome::NotFound,
  super::SourceOutcome::Declined,
  super::SourceOutcome::LimitReached,
];
