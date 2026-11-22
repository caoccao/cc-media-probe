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

//! Scanning of number, string and character literals.

use crate::constants::punctuation::{FULL_STOP, HYPHEN_MINUS, NUMBER_SIGN, UNDERSCORE};
use crate::cursor::Cursor;
use crate::lexer::characters::is_identifier_start;
use crate::lexer::constants::{
  AFTER_NEXT_OFFSET, BACKSLASH, BINARY_MARKER, DIGIT_ZERO, FLOATING_POINT_SUFFIX_START, HEXADECIMAL_MARKER,
  LOWER_CASE_EXPONENT, MALFORMED_RAW_STRING, OCTAL_MARKER, PLUS_SIGN, QUOTATION_MARK, UNTERMINATED_LITERAL,
  UPPER_CASE_EXPONENT,
};
use crate::lexer::token::{LiteralKind, TokenKind};
use crate::source_error::SourceError;

/// Scans a number that starts at the cursor.
pub fn scan_number(cursor: &mut Cursor) -> (TokenKind, String) {
  let mut text = cursor.advance().map_or_default(String::from);
  let has_radix_prefix = text.starts_with(DIGIT_ZERO)
    && matches!(cursor.peek(), Some(HEXADECIMAL_MARKER | OCTAL_MARKER | BINARY_MARKER))
    && cursor
      .peek_at(1)
      .is_some_and(|character| character.is_ascii_hexdigit() || character == UNDERSCORE);
  if has_radix_prefix {
    text.push_str(&cursor.advance_while(is_number_character));
    return (TokenKind::Literal(LiteralKind::Integer), text);
  }
  let mut kind = LiteralKind::Integer;
  text.push_str(&cursor.advance_while(is_decimal_character));
  if cursor.peek() == Some(FULL_STOP) {
    match cursor.peek_at(1) {
      Some(next) if next.is_ascii_digit() => {
        cursor.advance();
        text.push(FULL_STOP);
        text.push_str(&cursor.advance_while(is_decimal_character));
        kind = LiteralKind::Float;
      }
      Some(next) if next == FULL_STOP || is_identifier_start(next) => {}
      _ => {
        cursor.advance();
        text.push(FULL_STOP);
        kind = LiteralKind::Float;
      }
    }
  }
  if has_exponent(cursor) {
    text.extend(cursor.advance());
    if matches!(cursor.peek(), Some(PLUS_SIGN | HYPHEN_MINUS)) {
      text.extend(cursor.advance());
    }
    text.push_str(&cursor.advance_while(is_decimal_character));
    kind = LiteralKind::Float;
  }
  let suffix = cursor.advance_while(is_number_character);
  if suffix.starts_with(FLOATING_POINT_SUFFIX_START) {
    kind = LiteralKind::Float;
  }
  text.push_str(&suffix);
  (TokenKind::Literal(kind), text)
}

/// Scans a quoted literal whose opening quote is under the cursor. `prefix`
/// was already consumed and becomes part of the text. The literal ends at
/// the next unescaped copy of its opening quote.
///
/// # Errors
///
/// Returns an error when the input ends before the closing quote.
pub fn scan_quoted(cursor: &mut Cursor, kind: LiteralKind, prefix: &str) -> Result<(TokenKind, String), SourceError> {
  let line = cursor.line();
  let unterminated = SourceError {
    line,
    message: UNTERMINATED_LITERAL,
  };
  let mut text = prefix.to_owned();
  let quote = cursor.advance().ok_or_else(|| unterminated.clone())?;
  text.push(quote);
  loop {
    let character = cursor.advance().ok_or_else(|| unterminated.clone())?;
    text.push(character);
    if character == BACKSLASH {
      text.push(cursor.advance().ok_or_else(|| unterminated.clone())?);
    } else if character == quote {
      return Ok((TokenKind::Literal(kind), text));
    }
  }
}

/// Scans a raw string whose number signs or opening quotation mark are under
/// the cursor. `prefix` was already consumed and becomes part of the text.
///
/// # Errors
///
/// Returns an error when the quotation mark is missing or the input ends
/// before the closing quotation mark and number signs.
pub fn scan_raw(cursor: &mut Cursor, kind: LiteralKind, prefix: &str) -> Result<(TokenKind, String), SourceError> {
  let line = cursor.line();
  let mut text = prefix.to_owned();
  let hashes = cursor.advance_while(|character| character == NUMBER_SIGN);
  text.push_str(&hashes);
  if cursor.advance() != Some(QUOTATION_MARK) {
    return Err(SourceError {
      line,
      message: MALFORMED_RAW_STRING,
    });
  }
  text.push(QUOTATION_MARK);
  loop {
    let character = cursor.advance().ok_or(SourceError {
      line,
      message: UNTERMINATED_LITERAL,
    })?;
    text.push(character);
    if character == QUOTATION_MARK && cursor.starts_with(&hashes) {
      for _ in hashes.chars() {
        text.extend(cursor.advance());
      }
      return Ok((TokenKind::Literal(kind), text));
    }
  }
}

