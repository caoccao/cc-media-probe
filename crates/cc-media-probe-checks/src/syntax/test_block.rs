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

//! The embedded test block of a file (R31): one `#[cfg(test)] mod tests { … }`
//! that ends the file.

use crate::constants::keywords::{CONFIGURATION_ATTRIBUTE, TEST_CONFIGURATION, TESTS_MODULE_NAME};
use crate::constants::punctuation::{
  CLOSING_BRACKET, CLOSING_PARENTHESIS, NUMBER_SIGN, OPENING_BRACKET, OPENING_PARENTHESIS,
};
use crate::lexer::Token;
use crate::syntax::item::{Item, ItemKind};

/// Where the embedded test block starts and ends.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TestBlock {
  /// The index of its first token, the `#` of its first attribute.
  pub first_token: usize,
  /// The index of its closing brace.
  pub last_token: usize,
  /// The line of its first token; lines from here on are not counted.
  pub line: usize,
}

impl TestBlock {
  /// Whether the token at `index` lies inside the block.
  #[must_use]
  pub fn contains(&self, index: usize) -> bool {
    (self.first_token..=self.last_token).contains(&index)
  }
}

/// The embedded test block, if the last item of the file is one.
#[must_use]
pub fn find_test_block(tokens: &[Token], items: &[Item]) -> Option<TestBlock> {
  let last = items.last()?;
  let is_test_module = last.kind == ItemKind::InlineModule
    && last.is_named(TESTS_MODULE_NAME)
    && last
      .attributes
      .iter()
      .any(|range| is_test_configuration(tokens.get(range.clone()).unwrap_or_default()));
  is_test_module.then_some(TestBlock {
    first_token: last.first_token,
    last_token: last.last_token,
    line: last.line,
  })
}

/// Whether the attribute tokens are exactly `#[cfg(test)]`.
fn is_test_configuration(attribute: &[Token]) -> bool {
  let [
    number_sign,
    opening_bracket,
    configuration,
    opening_parenthesis,
    test,
    closing_parenthesis,
    closing_bracket,
  ] = attribute
  else {
    return false;
  };
  number_sign.is_punctuation(NUMBER_SIGN)
    && opening_bracket.is_punctuation(OPENING_BRACKET)
    && configuration.is_word(CONFIGURATION_ATTRIBUTE)
    && opening_parenthesis.is_punctuation(OPENING_PARENTHESIS)
    && test.is_word(TEST_CONFIGURATION)
    && closing_parenthesis.is_punctuation(CLOSING_PARENTHESIS)
    && closing_bracket.is_punctuation(CLOSING_BRACKET)
}

#[cfg(test)]
mod tests {
  use super::find_test_block;
  use crate::lexer::tokenize;
  use crate::syntax::delimiters::match_delimiters;
  use crate::syntax::outline::outline;

  fn test_block_line(source: &str) -> Option<usize> {
    let tokens = tokenize(source).unwrap();
    let delimiters = match_delimiters(&tokens).unwrap();
    let items = outline(&tokens, &delimiters);
    find_test_block(&tokens, &items).map(|block| block.line)
  }

  #[test]
  fn finds_the_final_test_module() {
    let source = "fn code() {}\n\n#[cfg(test)]\nmod tests {\n    #[test]\n    fn works() {}\n}\n";
    assert_eq!(test_block_line(source), Some(3));
  }

  #[test]
  fn ignores_other_final_items() {
    assert_eq!(test_block_line("#[cfg(test)]\nmod tests;\n"), None);
    assert_eq!(test_block_line("#[cfg(test)]\nmod checks {}\n"), None);
    assert_eq!(test_block_line("mod tests {}\n"), None);
    assert_eq!(test_block_line("#[cfg(not(test))]\nmod tests {}\n"), None);
    assert_eq!(test_block_line("#[cfg(test)]\nmod tests {}\nfn after() {}\n"), None);
    assert_eq!(test_block_line(""), None);
  }

  #[test]
  fn knows_which_tokens_it_contains() {
    let tokens = tokenize("fn code() {}\n#[cfg(test)]\nmod tests {}\n").unwrap();
    let delimiters = match_delimiters(&tokens).unwrap();
    let items = outline(&tokens, &delimiters);
    let block = find_test_block(&tokens, &items).unwrap();
    assert!(!block.contains(5));
    assert!(block.contains(6));
    assert!(block.contains(block.last_token));
  }
}
