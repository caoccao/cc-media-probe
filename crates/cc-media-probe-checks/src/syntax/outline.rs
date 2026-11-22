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

//! The top-level items of a source file.

use crate::constants::keywords::{
  ASYNCHRONOUS_KEYWORD, AUTO_KEYWORD, CONSTANT_FUNCTION_KEYWORDS, CONSTANT_KEYWORD, CRATE_KEYWORD, DEFAULT_KEYWORD,
  ENUMERATION_KEYWORD, EXTERN_KEYWORD, FUNCTION_KEYWORD, IMPLEMENTATION_KEYWORD, MACRO_RULES_NAME, MODULE_KEYWORD,
  MUTABLE_KEYWORD, PUBLIC_KEYWORD, SAFE_KEYWORD, STATIC_KEYWORD, STRUCTURE_KEYWORD, TRAIT_KEYWORD, TYPE_KEYWORD,
  UNION_KEYWORD, UNSAFE_KEYWORD, USE_KEYWORD,
};
use crate::constants::punctuation::{
  EXCLAMATION_MARK, NUMBER_SIGN, OPENING_BRACE, OPENING_BRACKET, OPENING_PARENTHESIS, SEMICOLON,
};
use crate::lexer::Token;
use crate::syntax::attributes::attribute_end;
use crate::syntax::constants::{THREE_TOKENS_AHEAD, TWO_TOKENS_AHEAD};
use crate::syntax::delimiters::Delimiters;
use crate::syntax::item::{Item, ItemKind};

/// Splits the tokens of a file into its top-level items.
#[must_use]
pub fn outline(tokens: &[Token], delimiters: &Delimiters) -> Vec<Item> {
  let mut items = Vec::new();
  let mut index = 0;
  while index < tokens.len() {
    let item = parse_item(tokens, delimiters, index);
    index = item.last_token.saturating_add(1);
    items.push(item);
  }
  items
}

fn parse_item(tokens: &[Token], delimiters: &Delimiters, start: usize) -> Item {
  let line = tokens.get(start).map_or(0, |token| token.line);
  let mut attributes = Vec::new();
  let mut index = start;
  while tokens.get(index).is_some_and(|token| token.is_punctuation(NUMBER_SIGN)) {
    let Some(end) = attribute_end(tokens, delimiters, index) else {
      break;
    };
    let is_inner = tokens
      .get(index.saturating_add(1))
      .is_some_and(|token| token.is_punctuation(EXCLAMATION_MARK));
    if is_inner {
      return finished(ItemKind::InnerAttribute, None, start, end, line, attributes, None);
    }
    attributes.push(index..end.saturating_add(1));
    index = end.saturating_add(1);
  }
  index = skip_visibility_and_qualifiers(tokens, delimiters, index);
  let (kind, name, last_token, body) = classify(tokens, delimiters, index);
  finished(kind, name, start, last_token, line, attributes, body)
}

fn finished(
  kind: ItemKind,
  name: Option<String>,
  first_token: usize,
  last_token: usize,
  line: usize,
  attributes: Vec<std::ops::Range<usize>>,
  body: Option<(usize, usize)>,
) -> Item {
  Item {
    kind,
    name,
    first_token,
    last_token: last_token.max(first_token),
    line,
    attributes,
    body,
  }
}

fn skip_visibility_and_qualifiers(tokens: &[Token], delimiters: &Delimiters, start: usize) -> usize {
  let mut index = start;
  if tokens.get(index).is_some_and(|token| token.is_word(PUBLIC_KEYWORD)) {
    index = index.saturating_add(1);
    if tokens
      .get(index)
      .is_some_and(|token| token.is_punctuation(OPENING_PARENTHESIS))
    {
      index = delimiters.skip_group(index);
    }
  }
  loop {
    let Some(token) = tokens.get(index) else {
      return index;
    };
    let next = tokens.get(index.saturating_add(1));
    let is_qualifier = token.is_any_word(&[
      UNSAFE_KEYWORD,
      ASYNCHRONOUS_KEYWORD,
      DEFAULT_KEYWORD,
      AUTO_KEYWORD,
      SAFE_KEYWORD,
    ]) || (token.is_word(CONSTANT_KEYWORD)
      && next.is_some_and(|following| following.is_any_word(CONSTANT_FUNCTION_KEYWORDS)))
      || (token.is_word(EXTERN_KEYWORD) && next.is_some_and(Token::is_string));
    if !is_qualifier {
      return index;
    }
    index = index.saturating_add(1);
    if token.is_word(EXTERN_KEYWORD) {
      index = index.saturating_add(1);
    }
  }
}

type Classified = (ItemKind, Option<String>, usize, Option<(usize, usize)>);

