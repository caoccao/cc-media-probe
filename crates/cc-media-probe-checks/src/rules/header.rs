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

//! The copyright header that every Rust file starts with (G1.2).

use crate::findings::{Finding, Rule};
use crate::rules::constants::{
  COPYRIGHT_LINE_NUMBER, COPYRIGHT_PREFIX, COPYRIGHT_SUFFIX, DIFFERENT_HEADER_LINE, HEADER_BODY,
  HEADER_BODY_FIRST_LINE_NUMBER, HEADER_OPENING, MALFORMED_COPYRIGHT, MISSING_HEADER, YEAR_DIGITS, YEAR_SEPARATOR,
};

/// Checks that `source` starts with the approved copyright header.
#[must_use]
pub fn check_header(path: &str, source: &str) -> Option<Finding> {
  let mut lines = source.lines();
  if lines.next() != Some(HEADER_OPENING) {
    return Some(Finding::new(path, 1, Rule::Header, MISSING_HEADER));
  }
  let copyright = lines.next().unwrap_or_default();
  let years = copyright
    .strip_prefix(COPYRIGHT_PREFIX)
    .and_then(|rest| rest.strip_suffix(COPYRIGHT_SUFFIX));
  if !years.is_some_and(is_year_or_range) {
    return Some(Finding::new(
      path,
      COPYRIGHT_LINE_NUMBER,
      Rule::Header,
      MALFORMED_COPYRIGHT,
    ));
  }
  for (offset, expected) in HEADER_BODY.iter().enumerate() {
    if lines.next() != Some(*expected) {
      let line = HEADER_BODY_FIRST_LINE_NUMBER.saturating_add(offset);
      return Some(Finding::new(path, line, Rule::Header, DIFFERENT_HEADER_LINE));
    }
  }
  None
}

/// Whether `text` is a year such as `2026` or a rising range such as
/// `2026-2027`.
fn is_year_or_range(text: &str) -> bool {
  match text.split_once(YEAR_SEPARATOR) {
    None => parse_year(text).is_some(),
    Some((first, last)) => parse_year(first)
      .zip(parse_year(last))
      .is_some_and(|(first_year, last_year)| first_year < last_year),
  }
}

fn parse_year(text: &str) -> Option<u32> {
  let is_year = text.len() == YEAR_DIGITS && text.chars().all(|character| character.is_ascii_digit());
  if is_year { text.parse().ok() } else { None }
}

#[cfg(test)]
mod tests {
  use super::check_header;

  const HEADER: &str = "/*
* Copyright (c) 2026. caoccao.com Sam Cao
* All rights reserved.

* Licensed under the Apache License, Version 2.0 (the \"License\");
* you may not use this file except in compliance with the License.
* You may obtain a copy of the License at

* http://www.apache.org/licenses/LICENSE-2.0

* Unless required by applicable law or agreed to in writing, software
* distributed under the License is distributed on an \"AS IS\" BASIS,
* WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
* See the License for the specific language governing permissions and
* limitations under the License.
*/
";

  fn line_of_finding(source: &str) -> Option<usize> {
    check_header("src/lib.rs", source).map(|finding| finding.line)
  }

  #[test]
  fn accepts_the_approved_header() {
    assert_eq!(line_of_finding(&format!("{HEADER}\n//! Docs.\n")), None);
    let range = HEADER.replace("2026.", "2026-2027.");
    assert_eq!(line_of_finding(&range), None);
  }

  #[test]
  fn rejects_a_missing_header() {
    assert_eq!(line_of_finding("//! Docs.\n"), Some(1));
    assert_eq!(line_of_finding(""), Some(1));
  }

  #[test]
  fn rejects_malformed_years() {
    for years in ["26", "2027-2026", "2026-", "year", "2026-2026"] {
      let source = HEADER.replace("2026", years);
      assert_eq!(line_of_finding(&source), Some(2), "{years}");
    }
    assert_eq!(line_of_finding("/*\n"), Some(2));
  }

  #[test]
  fn rejects_a_changed_body() {
    let source = HEADER.replace("All rights reserved.", "Some rights reserved.");
    assert_eq!(line_of_finding(&source), Some(3));
    let truncated: String = HEADER.lines().take(12).collect::<Vec<_>>().join("\n");
    assert_eq!(line_of_finding(&truncated), Some(13));
  }
}
