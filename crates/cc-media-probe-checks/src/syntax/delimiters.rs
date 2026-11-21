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

//! Pairing of opening and closing delimiters.

use crate::constants::punctuation::{
  CLOSING_BRACE, CLOSING_BRACKET, CLOSING_PARENTHESIS, OPENING_BRACE, OPENING_BRACKET, OPENING_PARENTHESIS,
};
use crate::lexer::{Token, TokenKind};
use crate::source_error::SourceError;
use crate::syntax::constants::{MISMATCHED_DELIMITER, UNCLOSED_DELIMITER, UNEXPECTED_CLOSING_DELIMITER};

/// The partner of every delimiter token in one token sequence.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Delimiters {
  partners: Vec<Option<usize>>,
}

impl Delimiters {
  /// The index of the token that closes the group opened at `index`, or
  /// that opens the group closed at `index`.
  #[must_use]
  pub fn partner(&self, index: usize) -> Option<usize> {
    self.partners.get(index).copied().flatten()
  }

  /// The index just past the group that opens at `index`, or the next
  /// index when no group opens there.
  #[must_use]
  pub fn skip_group(&self, index: usize) -> usize {
    self
      .partner(index)
      .filter(|partner| *partner > index)
      .unwrap_or(index)
      .saturating_add(1)
  }
}

/// Pairs every opening delimiter with its closing delimiter.
///
/// # Errors
///
/// Returns an error for a closing delimiter without an opening one, a
/// mismatched pair, or an opening delimiter that is never closed.
pub fn match_delimiters(tokens: &[Token]) -> Result<Delimiters, SourceError> {
  let mut partners = vec![None; tokens.len()];
  let mut open: Vec<(usize, char)> = Vec::new();
  for (index, token) in tokens.iter().enumerate() {
    if token.kind != TokenKind::Punctuation {
      continue;
    }
    let Some(character) = token.text.chars().next() else {
      continue;
    };
    if let Some(closing) = closing_partner(character) {
      open.push((index, closing));
    } else if is_closing(character) {
      let Some((opening_index, expected)) = open.pop() else {
        return Err(error(token, UNEXPECTED_CLOSING_DELIMITER));
      };
      if expected != character {
        return Err(error(token, MISMATCHED_DELIMITER));
      }
      if let Some(slot) = partners.get_mut(opening_index) {
        *slot = Some(index);
      }
      if let Some(slot) = partners.get_mut(index) {
        *slot = Some(opening_index);
      }
    }
  }
  if let Some((opening_index, _)) = open.first()
    && let Some(token) = tokens.get(*opening_index)
  {
    return Err(error(token, UNCLOSED_DELIMITER));
  }
  Ok(Delimiters { partners })
}

fn closing_partner(character: char) -> Option<char> {
  match character {
    OPENING_PARENTHESIS => Some(CLOSING_PARENTHESIS),
    OPENING_BRACKET => Some(CLOSING_BRACKET),
    OPENING_BRACE => Some(CLOSING_BRACE),
    _ => None,
  }
}

fn is_closing(character: char) -> bool {
  matches!(character, CLOSING_PARENTHESIS | CLOSING_BRACKET | CLOSING_BRACE)
}

fn error(token: &Token, message: &'static str) -> SourceError {
  SourceError {
    line: token.line,
    message,
  }
}

#[cfg(test)]
mod tests {
  use super::match_delimiters;
  use crate::lexer::tokenize;

  #[test]
  fn pairs_nested_groups() {
    let tokens = tokenize("f(a[b], {c})").unwrap();
    let delimiters = match_delimiters(&tokens).unwrap();
    assert_eq!(delimiters.partner(1), Some(10));
    assert_eq!(delimiters.partner(10), Some(1));
    assert_eq!(delimiters.partner(3), Some(5));
    assert_eq!(delimiters.partner(7), Some(9));
    assert_eq!(delimiters.partner(0), None);
    assert_eq!(delimiters.skip_group(1), 11);
    assert_eq!(delimiters.skip_group(0), 1);
    assert_eq!(delimiters.skip_group(10), 11);
  }

  #[test]
  fn reports_unbalanced_groups() {
    let stray = match_delimiters(&tokenize("a\n)").unwrap()).unwrap_err();
    assert_eq!(stray.line, 2);
    assert!(match_delimiters(&tokenize("(]").unwrap()).is_err());
    assert!(match_delimiters(&tokenize("{ (").unwrap()).is_err());
  }
}
