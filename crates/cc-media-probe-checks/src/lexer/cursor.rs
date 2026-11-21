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

//! A cursor over the characters of a source text.

use crate::constants::punctuation::{APOSTROPHE, UNDERSCORE};
use crate::lexer::constants::LINE_FEED;

/// Hands out the characters of a source text one at a time and counts lines.
pub struct Cursor {
  characters: Vec<char>,
  position: usize,
  line: usize,
}

impl Cursor {
  /// A cursor at the start of `source`, on line one.
  pub fn new(source: &str) -> Self {
    Self {
      characters: source.chars().collect(),
      position: 0,
      line: 1,
    }
  }

  /// The character under the cursor.
  pub fn peek(&self) -> Option<char> {
    self.peek_at(0)
  }

  /// The character `offset` places after the cursor.
  pub fn peek_at(&self, offset: usize) -> Option<char> {
    let index = self.position.checked_add(offset)?;
    self.characters.get(index).copied()
  }

  /// Whether the characters at the cursor spell `text`.
  pub fn starts_with(&self, text: &str) -> bool {
    text
      .chars()
      .enumerate()
      .all(|(offset, expected)| self.peek_at(offset) == Some(expected))
  }

  /// Moves past the character under the cursor and returns it.
  pub fn advance(&mut self) -> Option<char> {
    let character = self.peek()?;
    self.position = self.position.saturating_add(1);
    if character == LINE_FEED {
      self.line = self.line.saturating_add(1);
    }
    Some(character)
  }

  /// Moves past every character that satisfies `predicate` and returns them.
  pub fn advance_while(&mut self, predicate: impl Fn(char) -> bool) -> String {
    let mut text = String::new();
    while let Some(character) = self.peek() {
      if !predicate(character) {
        break;
      }
      text.push(character);
      self.advance();
    }
    text
  }

  /// The line of the character under the cursor, counted from one.
  pub fn line(&self) -> usize {
    self.line
  }
}

/// Whether `character` can start an identifier.
pub fn is_identifier_start(character: char) -> bool {
  character == UNDERSCORE || character.is_alphabetic()
}

/// Whether `character` can continue an identifier.
pub fn is_identifier_continue(character: char) -> bool {
  character == UNDERSCORE || character.is_alphanumeric()
}

/// Whether `character` closes a character literal.
pub fn is_apostrophe(character: char) -> bool {
  character == APOSTROPHE
}

#[cfg(test)]
mod tests {
  use super::{Cursor, is_apostrophe, is_identifier_continue, is_identifier_start};

  #[test]
  fn advances_and_counts_lines() {
    let mut cursor = Cursor::new("ab\ncd");
    assert_eq!(cursor.peek(), Some('a'));
    assert_eq!(cursor.peek_at(3), Some('c'));
    assert!(cursor.starts_with("ab\n"));
    assert!(!cursor.starts_with("abc"));
    assert_eq!(cursor.advance_while(|character| character != '\n'), "ab");
    assert_eq!(cursor.line(), 1);
    assert_eq!(cursor.advance(), Some('\n'));
    assert_eq!(cursor.line(), 2);
    assert_eq!(cursor.advance_while(char::is_alphabetic), "cd");
    assert_eq!(cursor.advance(), None);
    assert_eq!(cursor.peek_at(usize::MAX), None);
  }

  #[test]
  fn classifies_identifier_characters() {
    assert!(is_identifier_start('_'));
    assert!(is_identifier_start('é'));
    assert!(!is_identifier_start('7'));
    assert!(is_identifier_continue('7'));
    assert!(!is_identifier_continue('-'));
    assert!(is_apostrophe('\''));
  }
}
