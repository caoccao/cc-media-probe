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

//! Macro tables, number prefixes and messages of the literal check.

/// Macros whose arguments Rust accepts only as literals.
pub const LITERAL_ONLY_MACROS: &[&str] = &[
  "cfg",
  "column",
  "compile_error",
  "concat",
  "env",
  "file",
  "include_bytes",
  "include_str",
  "line",
  "module_path",
  "option_env",
  "stringify",
];

/// Macros with a format template, and the position of the template among
/// their arguments, counted from zero.
pub const TEMPLATE_MACROS: &[(&str, usize)] = &[
  ("assert", 1),
  ("assert_eq", 2),
  ("assert_ne", 2),
  ("debug_assert", 1),
  ("debug_assert_eq", 2),
  ("debug_assert_ne", 2),
  ("eprint", 0),
  ("eprintln", 0),
  ("format", 0),
  ("format_args", 0),
  ("panic", 0),
  ("print", 0),
  ("println", 0),
  ("todo", 0),
  ("unimplemented", 0),
  ("unreachable", 0),
  ("write", 1),
  ("writeln", 1),
];

/// The prefix of a hexadecimal integer.
pub const HEXADECIMAL_PREFIX: &str = "0x";
/// The prefix of an octal integer.
pub const OCTAL_PREFIX: &str = "0o";
/// The prefix of a binary integer.
pub const BINARY_PREFIX: &str = "0b";
/// The radix of hexadecimal integers.
pub const HEXADECIMAL_RADIX: u32 = 16;
/// The radix of octal integers.
pub const OCTAL_RADIX: u32 = 8;
/// The radix of binary integers.
pub const BINARY_RADIX: u32 = 2;
/// The radix of decimal integers.
pub const DECIMAL_RADIX: u32 = 10;
/// The first letter of an unsigned integer suffix.
pub const UNSIGNED_SUFFIX_START: char = 'u';
/// The first letter of a signed integer suffix.
pub const SIGNED_SUFFIX_START: char = 'i';
/// The first letter of a floating-point suffix.
pub const FLOATING_POINT_SUFFIX_START: char = 'f';
/// The number one as a floating-point value.
pub const FLOATING_POINT_ONE: f64 = 1.0;

/// The escape character of string literals.
pub const BACKSLASH: char = '\\';
/// The escape letter of a hexadecimal byte escape.
pub const HEXADECIMAL_ESCAPE: char = 'x';
/// The escape letter of a Unicode escape.
pub const UNICODE_ESCAPE: char = 'u';
/// The escape letter of a newline.
pub const NEWLINE_ESCAPE: char = 'n';
/// The escape letter of a carriage return.
pub const CARRIAGE_RETURN_ESCAPE: char = 'r';
/// The escape letter of a tab.
pub const TAB_ESCAPE: char = 't';
/// The escape digit of a null character.
pub const NULL_ESCAPE: char = '0';
/// The characters of a hexadecimal byte escape.
pub const HEXADECIMAL_ESCAPE_DIGITS: usize = 2;
/// Stands in for a decoded whitespace escape.
pub const SPACE: char = ' ';
/// The quotation mark around string literals.
pub const QUOTATION_MARK: char = '"';
/// The first letter of a raw string's prefix.
pub const RAW_MARKER: char = 'r';

/// A literal outside a constants module and outside every exempt place.
pub const LITERAL_OUTSIDE_CONSTANTS: &str = "literal outside a constants module; name it as a constant (R33)";
/// A format template with a letter or digit outside its placeholders.
pub const TEMPLATE_WITH_WORDS: &str =
  "format template holds text; move the text into a constant and name it inline (R33)";
/// Something other than a constant in a constants module.
pub const NOT_A_CONSTANT: &str = "a constants module holds only constants, imports and documentation (R33)";
/// The longest part of a literal that a finding quotes.
pub const QUOTED_LITERAL_LENGTH: usize = 40;
/// Marks a quoted literal that was cut short.
pub const ELLIPSIS: &str = "…";
