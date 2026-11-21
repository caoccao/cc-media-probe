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

//! Names that a keyword introduces (`fn`, `struct`, `const`, …), names given
//! by `as` in `use` declarations, loop labels and macro metavariables.

use crate::constants::keywords::{
  AS_KEYWORD, ASYNCHRONOUS_KEYWORD, CONSTANT_KEYWORD, CRATE_KEYWORD, ENUMERATION_KEYWORD, EXTERN_KEYWORD, FOR_KEYWORD,
  FUNCTION_KEYWORD, LOOP_KEYWORD, MACRO_RULES_NAME, MODULE_KEYWORD, MUTABLE_KEYWORD, STATIC_KEYWORD, STRUCTURE_KEYWORD,
  TRAIT_KEYWORD, TYPE_KEYWORD, UNION_KEYWORD, UNSAFE_KEYWORD, USE_KEYWORD, WHILE_KEYWORD,
};
use crate::constants::punctuation::{COLON, DOLLAR_SIGN, EXCLAMATION_MARK, LESS_THAN_SIGN, OPENING_BRACE, SEMICOLON};
use crate::lexer::{Token, TokenKind};
use crate::syntax::is_path_separator;

/// The names after `fn`, `struct`, `enum`, `union`, `trait`, `type`, `mod`,
/// `const`, `static` and `macro_rules!`.
#[must_use]
pub fn item_names(tokens: &[Token]) -> Vec<usize> {
  let mut indices = Vec::new();
  for (index, token) in tokens.iter().enumerate() {
    if token.kind != TokenKind::Identifier {
      continue;
    }
    let next = index.saturating_add(1);
    let after_next = next.saturating_add(1);
    let candidate = match token.text.as_str() {
      STATIC_KEYWORD
        if tokens
          .get(next)
          .is_some_and(|following| following.is_word(MUTABLE_KEYWORD)) =>
      {
        Some(after_next)
      }
      FUNCTION_KEYWORD | STRUCTURE_KEYWORD | ENUMERATION_KEYWORD | TRAIT_KEYWORD | TYPE_KEYWORD | MODULE_KEYWORD
      | STATIC_KEYWORD => Some(next),
      UNION_KEYWORD => tokens
        .get(after_next)
        .filter(|following| following.is_punctuation(OPENING_BRACE) || following.is_punctuation(LESS_THAN_SIGN))
        .map(|_| next),
      CONSTANT_KEYWORD => tokens
        .get(next)
        .filter(|following| {
          ![FUNCTION_KEYWORD, UNSAFE_KEYWORD, ASYNCHRONOUS_KEYWORD, EXTERN_KEYWORD]
            .iter()
            .any(|word| following.is_word(word))
        })
        .map(|_| next),
      MACRO_RULES_NAME
        if tokens
          .get(next)
          .is_some_and(|following| following.is_punctuation(EXCLAMATION_MARK)) =>
      {
        Some(after_next)
      }
      _ => None,
    };
    if let Some(name) = candidate.filter(|name| tokens.get(*name).is_some_and(Token::is_identifier)) {
      indices.push(name);
    }
  }
  indices
}

/// The names that `as` gives in `use` declarations and `extern crate`.
#[must_use]
pub fn rename_names(tokens: &[Token]) -> Vec<usize> {
  let mut indices = Vec::new();
  let mut in_declaration = false;
  for (index, token) in tokens.iter().enumerate() {
    let next = tokens.get(index.saturating_add(1));
    if token.is_word(USE_KEYWORD)
      || (token.is_word(EXTERN_KEYWORD) && next.is_some_and(|following| following.is_word(CRATE_KEYWORD)))
    {
      in_declaration = true;
    } else if token.is_punctuation(SEMICOLON) {
      in_declaration = false;
    } else if in_declaration && token.is_word(AS_KEYWORD) && next.is_some_and(Token::is_identifier) {
      indices.push(index.saturating_add(1));
    }
  }
  indices
}

/// Loop and block labels: a lifetime followed by a single colon and a loop
/// or a block.
#[must_use]
pub fn label_names(tokens: &[Token]) -> Vec<usize> {
  let mut indices = Vec::new();
  for (index, token) in tokens.iter().enumerate() {
    if token.kind != TokenKind::Lifetime {
      continue;
    }
    let colon = index.saturating_add(1);
    let target = tokens.get(colon.saturating_add(1));
    let has_colon = tokens
      .get(colon)
      .is_some_and(|following| following.is_punctuation(COLON))
      && !is_path_separator(tokens, colon);
    let labels_a_loop = target.is_some_and(|following| {
      [LOOP_KEYWORD, WHILE_KEYWORD, FOR_KEYWORD]
        .iter()
        .any(|word| following.is_word(word))
        || following.is_punctuation(OPENING_BRACE)
    });
    if has_colon && labels_a_loop {
      indices.push(index);
    }
  }
  indices
}

/// Metavariables of declarative macros: `$name`, except `$crate`.
#[must_use]
pub fn metavariable_names(tokens: &[Token]) -> Vec<usize> {
  let mut indices = Vec::new();
  for (index, token) in tokens.iter().enumerate() {
    let name = index.saturating_add(1);
    let is_metavariable = token.is_punctuation(DOLLAR_SIGN)
      && tokens
        .get(name)
        .is_some_and(|following| following.is_identifier() && !following.is_word(CRATE_KEYWORD));
    if is_metavariable {
      indices.push(name);
    }
  }
  indices
}

#[cfg(test)]
mod tests {
  use super::{item_names, label_names, metavariable_names, rename_names};
  use crate::lexer::{Token, tokenize};

  fn texts(tokens: &[Token], indices: Vec<usize>) -> Vec<String> {
    indices.into_iter().map(|index| tokens[index].text.clone()).collect()
  }

  #[test]
  fn finds_names_after_keywords() {
    let tokens = tokenize("fn run() {} struct Unit; enum Choice {} union Overlay {} union = 2; trait Marker {} type Alias = u8; mod segment; const LIMIT: u8 = 2; const fn build() {} static mut COUNTER: u8 = 0; static NAME: &str = \"\"; macro_rules! repeat {} fn(u8); r#fn").unwrap();
    assert_eq!(
      texts(&tokens, item_names(&tokens)),
      vec![
        "run", "Unit", "Choice", "Overlay", "Marker", "Alias", "segment", "LIMIT", "build", "COUNTER", "NAME", "repeat"
      ]
    );
  }

  #[test]
  fn finds_renames_only_in_declarations() {
    let tokens =
      tokenize("use std::io::{Read as Reading, Write}; extern crate alloc as allocation; let value = other as u32;")
        .unwrap();
    assert_eq!(texts(&tokens, rename_names(&tokens)), vec!["Reading", "allocation"]);
  }

  #[test]
  fn finds_labels_and_metavariables() {
    let tokens = tokenize("'outer: loop {} 'block: { } fn first<'input>() {} ($count:expr, $crate::run)").unwrap();
    assert_eq!(texts(&tokens, label_names(&tokens)), vec!["'outer", "'block"]);
    assert_eq!(texts(&tokens, metavariable_names(&tokens)), vec!["count"]);
  }
}
