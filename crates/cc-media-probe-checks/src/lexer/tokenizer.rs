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

//! Turns source text into tokens.

use crate::constants::punctuation::{APOSTROPHE, NUMBER_SIGN};
use crate::lexer::constants::{
  AFTER_NEXT_OFFSET, ASTERISK, BACKSLASH, BYTE_PREFIX, C_STRING_PREFIX, LINE_FEED, NO_PREFIX, QUOTATION_MARK,
  RAW_BYTE_PREFIX, RAW_C_STRING_PREFIX, RAW_PREFIX, SLASH, UNTERMINATED_COMMENT, UNTERMINATED_LITERAL,
};
use crate::lexer::cursor::{Cursor, is_apostrophe, is_identifier_continue, is_identifier_start};
use crate::lexer::literal_scanner::{scan_number, scan_quoted, scan_raw};
use crate::lexer::token::{LiteralKind, Token, TokenKind};
use crate::source_error::SourceError;

/// Tokenizes `source`, skipping whitespace and comments.
///
/// # Errors
///
/// Returns an error for a block comment or literal that is not closed.
pub fn tokenize(source: &str) -> Result<Vec<Token>, SourceError> {
  let mut cursor = Cursor::new(source);
  let mut tokens: Vec<Token> = Vec::new();
  loop {
    if skip_trivia(&mut cursor)?
      && let Some(last) = tokens.last_mut()
    {
      last.joint = false;
    }
    let Some(character) = cursor.peek() else {
      break;
    };
    let line = cursor.line();
    let (kind, text) = scan_token(&mut cursor, character)?;
    tokens.push(Token {
      kind,
      text,
      line,
      joint: true,
    });
  }
  if let Some(last) = tokens.last_mut() {
    last.joint = false;
  }
  Ok(tokens)
}

/// Skips whitespace and comments; returns whether anything was skipped.
fn skip_trivia(cursor: &mut Cursor) -> Result<bool, SourceError> {
  let mut skipped = false;
  loop {
    if cursor.peek().is_some_and(char::is_whitespace) {
      cursor.advance();
    } else if cursor.peek() == Some(SLASH) && cursor.peek_at(1) == Some(SLASH) {
      cursor.advance_while(|character| character != LINE_FEED);
    } else if cursor.peek() == Some(SLASH) && cursor.peek_at(1) == Some(ASTERISK) {
      skip_block_comment(cursor)?;
    } else {
      return Ok(skipped);
    }
    skipped = true;
  }
}

fn skip_block_comment(cursor: &mut Cursor) -> Result<(), SourceError> {
  let line = cursor.line();
  let mut depth: usize = 0;
  loop {
    if cursor.peek() == Some(SLASH) && cursor.peek_at(1) == Some(ASTERISK) {
      cursor.advance();
      cursor.advance();
      depth = depth.saturating_add(1);
    } else if cursor.peek() == Some(ASTERISK) && cursor.peek_at(1) == Some(SLASH) {
      cursor.advance();
      cursor.advance();
      depth = depth.saturating_sub(1);
      if depth == 0 {
        return Ok(());
      }
    } else if cursor.advance().is_none() {
      return Err(SourceError {
        line,
        message: UNTERMINATED_COMMENT,
      });
    }
  }
}

fn scan_token(cursor: &mut Cursor, character: char) -> Result<(TokenKind, String), SourceError> {
  if is_identifier_start(character) {
    scan_word(cursor)
  } else if character.is_ascii_digit() {
    Ok(scan_number(cursor))
  } else if character == QUOTATION_MARK {
    scan_quoted(cursor, LiteralKind::String, NO_PREFIX)
  } else if character == APOSTROPHE {
    scan_apostrophe(cursor)
  } else {
    cursor.advance();
    Ok((TokenKind::Punctuation, character.to_string()))
  }
}

/// Scans an identifier, a keyword, or a literal that starts with a prefix
/// such as `b"…"` or `r#"…"#`.
fn scan_word(cursor: &mut Cursor) -> Result<(TokenKind, String), SourceError> {
  let word = cursor.advance_while(is_identifier_continue);
  let next = cursor.peek();
  let after_next = cursor.peek_at(1);
  let is_raw_identifier =
    word == RAW_PREFIX && next == Some(NUMBER_SIGN) && after_next.is_some_and(is_identifier_start);
  if is_raw_identifier {
    cursor.advance();
    let name = cursor.advance_while(is_identifier_continue);
    return Ok((TokenKind::RawIdentifier, name));
  }
  let starts_raw_string = matches!(next, Some(QUOTATION_MARK | NUMBER_SIGN));
  match word.as_str() {
    RAW_PREFIX if starts_raw_string => scan_raw(cursor, LiteralKind::RawString, &word),
    RAW_BYTE_PREFIX if starts_raw_string => scan_raw(cursor, LiteralKind::RawByteString, &word),
    RAW_C_STRING_PREFIX if starts_raw_string => scan_raw(cursor, LiteralKind::RawCString, &word),
    BYTE_PREFIX if next == Some(QUOTATION_MARK) => scan_quoted(cursor, LiteralKind::ByteString, &word),
    BYTE_PREFIX if next == Some(APOSTROPHE) => scan_quoted(cursor, LiteralKind::Byte, &word),
    C_STRING_PREFIX if next == Some(QUOTATION_MARK) => scan_quoted(cursor, LiteralKind::CString, &word),
    _ => Ok((TokenKind::Identifier, word)),
  }
}

