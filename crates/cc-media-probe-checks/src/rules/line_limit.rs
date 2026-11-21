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

//! At most 500 lines per Rust file before its embedded tests (R31).

use crate::findings::{Finding, Rule};
use crate::rules::constants::{LINE_LIMIT_EXCEEDED, MAXIMUM_COUNTED_LINES};
use crate::syntax::TestBlock;

/// The lines that count against the limit: every physical line before the
/// embedded test block, or every line when there is none.
#[must_use]
pub fn counted_lines(source: &str, test_block: Option<TestBlock>) -> usize {
  match test_block {
    Some(block) => block.line.saturating_sub(1),
    None => source.lines().count(),
  }
}

/// Checks the limit of R31.
#[must_use]
pub fn check_line_limit(path: &str, source: &str, test_block: Option<TestBlock>) -> Option<Finding> {
  let counted = counted_lines(source, test_block);
  (counted > MAXIMUM_COUNTED_LINES).then(|| {
    Finding::new(
      path,
      MAXIMUM_COUNTED_LINES.saturating_add(1),
      Rule::LineLimit,
      format!("{LINE_LIMIT_EXCEEDED} ({counted} > {MAXIMUM_COUNTED_LINES})"),
    )
  })
}

#[cfg(test)]
mod tests {
  use super::{check_line_limit, counted_lines};
  use crate::syntax::ParsedSource;

  fn lines(count: usize) -> String {
    "// line\n".repeat(count)
  }

  fn finding_for(source: &str) -> Option<String> {
    let parsed = ParsedSource::parse(source).unwrap();
    check_line_limit("src/large.rs", source, parsed.test_block).map(|finding| finding.message)
  }

  #[test]
  fn counts_every_line_without_tests() {
    assert_eq!(counted_lines("a\nb\n", None), 2);
    assert_eq!(counted_lines("", None), 0);
    assert_eq!(finding_for(&lines(500)), None);
    let message = finding_for(&lines(501)).unwrap();
    assert!(message.ends_with("(501 > 500)"), "{message}");
  }

  #[test]
  fn stops_counting_at_the_embedded_tests() {
    let tests = "    // test line\n".repeat(2000);
    let source = format!("{}#[cfg(test)]\nmod tests {{\n{tests}}}\n", lines(500));
    assert_eq!(finding_for(&source), None);
    let over = format!("{}#[cfg(test)]\nmod tests {{\n}}\n", lines(501));
    assert!(finding_for(&over).is_some());
  }

  #[test]
  fn counts_test_only_files_in_full() {
    let source = format!("{}#[test]\nfn works() {{}}\n", lines(499));
    assert!(finding_for(&source).is_some());
  }
}
