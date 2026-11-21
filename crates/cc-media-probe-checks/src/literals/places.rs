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

//! The places where a literal may stay inline: arguments that Rust accepts
//! only as literals, `extern` ABI strings, tuple indices, and format
//! templates, which are checked for words instead.

use crate::constants::keywords::EXTERN_KEYWORD;
use crate::constants::punctuation::{
  CLOSING_BRACKET, CLOSING_PARENTHESIS, COMMA, EXCLAMATION_MARK, FULL_STOP, OPENING_BRACE, OPENING_BRACKET,
  OPENING_PARENTHESIS,
};
use crate::lexer::Token;
use crate::literals::constants::{LITERAL_ONLY_MACROS, TEMPLATE_MACROS};
use crate::syntax::ParsedSource;
use crate::syntax::constants::TWO_TOKENS_AHEAD;

/// The literal tokens that may stay inline, and the format templates.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LiteralPlaces {
  /// For every token, whether a literal there is exempt.
  pub exempt: Vec<bool>,
  /// The indices of the format templates.
  pub templates: Vec<usize>,
}

impl LiteralPlaces {
  /// Whether a literal at `index` is exempt.
  #[must_use]
  pub fn is_exempt(&self, index: usize) -> bool {
    self.exempt.get(index).copied().unwrap_or(false)
  }
}

/// Finds the exempt literals and the templates of one file.
#[must_use]
pub fn literal_places(parsed: &ParsedSource) -> LiteralPlaces {
  let tokens = &parsed.tokens;
  let mut places = LiteralPlaces {
    exempt: vec![false; tokens.len()],
    templates: Vec::new(),
  };
  for (index, token) in tokens.iter().enumerate() {
    let next = index.saturating_add(1);
    if token.is_word(EXTERN_KEYWORD) && tokens.get(next).is_some_and(Token::is_string) {
      mark(&mut places, next);
    }
    if token.is_literal() && is_tuple_index(tokens, index) {
      mark(&mut places, index);
    }
    let is_macro_call = token.is_identifier()
      && tokens
        .get(next)
        .is_some_and(|following| following.is_punctuation(EXCLAMATION_MARK));
    if is_macro_call {
      examine_macro_call(parsed, &mut places, index);
    }
  }
  places
}

fn examine_macro_call(parsed: &ParsedSource, places: &mut LiteralPlaces, name_index: usize) {
  let tokens = &parsed.tokens;
  let opening = name_index.saturating_add(TWO_TOKENS_AHEAD);
  let opens_group = tokens.get(opening).is_some_and(|token| {
    [OPENING_PARENTHESIS, OPENING_BRACKET, OPENING_BRACE]
      .iter()
      .any(|character| token.is_punctuation(*character))
  });
  let Some(closing) = parsed.delimiters.partner(opening).filter(|_| opens_group) else {
    return;
  };
  let name = tokens.get(name_index).map_or_default(|token| token.text.as_str());
  if LITERAL_ONLY_MACROS.contains(&name) {
    for index in opening..closing {
      mark(places, index);
    }
    return;
  }
  let Some((_, position)) = TEMPLATE_MACROS.iter().find(|(macro_name, _)| *macro_name == name) else {
    return;
  };
  let arguments = argument_ranges(parsed, opening, closing);
  if let Some(argument) = arguments.get(*position) {
    let is_single_string = argument.len() == 1 && tokens.get(argument.start).is_some_and(Token::is_string);
    if is_single_string {
      mark(places, argument.start);
      places.templates.push(argument.start);
    }
  }
}

/// The token ranges of the comma-separated arguments inside a group.
fn argument_ranges(parsed: &ParsedSource, opening: usize, closing: usize) -> Vec<std::ops::Range<usize>> {
  let mut ranges = Vec::new();
  let mut start = opening.saturating_add(1);
  let mut index = start;
  while index < closing {
    if parsed.token(index).is_some_and(|token| token.is_punctuation(COMMA)) {
      ranges.push(start..index);
      start = index.saturating_add(1);
    }
    index = parsed.delimiters.skip_group(index);
  }
  if start < closing {
    ranges.push(start..closing);
  }
  ranges
}

/// Whether the literal at `index` is a tuple index such as the `0` of
/// `pair.0`.
fn is_tuple_index(tokens: &[Token], index: usize) -> bool {
  let Some(dot_index) = index.checked_sub(1) else {
    return false;
  };
  let Some(dot) = tokens.get(dot_index) else {
    return false;
  };
  let receiver = dot_index.checked_sub(1).and_then(|before| tokens.get(before));
  dot.is_punctuation(FULL_STOP)
    && dot.joint
    && receiver.is_some_and(|token| {
      token.is_identifier()
        || token.is_literal()
        || token.is_punctuation(CLOSING_PARENTHESIS)
        || token.is_punctuation(CLOSING_BRACKET)
    })
}

fn mark(places: &mut LiteralPlaces, index: usize) {
  if let Some(slot) = places.exempt.get_mut(index) {
    *slot = true;
  }
}

#[cfg(test)]
mod tests {
  use super::literal_places;
  use crate::syntax::ParsedSource;

  fn exempt_and_templates(source: &str) -> (Vec<String>, Vec<String>) {
    let parsed = ParsedSource::parse(source).unwrap();
    let places = literal_places(&parsed);
    let exempt = parsed
      .tokens
      .iter()
      .enumerate()
      .filter(|(index, token)| token.is_literal() && places.is_exempt(*index))
      .map(|(_, token)| token.text.clone())
      .collect();
    let templates = places
      .templates
      .iter()
      .map(|index| parsed.tokens[*index].text.clone())
      .collect();
    (exempt, templates)
  }

  #[test]
  fn exempts_literal_only_arguments() {
    let (exempt, templates) = exempt_and_templates(
      "const PATH: &str = concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/data\"); unsafe extern \"C\" {} let pair = (2, 3); pair.1; matrix[2].0;",
    );
    assert_eq!(exempt, vec!["\"CARGO_MANIFEST_DIR\"", "\"/data\"", "\"C\"", "1", "0"]);
    assert_eq!(templates, Vec::<String>::new());
  }

  #[test]
  fn finds_templates_at_their_position() {
    let (exempt, templates) = exempt_and_templates(
      "format!(\"{NAME}: {value}\", value = 2); write!(output, \"{value}\"); assert_eq!(first, second, \"{first}\"); println!(); writeln!(output); format!(TEMPLATE);",
    );
    assert_eq!(templates, vec!["\"{NAME}: {value}\"", "\"{value}\"", "\"{first}\""]);
    assert_eq!(exempt, templates);
  }

  #[test]
  fn ignores_macros_without_groups() {
    let (exempt, templates) = exempt_and_templates("let value = name!; format! \"text\";");
    assert_eq!(exempt, Vec::<String>::new());
    assert_eq!(templates, Vec::<String>::new());
  }
}
