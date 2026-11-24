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

//! The findings of the name check.

use crate::constants::keywords::UNDERSCORE_NAME;
use crate::constants::punctuation::APOSTROPHE;
use crate::findings::{Finding, Rule};
use crate::lexer::TokenKind;
use crate::names::abbreviations::{ABBREVIATIONS, FIXED_NAMES};
use crate::names::constants::{ABBREVIATED_NAME, SINGLE_LETTER_LIFETIME, SINGLE_LETTER_NAME};
use crate::names::definitions::defined_names;
use crate::names::words::split_words;
use crate::syntax::ParsedSource;

/// Checks one file against R32.
#[must_use]
pub fn check_names(path: &str, parsed: &ParsedSource) -> Vec<Finding> {
  let mut findings = Vec::new();
  for token in &parsed.tokens {
    let message = match token.kind {
      TokenKind::Identifier | TokenKind::RawIdentifier if is_single_letter(&token.text) => SINGLE_LETTER_NAME,
      TokenKind::Lifetime if is_single_letter(token.text.trim_start_matches(APOSTROPHE)) => SINGLE_LETTER_LIFETIME,
      _ => continue,
    };
    let text = &token.text;
    findings.push(Finding::new(
      path,
      token.line,
      Rule::Names,
      format!("{message} `{text}`"),
    ));
  }
  for index in defined_names(parsed) {
    let Some(token) = parsed.token(index) else {
      continue;
    };
    let name = token.text.trim_start_matches(APOSTROPHE);
    if FIXED_NAMES.contains(&name) {
      continue;
    }
    let abbreviation = split_words(name)
      .into_iter()
      .find(|word| ABBREVIATIONS.contains(&word.to_lowercase().as_str()));
    if let Some(word) = abbreviation {
      let text = &token.text;
      findings.push(Finding::new(
        path,
        token.line,
        Rule::Names,
        format!("{ABBREVIATED_NAME}: `{word}` (`{text}`)"),
      ));
    }
  }
  findings
}

fn is_single_letter(text: &str) -> bool {
  text != UNDERSCORE_NAME && text.chars().count() == 1
}

#[cfg(test)]
mod tests {
  use super::check_names;
  use crate::syntax::ParsedSource;

  fn messages(source: &str) -> Vec<String> {
    let parsed = ParsedSource::parse(source).unwrap();
    check_names("src/lib.rs", &parsed)
      .into_iter()
      .map(|finding| finding.message)
      .collect()
  }

  #[test]
  fn rejects_single_letters_everywhere() {
    let found = messages("fn first<T>(value: T) -> T { let x = value; x }");
    assert_eq!(found.len(), 5);
    assert!(found.iter().all(|message| message.starts_with("single-letter name")));
    let lifetime = messages("fn first<'a>(value: &'a str) {}");
    assert_eq!(lifetime.len(), 2);
    assert!(lifetime[0].contains("`'a`"));
  }

  #[test]
  fn rejects_abbreviations_in_defined_names() {
    let found = messages("fn read(header_buf: &[u8]) {}");
    assert_eq!(
      found,
      vec!["abbreviation in a defined name; spell the word out: `buf` (`header_buf`)".to_owned()]
    );
    assert_eq!(messages("struct Packet { header_len: u8 }").len(), 1);
    assert_eq!(messages("fn parse<'input_buf>() {}").len(), 1);
  }

  #[test]
  fn accepts_format_names_and_fixed_names() {
    assert_eq!(
      messages("struct Id3v2Header; fn h264_profile() {}"),
      Vec::<String>::new()
    );
    let display =
      "impl fmt::Display for Unit { fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result { Ok(()) } }";
    assert_eq!(messages(display), Vec::<String>::new());
    assert_eq!(
      messages("impl FromStr for Unit { type Err = Error; fn from_str(text: &str) {} }"),
      Vec::<String>::new()
    );
    // Serde's `Visitor` fixes these method names.
    assert_eq!(
      messages("impl Visitor for Reader { fn visit_str(self, text: &str) {} fn visit_seq(self, items: Items) {} }"),
      Vec::<String>::new()
    );
    assert_eq!(
      messages("impl Visitor for Reader { fn visit_borrowed_str(self, text: &'input str) {} }"),
      Vec::<String>::new()
    );
    assert_eq!(
      messages("impl Visitor for Reader { fn visit_byte_buf(self, value: Vec<u8>) {} }"),
      Vec::<String>::new()
    );
  }

  #[test]
  fn ignores_names_that_are_only_used() {
    assert_eq!(
      messages("use std::io::BufReader; fn open(reader: BufReader) { std::env::args(); }"),
      Vec::<String>::new()
    );
  }
}
