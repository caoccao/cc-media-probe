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

//! The serialized names and texts of codec information (`json-output.md` sections 6.1 and 7.3).

/// The display of a codec that the input never named, as a Matroska track entry cut short before its `CodecID`.
pub const UNNAMED_CODEC_TEXT: &str = "a codec that was not read";
/// The display of a codec that the file named and this version does not know, without a raw identifier to show.
pub const UNKNOWN_CODEC_TEXT: &str = "an unknown codec";
/// Between the candidates in the display of an unresolved codec.
pub const CANDIDATE_SEPARATOR: &str = " or ";
/// The display of a candidate that the probe could not read or keep.
pub const MISSING_CANDIDATE_TEXT: &str = "a description that was not read";

/// The serialized name of `CodecNamespace::MatroskaCodecIdentifier`.
pub const MATROSKA_CODEC_IDENTIFIER_NAME: &str = "matroska_codec_identifier";
/// The serialized name of `CodecNamespace::RiffFourCharacterCode`.
pub const RIFF_FOUR_CHARACTER_CODE_NAME: &str = "riff_four_character_code";
/// The serialized name of `CodecNamespace::WaveFormatTag`.
pub const WAVE_FORMAT_TAG_NAME: &str = "wave_format_tag";
/// The serialized name of `CodecNamespace::WaveSubformat`.
pub const WAVE_SUBFORMAT_NAME: &str = "wave_subformat";
/// The serialized name of `CodecNamespace::IsoSampleEntry`.
pub const ISO_SAMPLE_ENTRY_NAME: &str = "iso_sample_entry";
/// The serialized name of `CodecNamespace::Mpeg4ObjectType`.
pub const MPEG4_OBJECT_TYPE_NAME: &str = "mpeg4_object_type";
/// The serialized name of `CodecNamespace::QuickTimeCode`.
pub const QUICKTIME_CODE_NAME: &str = "quicktime_code";

/// The bytes of a four-character code, a sample entry code and a QuickTime code (`json-output.md` section 7.3).
#[cfg(feature = "serde")]
pub const FOUR_CHARACTER_BYTES: usize = 4;

/// The bytes of the `SubFormat` GUID of an extensible WAVE format.
#[cfg(feature = "serde")]
pub const SUBFORMAT_BYTES: usize = 16;

/// The first printable ASCII character, the space (RFC 8794 section 7.4).
#[cfg(feature = "serde")]
pub const PRINTABLE_FIRST: char = ' ';

/// The last printable ASCII character, the tilde.
#[cfg(feature = "serde")]
pub const PRINTABLE_LAST: char = '~';

/// The message of a raw code that is not in the spelling of its namespace (`json-output.md` section 7.3).
#[cfg(feature = "serde")]
pub const RAW_CODE_MESSAGE: &str = "a raw codec identifier is not in the spelling of its namespace";

/// The message of a codec's information that disagrees with its candidates, or of a candidate whose index is 0
/// (`api.md` section 6.4).
#[cfg(feature = "serde")]
pub const CODEC_AGREEMENT_MESSAGE: &str = "a codec disagrees with its candidates, or its candidates do not rise from 1";
