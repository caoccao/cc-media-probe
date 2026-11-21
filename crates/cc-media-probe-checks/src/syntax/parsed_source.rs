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

//! A source file with everything the checks need to know about its structure.

use std::ops::Range;

use crate::lexer::{Token, tokenize};
use crate::source_error::SourceError;
use crate::syntax::attributes::attribute_ranges;
use crate::syntax::delimiters::{Delimiters, match_delimiters};
use crate::syntax::item::Item;
use crate::syntax::outline::outline;
use crate::syntax::test_block::{TestBlock, find_test_block};

/// The tokens, delimiters, items, attributes and test block of one file.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParsedSource {
  /// Every token of the file.
  pub tokens: Vec<Token>,
  /// The partner of every delimiter token.
  pub delimiters: Delimiters,
  /// The top-level items.
  pub items: Vec<Item>,
  /// The token range of every attribute, at any depth.
  pub attributes: Vec<Range<usize>>,
  /// The embedded test block, if the file ends with one.
  pub test_block: Option<TestBlock>,
}

impl ParsedSource {
  /// Parses `source`.
  ///
  /// # Errors
  ///
  /// Returns an error when the source cannot be tokenized or its
  /// delimiters do not match.
  pub fn parse(source: &str) -> Result<Self, SourceError> {
    let tokens = tokenize(source)?;
    let delimiters = match_delimiters(&tokens)?;
    let items = outline(&tokens, &delimiters);
    let attributes = attribute_ranges(&tokens, &delimiters);
    let test_block = find_test_block(&tokens, &items);
    Ok(Self {
      tokens,
      delimiters,
      items,
      attributes,
      test_block,
    })
  }

  /// Whether the token at `index` lies inside an attribute.
  #[must_use]
  pub fn is_in_attribute(&self, index: usize) -> bool {
    self.attributes.iter().any(|range| range.contains(&index))
  }

  /// Whether the token at `index` lies inside the embedded test block.
  #[must_use]
  pub fn is_in_test_block(&self, index: usize) -> bool {
    self.test_block.is_some_and(|block| block.contains(index))
  }

  /// The token at `index`, if there is one.
  #[must_use]
  pub fn token(&self, index: usize) -> Option<&Token> {
    self.tokens.get(index)
  }
}

#[cfg(test)]
mod tests {
  use super::ParsedSource;

  #[test]
  fn parses_a_file_with_tests() {
    let parsed = ParsedSource::parse("#[must_use]\nfn code() {}\n#[cfg(test)]\nmod tests {}\n").unwrap();
    assert_eq!(parsed.items.len(), 2);
    assert!(parsed.is_in_attribute(1));
    assert!(!parsed.is_in_attribute(5));
    assert!(parsed.is_in_test_block(parsed.tokens.len() - 1));
    assert!(!parsed.is_in_test_block(0));
    assert!(parsed.token(0).is_some());
    assert!(parsed.token(999).is_none());
  }

  #[test]
  fn reports_errors() {
    assert!(ParsedSource::parse("fn (").is_err());
    assert!(ParsedSource::parse("\"open").is_err());
  }
}
