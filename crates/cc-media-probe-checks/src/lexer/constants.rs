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

//! Characters, prefixes and messages of the tokenizer.

/// Starts and ends a string literal.
pub const QUOTATION_MARK: char = '"';
/// Starts an escape sequence.
pub const BACKSLASH: char = '\\';
/// The slash of a comment.
pub const SLASH: char = '/';
/// The asterisk of a block comment.
pub const ASTERISK: char = '*';
/// The digit that starts a radix prefix.
pub const DIGIT_ZERO: char = '0';
/// The radix marker of hexadecimal numbers.
pub const HEXADECIMAL_MARKER: char = 'x';
/// The radix marker of octal numbers.
pub const OCTAL_MARKER: char = 'o';
/// The radix marker of binary numbers.
pub const BINARY_MARKER: char = 'b';
/// The exponent marker in lower case.
pub const LOWER_CASE_EXPONENT: char = 'e';
/// The exponent marker in upper case.
pub const UPPER_CASE_EXPONENT: char = 'E';
/// A sign of an exponent.
pub const PLUS_SIGN: char = '+';
/// The first letter of a floating-point suffix such as `f64`.
pub const FLOATING_POINT_SUFFIX_START: char = 'f';
/// The offset of the character after the next one.
pub const AFTER_NEXT_OFFSET: usize = 2;
/// No prefix before a quoted literal.
pub const NO_PREFIX: &str = "";
/// The prefix of a raw string or raw identifier.
pub const RAW_PREFIX: &str = "r";
/// The prefix of a byte string or byte literal.
pub const BYTE_PREFIX: &str = "b";
/// The prefix of a C string.
pub const C_STRING_PREFIX: &str = "c";
/// The prefix of a raw byte string.
pub const RAW_BYTE_PREFIX: &str = "br";
/// The prefix of a raw C string.
pub const RAW_C_STRING_PREFIX: &str = "cr";
/// A block comment that is not closed.
pub const UNTERMINATED_COMMENT: &str = "unterminated block comment";
/// A literal that is not closed.
pub const UNTERMINATED_LITERAL: &str = "unterminated literal";
/// A raw string without its opening quotation mark.
pub const MALFORMED_RAW_STRING: &str = "malformed raw string";
