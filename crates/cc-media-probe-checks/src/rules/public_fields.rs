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

//! No public field in the library (plan sections 8.1 and 8.10): each field is private, or visible within the crate
//! at most, and a caller reads it through an accessor. One private field already makes a struct literal fail in a
//! dependent crate, as the compile-fail tests of `src/downstream/` show for every type, so only this rule keeps the
//! struct's other fields private too.

use crate::constants::keywords::{PUBLIC_KEYWORD, VISIBLE_ITEM_KEYWORDS};
use crate::constants::punctuation::OPENING_PARENTHESIS;
use crate::findings::{Finding, Rule};
use crate::lexer::Token;
use crate::rules::constants::{LIBRARY_SOURCE_PREFIX, PUBLIC_FIELD};
use crate::syntax::ParsedSource;

/// Checks a file of the library's source for a field declared `pub` without a restriction, outside its embedded
/// tests. Such a `pub` neither starts an item, as `pub fn` does, nor restricts itself, as `pub(crate)` does: it marks
/// a field, named or of a tuple struct. A file of another crate is not checked.
#[must_use]
pub fn check_public_fields(path: &str, parsed: &ParsedSource) -> Vec<Finding> {
  if !path.starts_with(LIBRARY_SOURCE_PREFIX) {
    return Vec::new();
  }
  parsed
    .tokens
    .iter()
    .enumerate()
    .filter(|(index, token)| {
      token.is_word(PUBLIC_KEYWORD)
        && !parsed.is_in_test_block(*index)
        && parsed.token(index.saturating_add(1)).is_some_and(marks_a_field)
    })
    .map(|(_, token)| Finding::new(path, token.line, Rule::PublicField, PUBLIC_FIELD))
    .collect()
}

/// Whether the token after `pub` makes it a field's visibility: it is neither the parenthesis of a restriction nor
/// a keyword that starts an item.
fn marks_a_field(next: &Token) -> bool {
  !next.is_punctuation(OPENING_PARENTHESIS) && !next.is_any_word(VISIBLE_ITEM_KEYWORDS)
}

#[cfg(test)]
mod tests {
  use super::check_public_fields;
  use crate::syntax::ParsedSource;

  fn lines(path: &str, source: &str) -> Vec<usize> {
    let parsed = ParsedSource::parse(source).unwrap();
    check_public_fields(path, &parsed)
      .into_iter()
      .map(|finding| finding.line)
      .collect()
  }

  #[test]
  fn finds_public_named_and_tuple_fields() {
    let path = "crates/cc-media-probe/src/model/size.rs";
    assert_eq!(
      lines(path, "pub struct Size {\n  pub width: u32,\n  height: u32,\n}\n"),
      vec![2]
    );
    assert_eq!(lines(path, "pub struct Pair(u8, pub u8);\n"), vec![1]);
    assert_eq!(lines(path, "pub struct Kind {\n  pub r#type: u8,\n}\n"), vec![2]);
  }

  #[test]
  fn accepts_restricted_fields_and_public_items() {
    let path = "crates/cc-media-probe/src/model/size.rs";
    assert_eq!(
      lines(
        path,
        "pub(crate) struct Size { pub(crate) width: u32, pub(super) height: u32, pub(in crate::model) depth: u8 }\n\
         pub fn width() {}\npub const fn height() {}\npub async fn read() {}\npub unsafe fn raw() {}\n\
         pub extern \"C\" fn call() {}\npub use crate::model;\npub mod tags;\npub enum Kind {}\npub union Bits { whole: u8 }\n\
         pub trait Probe {}\npub type Alias = u8;\npub static NAME: &str = \"\";\npub const LIMIT: u8 = 0;\n\
         pub struct Unit;\npub struct Tuple(u8);\n"
      ),
      Vec::<usize>::new()
    );
  }

  #[test]
  fn leaves_tests_and_other_crates_alone() {
    assert_eq!(
      lines(
        "crates/cc-media-probe/src/model/size.rs",
        "struct Size;\n\n#[cfg(test)]\nmod tests {\n  struct Sample {\n    pub width: u32,\n  }\n}\n"
      ),
      Vec::<usize>::new()
    );
    assert_eq!(
      lines(
        "crates/cc-media-probe-checks/src/lexer/token.rs",
        "pub struct Token {\n  pub line: usize,\n}\n"
      ),
      Vec::<usize>::new()
    );
    assert_eq!(lines("crates/cc-media-probe/src/lib.rs", "pub"), Vec::<usize>::new());
  }
}
