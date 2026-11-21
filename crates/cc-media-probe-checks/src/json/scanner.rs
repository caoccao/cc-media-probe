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

//! Reading JSON scalars: strings, numbers and words.

use crate::json::constants::{
  BACKSLASH, BACKSPACE, BACKSPACE_ESCAPE, CARRIAGE_RETURN, CARRIAGE_RETURN_ESCAPE, FORM_FEED, FORM_FEED_ESCAPE,
  HEXADECIMAL_RADIX, HIGH_SURROGATE_START, INVALID_JSON, LINE_FEED, LINE_FEED_ESCAPE, LOW_SURROGATE_END,
  LOW_SURROGATE_START, NUMBER_CHARACTERS, QUOTATION_MARK, SOLIDUS, SUPPLEMENTARY_START, SURROGATE_PAYLOAD_BITS, TAB,
  TAB_ESCAPE, UNEXPECTED_END, UNICODE_ESCAPE, UNICODE_ESCAPE_DIGITS,
};
use crate::source_error::SourceError;

/// A position in JSON text, with the line for error messages.
pub struct Scanner {
  characters: Vec<char>,
  position: usize,
  line: usize,
}

impl Scanner {
  /// A scanner at the start of `text`.
  pub fn new(text: &str) -> Self {
    Self {
      characters: text.chars().collect(),
      position: 0,
      line: 1,
    }
  }

  /// The character at the position.
  pub fn peek(&self) -> Option<char> {
    self.characters.get(self.position).copied()
  }

  /// Moves past the character at the position and returns it.
  pub fn advance(&mut self) -> Option<char> {
    let character = self.peek()?;
    self.position = self.position.saturating_add(1);
    if character == LINE_FEED {
      self.line = self.line.saturating_add(1);
    }
    Some(character)
  }

  /// Skips whitespace.
  pub fn skip_whitespace(&mut self) {
    while self.peek().is_some_and(char::is_whitespace) {
      self.advance();
    }
  }

  /// An error at the current line.
  pub fn error(&self, message: &'static str) -> SourceError {
    SourceError {
      line: self.line,
      message,
    }
  }

  /// Moves past `expected`, or fails.
  pub fn expect(&mut self, expected: char) -> Result<(), SourceError> {
    match self.advance() {
      Some(character) if character == expected => Ok(()),
      Some(_) => Err(self.error(INVALID_JSON)),
      None => Err(self.error(UNEXPECTED_END)),
    }
  }

  /// Moves past `word`, or fails.
  pub fn expect_word(&mut self, word: &str) -> Result<(), SourceError> {
    word.chars().try_for_each(|expected| self.expect(expected))
  }

  /// Reads a number as written.
  pub fn number(&mut self) -> String {
    let mut text = String::new();
    while let Some(character) = self.peek().filter(|character| NUMBER_CHARACTERS.contains(*character)) {
      text.push(character);
      self.advance();
    }
    text
  }

  /// Reads a string whose opening quotation mark is at the position.
  pub fn string(&mut self) -> Result<String, SourceError> {
    self.expect(QUOTATION_MARK)?;
    let mut text = String::new();
    loop {
      match self.advance().ok_or_else(|| self.error(UNEXPECTED_END))? {
        QUOTATION_MARK => return Ok(text),
        BACKSLASH => text.push(self.escape()?),
        character => text.push(character),
      }
    }
  }

  fn escape(&mut self) -> Result<char, SourceError> {
    let character = match self.advance().ok_or_else(|| self.error(UNEXPECTED_END))? {
      QUOTATION_MARK => QUOTATION_MARK,
      BACKSLASH => BACKSLASH,
      SOLIDUS => SOLIDUS,
      BACKSPACE_ESCAPE => BACKSPACE,
      FORM_FEED_ESCAPE => FORM_FEED,
      LINE_FEED_ESCAPE => LINE_FEED,
      CARRIAGE_RETURN_ESCAPE => CARRIAGE_RETURN,
      TAB_ESCAPE => TAB,
      UNICODE_ESCAPE => self.unicode_escape()?,
      _ => return Err(self.error(INVALID_JSON)),
    };
    Ok(character)
  }

  fn unicode_escape(&mut self) -> Result<char, SourceError> {
    let first = self.code_unit()?;
    if !(HIGH_SURROGATE_START..LOW_SURROGATE_START).contains(&first) {
      return Ok(char::from_u32(first).unwrap_or(char::REPLACEMENT_CHARACTER));
    }
    self.expect(BACKSLASH)?;
    self.expect(UNICODE_ESCAPE)?;
    let second = self.code_unit()?;
    if !(LOW_SURROGATE_START..=LOW_SURROGATE_END).contains(&second) {
      return Err(self.error(INVALID_JSON));
    }
    let high = first.saturating_sub(HIGH_SURROGATE_START);
    let low = second.saturating_sub(LOW_SURROGATE_START);
    let code_point = SUPPLEMENTARY_START
      .saturating_add(high.checked_shl(SURROGATE_PAYLOAD_BITS).unwrap_or_default())
      .saturating_add(low);
    Ok(char::from_u32(code_point).unwrap_or(char::REPLACEMENT_CHARACTER))
  }

  fn code_unit(&mut self) -> Result<u32, SourceError> {
    let mut digits = String::new();
    for _ in 0..UNICODE_ESCAPE_DIGITS {
      digits.push(self.advance().ok_or_else(|| self.error(UNEXPECTED_END))?);
    }
    u32::from_str_radix(&digits, HEXADECIMAL_RADIX).map_err(|_| self.error(INVALID_JSON))
  }
}

#[cfg(test)]
mod tests {
  use super::Scanner;

  #[test]
  fn reads_strings_with_escapes() {
    let mut scanner = Scanner::new(r#""a\"b\\c\/d\b\f\n\r\té😀""#);
    assert_eq!(scanner.string().unwrap(), "a\"b\\c/d\u{8}\u{c}\n\r\té😀");
  }

  #[test]
  fn rejects_broken_strings() {
    for text in [
      r#""open"#,
      r#""\x""#,
      r#""\u12""#,
      r#""\ud83d""#,
      r#""\ud83dA""#,
      r#""\uzzzz""#,
      "x",
    ] {
      assert!(Scanner::new(text).string().is_err(), "{text}");
    }
  }

  #[test]
  fn reads_numbers_and_words() {
    let mut scanner = Scanner::new("-12.5e+3,");
    assert_eq!(scanner.number(), "-12.5e+3");
    assert_eq!(scanner.peek(), Some(','));
    let mut word = Scanner::new("\n true");
    word.skip_whitespace();
    assert!(word.expect_word("true").is_ok());
    assert!(Scanner::new("tru").expect_word("true").is_err());
    assert_eq!(Scanner::new("\n\nx").error("message").line, 1);
  }
}
