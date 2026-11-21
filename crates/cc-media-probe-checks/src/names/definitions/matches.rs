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

//! Bindings in the arms of `match` expressions.

use crate::constants::keywords::{IF_KEYWORD, MATCH_KEYWORD};
use crate::constants::punctuation::{COMMA, OPENING_BRACE, SEMICOLON};
use crate::lexer::Token;
use crate::names::definitions::patterns::pattern_bindings;
use crate::syntax::constants::PAIR_LENGTH;
use crate::syntax::{Delimiters, is_fat_arrow};

/// The bindings of every match arm pattern.
#[must_use]
pub fn match_arm_names(tokens: &[Token], delimiters: &Delimiters) -> Vec<usize> {
  let mut indices = Vec::new();
  for (index, token) in tokens.iter().enumerate() {
    if !token.is_word(MATCH_KEYWORD) {
      continue;
    }
    let Some(opening) = arms_opening(tokens, delimiters, index.saturating_add(1)) else {
      continue;
    };
    let closing = delimiters.partner(opening).unwrap_or(opening);
    indices.extend(arm_bindings(tokens, delimiters, opening, closing));
  }
  indices
}

/// The brace that opens the arms: the first `{` after the scrutinee outside
/// every group.
fn arms_opening(tokens: &[Token], delimiters: &Delimiters, start: usize) -> Option<usize> {
  let mut index = start;
  while let Some(token) = tokens.get(index) {
    if token.is_punctuation(OPENING_BRACE) {
      return Some(index);
    }
    if token.is_punctuation(SEMICOLON) {
      return None;
    }
    index = delimiters.skip_group(index);
  }
  None
}

fn arm_bindings(tokens: &[Token], delimiters: &Delimiters, opening: usize, closing: usize) -> Vec<usize> {
  let mut indices = Vec::new();
  let mut arm_start = opening.saturating_add(1);
  while arm_start < closing {
    let mut arrow = arm_start;
    let mut guard: Option<usize> = None;
    while arrow < closing && !is_fat_arrow(tokens, arrow) {
      if guard.is_none() && tokens.get(arrow).is_some_and(|token| token.is_word(IF_KEYWORD)) {
        guard = Some(arrow);
      }
      arrow = delimiters.skip_group(arrow);
    }
    indices.extend(pattern_bindings(tokens, arm_start..guard.unwrap_or(arrow)));
    arm_start = arm_end(tokens, delimiters, arrow.saturating_add(PAIR_LENGTH), closing);
  }
  indices
}

/// The index after the body of an arm that starts at `body`.
fn arm_end(tokens: &[Token], delimiters: &Delimiters, body: usize, closing: usize) -> usize {
  let is_comma = |index: usize| tokens.get(index).is_some_and(|token| token.is_punctuation(COMMA));
  if tokens
    .get(body)
    .is_some_and(|token| token.is_punctuation(OPENING_BRACE))
  {
    let after = delimiters.skip_group(body);
    return if is_comma(after) {
      after.saturating_add(1)
    } else {
      after
    };
  }
  let mut index = body;
  while index < closing && !is_comma(index) {
    index = delimiters.skip_group(index);
  }
  index.saturating_add(1)
}

#[cfg(test)]
mod tests {
  use super::match_arm_names;
  use crate::lexer::{Token, tokenize};
  use crate::syntax::match_delimiters;

  fn texts(source: &str) -> Vec<String> {
    let tokens: Vec<Token> = tokenize(source).unwrap();
    let delimiters = match_delimiters(&tokens).unwrap();
    match_arm_names(&tokens, &delimiters)
      .into_iter()
      .map(|index| tokens[index].text.clone())
      .collect()
  }

  #[test]
  fn finds_arm_bindings() {
    let source = "match packet { Packet { header, .. } if header > 2 => { header } Kind::Single(value) => value, other => Shape { side: other }, Kind::Empty | Kind::Unit => 0, }";
    assert_eq!(texts(source), vec!["header", "value", "other"]);
  }

  #[test]
  fn skips_the_scrutinee_and_nested_matches() {
    let source = "match (first, Unit {}) { (left, right) => match left { inner => inner }, }";
    assert_eq!(texts(source), vec!["left", "right", "inner"]);
    assert_eq!(texts("let matched = match;"), Vec::<String>::new());
  }
}
