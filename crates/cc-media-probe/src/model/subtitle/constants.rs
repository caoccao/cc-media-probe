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

//! The serialized names of the subtitle types (`json-output.md` sections 6.1 and 6.2).

/// The serialized name of `SubtitleKind::Text`.
pub const TEXT_NAME: &str = "text";
/// The serialized name of `SubtitleKind::Bitmap`.
pub const BITMAP_NAME: &str = "bitmap";

/// The serialized kind of `TextEncoding::Utf8`.
pub const UTF8_KIND: &str = "utf8";
/// The serialized kind of `TextEncoding::Utf16LittleEndian`.
pub const UTF16_LITTLE_ENDIAN_KIND: &str = "utf16_little_endian";
/// The serialized kind of `TextEncoding::Utf16BigEndian`.
pub const UTF16_BIG_ENDIAN_KIND: &str = "utf16_big_endian";
/// The serialized kind of `TextEncoding::Utf32LittleEndian`.
pub const UTF32_LITTLE_ENDIAN_KIND: &str = "utf32_little_endian";
/// The serialized kind of `TextEncoding::Utf32BigEndian`.
pub const UTF32_BIG_ENDIAN_KIND: &str = "utf32_big_endian";
/// The serialized kind of `TextEncoding::Iso8859Part1`.
pub const ISO8859_PART1_KIND: &str = "iso8859_part1";
/// The serialized kind of `TextEncoding::Windows1252`.
pub const WINDOWS1252_KIND: &str = "windows1252";
/// The serialized kind of `TextEncoding::MacRoman`.
pub const MAC_ROMAN_KIND: &str = "mac_roman";
/// The serialized kind of `TextEncoding::Other`, also the name of the member that holds the encoding's name.
pub const OTHER_KIND: &str = "other";
