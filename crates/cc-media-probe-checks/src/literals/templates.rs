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

//! Format templates hold only placeholders, spacing and punctuation (R33).

use crate::constants::punctuation::{CLOSING_BRACE, NUMBER_SIGN, OPENING_BRACE};
use crate::literals::constants::{
  BACKSLASH, CARRIAGE_RETURN_ESCAPE, HEXADECIMAL_ESCAPE, HEXADECIMAL_ESCAPE_DIGITS, HEXADECIMAL_RADIX, NEWLINE_ESCAPE,
  NULL_ESCAPE, QUOTATION_MARK, RAW_MARKER, SPACE, TAB_ESCAPE, UNICODE_ESCAPE,
};

/// Whether the string literal `literal` has a letter or digit outside its
/// placeholders. Escapes are decoded first, so `\n` is a newline, not a
/// letter.
#[must_use]
pub fn template_has_words(literal: &str) -> bool {
  let content = string_content(literal);
  let mut characters = content.chars().peekable();
  while let Some(character) = characters.next() {
    if character == OPENING_BRACE {
      if characters.next_if_eq(&OPENING_BRACE).is_none() {
        characters
          .by_ref()
          .take_while(|inner| *inner != CLOSING_BRACE)
          .for_each(drop);
      }
    } else if character.is_alphanumeric() {
      return true;
    }
  }
  false
}

/// The text of a string literal without its quotes, with escapes decoded.
fn string_content(literal: &str) -> String {
  if let Some(raw) = literal.strip_prefix(RAW_MARKER) {
    let hashes = raw.chars().take_while(|character| *character == NUMBER_SIGN).count();
    let inner = raw.get(hashes..).unwrap_or_default();
    let without_hashes = inner.get(..inner.len().saturating_sub(hashes)).unwrap_or_default();
    return trim_quotes(without_hashes).to_owned();
  }
  decode_escapes(trim_quotes(literal))
}

fn trim_quotes(text: &str) -> &str {
  let without_opening = text.strip_prefix(QUOTATION_MARK).unwrap_or(text);
  without_opening.strip_suffix(QUOTATION_MARK).unwrap_or(without_opening)
}

fn decode_escapes(text: &str) -> String {
  let mut decoded = String::new();
  let mut characters = text.chars().peekable();
  while let Some(character) = characters.next() {
    if character != BACKSLASH {
      decoded.push(character);
      continue;
    }
    match characters.next() {
      Some(NEWLINE_ESCAPE | CARRIAGE_RETURN_ESCAPE | TAB_ESCAPE | NULL_ESCAPE) => {
        decoded.push(SPACE);
      }
      Some(HEXADECIMAL_ESCAPE) => {
        let digits: String = characters.by_ref().take(HEXADECIMAL_ESCAPE_DIGITS).collect();
        decoded.extend(hexadecimal_character(&digits));
      }
      Some(UNICODE_ESCAPE) => {
        let digits: String = characters
          .by_ref()
          .skip_while(|inner| *inner == OPENING_BRACE)
          .take_while(|inner| *inner != CLOSING_BRACE)
          .collect();
        decoded.extend(hexadecimal_character(&digits));
      }
      Some(other) if other.is_whitespace() => while characters.next_if(|inner| inner.is_whitespace()).is_some() {},
      Some(other) => decoded.push(other),
      None => {}
    }
  }
  decoded
}

fn hexadecimal_character(digits: &str) -> Option<char> {
  u32::from_str_radix(digits, HEXADECIMAL_RADIX)
    .ok()
    .and_then(char::from_u32)
}

#[cfg(test)]
mod tests {
  use super::template_has_words;

  #[test]
  fn accepts_placeholders_spacing_and_punctuation() {
    for template in [
      r#""{UNEXPECTED_END}: {element}""#,
      r#""{path}:{line}: {rule}: {message}\n""#,
      r#""{{{value:>8}}} ({count} > {MAXIMUM})""#,
      "r#\"{name}: \"{value}\"\"#",
      r#""\t\r\0 - \
                {next}""#,
      r#""""#,
    ] {
      assert!(!template_has_words(template), "{template}");
    }
  }

  #[test]
  fn rejects_text_outside_placeholders() {
    for template in [
      r#""unexpected end: {element}""#,
      r#""{value} 2""#,
      r#""\x41""#,
      r#""\u{1F600}é""#,
      "r#\"text {name}\"#",
      r#""{{literal}}""#,
    ] {
      assert!(template_has_words(template), "{template}");
    }
    assert!(!template_has_words(r#""\u{2014}""#));
    assert!(!template_has_words(r#""\"""#));
  }
}