fn classify(tokens: &[Token], delimiters: &Delimiters, index: usize) -> Classified {
  let Some(token) = tokens.get(index) else {
    return (ItemKind::Other, None, index, None);
  };
  let name_at = |offset: usize| {
    tokens
      .get(index.saturating_add(offset))
      .filter(|candidate| candidate.is_identifier())
      .map(|candidate| candidate.text.clone())
  };
  let next = tokens.get(index.saturating_add(1));
  if token.is_word(MODULE_KEYWORD) {
    return classify_module(tokens, delimiters, index, name_at(1));
  }
  if token.is_word(USE_KEYWORD) {
    return (ItemKind::Use, None, semicolon_end(tokens, delimiters, index), None);
  }
  if token.is_word(EXTERN_KEYWORD) && next.is_some_and(|following| following.is_word(CRATE_KEYWORD)) {
    return (
      ItemKind::ExternCrate,
      name_at(TWO_TOKENS_AHEAD),
      semicolon_end(tokens, delimiters, index),
      None,
    );
  }
  if token.is_word(MACRO_RULES_NAME) && next.is_some_and(|following| following.is_punctuation(EXCLAMATION_MARK)) {
    let (last, body) = macro_end(tokens, delimiters, index.saturating_add(THREE_TOKENS_AHEAD));
    return (ItemKind::MacroRules, name_at(TWO_TOKENS_AHEAD), last, body);
  }
  if token.is_identifier() && next.is_some_and(|following| following.is_punctuation(EXCLAMATION_MARK)) {
    let (last, body) = macro_end(tokens, delimiters, index.saturating_add(TWO_TOKENS_AHEAD));
    return (ItemKind::MacroCall, Some(token.text.clone()), last, body);
  }
  let static_name = if next.is_some_and(|following| following.is_word(MUTABLE_KEYWORD)) {
    name_at(TWO_TOKENS_AHEAD)
  } else {
    name_at(1)
  };
  let (kind, name, ends_at_semicolon) = match token.text.as_str() {
    FUNCTION_KEYWORD => (ItemKind::Function, name_at(1), false),
    CONSTANT_KEYWORD => (ItemKind::Constant, name_at(1), true),
    STATIC_KEYWORD => (ItemKind::Static, static_name, true),
    STRUCTURE_KEYWORD => (ItemKind::Structure, name_at(1), false),
    ENUMERATION_KEYWORD => (ItemKind::Enumeration, name_at(1), false),
    UNION_KEYWORD if name_at(1).is_some() => (ItemKind::Union, name_at(1), false),
    TRAIT_KEYWORD => (ItemKind::Trait, name_at(1), false),
    TYPE_KEYWORD => (ItemKind::TypeAlias, name_at(1), true),
    IMPLEMENTATION_KEYWORD => (ItemKind::Implementation, None, false),
    _ => (ItemKind::Other, None, false),
  };
  if ends_at_semicolon {
    return (kind, name, semicolon_end(tokens, delimiters, index), None);
  }
  let (last, body) = body_end(tokens, delimiters, index);
  (kind, name, last, body)
}

fn classify_module(tokens: &[Token], delimiters: &Delimiters, index: usize, name: Option<String>) -> Classified {
  let after_name = index.saturating_add(TWO_TOKENS_AHEAD);
  match tokens.get(after_name) {
    Some(token) if token.is_punctuation(OPENING_BRACE) => {
      let closing = delimiters.partner(after_name).unwrap_or(after_name);
      (ItemKind::InlineModule, name, closing, Some((after_name, closing)))
    }
    _ => (
      ItemKind::ModuleDeclaration,
      name,
      semicolon_end(tokens, delimiters, index),
      None,
    ),
  }
}

/// The index of the `;` that ends an item, skipping every group.
fn semicolon_end(tokens: &[Token], delimiters: &Delimiters, start: usize) -> usize {
  let mut index = start;
  while let Some(token) = tokens.get(index) {
    if token.is_punctuation(SEMICOLON) {
      return index;
    }
    index = delimiters.skip_group(index);
  }
  tokens.len().saturating_sub(1)
}

/// The end of an item with an optional braced body: the closing brace of
/// its first braced group, or the `;` that comes first.
fn body_end(tokens: &[Token], delimiters: &Delimiters, start: usize) -> (usize, Option<(usize, usize)>) {
  let mut index = start;
  while let Some(token) = tokens.get(index) {
    if token.is_punctuation(SEMICOLON) {
      return (index, None);
    }
    if token.is_punctuation(OPENING_BRACE) {
      let closing = delimiters.partner(index).unwrap_or(index);
      return (closing, Some((index, closing)));
    }
    index = delimiters.skip_group(index);
  }
  (tokens.len().saturating_sub(1), None)
}

