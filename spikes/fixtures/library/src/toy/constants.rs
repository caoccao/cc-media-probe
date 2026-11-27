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

//! The values of the toy format and of its codec (`spikes/input/README.md` sections 2 and 3), and the names of the
//! outcomes as fixtures record them.

/// The signature, `CCToy` and a zero byte.
pub const SIGNATURE: [u8; SIGNATURE_LENGTH] = *b"CCToy\0";
/// The length of the signature, and the offset of the version.
pub const SIGNATURE_LENGTH: usize = 6;
/// The version that the walker reads.
pub const VERSION: u8 = 1;
/// The offset of the field that gives the first element's offset.
pub const FIRST_ELEMENT_FIELD: usize = 8;
/// The length of the file header, and the least offset of the first element.
pub const HEADER_LENGTH: usize = 12;
/// The length of an element's code.
pub const CODE_LENGTH: usize = 4;
/// The length of an element's header: its code and its 32-bit length.
pub const ELEMENT_HEADER_LENGTH: usize = 8;
/// The length of a 32-bit field.
pub const WORD_LENGTH: usize = 4;

/// The code of a stream declaration.
pub const STREAM_CODE: [u8; CODE_LENGTH] = *b"STRM";
/// The code of a packet.
pub const PACKET_CODE: [u8; CODE_LENGTH] = *b"PACK";
/// The code of a title.
pub const TITLE_CODE: [u8; CODE_LENGTH] = *b"TITL";
/// The code of a title reference.
pub const TITLE_REFERENCE_CODE: [u8; CODE_LENGTH] = *b"TREF";
/// The code of a list of child elements.
pub const LIST_CODE: [u8; CODE_LENGTH] = *b"LIST";
/// The code of a companion.
pub const COMPANION_CODE: [u8; CODE_LENGTH] = *b"CMPN";
/// The six codes, one of which detection requires of the first element.
pub const KNOWN_CODES: [[u8; CODE_LENGTH]; 6] = [
  STREAM_CODE,
  PACKET_CODE,
  TITLE_CODE,
  TITLE_REFERENCE_CODE,
  LIST_CODE,
  COMPANION_CODE,
];
/// The codec code of the toy codec, the only codec that gets an assembler.
pub const TOY_CODEC: [u8; CODE_LENGTH] = *b"TCOD";
/// The length of a stream declaration's body: stream number, framing, representation and codec code.
pub const STREAM_BODY_LENGTH: usize = 7;
/// The length of the stream number, framing and representation before the codec code.
pub const STREAM_HEAD_LENGTH: usize = 3;

/// The framing of packet mode.
pub const PACKET_MODE: u8 = 0;
/// The framing of stream mode.
pub const STREAM_MODE: u8 = 1;
/// The code of units with one-byte lengths.
pub const ONE_BYTE_LENGTHS: u8 = 0;
/// The code of units with two-byte lengths.
pub const TWO_BYTE_LENGTHS: u8 = 1;
/// The name of units with one-byte lengths, as fixtures write it.
pub const ONE_BYTE_LENGTHS_NAME: &str = "one_byte_lengths";
/// The name of units with two-byte lengths, as fixtures write it.
pub const TWO_BYTE_LENGTHS_NAME: &str = "two_byte_lengths";
/// The size of a two-byte length field.
pub const TWO_BYTE_LENGTH_SIZE: usize = 2;

/// The length of a unit's kind.
pub const KIND_LENGTH: usize = 1;
/// The kind of a payload unit.
pub const PAYLOAD_KIND: u8 = 0;
/// The kind of the setup.
pub const SETUP_KIND: u8 = 1;
/// The kind of the parameters.
pub const PARAMETERS_KIND: u8 = 2;
/// The length of the setup's body: width, height, parameter count and flags.
pub const SETUP_LENGTH: usize = 6;
/// The offset of the parameter count in the setup's body.
pub const SETUP_COUNT_OFFSET: usize = 4;
/// The offset of the flags in the setup's body.
pub const SETUP_FLAGS_OFFSET: usize = 5;
/// Flag bit 0 of the setup: the parameters cannot be read without decoding.
pub const NEEDS_DECODING_FLAG: u8 = 1;
/// The length of one parameter.
pub const PARAMETER_LENGTH: usize = 2;

/// The outcome of a header that was parsed.
pub const PARSED: &str = "parsed";
/// The outcome of a header whose rest cannot be read without decoding.
pub const PARTIALLY_PARSED_BY_DESIGN: &str = "partially_parsed_by_design";
/// The outcome of a header that needs more bytes.
pub const NEEDS_MORE_BYTES: &str = "needs_more_bytes";
/// The outcome of a packet without a header.
pub const NO_HEADER_PRESENT: &str = "no_header_present";
/// The outcome of a malformed header.
pub const MALFORMED: &str = "malformed";
/// The state of a stream whose input ended after some fields of its header were read.
pub const CUT_SHORT: &str = "partially_parsed_cut_short";
/// The state of a stream whose input ended before any field of its header was read.
pub const NOT_FOUND: &str = "not_found";
/// The state of a stream of another codec than the toy codec, which gets no assembler.
pub const UNSUPPORTED: &str = "unsupported";
/// The outcome of a file that the walker walked to its end.
pub const REPORT: &str = "report";
/// The outcome of a file whose element runs past its end.
pub const TRUNCATED: &str = "truncated";
/// The outcome of bytes that are not a toy file.
pub const UNKNOWN_FORMAT: &str = "unknown_format";
/// The outcome of bytes too few to tell, which can still begin a toy file.
pub const INSUFFICIENT_DATA: &str = "insufficient_data";