/// Scans a character literal or a lifetime; the apostrophe is under the
/// cursor.
fn scan_apostrophe(cursor: &mut Cursor) -> Result<(TokenKind, String), SourceError> {
  match (cursor.peek_at(1), cursor.peek_at(AFTER_NEXT_OFFSET)) {
    (Some(BACKSLASH), _) => scan_quoted(cursor, LiteralKind::Character, NO_PREFIX),
    (Some(_), Some(closing)) if is_apostrophe(closing) => scan_quoted(cursor, LiteralKind::Character, NO_PREFIX),
    (Some(first), _) if is_identifier_start(first) => {
      cursor.advance();
      let name = cursor.advance_while(is_identifier_continue);
      Ok((TokenKind::Lifetime, format!("{APOSTROPHE}{name}")))
    }
    (Some(_), _) => scan_quoted(cursor, LiteralKind::Character, NO_PREFIX),
    (None, _) => Err(SourceError {
      line: cursor.line(),
      message: UNTERMINATED_LITERAL,
    }),
  }
}

#[cfg(test)]
mod tests {
  use super::tokenize;
  use crate::lexer::token::{LiteralKind, TokenKind};

  fn kinds_and_texts(source: &str) -> Vec<(TokenKind, String)> {
    tokenize(source)
      .unwrap()
      .into_iter()
      .map(|token| (token.kind, token.text))
      .collect()
  }

  #[test]
  fn skips_comments_and_tracks_lines() {
    let tokens = tokenize("// line\n/* block /* nested */ */\nfn\n  name").unwrap();
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].text, "fn");
    assert_eq!(tokens[0].line, 3);
    assert_eq!(tokens[1].line, 4);
    assert!(!tokens[0].joint);
  }

  #[test]
  fn marks_joint_tokens() {
    let tokens = tokenize("a::b /**/ c").unwrap();
    let joints: Vec<bool> = tokens.iter().map(|token| token.joint).collect();
    assert_eq!(joints, vec![true, true, true, false, false]);
  }

  #[test]
  fn scans_prefixed_literals_and_raw_identifiers() {
    let tokens = kinds_and_texts(r##"r#type b"x" b'y' c"z" r"w" r#"v"# br"u" cr"t" brand"##);
    let expected = vec![
      (TokenKind::RawIdentifier, "type".to_owned()),
      (TokenKind::Literal(LiteralKind::ByteString), "b\"x\"".to_owned()),
      (TokenKind::Literal(LiteralKind::Byte), "b'y'".to_owned()),
      (TokenKind::Literal(LiteralKind::CString), "c\"z\"".to_owned()),
      (TokenKind::Literal(LiteralKind::RawString), "r\"w\"".to_owned()),
      (TokenKind::Literal(LiteralKind::RawString), "r#\"v\"#".to_owned()),
      (TokenKind::Literal(LiteralKind::RawByteString), "br\"u\"".to_owned()),
      (TokenKind::Literal(LiteralKind::RawCString), "cr\"t\"".to_owned()),
      (TokenKind::Identifier, "brand".to_owned()),
    ];
    assert_eq!(tokens, expected);
  }

  #[test]
  fn separates_lifetimes_from_characters() {
    let tokens = kinds_and_texts(r"'input 'x' '\n' ' ' '_ 'static");
    let expected = vec![
      (TokenKind::Lifetime, "'input".to_owned()),
      (TokenKind::Literal(LiteralKind::Character), "'x'".to_owned()),
      (TokenKind::Literal(LiteralKind::Character), r"'\n'".to_owned()),
      (TokenKind::Literal(LiteralKind::Character), "' '".to_owned()),
      (TokenKind::Lifetime, "'_".to_owned()),
      (TokenKind::Lifetime, "'static".to_owned()),
    ];
    assert_eq!(tokens, expected);
  }

  #[test]
  fn scans_numbers_strings_and_punctuation() {
    let tokens = kinds_and_texts("x.0 + 2.5 - \"s\";");
    let kinds: Vec<TokenKind> = tokens.iter().map(|(kind, _)| *kind).collect();
    assert_eq!(
      kinds,
      vec![
        TokenKind::Identifier,
        TokenKind::Punctuation,
        TokenKind::Literal(LiteralKind::Integer),
        TokenKind::Punctuation,
        TokenKind::Literal(LiteralKind::Float),
        TokenKind::Punctuation,
        TokenKind::Literal(LiteralKind::String),
        TokenKind::Punctuation,
      ]
    );
  }

  #[test]
  fn reports_unterminated_input() {
    assert_eq!(tokenize("/* open").unwrap_err().line, 1);
    assert!(tokenize("\"open").is_err());
    assert!(tokenize("'").is_err());
    assert!(tokenize("r#\"open").is_err());
  }
}