/// The end of a macro call or definition whose group may start at `start`
/// (after an optional name): the group's closing delimiter, or the `;` that
/// follows a parenthesized or bracketed group.
fn macro_end(tokens: &[Token], delimiters: &Delimiters, start: usize) -> (usize, Option<(usize, usize)>) {
  let mut index = start;
  while let Some(token) = tokens.get(index) {
    if token.is_punctuation(OPENING_BRACE) {
      let closing = delimiters.partner(index).unwrap_or(index);
      return (closing, Some((index, closing)));
    }
    if token.is_punctuation(OPENING_PARENTHESIS) || token.is_punctuation(OPENING_BRACKET) {
      let closing = delimiters.partner(index).unwrap_or(index);
      let after = closing.saturating_add(1);
      let has_semicolon = tokens
        .get(after)
        .is_some_and(|following| following.is_punctuation(SEMICOLON));
      return (if has_semicolon { after } else { closing }, Some((index, closing)));
    }
    if token.is_punctuation(SEMICOLON) {
      return (index, None);
    }
    index = index.saturating_add(1);
  }
  (tokens.len().saturating_sub(1), None)
}

#[cfg(test)]
mod tests {
  use super::outline;
  use crate::lexer::tokenize;
  use crate::syntax::delimiters::match_delimiters;
  use crate::syntax::item::ItemKind;

  fn kinds_and_names(source: &str) -> Vec<(ItemKind, Option<String>)> {
    let tokens = tokenize(source).unwrap();
    let delimiters = match_delimiters(&tokens).unwrap();
    outline(&tokens, &delimiters)
      .into_iter()
      .map(|item| (item.kind, item.name))
      .collect()
  }

  fn named(kind: ItemKind, name: &str) -> (ItemKind, Option<String>) {
    (kind, Some(name.to_owned()))
  }

  #[test]
  fn outlines_module_roots() {
    let source = "#![forbid(unsafe_code)]\n/// Docs.\npub mod segment;\n#[cfg(test)]\nmod tests;\npub(crate) use self::segment::Segment;\nextern crate alloc as allocation;\nmod inline { fn helper() {} }\n";
    assert_eq!(
      kinds_and_names(source),
      vec![
        (ItemKind::InnerAttribute, None),
        named(ItemKind::ModuleDeclaration, "segment"),
        named(ItemKind::ModuleDeclaration, "tests"),
        (ItemKind::Use, None),
        named(ItemKind::ExternCrate, "alloc"),
        named(ItemKind::InlineModule, "inline"),
      ]
    );
  }

  #[test]
  fn outlines_definitions() {
    let source = "pub const fn build() -> u8 { 2 }\nconst LIMIT: Shape = Shape { side: 2 };\nstatic mut COUNTER: u8 = 0;\npub struct Unit;\nstruct Pair(u8, u8);\nstruct Named { field: u8 }\nenum Choice { First }\nunion Overlay { whole: u32 }\npub unsafe trait Marker {}\ntype Alias = u8;\nimpl Marker for Unit {}\nunsafe extern \"C\" { fn external(); }\n";
    assert_eq!(
      kinds_and_names(source),
      vec![
        named(ItemKind::Function, "build"),
        named(ItemKind::Constant, "LIMIT"),
        named(ItemKind::Static, "COUNTER"),
        named(ItemKind::Structure, "Unit"),
        named(ItemKind::Structure, "Pair"),
        named(ItemKind::Structure, "Named"),
        named(ItemKind::Enumeration, "Choice"),
        named(ItemKind::Union, "Overlay"),
        named(ItemKind::Trait, "Marker"),
        named(ItemKind::TypeAlias, "Alias"),
        (ItemKind::Implementation, None),
        (ItemKind::Other, None),
      ]
    );
  }

  #[test]
  fn outlines_macros() {
    let source = "macro_rules! repeat { () => {} }\nthread_local!(static VALUE: u8 = 1);\ninclude!(\"other.rs\");\nmacro_rules! square [ () => {} ];\nitems! { }\n";
    assert_eq!(
      kinds_and_names(source),
      vec![
        named(ItemKind::MacroRules, "repeat"),
        named(ItemKind::MacroCall, "thread_local"),
        named(ItemKind::MacroCall, "include"),
        named(ItemKind::MacroRules, "square"),
        named(ItemKind::MacroCall, "items"),
      ]
    );
  }

  #[test]
  fn records_lines_attributes_and_bodies() {
    let tokens = tokenize("#[cfg(test)]\n#[allow(dead_code)]\nmod tests {\n    fn helper() {}\n}\n").unwrap();
    let delimiters = match_delimiters(&tokens).unwrap();
    let items = outline(&tokens, &delimiters);
    assert_eq!(items.len(), 1);
    let item = &items[0];
    assert_eq!(item.line, 1);
    assert_eq!(item.attributes, vec![0..7, 7..14]);
    assert_eq!(item.body, Some((16, 23)));
    assert_eq!(item.last_token, 23);
  }

  #[test]
  fn tolerates_truncated_items() {
    assert_eq!(kinds_and_names("fn"), vec![(ItemKind::Function, None)]);
    assert_eq!(kinds_and_names("use a"), vec![(ItemKind::Use, None)]);
    assert_eq!(kinds_and_names("name!"), vec![named(ItemKind::MacroCall, "name")]);
    assert_eq!(kinds_and_names("#"), vec![(ItemKind::Other, None)]);
  }
}
