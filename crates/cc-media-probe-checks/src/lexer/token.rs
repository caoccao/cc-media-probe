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

//! Tokens produced by the tokenizer.

use crate::constants::punctuation::{CLOSING_DELIMITERS, OPENING_DELIMITERS};

/// What a token is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenKind {
  /// An identifier or a keyword.
  Identifier,
  /// A raw identifier such as `r#type`; the text holds the name without
  /// its prefix.
  RawIdentifier,
  /// A lifetime or a label, including its apostrophe.
  Lifetime,
  /// A literal.
  Literal(LiteralKind),
  /// A single punctuation character.
  Punctuation,
}

/// The kind of a literal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LiteralKind {
  /// An integer such as `42` or `0xff_u8`.
  Integer,
  /// A floating-point number such as `2.5` or `1e3`.
  Float,
  /// A character such as `'a'`.
  Character,
  /// A byte such as `b'a'`.
  Byte,
  /// A string with escapes.
  String,
  /// A byte string with escapes.
  ByteString,
  /// A C string with escapes.
  CString,
  /// A raw string.
  RawString,
  /// A raw byte string.
  RawByteString,
  /// A raw C string.
  RawCString,
}

/// One token and the line it starts on.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Token {
  /// What the token is.
  pub kind: TokenKind,
  /// The source text of the token.
  pub text: String,
  /// The line on which the token starts, counted from one.
  pub line: usize,
  /// Whether the next token follows without whitespace or a comment.
  pub joint: bool,
}

impl Token {
  /// Whether this token is the identifier or keyword `word`.
  #[must_use]
  pub fn is_word(&self, word: &str) -> bool {
    self.kind == TokenKind::Identifier && self.text == word
  }

  /// Whether this token is one of the identifiers or keywords `words`.
  #[must_use]
  pub fn is_any_word(&self, words: &[&str]) -> bool {
    words.iter().any(|word| self.is_word(word))
  }

  /// Whether this token is the punctuation character `character`.
  #[must_use]
  pub fn is_punctuation(&self, character: char) -> bool {
    self.kind == TokenKind::Punctuation && self.text.chars().eq(std::iter::once(character))
  }

  /// Whether this token is one of the punctuation characters `characters`.
  #[must_use]
  pub fn is_any_punctuation(&self, characters: &[char]) -> bool {
    characters.iter().any(|character| self.is_punctuation(*character))
  }

  /// Whether this token opens a parenthesized, bracketed or braced group.
  #[must_use]
  pub fn opens_group(&self) -> bool {
    self.is_any_punctuation(&OPENING_DELIMITERS)
  }

  /// Whether this token closes a parenthesized, bracketed or braced group.
  #[must_use]
  pub fn closes_group(&self) -> bool {
    self.is_any_punctuation(&CLOSING_DELIMITERS)
  }

  /// Whether this token is an identifier or a raw identifier.
  #[must_use]
  pub fn is_identifier(&self) -> bool {
    matches!(self.kind, TokenKind::Identifier | TokenKind::RawIdentifier)
  }

  /// Whether this token is a literal of any kind.
  #[must_use]
  pub fn is_literal(&self) -> bool {
    matches!(self.kind, TokenKind::Literal(_))
  }

  /// Whether this token is a string literal, raw or not.
  #[must_use]
  pub fn is_string(&self) -> bool {
    matches!(
      self.kind,
      TokenKind::Literal(LiteralKind::String | LiteralKind::RawString)
    )
  }
}

#[cfg(test)]
mod tests {
  use super::{LiteralKind, Token, TokenKind};

  fn token(kind: TokenKind, text: &str) -> Token {
    Token {
      kind,
      text: text.to_owned(),
      line: 1,
      joint: false,
    }
  }

  #[test]
  fn recognizes_words_and_punctuation() {
    assert!(token(TokenKind::Identifier, "fn").is_word("fn"));
    assert!(!token(TokenKind::RawIdentifier, "fn").is_word("fn"));
    assert!(token(TokenKind::Punctuation, ";").is_punctuation(';'));
    assert!(!token(TokenKind::Punctuation, ";").is_punctuation(','));
    assert!(token(TokenKind::RawIdentifier, "type").is_identifier());
  }

  #[test]
  fn recognizes_one_of_several_words_or_characters() {
    assert!(token(TokenKind::Identifier, "fn").is_any_word(&["const", "fn"]));
    assert!(!token(TokenKind::Identifier, "fn").is_any_word(&["const", "static"]));
    assert!(token(TokenKind::Punctuation, ",").is_any_punctuation(&[';', ',']));
    assert!(!token(TokenKind::Punctuation, ":").is_any_punctuation(&[';', ',']));
    for opening in ["(", "[", "{"] {
      assert!(token(TokenKind::Punctuation, opening).opens_group());
      assert!(!token(TokenKind::Punctuation, opening).closes_group());
    }
    for closing in [")", "]", "}"] {
      assert!(token(TokenKind::Punctuation, closing).closes_group());
      assert!(!token(TokenKind::Punctuation, closing).opens_group());
    }
    assert!(!token(TokenKind::Punctuation, "<").opens_group());
  }

  #[test]
  fn recognizes_literals_and_strings() {
    let string = token(TokenKind::Literal(LiteralKind::String), "\"text\"");
    let raw = token(TokenKind::Literal(LiteralKind::RawString), "r\"text\"");
    let byte_string = token(TokenKind::Literal(LiteralKind::ByteString), "b\"text\"");
    assert!(string.is_literal() && string.is_string());
    assert!(raw.is_string());
    assert!(byte_string.is_literal() && !byte_string.is_string());
    assert!(!token(TokenKind::Lifetime, "'input").is_literal());
  }
}
