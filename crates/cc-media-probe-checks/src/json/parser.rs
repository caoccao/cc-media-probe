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

//! Parsing a JSON document with an explicit stack of open containers.

use crate::json::constants::{
  CLOSING_BRACE, CLOSING_BRACKET, COLON, COMMA, FALSE_WORD, INVALID_JSON, MINUS_SIGN, NULL_WORD, OPENING_BRACE,
  OPENING_BRACKET, QUOTATION_MARK, TRAILING_CHARACTERS, TRUE_WORD, UNEXPECTED_END,
};
use crate::json::scanner::Scanner;
use crate::json::value::JsonValue;
use crate::source_error::SourceError;

/// An array or object that is still open.
enum Container {
  Array(Vec<JsonValue>),
  Object(Vec<(String, JsonValue)>, String),
}

/// What happened after a value was attached to its container.
enum Next {
  Value,
  Finished(JsonValue),
}

/// Parses one JSON document.
///
/// # Errors
///
/// Returns an error with its line for text that is not one valid JSON
/// value.
pub fn parse_json(text: &str) -> Result<JsonValue, SourceError> {
  let mut scanner = Scanner::new(text);
  let mut open: Vec<Container> = Vec::new();
  loop {
    scanner.skip_whitespace();
    let value = match scanner.peek().ok_or_else(|| scanner.error(UNEXPECTED_END))? {
      OPENING_BRACE => {
        scanner.advance();
        scanner.skip_whitespace();
        if scanner.peek() == Some(CLOSING_BRACE) {
          scanner.advance();
          JsonValue::Object(Vec::new())
        } else {
          let name = member_name(&mut scanner)?;
          open.push(Container::Object(Vec::new(), name));
          continue;
        }
      }
      OPENING_BRACKET => {
        scanner.advance();
        scanner.skip_whitespace();
        if scanner.peek() == Some(CLOSING_BRACKET) {
          scanner.advance();
          JsonValue::Array(Vec::new())
        } else {
          open.push(Container::Array(Vec::new()));
          continue;
        }
      }
      QUOTATION_MARK => JsonValue::Text(scanner.string()?),
      character if character == MINUS_SIGN || character.is_ascii_digit() => JsonValue::Number(scanner.number()),
      _ => word(&mut scanner)?,
    };
    if let Next::Finished(document) = attach(&mut scanner, &mut open, value)? {
      return Ok(document);
    }
  }
}

/// Attaches `value` to the innermost open container, closing containers as
/// their closing delimiters arrive.
fn attach(scanner: &mut Scanner, open: &mut Vec<Container>, value: JsonValue) -> Result<Next, SourceError> {
  let mut value = value;
  loop {
    let Some(container) = open.last_mut() else {
      scanner.skip_whitespace();
      return match scanner.peek() {
        None => Ok(Next::Finished(value)),
        Some(_) => Err(scanner.error(TRAILING_CHARACTERS)),
      };
    };
    scanner.skip_whitespace();
    let separator = scanner.advance().ok_or_else(|| scanner.error(UNEXPECTED_END))?;
    match container {
      Container::Array(elements) => {
        elements.push(value);
        match separator {
          COMMA => return Ok(Next::Value),
          CLOSING_BRACKET => {}
          _ => return Err(scanner.error(INVALID_JSON)),
        }
      }
      Container::Object(members, name) => {
        members.push((std::mem::take(name), value));
        match separator {
          COMMA => {
            *name = member_name(scanner)?;
            return Ok(Next::Value);
          }
          CLOSING_BRACE => {}
          _ => return Err(scanner.error(INVALID_JSON)),
        }
      }
    }
    value = match open.pop() {
      Some(Container::Array(elements)) => JsonValue::Array(elements),
      Some(Container::Object(members, _)) => JsonValue::Object(members),
      None => return Err(scanner.error(INVALID_JSON)),
    };
  }
}

fn member_name(scanner: &mut Scanner) -> Result<String, SourceError> {
  scanner.skip_whitespace();
  let name = scanner.string()?;
  scanner.skip_whitespace();
  scanner.expect(COLON)?;
  Ok(name)
}

fn word(scanner: &mut Scanner) -> Result<JsonValue, SourceError> {
  for (text, value) in [
    (TRUE_WORD, JsonValue::Boolean(true)),
    (FALSE_WORD, JsonValue::Boolean(false)),
    (NULL_WORD, JsonValue::Null),
  ] {
    if text.starts_with(scanner.peek().unwrap_or_default()) {
      scanner.expect_word(text)?;
      return Ok(value);
    }
  }
  Err(scanner.error(INVALID_JSON))
}

#[cfg(test)]
mod tests {
  use super::parse_json;
  use crate::json::value::JsonValue;

  #[test]
  fn parses_nested_documents() {
    let value =
      parse_json(r#" {"packages": [{"name": "library", "targets": [], "links": null}, 2, true, false], "empty": {}} "#)
        .unwrap();
    let packages = value.get("packages").unwrap().elements();
    assert_eq!(packages.len(), 4);
    assert_eq!(packages[0].get("name").and_then(JsonValue::as_text), Some("library"));
    assert!(packages[0].get("links").unwrap().is_null());
    assert_eq!(packages[1], JsonValue::Number("2".to_owned()));
    assert_eq!(packages[2], JsonValue::Boolean(true));
    assert_eq!(value.get("empty"), Some(&JsonValue::Object(Vec::new())));
  }

  #[test]
  fn parses_deep_nesting_without_recursion() {
    let depth = 100_000;
    let text = format!("{}{}", "[".repeat(depth), "]".repeat(depth));
    assert!(parse_json(&text).is_ok());
  }

  #[test]
  fn rejects_invalid_documents() {
    for text in [
      "",
      "[1,",
      "[1 2]",
      "{\"a\" 1}",
      "{\"a\": 1 \"b\"}",
      "{1: 2}",
      "nul",
      "@",
      "[] x",
      "{\"a\": 1,}",
      "{\"a\":1]",
    ] {
      assert!(parse_json(text).is_err(), "{text}");
    }
  }
}