fn has_exponent(cursor: &Cursor) -> bool {
  let is_marker = matches!(cursor.peek(), Some(LOWER_CASE_EXPONENT | UPPER_CASE_EXPONENT));
  let next = cursor.peek_at(1);
  let after_sign = cursor.peek_at(AFTER_NEXT_OFFSET);
  is_marker
    && (next.is_some_and(|character| character.is_ascii_digit())
      || (matches!(next, Some(PLUS_SIGN | HYPHEN_MINUS))
        && after_sign.is_some_and(|character| character.is_ascii_digit())))
}

fn is_decimal_character(character: char) -> bool {
  character.is_ascii_digit() || character == UNDERSCORE
}

fn is_number_character(character: char) -> bool {
  character.is_ascii_alphanumeric() || character == UNDERSCORE
}

#[cfg(test)]
mod tests {
  use super::{scan_number, scan_quoted, scan_raw};
  use crate::cursor::Cursor;
  use crate::lexer::token::{LiteralKind, TokenKind};

  fn number(source: &str) -> (LiteralKind, String, Option<char>) {
    let mut cursor = Cursor::new(source);
    let (kind, text) = scan_number(&mut cursor);
    let TokenKind::Literal(literal) = kind else {
      panic!("not a literal");
    };
    (literal, text, cursor.peek())
  }

  #[test]
  fn scans_integers() {
    assert_eq!(number("42;"), (LiteralKind::Integer, "42".into(), Some(';')));
    assert_eq!(number("0xff_u8 "), (LiteralKind::Integer, "0xff_u8".into(), Some(' ')));
    assert_eq!(number("0b1010"), (LiteralKind::Integer, "0b1010".into(), None));
    assert_eq!(number("0o17"), (LiteralKind::Integer, "0o17".into(), None));
    assert_eq!(number("1_000usize"), (LiteralKind::Integer, "1_000usize".into(), None));
    assert_eq!(number("0..4"), (LiteralKind::Integer, "0".into(), Some('.')));
    assert_eq!(number("7.max(2)"), (LiteralKind::Integer, "7".into(), Some('.')));
  }

  #[test]
  fn scans_floats() {
    assert_eq!(number("2.5"), (LiteralKind::Float, "2.5".into(), None));
    assert_eq!(number("2.;"), (LiteralKind::Float, "2.".into(), Some(';')));
    assert_eq!(number("2."), (LiteralKind::Float, "2.".into(), None));
    assert_eq!(number("1e3"), (LiteralKind::Float, "1e3".into(), None));
    assert_eq!(number("1.5E-3"), (LiteralKind::Float, "1.5E-3".into(), None));
    assert_eq!(number("3e+2"), (LiteralKind::Float, "3e+2".into(), None));
    assert_eq!(number("2f32"), (LiteralKind::Float, "2f32".into(), None));
    assert_eq!(number("2e"), (LiteralKind::Integer, "2e".into(), None));
  }

  #[test]
  fn scans_quoted_literals() {
    let mut cursor = Cursor::new(r#""a\"b" rest"#);
    let result = scan_quoted(&mut cursor, LiteralKind::String, "");
    assert_eq!(
      result,
      Ok((TokenKind::Literal(LiteralKind::String), r#""a\"b""#.into()))
    );
    let mut character = Cursor::new(r"'\''");
    let result = scan_quoted(&mut character, LiteralKind::Character, "");
    assert_eq!(result, Ok((TokenKind::Literal(LiteralKind::Character), r"'\''".into())));
    let mut byte_string = Cursor::new("\"xy\"");
    let result = scan_quoted(&mut byte_string, LiteralKind::ByteString, "b");
    assert_eq!(
      result,
      Ok((TokenKind::Literal(LiteralKind::ByteString), "b\"xy\"".into()))
    );
  }

  #[test]
  fn reports_unterminated_quoted_literals() {
    assert!(scan_quoted(&mut Cursor::new("\"open"), LiteralKind::String, "").is_err());
    assert!(scan_quoted(&mut Cursor::new("\"open\\"), LiteralKind::String, "").is_err());
    assert!(scan_quoted(&mut Cursor::new(""), LiteralKind::String, "").is_err());
  }

  #[test]
  fn scans_raw_strings() {
    let mut cursor = Cursor::new(r###"##"a "# b"## rest"###);
    let result = scan_raw(&mut cursor, LiteralKind::RawString, "r");
    assert_eq!(
      result,
      Ok((
        TokenKind::Literal(LiteralKind::RawString),
        r###"r##"a "# b"##"###.into()
      ))
    );
    assert_eq!(cursor.peek(), Some(' '));
    let mut plain = Cursor::new(r#""x""#);
    assert!(scan_raw(&mut plain, LiteralKind::RawByteString, "br").is_ok());
  }

  #[test]
  fn reports_malformed_raw_strings() {
    assert!(scan_raw(&mut Cursor::new("#x"), LiteralKind::RawString, "r").is_err());
    assert!(scan_raw(&mut Cursor::new("#\"open"), LiteralKind::RawString, "r").is_err());
  }
}
