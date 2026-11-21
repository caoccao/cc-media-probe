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

//! No `#[path]` attributes and no `include!` of Rust source (R31), so no file
//! can pass the line limit by pulling in another.

use crate::constants::keywords::{CONDITIONAL_ATTRIBUTE, INCLUDE_MACRO, PATH_ATTRIBUTE};
use crate::constants::punctuation::{EQUALS_SIGN, EXCLAMATION_MARK};
use crate::findings::{Finding, Rule};
use crate::lexer::Token;
use crate::rules::constants::{INCLUDE_FORBIDDEN, PATH_ATTRIBUTE_FORBIDDEN};
use crate::syntax::ParsedSource;
use crate::syntax::constants::PAIR_LENGTH;

/// Checks one file for `#[path]` attributes and `include!` calls.
#[must_use]
pub fn check_inclusion(path: &str, parsed: &ParsedSource) -> Vec<Finding> {
  let mut findings = Vec::new();
  for range in &parsed.attributes {
    let tokens = parsed.tokens.get(range.clone()).unwrap_or_default();
    if sets_a_path(tokens) {
      let line = tokens.first().map_or(0, |token| token.line);
      findings.push(Finding::new(path, line, Rule::Inclusion, PATH_ATTRIBUTE_FORBIDDEN));
    }
  }
  for (index, token) in parsed.tokens.iter().enumerate() {
    let next = parsed.token(index.saturating_add(1));
    if token.is_word(INCLUDE_MACRO) && next.is_some_and(|following| following.is_punctuation(EXCLAMATION_MARK)) {
      findings.push(Finding::new(path, token.line, Rule::Inclusion, INCLUDE_FORBIDDEN));
    }
  }
  findings
}

/// Whether the attribute is `#[path = …]` or a `cfg_attr` that sets one.
fn sets_a_path(attribute: &[Token]) -> bool {
  let Some(first_word) = attribute.iter().find(|token| token.is_identifier()) else {
    return false;
  };
  if first_word.is_word(PATH_ATTRIBUTE) {
    return true;
  }
  first_word.is_word(CONDITIONAL_ATTRIBUTE)
    && attribute
      .windows(PAIR_LENGTH)
      .any(|pair| matches!(pair, [name, equals] if name.is_word(PATH_ATTRIBUTE) && equals.is_punctuation(EQUALS_SIGN)))
}

#[cfg(test)]
mod tests {
  use super::check_inclusion;
  use crate::syntax::ParsedSource;

  fn lines(source: &str) -> Vec<usize> {
    let parsed = ParsedSource::parse(source).unwrap();
    check_inclusion("src/lib.rs", &parsed)
      .into_iter()
      .map(|finding| finding.line)
      .collect()
  }

  #[test]
  fn rejects_path_attributes() {
    assert_eq!(lines("#[path = \"other.rs\"]\nmod segment;\n"), vec![1]);
    assert_eq!(
      lines("\n#[cfg_attr(unix, path = \"unix.rs\")]\nmod platform;\n"),
      vec![2]
    );
  }

  #[test]
  fn rejects_include() {
    assert_eq!(lines("fn code() {\n    include!(\"generated.rs\");\n}\n"), vec![2]);
  }

  #[test]
  fn accepts_other_attributes_and_macros() {
    assert_eq!(
      lines(
        "#[cfg_attr(test, derive(Debug))]\nstruct Unit;\nconst TEXT: &str = include_str!(\"text.txt\");\n#[doc = \"path\"]\nfn path() {}\n"
      ),
      Vec::<usize>::new()
    );
    assert_eq!(lines("# [ ]\n"), Vec::<usize>::new());
  }
}
