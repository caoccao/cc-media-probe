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

//! Generic parameters: type, lifetime and const parameters in the `<…>` after
//! an item name, after `impl` and after `for` (higher-ranked bounds).

use crate::constants::keywords::{
  CONSTANT_KEYWORD, ENUMERATION_KEYWORD, FOR_KEYWORD, FUNCTION_KEYWORD, IMPLEMENTATION_KEYWORD, STRUCTURE_KEYWORD,
  TRAIT_KEYWORD, TYPE_KEYWORD, UNION_KEYWORD,
};
use crate::constants::punctuation::{
  CLOSING_BRACKET, COMMA, EQUALS_SIGN, GREATER_THAN_SIGN, HYPHEN_MINUS, LESS_THAN_SIGN, NUMBER_SIGN,
};
use crate::lexer::{Token, TokenKind};

/// The defining tokens of every generic parameter list.
#[must_use]
pub fn generic_parameter_names(tokens: &[Token]) -> Vec<usize> {
  let mut indices = Vec::new();
  for (index, token) in tokens.iter().enumerate() {
    let next = index.saturating_add(1);
    let opening = if token.is_any_word(&[
      FUNCTION_KEYWORD,
      STRUCTURE_KEYWORD,
      ENUMERATION_KEYWORD,
      UNION_KEYWORD,
      TRAIT_KEYWORD,
      TYPE_KEYWORD,
    ]) {
      next.saturating_add(1)
    } else if token.is_word(IMPLEMENTATION_KEYWORD) || token.is_word(FOR_KEYWORD) {
      next
    } else {
      continue;
    };
    if tokens
      .get(opening)
      .is_some_and(|candidate| candidate.is_punctuation(LESS_THAN_SIGN))
    {
      indices.extend(parameters_of_list(tokens, opening));
    }
  }
  indices
}

/// The index of the `>` that closes the angle-bracket list opened at
/// `opening`, ignoring the `>` of `->` and `=>`.
#[must_use]
pub fn closing_angle(tokens: &[Token], opening: usize) -> usize {
  let mut depth: usize = 0;
  let mut index = opening;
  while let Some(token) = tokens.get(index) {
    if token.is_punctuation(LESS_THAN_SIGN) {
      depth = depth.saturating_add(1);
    } else if token.is_punctuation(GREATER_THAN_SIGN) && !is_arrow_head(tokens, index) {
      depth = depth.saturating_sub(1);
      if depth == 0 {
        return index;
      }
    }
    index = index.saturating_add(1);
  }
  index
}

fn parameters_of_list(tokens: &[Token], opening: usize) -> Vec<usize> {
  let closing = closing_angle(tokens, opening);
  let mut indices = Vec::new();
  let mut depth: usize = 0;
  let mut expects_parameter = true;
  let mut index = opening.saturating_add(1);
  while index < closing {
    let Some(token) = tokens.get(index) else {
      break;
    };
    if token.is_punctuation(LESS_THAN_SIGN) {
      depth = depth.saturating_add(1);
    } else if token.is_punctuation(GREATER_THAN_SIGN) && !is_arrow_head(tokens, index) {
      depth = depth.saturating_sub(1);
    } else if depth == 0 && token.is_punctuation(COMMA) {
      expects_parameter = true;
    } else if depth == 0 && expects_parameter && token.is_punctuation(NUMBER_SIGN) {
      index = skip_attribute(tokens, index);
      continue;
    } else if depth == 0 && expects_parameter {
      expects_parameter = false;
      let name = if token.is_word(CONSTANT_KEYWORD) {
        index.saturating_add(1)
      } else {
        index
      };
      let defines = tokens
        .get(name)
        .is_some_and(|candidate| candidate.is_identifier() || candidate.kind == TokenKind::Lifetime);
      if defines {
        indices.push(name);
      }
    }
    index = index.saturating_add(1);
  }
  indices
}

fn skip_attribute(tokens: &[Token], number_sign: usize) -> usize {
  let mut index = number_sign;
  while let Some(token) = tokens.get(index) {
    index = index.saturating_add(1);
    if token.is_punctuation(CLOSING_BRACKET) {
      break;
    }
  }
  index
}

fn is_arrow_head(tokens: &[Token], index: usize) -> bool {
  index
    .checked_sub(1)
    .and_then(|before| tokens.get(before))
    .is_some_and(|previous| {
      previous.joint && (previous.is_punctuation(HYPHEN_MINUS) || previous.is_punctuation(EQUALS_SIGN))
    })
}

#[cfg(test)]
mod tests {
  use super::{closing_angle, generic_parameter_names};
  use crate::lexer::{Token, tokenize};

  fn texts(tokens: &[Token], indices: Vec<usize>) -> Vec<String> {
    indices.into_iter().map(|index| tokens[index].text.clone()).collect()
  }

  #[test]
  fn finds_type_lifetime_and_const_parameters() {
    let tokens = tokenize("fn run<'input, Source: Iterator<Item = u8>, const SIZE: usize>() {} impl<Reader> Trait for Wrapper<Reader> {} struct Pair<#[allow(unused)] First, Second = u8>;").unwrap();
    assert_eq!(
      texts(&tokens, generic_parameter_names(&tokens)),
      vec!["'input", "Source", "SIZE", "Reader", "First", "Second"]
    );
  }

  #[test]
  fn finds_higher_ranked_lifetimes_and_skips_arrows() {
    let tokens = tokenize("fn apply<Function: for<'item> Fn(&'item u8) -> u8>() {}").unwrap();
    assert_eq!(
      texts(&tokens, generic_parameter_names(&tokens)),
      vec!["Function", "'item"]
    );
    assert_eq!(closing_angle(&tokens, 2), 18);
    let open = tokenize("< a <").unwrap();
    assert_eq!(closing_angle(&open, 0), 3);
  }
}
