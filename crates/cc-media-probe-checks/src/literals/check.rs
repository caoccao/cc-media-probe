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

//! The findings of the literal check.

use crate::findings::{Finding, Rule};
use crate::lexer::TokenKind;
use crate::literals::constants::{ELLIPSIS, LITERAL_OUTSIDE_CONSTANTS, QUOTED_LITERAL_LENGTH, TEMPLATE_WITH_WORDS};
use crate::literals::constants_module::{
  check_constants_module_content, holds_only_constants, is_named_constants_module,
};
use crate::literals::numbers::is_zero_or_one;
use crate::literals::places::literal_places;
use crate::literals::templates::template_has_words;
use crate::syntax::ParsedSource;

/// Checks one file against R33. `is_exempt_file` is true for test code,
/// benchmarks and example programs; module roots are never constants
/// modules.
#[must_use]
pub fn check_literals(path: &str, parsed: &ParsedSource, is_exempt_file: bool, is_module_root: bool) -> Vec<Finding> {
  if is_exempt_file {
    return Vec::new();
  }
  if !is_module_root && is_named_constants_module(path) {
    return check_constants_module_content(path, parsed);
  }
  if !is_module_root && holds_only_constants(parsed) {
    return Vec::new();
  }
  let places = literal_places(parsed);
  let mut findings = Vec::new();
  for (index, token) in parsed.tokens.iter().enumerate() {
    let TokenKind::Literal(kind) = token.kind else {
      continue;
    };
    let is_exempt = places.is_exempt(index)
      || parsed.is_in_test_block(index)
      || parsed.is_in_attribute(index)
      || is_zero_or_one(&token.text, kind);
    if !is_exempt {
      let quoted = quote(&token.text);
      findings.push(Finding::new(
        path,
        token.line,
        Rule::Constants,
        format!("{LITERAL_OUTSIDE_CONSTANTS}: {quoted}"),
      ));
    }
  }
  for index in places.templates {
    let Some(token) = parsed.token(index) else {
      continue;
    };
    if !parsed.is_in_test_block(index) && template_has_words(&token.text) {
      findings.push(Finding::new(path, token.line, Rule::Constants, TEMPLATE_WITH_WORDS));
    }
  }
  findings.sort();
  findings
}

/// The literal as it is quoted in a finding, cut short when it is long.
fn quote(text: &str) -> String {
  if text.chars().count() <= QUOTED_LITERAL_LENGTH {
    return text.to_owned();
  }
  let shortened: String = text.chars().take(QUOTED_LITERAL_LENGTH).collect();
  format!("{shortened}{ELLIPSIS}")
}

#[cfg(test)]
mod tests {
  use super::check_literals;
  use crate::syntax::ParsedSource;

  fn messages(path: &str, source: &str) -> Vec<String> {
    let parsed = ParsedSource::parse(source).unwrap();
    check_literals(path, &parsed, false, false)
      .into_iter()
      .map(|finding| finding.message)
      .collect()
  }

  #[test]
  fn rejects_literals_outside_constants_modules() {
    let found = messages(
      "src/reader.rs",
      "fn read() -> u8 { let size = 4; let name = \"tracks\"; let marker = 'x'; let bytes = b\"ab\"; size }",
    );
    assert_eq!(found.len(), 4);
    assert!(found.iter().any(|message| message.ends_with(": 4")));
  }

  #[test]
  fn rejects_words_in_templates() {
    let found = messages(
      "src/error.rs",
      "fn describe(element: &str) -> String { format!(\"unexpected end: {element}\") }",
    );
    assert_eq!(found.len(), 1);
    assert!(found[0].starts_with("format template holds text"));
    let named = messages(
      "src/error.rs",
      "fn describe(element: &str) -> String { format!(\"{UNEXPECTED_END}: {element}\") }",
    );
    assert_eq!(named, Vec::<String>::new());
  }

  #[test]
  fn accepts_the_exempt_places() {
    let source = "#[cfg(feature = \"serde\")]\nfn first(values: &[u8]) -> u8 { values.len().min(1).max(0) as u8 + pair.0 }\nconst NAME: &str = env!(\"CARGO_PKG_NAME\");\n#[cfg(test)]\nmod tests {\n    fn sample() -> u8 { 42 }\n}\n";
    assert_eq!(messages("src/reader.rs", source), Vec::<String>::new());
  }

  #[test]
  fn accepts_constants_modules_and_data_tables() {
    assert_eq!(
      messages("src/constants.rs", "pub const LIMIT: u8 = 42;\n"),
      Vec::<String>::new()
    );
    assert_eq!(
      messages("src/signatures.rs", "pub static MAGIC: &[u8] = b\"RIFF\";\n"),
      Vec::<String>::new()
    );
    let code = messages("src/constants.rs", "pub fn limit() -> u8 { 42 }\n");
    assert_eq!(code.len(), 1);
    assert!(code[0].starts_with("a constants module holds only constants"));
  }

  #[test]
  fn skips_exempt_files_and_checks_module_roots() {
    let parsed = ParsedSource::parse("fn sample() -> u8 { 42 }").unwrap();
    assert_eq!(check_literals("tests/sample.rs", &parsed, true, false), Vec::new());
    let root = ParsedSource::parse("pub const LIMIT: u8 = 42;\n").unwrap();
    assert_eq!(check_literals("src/constants/mod.rs", &root, false, true).len(), 1);
  }

  #[test]
  fn quotes_long_literals_shortly() {
    let long = format!("fn text() -> &'static str {{ \"{}\" }}", "word ".repeat(20));
    let found = messages("src/text.rs", &long);
    assert_eq!(found.len(), 1);
    assert!(found[0].ends_with('…'));
  }
}
