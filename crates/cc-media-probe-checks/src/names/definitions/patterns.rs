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

//! Bindings in patterns: in `let` (including `if let` and `while let`), in
//! `for` loops, and the shared rule that tells a binding from a path.

use std::ops::Range;

use crate::constants::keywords::{ELSE_KEYWORD, FOR_KEYWORD, IN_KEYWORD, LET_KEYWORD, PATTERN_KEYWORDS};
use crate::constants::punctuation::{
  CLOSING_BRACE, CLOSING_BRACKET, CLOSING_PARENTHESIS, COLON, EQUALS_SIGN, EXCLAMATION_MARK, FULL_STOP,
  GREATER_THAN_SIGN, LESS_THAN_SIGN, OPENING_BRACE, OPENING_BRACKET, OPENING_PARENTHESIS, SEMICOLON,
};
use crate::lexer::{Token, TokenKind};
use crate::syntax::{is_path_separator, previous_is_path_separator};

/// The bindings that the pattern in `range` introduces: lower-case names
/// that are not keywords, not path segments, not called as tuple structs or
/// macros, and not field names of a struct pattern.
#[must_use]
pub fn pattern_bindings(tokens: &[Token], range: Range<usize>) -> Vec<usize> {
  let mut indices = Vec::new();
  let mut brace_depth: usize = 0;
  for index in range {
    let Some(token) = tokens.get(index) else {
      break;
    };
    if token.is_punctuation(OPENING_BRACE) {
      brace_depth = brace_depth.saturating_add(1);
    } else if token.is_punctuation(CLOSING_BRACE) {
      brace_depth = brace_depth.saturating_sub(1);
    }
    if is_binding(tokens, index, brace_depth > 0) {
      indices.push(index);
    }
  }
  indices
}

fn is_binding(tokens: &[Token], index: usize, inside_braces: bool) -> bool {
  let Some(token) = tokens.get(index) else {
    return false;
  };
  if !token.is_identifier() {
    return false;
  }
  let is_keyword = token.kind == TokenKind::Identifier && PATTERN_KEYWORDS.contains(&token.text.as_str());
  let starts_upper_case = token.text.chars().next().is_some_and(char::is_uppercase);
  let next_index = index.saturating_add(1);
  let next = tokens.get(next_index);
  let is_called = next.is_some_and(|following| {
    following.is_punctuation(OPENING_PARENTHESIS)
      || following.is_punctuation(OPENING_BRACE)
      || following.is_punctuation(EXCLAMATION_MARK)
  });
  let is_path_segment = is_path_separator(tokens, next_index) || previous_is_path_separator(tokens, index);
  let follows_full_stop = index
    .checked_sub(1)
    .and_then(|before| tokens.get(before))
    .is_some_and(|previous| previous.is_punctuation(FULL_STOP));
  let is_field_name = inside_braces && is_single_colon(tokens, next_index);
  !(is_keyword || starts_upper_case || is_called || is_path_segment || follows_full_stop || is_field_name)
}

/// Whether the token at `index` is a `:` that is not part of `::`.
#[must_use]
pub fn is_single_colon(tokens: &[Token], index: usize) -> bool {
  tokens.get(index).is_some_and(|token| token.is_punctuation(COLON))
    && !is_path_separator(tokens, index)
    && !index
      .checked_sub(1)
      .is_some_and(|before| is_path_separator(tokens, before))
}

/// The end of a pattern that starts at `start`: the first `=`, `;`, `:`,
/// `else` or `in` outside every group, or the closing of the enclosing
/// group.
#[must_use]
pub fn pattern_end(tokens: &[Token], start: usize) -> usize {
  let mut depth: usize = 0;
  let mut index = start;
  while let Some(token) = tokens.get(index) {
    if [OPENING_PARENTHESIS, OPENING_BRACKET, OPENING_BRACE]
      .iter()
      .any(|character| token.is_punctuation(*character))
    {
      depth = depth.saturating_add(1);
    } else if [CLOSING_PARENTHESIS, CLOSING_BRACKET, CLOSING_BRACE]
      .iter()
      .any(|character| token.is_punctuation(*character))
    {
      if depth == 0 {
        return index;
      }
      depth = depth.saturating_sub(1);
    } else if depth == 0 && ends_pattern(tokens, index) {
      return index;
    }
    index = index.saturating_add(1);
  }
  index
}

fn ends_pattern(tokens: &[Token], index: usize) -> bool {
  let Some(token) = tokens.get(index) else {
    return true;
  };
  let next = tokens.get(index.saturating_add(1));
  let is_assignment = token.is_punctuation(EQUALS_SIGN)
    && !(token.joint
      && next
        .is_some_and(|following| following.is_punctuation(EQUALS_SIGN) || following.is_punctuation(GREATER_THAN_SIGN)));
  is_assignment
    || token.is_punctuation(SEMICOLON)
    || is_single_colon(tokens, index)
    || token.is_word(ELSE_KEYWORD)
    || token.is_word(IN_KEYWORD)
}

/// The bindings of every `let` pattern.
#[must_use]
pub fn let_names(tokens: &[Token]) -> Vec<usize> {
  let mut indices = Vec::new();
  for (index, token) in tokens.iter().enumerate() {
    if token.is_word(LET_KEYWORD) {
      let start = index.saturating_add(1);
      indices.extend(pattern_bindings(tokens, start..pattern_end(tokens, start)));
    }
  }
  indices
}

/// The bindings of every `for` loop; `impl … for` and `for<…>` are skipped.
#[must_use]
pub fn for_loop_names(tokens: &[Token]) -> Vec<usize> {
  let mut indices = Vec::new();
  for (index, token) in tokens.iter().enumerate() {
    let start = index.saturating_add(1);
    let starts_bound = tokens
      .get(start)
      .is_some_and(|next| next.is_punctuation(LESS_THAN_SIGN));
    if !token.is_word(FOR_KEYWORD) || starts_bound {
      continue;
    }
    let end = pattern_end(tokens, start);
    if tokens.get(end).is_some_and(|candidate| candidate.is_word(IN_KEYWORD)) {
      indices.extend(pattern_bindings(tokens, start..end));
    }
  }
  indices
}

#[cfg(test)]
mod tests {
  use super::{for_loop_names, let_names, pattern_end};
  use crate::lexer::{Token, tokenize};

  fn texts(tokens: &[Token], indices: Vec<usize>) -> Vec<String> {
    indices.into_iter().map(|index| tokens[index].text.clone()).collect()
  }

  #[test]
  fn finds_let_bindings() {
    let tokens = tokenize("let (first, mut second): (u8, u8) = pair; let Some(value) = option else { return }; if let Packet { header, length: size, .. } = packet {} let ref name @ Kind::Large = other; let result = value == 2;").unwrap();
    assert_eq!(
      texts(&tokens, let_names(&tokens)),
      vec!["first", "second", "value", "header", "size", "name", "result"]
    );
  }

  #[test]
  fn finds_for_loop_bindings_only() {
    let tokens = tokenize(
      "for (index, item) in items {} impl Trait for Unit {} fn bound() where Function: for<'item> Fn(&'item u8) {}",
    )
    .unwrap();
    assert_eq!(texts(&tokens, for_loop_names(&tokens)), vec!["index", "item"]);
  }

  #[test]
  fn ends_patterns_at_their_boundaries() {
    let tokens = tokenize("value) rest").unwrap();
    assert_eq!(pattern_end(&tokens, 0), 1);
    let unterminated = tokenize("value").unwrap();
    assert_eq!(pattern_end(&unterminated, 0), 1);
  }
}
