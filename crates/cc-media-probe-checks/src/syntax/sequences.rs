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

//! Recognition of punctuation sequences that the tokenizer keeps apart.

use crate::constants::punctuation::{COLON, EQUALS_SIGN, GREATER_THAN_SIGN, HYPHEN_MINUS};
use crate::lexer::Token;
use crate::syntax::constants::PAIR_LENGTH;

/// Whether the tokens at `index` form `::`.
#[must_use]
pub fn is_path_separator(tokens: &[Token], index: usize) -> bool {
  is_joint_pair(tokens, index, COLON, COLON)
}

/// Whether the token at `index` is followed by `::`.
#[must_use]
pub fn is_path_separator_before(tokens: &[Token], index: usize) -> bool {
  index.checked_add(1).is_some_and(|next| is_path_separator(tokens, next))
}

/// Whether the token at `index` is preceded by `::`.
#[must_use]
pub fn previous_is_path_separator(tokens: &[Token], index: usize) -> bool {
  index
    .checked_sub(PAIR_LENGTH)
    .is_some_and(|start| is_path_separator(tokens, start))
}

/// Whether the tokens at `index` form `=>`.
#[must_use]
pub fn is_fat_arrow(tokens: &[Token], index: usize) -> bool {
  is_joint_pair(tokens, index, EQUALS_SIGN, GREATER_THAN_SIGN)
}

/// Whether the tokens at `index` form `->`.
#[must_use]
pub fn is_thin_arrow(tokens: &[Token], index: usize) -> bool {
  is_joint_pair(tokens, index, HYPHEN_MINUS, GREATER_THAN_SIGN)
}

fn is_joint_pair(tokens: &[Token], index: usize, first: char, second: char) -> bool {
  let Some(first_token) = tokens.get(index) else {
    return false;
  };
  let second_token = index.checked_add(1).and_then(|next| tokens.get(next));
  first_token.is_punctuation(first)
    && first_token.joint
    && second_token.is_some_and(|token| token.is_punctuation(second))
}

#[cfg(test)]
mod tests {
  use super::{is_fat_arrow, is_path_separator, is_path_separator_before, is_thin_arrow, previous_is_path_separator};
  use crate::lexer::tokenize;

  #[test]
  fn recognizes_joint_sequences() {
    let tokens = tokenize("a::b => c -> d : : e").unwrap();
    assert!(is_path_separator(&tokens, 1));
    assert!(is_path_separator_before(&tokens, 0));
    assert!(previous_is_path_separator(&tokens, 3));
    assert!(!previous_is_path_separator(&tokens, 0));
    assert!(is_fat_arrow(&tokens, 4));
    assert!(is_thin_arrow(&tokens, 7));
    assert!(!is_path_separator(&tokens, 10));
    assert!(!is_path_separator(&tokens, 99));
  }
}
