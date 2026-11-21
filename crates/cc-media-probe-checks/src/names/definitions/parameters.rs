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

//! Function parameters: the patterns before the `:` of each parameter in the
//! parentheses after `fn name<…>`.

use crate::constants::keywords::{FUNCTION_KEYWORD, SELF_VALUE_KEYWORD};
use crate::constants::punctuation::{COMMA, LESS_THAN_SIGN, OPENING_PARENTHESIS};
use crate::lexer::Token;
use crate::names::definitions::generics::closing_angle;
use crate::names::definitions::patterns::{is_single_colon, pattern_bindings};
use crate::syntax::Delimiters;

/// The bindings of every function parameter except `self`.
#[must_use]
pub fn function_parameter_names(tokens: &[Token], delimiters: &Delimiters) -> Vec<usize> {
  let mut indices = Vec::new();
  for (index, token) in tokens.iter().enumerate() {
    let name = index.saturating_add(1);
    if !token.is_word(FUNCTION_KEYWORD) || !tokens.get(name).is_some_and(Token::is_identifier) {
      continue;
    }
    let mut opening = name.saturating_add(1);
    if tokens
      .get(opening)
      .is_some_and(|candidate| candidate.is_punctuation(LESS_THAN_SIGN))
    {
      opening = closing_angle(tokens, opening).saturating_add(1);
    }
    let is_parameter_list = tokens
      .get(opening)
      .is_some_and(|candidate| candidate.is_punctuation(OPENING_PARENTHESIS));
    if let Some(closing) = delimiters.partner(opening).filter(|_| is_parameter_list) {
      indices.extend(parameters_between(tokens, delimiters, opening, closing));
    }
  }
  indices
}

fn parameters_between(tokens: &[Token], delimiters: &Delimiters, opening: usize, closing: usize) -> Vec<usize> {
  let mut indices = Vec::new();
  let mut parameter_start = opening.saturating_add(1);
  let mut colon: Option<usize> = None;
  let mut index = parameter_start;
  while index <= closing {
    let at_end = index == closing || tokens.get(index).is_some_and(|token| token.is_punctuation(COMMA));
    if at_end {
      if let Some(colon_index) = colon {
        let pattern = parameter_start..colon_index;
        let is_self = tokens
          .get(pattern.clone())
          .unwrap_or_default()
          .iter()
          .any(|token| token.is_word(SELF_VALUE_KEYWORD));
        if !is_self {
          indices.extend(pattern_bindings(tokens, pattern));
        }
      }
      parameter_start = index.saturating_add(1);
      colon = None;
      index = parameter_start;
      continue;
    }
    if colon.is_none() && is_single_colon(tokens, index) {
      colon = Some(index);
    }
    index = delimiters.skip_group(index);
  }
  indices
}

#[cfg(test)]
mod tests {
  use super::function_parameter_names;
  use crate::lexer::{Token, tokenize};
  use crate::syntax::match_delimiters;

  fn texts(source: &str) -> Vec<String> {
    let tokens: Vec<Token> = tokenize(source).unwrap();
    let delimiters = match_delimiters(&tokens).unwrap();
    function_parameter_names(&tokens, &delimiters)
      .into_iter()
      .map(|index| tokens[index].text.clone())
      .collect()
  }

  #[test]
  fn finds_parameters_and_skips_self() {
    assert_eq!(
      texts("fn read<Source: Read>(&mut self, source: Source, (first, second): (u8, u8), callback: fn(u8) -> u8) {}"),
      vec!["source", "first", "second", "callback"]
    );
    assert_eq!(texts("fn take(self: Box<Self>, mut count: usize);"), vec!["count"]);
  }

  #[test]
  fn ignores_function_pointer_types() {
    assert_eq!(texts("type Callback = fn(value: u8) -> u8;"), Vec::<String>::new());
    assert_eq!(texts("fn broken<"), Vec::<String>::new());
  }
}
