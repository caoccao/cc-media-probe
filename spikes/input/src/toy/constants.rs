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

//! The constants of the toy format, which `README.md` section 2 defines.

/// The length of a four-character code.
pub const CODE_LENGTH: usize = 4;
/// The length of a 32-bit field.
pub const WORD_LENGTH: usize = 4;
/// The length of a 64-bit field.
pub const DOUBLE_WORD_LENGTH: usize = 8;

/// The signature that starts a toy file.
pub const SIGNATURE: [u8; SIGNATURE_LENGTH] = *b"CCToy\0";
/// The length of the signature.
pub const SIGNATURE_LENGTH: usize = 6;
/// The only version of the format.
pub const VERSION: u8 = 1;
/// The length of the version byte and the flags byte.
pub const VERSION_AND_FLAGS_LENGTH: usize = 2;
/// The length of the file header: the signature, the version, the flags and the offset of the first element.
pub const FILE_HEADER_LENGTH: usize = 12;
/// The length of an element header: its code and the length of its body.
pub const ELEMENT_HEADER_LENGTH: usize = 8;

/// A stream declaration: its number, its framing mode, its representation and its codec.
pub const STREAM_CODE: [u8; CODE_LENGTH] = *b"STRM";
/// A packet of one stream: its number, then the payload.
pub const PACKET_CODE: [u8; CODE_LENGTH] = *b"PACK";
/// A title in UTF-8.
pub const TITLE_CODE: [u8; CODE_LENGTH] = *b"TITL";
/// A reference to a title element elsewhere in the file, by its absolute offset.
pub const TITLE_REFERENCE_CODE: [u8; CODE_LENGTH] = *b"TREF";
/// A list, whose body is a sequence of elements.
pub const LIST_CODE: [u8; CODE_LENGTH] = *b"LIST";
/// A companion file, named by an identifier.
pub const COMPANION_CODE: [u8; CODE_LENGTH] = *b"CMPN";
/// The codes that detection accepts as a first element.
pub const KNOWN_CODES: [[u8; CODE_LENGTH]; KNOWN_CODE_COUNT] = [
  STREAM_CODE,
  PACKET_CODE,
  TITLE_CODE,
  TITLE_REFERENCE_CODE,
  LIST_CODE,
  COMPANION_CODE,
];
/// The number of known codes.
pub const KNOWN_CODE_COUNT: usize = 6;

/// The toy codec, the only codec whose header the spike parses.
pub const TOY_CODEC: [u8; CODE_LENGTH] = *b"TCOD";
/// The length of a stream declaration's body.
pub const STREAM_BODY_LENGTH: usize = 7;
/// The length of a stream declaration's number, framing byte and representation byte, which its codec follows.
pub const STREAM_HEAD_LENGTH: usize = 3;
/// The framing byte of a stream in packet mode.
pub const PACKET_FRAMING: u8 = 0;
/// The framing byte of a stream in stream mode.
pub const STREAM_FRAMING: u8 = 1;
/// The representation byte of a stream whose units have one-byte lengths.
pub const ONE_BYTE_LENGTHS: u8 = 0;
/// The representation byte of a stream whose units have two-byte lengths.
pub const TWO_BYTE_LENGTHS: u8 = 1;
/// The length of the stream number at the start of a packet.
pub const STREAM_NUMBER_LENGTH: usize = 1;
/// The longest body of a title or a companion that the walker reads; a longer one is skipped with a diagnostic.
pub const MAXIMUM_TEXT_LENGTH: usize = 255;

/// The longest identifier of a companion.
pub const MAXIMUM_IDENTIFIER_LENGTH: usize = 16;
/// The fixed extension of a companion file, after the period.
pub const COMPANION_EXTENSION: &str = "toyc";
/// The period between an identifier and the extension.
pub const EXTENSION_SEPARATOR: char = '.';
