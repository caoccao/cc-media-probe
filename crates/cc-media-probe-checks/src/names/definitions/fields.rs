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

//! Field names of structs and unions, and variants and variant fields of
//! enums.

use crate::constants::keywords::{
  ENUMERATION_KEYWORD, PUBLIC_KEYWORD, STRUCTURE_KEYWORD, UNION_KEYWORD, WHERE_KEYWORD,
};
use crate::constants::punctuation::{
  COMMA, LESS_THAN_SIGN, NUMBER_SIGN, OPENING_BRACE, OPENING_PARENTHESIS, SEMICOLON,
};
use crate::lexer::Token;
use crate::names::definitions::generics::closing_angle;
use crate::names::definitions::patterns::is_single_colon;
use crate::syntax::Delimiters;

/// The field names of every struct and union, and the variant and field
/// names of every enum.
#[must_use]
pub fn field_and_variant_names(tokens: &[Token], delimiters: &Delimiters) -> Vec<usize> {
  let mut indices = Vec::new();
  for (index, token) in tokens.iter().enumerate() {
    let is_record = token.is_word(STRUCTURE_KEYWORD) || token.is_word(UNION_KEYWORD);
    let is_enumeration = token.is_word(ENUMERATION_KEYWORD);
    let name = index.saturating_add(1);
    if !(is_record || is_enumeration) || !tokens.get(name).is_some_and(Token::is_identifier) {
      continue;
    }
    let Some(opening) = body_opening(tokens, delimiters, name.saturating_add(1)) else {
      continue;
    };
    let closing = delimiters.partner(opening).unwrap_or(opening);
    for start in segment_starts(tokens, delimiters, opening, closing) {
      if is_record {
        indices.extend(field_name(tokens, delimiters, start));
      } else {
        indices.extend(variant_names(tokens, delimiters, start));
      }
    }
  }
  indices
}

/// The brace of the body: after the generics and a `where` clause; none for
/// a unit or tuple struct.
fn body_opening(tokens: &[Token], delimiters: &Delimiters, start: usize) -> Option<usize> {
  let mut index = start;
  if tokens
    .get(index)
    .is_some_and(|token| token.is_punctuation(LESS_THAN_SIGN))
  {
    index = closing_angle(tokens, index).saturating_add(1);
  }
  let mut in_where_clause = false;
  while let Some(token) = tokens.get(index) {
    if token.is_punctuation(OPENING_BRACE) {
      return Some(index);
    }
    let ends_without_body =
      token.is_punctuation(SEMICOLON) || (!in_where_clause && token.is_punctuation(OPENING_PARENTHESIS));
    if ends_without_body {
      return None;
    }
    in_where_clause = in_where_clause || token.is_word(WHERE_KEYWORD);
    index = delimiters.skip_group(index);
  }
  None
}

/// The first token of every comma-separated segment inside the group.
fn segment_starts(tokens: &[Token], delimiters: &Delimiters, opening: usize, closing: usize) -> Vec<usize> {
  let mut starts = vec![opening.saturating_add(1)];
  let mut index = opening.saturating_add(1);
  while index < closing {
    if tokens.get(index).is_some_and(|token| token.is_punctuation(COMMA)) {
      starts.push(index.saturating_add(1));
    }
    index = delimiters.skip_group(index);
  }
  starts.retain(|start| *start < closing);
  starts
}

/// The first token after the attributes and the visibility of a segment.
fn after_attributes_and_visibility(tokens: &[Token], delimiters: &Delimiters, start: usize) -> usize {
  let mut index = start;
  while tokens.get(index).is_some_and(|token| token.is_punctuation(NUMBER_SIGN)) {
    index = delimiters.skip_group(index.saturating_add(1));
  }
  if tokens.get(index).is_some_and(|token| token.is_word(PUBLIC_KEYWORD)) {
    index = index.saturating_add(1);
    if tokens
      .get(index)
      .is_some_and(|token| token.is_punctuation(OPENING_PARENTHESIS))
    {
      index = delimiters.skip_group(index);
    }
  }
  index
}

fn field_name(tokens: &[Token], delimiters: &Delimiters, start: usize) -> Option<usize> {
  let name = after_attributes_and_visibility(tokens, delimiters, start);
  let is_field = tokens.get(name).is_some_and(Token::is_identifier) && is_single_colon(tokens, name.saturating_add(1));
  is_field.then_some(name)
}

fn variant_names(tokens: &[Token], delimiters: &Delimiters, start: usize) -> Vec<usize> {
  let name = after_attributes_and_visibility(tokens, delimiters, start);
  if !tokens.get(name).is_some_and(Token::is_identifier) {
    return Vec::new();
  }
  let mut indices = vec![name];
  let fields = name.saturating_add(1);
  if tokens
    .get(fields)
    .is_some_and(|token| token.is_punctuation(OPENING_BRACE))
  {
    let closing = delimiters.partner(fields).unwrap_or(fields);
    for field_start in segment_starts(tokens, delimiters, fields, closing) {
      indices.extend(field_name(tokens, delimiters, field_start));
    }
  }
  indices
}

#[cfg(test)]
mod tests {
  use super::field_and_variant_names;
  use crate::lexer::{Token, tokenize};
  use crate::syntax::match_delimiters;

  fn texts(source: &str) -> Vec<String> {
    let tokens: Vec<Token> = tokenize(source).unwrap();
    let delimiters = match_delimiters(&tokens).unwrap();
    field_and_variant_names(&tokens, &delimiters)
      .into_iter()
      .map(|index| tokens[index].text.clone())
      .collect()
  }

  #[test]
  fn finds_struct_and_union_fields() {
    assert_eq!(
      texts(
        "pub struct Packet<Source> where Source: Fn(u8) { #[doc = \"x\"] pub(crate) header: u8, payload: Vec<Source>, } union Overlay { whole: u32 }"
      ),
      vec!["header", "payload", "whole"]
    );
    assert_eq!(
      texts("struct Pair(u8, u8); struct Unit; struct Generic<Item>(Item);"),
      Vec::<String>::new()
    );
  }

  #[test]
  fn finds_variants_and_their_fields() {
    assert_eq!(
      texts("enum Shape { #[default] Empty, Circle(u8), Rectangle { width: u8, height: u8 }, Valued = 2 }"),
      vec!["Empty", "Circle", "Rectangle", "width", "height", "Valued"]
    );
    assert_eq!(texts("enum Broken"), Vec::<String>::new());
  }
}
