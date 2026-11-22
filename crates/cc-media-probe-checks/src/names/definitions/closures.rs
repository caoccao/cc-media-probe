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

//! Closure parameters: the patterns between the vertical lines of a closure.

use crate::constants::keywords::{MOVE_KEYWORD, RETURN_KEYWORD};
use crate::constants::punctuation::{
  COLON, COMMA, EQUALS_SIGN, EXCLAMATION_MARK, GREATER_THAN_SIGN, SEMICOLON, VERTICAL_LINE,
};
use crate::lexer::Token;
use crate::names::definitions::patterns::{is_single_colon, pattern_bindings};
use crate::syntax::constants::PAIR_LENGTH;

/// The bindings of every closure parameter.
#[must_use]
pub fn closure_parameter_names(tokens: &[Token]) -> Vec<usize> {
  let mut indices = Vec::new();
  let mut index = 0;
  while let Some(token) = tokens.get(index) {
    if token.is_punctuation(VERTICAL_LINE) && opens_closure(tokens, index) {
      let closing = closing_line(tokens, index);
      indices.extend(parameters_between(tokens, index, closing));
      index = closing;
    }
    index = index.saturating_add(1);
  }
  indices
}

/// Whether the vertical line at `index` can only start closure parameters,
/// judged by the token before it.
fn opens_closure(tokens: &[Token], index: usize) -> bool {
  let Some(previous) = index.checked_sub(1).and_then(|before| tokens.get(before)) else {
    return true;
  };
  let follows_punctuation =
    previous.opens_group() || previous.is_any_punctuation(&[COMMA, SEMICOLON, COLON, EQUALS_SIGN, EXCLAMATION_MARK]);
  let follows_fat_arrow = previous.is_punctuation(GREATER_THAN_SIGN)
    && index
      .checked_sub(PAIR_LENGTH)
      .and_then(|before| tokens.get(before))
      .is_some_and(|equals| equals.joint && equals.is_punctuation(EQUALS_SIGN));
  follows_punctuation || follows_fat_arrow || previous.is_word(MOVE_KEYWORD) || previous.is_word(RETURN_KEYWORD)
}

/// The index of the vertical line that closes the parameters opened at
/// `opening`, outside every group.
fn closing_line(tokens: &[Token], opening: usize) -> usize {
  let mut depth: usize = 0;
  let mut index = opening.saturating_add(1);
  while let Some(token) = tokens.get(index) {
    if token.opens_group() {
      depth = depth.saturating_add(1);
    } else if token.closes_group() {
      depth = depth.saturating_sub(1);
    } else if depth == 0 && token.is_punctuation(VERTICAL_LINE) {
      return index;
    }
    index = index.saturating_add(1);
  }
  index
}

fn parameters_between(tokens: &[Token], opening: usize, closing: usize) -> Vec<usize> {
  let mut indices = Vec::new();
  let mut start = opening.saturating_add(1);
  let mut depth: usize = 0;
  let mut pattern_end: Option<usize> = None;
  let mut index = start;
  while index <= closing {
    let Some(token) = tokens.get(index) else {
      break;
    };
    if token.opens_group() {
      depth = depth.saturating_add(1);
    } else if token.closes_group() {
      depth = depth.saturating_sub(1);
    } else if depth == 0 && (index == closing || token.is_punctuation(COMMA)) {
      indices.extend(pattern_bindings(tokens, start..pattern_end.unwrap_or(index)));
      start = index.saturating_add(1);
      pattern_end = None;
    } else if depth == 0 && pattern_end.is_none() && is_single_colon(tokens, index) {
      pattern_end = Some(index);
    }
    index = index.saturating_add(1);
  }
  indices
}

#[cfg(test)]
mod tests {
  use super::closure_parameter_names;
  use crate::lexer::{Token, tokenize};

  fn texts(source: &str) -> Vec<String> {
    let tokens: Vec<Token> = tokenize(source).unwrap();
    closure_parameter_names(&tokens)
      .into_iter()
      .map(|index| tokens[index].text.clone())
      .collect()
  }

  #[test]
  fn finds_closure_parameters() {
    assert_eq!(
      texts(
        "items.map(|item| item).fold(0, |total, (first, _): (u8, u8)| total); let call = move |value: Vec<u8>| value; || done;"
      ),
      vec!["item", "total", "first", "value"]
    );
    assert_eq!(texts("match value { Kind::First => |inner| inner }"), vec!["inner"]);
  }

  #[test]
  fn ignores_bitwise_or_and_pattern_alternatives() {
    assert_eq!(
      texts("let flags = first | second; matches!(kind, Kind::First | Kind::Second);"),
      Vec::<String>::new()
    );
    assert_eq!(texts("|open"), Vec::<String>::new());
  }
}
