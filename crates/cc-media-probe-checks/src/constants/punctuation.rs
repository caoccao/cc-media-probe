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

//! Punctuation characters of Rust syntax.

/// Opens a parenthesized group.
pub const OPENING_PARENTHESIS: char = '(';
/// Closes a parenthesized group.
pub const CLOSING_PARENTHESIS: char = ')';
/// Opens a bracketed group.
pub const OPENING_BRACKET: char = '[';
/// Closes a bracketed group.
pub const CLOSING_BRACKET: char = ']';
/// Opens a braced group.
pub const OPENING_BRACE: char = '{';
/// Closes a braced group.
pub const CLOSING_BRACE: char = '}';
/// The characters that open a group.
pub const OPENING_DELIMITERS: [char; 3] = [OPENING_PARENTHESIS, OPENING_BRACKET, OPENING_BRACE];
/// The characters that close a group.
pub const CLOSING_DELIMITERS: [char; 3] = [CLOSING_PARENTHESIS, CLOSING_BRACKET, CLOSING_BRACE];
/// Ends a statement or an item.
pub const SEMICOLON: char = ';';
/// Separates list elements.
pub const COMMA: char = ',';
/// Separates a name from its type; doubled, it separates path segments.
pub const COLON: char = ':';
/// Starts an attribute; marks raw strings and raw identifiers.
pub const NUMBER_SIGN: char = '#';
/// Marks a macro call and an inner attribute.
pub const EXCLAMATION_MARK: char = '!';
/// Opens a list of generic parameters.
pub const LESS_THAN_SIGN: char = '<';
/// Closes a list of generic parameters.
pub const GREATER_THAN_SIGN: char = '>';
/// Assignment; the first character of `=>`.
pub const EQUALS_SIGN: char = '=';
/// The first character of `->`.
pub const HYPHEN_MINUS: char = '-';
/// Delimits closure parameters.
pub const VERTICAL_LINE: char = '|';
/// Field access and method calls.
pub const FULL_STOP: char = '.';
/// Binds a name to a pattern.
pub const COMMERCIAL_AT: char = '@';
/// Marks a metavariable in a declarative macro.
pub const DOLLAR_SIGN: char = '$';
/// References.
pub const AMPERSAND: char = '&';
/// Starts a character literal or a lifetime.
pub const APOSTROPHE: char = '\'';
/// Separates the words of a snake-case name; allowed in numbers.
pub const UNDERSCORE: char = '_';
