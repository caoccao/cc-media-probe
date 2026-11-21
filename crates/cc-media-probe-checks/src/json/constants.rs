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

//! Characters, words and messages of the JSON reader.

/// Opens an object.
pub const OPENING_BRACE: char = '{';
/// Closes an object.
pub const CLOSING_BRACE: char = '}';
/// Opens an array.
pub const OPENING_BRACKET: char = '[';
/// Closes an array.
pub const CLOSING_BRACKET: char = ']';
/// Separates members and elements.
pub const COMMA: char = ',';
/// Separates a member's name from its value.
pub const COLON: char = ':';
/// Delimits strings.
pub const QUOTATION_MARK: char = '"';
/// Starts an escape.
pub const BACKSLASH: char = '\\';
/// The escape of a solidus.
pub const SOLIDUS: char = '/';
/// The escape letter of a backspace.
pub const BACKSPACE_ESCAPE: char = 'b';
/// The escape letter of a form feed.
pub const FORM_FEED_ESCAPE: char = 'f';
/// The escape letter of a line feed.
pub const LINE_FEED_ESCAPE: char = 'n';
/// The escape letter of a carriage return.
pub const CARRIAGE_RETURN_ESCAPE: char = 'r';
/// The escape letter of a tab.
pub const TAB_ESCAPE: char = 't';
/// The escape letter of a Unicode code unit.
pub const UNICODE_ESCAPE: char = 'u';
/// A backspace.
pub const BACKSPACE: char = '\u{8}';
/// A form feed.
pub const FORM_FEED: char = '\u{c}';
/// A line feed.
pub const LINE_FEED: char = '\n';
/// A carriage return.
pub const CARRIAGE_RETURN: char = '\r';
/// A tab.
pub const TAB: char = '\t';
/// The digits of a Unicode escape.
pub const UNICODE_ESCAPE_DIGITS: usize = 4;
/// The radix of a Unicode escape.
pub const HEXADECIMAL_RADIX: u32 = 16;
/// The first high surrogate.
pub const HIGH_SURROGATE_START: u32 = 0xD800;
/// The first low surrogate.
pub const LOW_SURROGATE_START: u32 = 0xDC00;
/// The end of the low surrogates.
pub const LOW_SURROGATE_END: u32 = 0xDFFF;
/// The bits of a surrogate's payload.
pub const SURROGATE_PAYLOAD_BITS: u32 = 10;
/// The first code point above the basic plane.
pub const SUPPLEMENTARY_START: u32 = 0x1_0000;
/// The word `true`.
pub const TRUE_WORD: &str = "true";
/// The word `false`.
pub const FALSE_WORD: &str = "false";
/// The word `null`.
pub const NULL_WORD: &str = "null";
/// A minus sign before a number.
pub const MINUS_SIGN: char = '-';
/// Characters that can occur inside a number.
pub const NUMBER_CHARACTERS: &str = "0123456789+-.eE";
/// Input that is not valid JSON.
pub const INVALID_JSON: &str = "invalid JSON";
/// Input that ends inside a value.
pub const UNEXPECTED_END: &str = "unexpected end of JSON";
/// Input after the value.
pub const TRAILING_CHARACTERS: &str = "characters after the JSON value";
