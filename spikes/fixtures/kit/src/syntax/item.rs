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

//! The items of a line of bytes: hex bytes, quoted text, and a repeat of the item before it.

use super::constants::{
  DECIMAL_RADIX, DIGITS_PER_BYTE, FIRST_PRINTABLE, HEXADECIMAL_RADIX, LAST_PRINTABLE, MAXIMUM_LENGTH, QUOTE, REPEAT,
};
use super::error::SyntaxErrorKind;

/// Appends the bytes of the items of one line to `output`. An item is written once, or as many times as a repeat
/// after it says: `00 * 4096` is 4,096 zero bytes, and a repeat applies to the one item before it.
pub(crate) fn assemble_items(items: &str, output: &mut Vec<u8>) -> Result<(), SyntaxErrorKind> {
  let mut rest = items.trim_start();
  let mut pending: Option<Vec<u8>> = None;
  while !rest.is_empty() {
    if let Some(after) = rest.strip_prefix(QUOTE) {
      write(output, pending.take(), 1)?;
      let (text, tail) = quoted(after)?;
      pending = Some(text);
      rest = tail.trim_start();
      continue;
    }
    let (word, tail) = next_word(rest);
    rest = tail;
    if word == REPEAT {
      let item = pending.take().ok_or(SyntaxErrorKind::MalformedRepeat)?;
      let (count, tail) = next_word(rest);
      rest = tail;
      write(output, Some(item), repeat_count(count)?)?;
    } else {
      write(output, pending.take(), 1)?;
      pending = Some(hexadecimal_bytes(word)?);
    }
  }
  write(output, pending, 1)
}

/// The bytes of a word of hex digits, two digits to a byte, in either case.
pub(crate) fn hexadecimal_bytes(word: &str) -> Result<Vec<u8>, SyntaxErrorKind> {
  if word.is_empty() || !word.chars().all(|character| character.is_ascii_hexdigit()) {
    return Err(SyntaxErrorKind::NotHexadecimal(word.to_owned()));
  }
  let (pairs, remainder) = word.as_bytes().as_chunks::<DIGITS_PER_BYTE>();
  if !remainder.is_empty() {
    return Err(SyntaxErrorKind::OddDigits(word.to_owned()));
  }
  pairs
    .iter()
    .map(|pair| {
      std::str::from_utf8(pair)
        .ok()
        .and_then(|digits| u8::from_str_radix(digits, HEXADECIMAL_RADIX).ok())
        .ok_or_else(|| SyntaxErrorKind::NotHexadecimal(word.to_owned()))
    })
    .collect()
}

/// Writes `item` `count` times, within the format's bound.
fn write(output: &mut Vec<u8>, item: Option<Vec<u8>>, count: usize) -> Result<(), SyntaxErrorKind> {
  let Some(item) = item else {
    return Ok(());
  };
  let total = item
    .len()
    .checked_mul(count)
    .and_then(|length| length.checked_add(output.len()))
    .filter(|total| *total <= MAXIMUM_LENGTH)
    .ok_or(SyntaxErrorKind::TooLong)?;
  output.reserve(total.saturating_sub(output.len()));
  for _ in 0..count {
    output.extend_from_slice(&item);
  }
  Ok(())
}

/// The bytes of quoted text and the rest of the line after its closing quote.
fn quoted(after_quote: &str) -> Result<(Vec<u8>, &str), SyntaxErrorKind> {
  let end = after_quote.find(QUOTE).ok_or(SyntaxErrorKind::UnterminatedText)?;
  let (text, tail) = after_quote.split_at(end);
  if text.is_empty() {
    return Err(SyntaxErrorKind::EmptyText);
  }
  if !text
    .chars()
    .all(|character| (FIRST_PRINTABLE..=LAST_PRINTABLE).contains(&character))
  {
    return Err(SyntaxErrorKind::TextBeyondAscii);
  }
  Ok((text.as_bytes().to_vec(), tail.strip_prefix(QUOTE).unwrap_or(tail)))
}

/// The count of a repeat: a decimal number of at least 1.
fn repeat_count(word: &str) -> Result<usize, SyntaxErrorKind> {
  if word.is_empty() {
    return Err(SyntaxErrorKind::MalformedRepeat);
  }
  let digits = word.chars().all(|character| character.is_digit(DECIMAL_RADIX));
  digits
    .then(|| word.parse::<usize>().ok())
    .flatten()
    .filter(|count| *count >= 1)
    .ok_or_else(|| SyntaxErrorKind::MalformedCount(word.to_owned()))
}

/// The next word of `text`, which starts at a word, and the rest after the white space that follows it.
fn next_word(text: &str) -> (&str, &str) {
  let end = text.find(char::is_whitespace).unwrap_or(text.len());
  let (word, tail) = text.split_at(end);
  (word, tail.trim_start())
}

#[cfg(test)]
mod tests {
  use super::{SyntaxErrorKind, assemble_items};

  fn assembled(items: &str) -> Result<Vec<u8>, SyntaxErrorKind> {
    let mut output = Vec::new();
    assemble_items(items, &mut output).map(|()| output)
  }

  #[test]
  fn hexadecimal_text_and_repeats_assemble_in_order() {
    assert_eq!(
      assembled("00 0a0B \"CC\" ff * 3").unwrap(),
      [0, 0x0a, 0x0b, b'C', b'C', 0xff, 0xff, 0xff]
    );
    assert_eq!(assembled("\"ab\" * 2 01").unwrap(), b"abab\x01");
    assert_eq!(assembled("\"a # b\"").unwrap(), b"a # b");
  }

  #[test]
  fn a_malformed_item_is_named() {
    assert_eq!(assembled("0g"), Err(SyntaxErrorKind::NotHexadecimal("0g".to_owned())));
    assert_eq!(assembled("abc"), Err(SyntaxErrorKind::OddDigits("abc".to_owned())));
    assert_eq!(assembled("* 2"), Err(SyntaxErrorKind::MalformedRepeat));
    assert_eq!(assembled("00 *"), Err(SyntaxErrorKind::MalformedRepeat));
    assert_eq!(assembled("00 * 2 * 2"), Err(SyntaxErrorKind::MalformedRepeat));
    assert_eq!(
      assembled("00 * 0"),
      Err(SyntaxErrorKind::MalformedCount("0".to_owned()))
    );
    assert_eq!(
      assembled("00 * +2"),
      Err(SyntaxErrorKind::MalformedCount("+2".to_owned()))
    );
    assert_eq!(assembled("00 * 16777217"), Err(SyntaxErrorKind::TooLong));
    assert_eq!(assembled("\"open"), Err(SyntaxErrorKind::UnterminatedText));
    assert_eq!(assembled("\"\""), Err(SyntaxErrorKind::EmptyText));
    assert_eq!(assembled("\"é\""), Err(SyntaxErrorKind::TextBeyondAscii));
  }
}
