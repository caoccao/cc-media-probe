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

//! Attribute ranges: every `#[…]` and `#![…]`, at any depth.

use std::ops::Range;

use crate::constants::punctuation::{EXCLAMATION_MARK, NUMBER_SIGN, OPENING_BRACKET};
use crate::lexer::Token;
use crate::syntax::delimiters::Delimiters;

/// The token range of every attribute, from its `#` to its closing `]`.
#[must_use]
pub fn attribute_ranges(tokens: &[Token], delimiters: &Delimiters) -> Vec<Range<usize>> {
  let mut ranges = Vec::new();
  for (index, token) in tokens.iter().enumerate() {
    if !token.is_punctuation(NUMBER_SIGN) {
      continue;
    }
    if let Some(end) = attribute_end(tokens, delimiters, index) {
      ranges.push(index..end.saturating_add(1));
    }
  }
  ranges
}

/// The index of the closing bracket of the attribute whose `#` is at
/// `index`, if an attribute starts there.
#[must_use]
pub fn attribute_end(tokens: &[Token], delimiters: &Delimiters, index: usize) -> Option<usize> {
  let next = index.checked_add(1)?;
  let bracket = if tokens.get(next)?.is_punctuation(EXCLAMATION_MARK) {
    next.checked_add(1)?
  } else {
    next
  };
  if tokens.get(bracket)?.is_punctuation(OPENING_BRACKET) {
    delimiters.partner(bracket)
  } else {
    None
  }
}

#[cfg(test)]
mod tests {
  use super::attribute_ranges;
  use crate::lexer::tokenize;
  use crate::syntax::delimiters::match_delimiters;

  #[test]
  fn finds_inner_and_outer_attributes() {
    let tokens = tokenize("#![forbid(x)] #[cfg(test)] mod m { #[test] fn f() {} } r#x # y").unwrap();
    let delimiters = match_delimiters(&tokens).unwrap();
    let ranges = attribute_ranges(&tokens, &delimiters);
    assert_eq!(ranges, vec![0..8, 8..15, 18..22]);
  }
}
