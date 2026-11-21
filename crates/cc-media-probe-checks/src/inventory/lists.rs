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

//! The committed lists of the pure-Rust verification: reviewed build
//! scripts and allowed operating-system libraries.

use crate::constants::data_files::{COLUMN_SEPARATOR, COMMENT_PREFIX};
use crate::findings::{Finding, Rule};
use crate::inventory::constants::{ALLOWED_LIBRARY_COLUMNS, MALFORMED_LIST_LINE, REVIEWED_SCRIPT_COLUMNS};

/// Pairs read from a list, and the findings for its malformed lines.
pub type ParsedList = (Vec<(String, String)>, Vec<Finding>);

/// The reviewed build scripts: `name`, `version`, what it runs and what it
/// emits, tab-separated. Returns the `(name, version)` pairs.
#[must_use]
pub fn parse_reviewed_scripts(path: &str, text: &str) -> ParsedList {
  parse_pairs(path, text, |columns| columns.len() == REVIEWED_SCRIPT_COLUMNS)
}

/// The allowed libraries: `platform` and `library`, tab-separated. A
/// library that ends with `*` matches by prefix.
#[must_use]
pub fn parse_allowed_libraries(path: &str, text: &str) -> ParsedList {
  parse_pairs(path, text, |columns| columns.len() == ALLOWED_LIBRARY_COLUMNS)
}

fn parse_pairs(path: &str, text: &str, is_valid: impl Fn(&[&str]) -> bool) -> ParsedList {
  let mut pairs = Vec::new();
  let mut findings = Vec::new();
  for (offset, line) in text.lines().enumerate() {
    if line.trim().is_empty() || line.starts_with(COMMENT_PREFIX) {
      continue;
    }
    let columns: Vec<&str> = line.split(COLUMN_SEPARATOR).collect();
    match columns.as_slice() {
      [first, second, ..] if is_valid(&columns) => {
        pairs.push(((*first).to_owned(), (*second).to_owned()));
      }
      _ => findings.push(Finding::new(
        path,
        offset.saturating_add(1),
        Rule::Inventory,
        MALFORMED_LIST_LINE,
      )),
    }
  }
  (pairs, findings)
}

#[cfg(test)]
mod tests {
  use super::{parse_allowed_libraries, parse_reviewed_scripts};

  #[test]
  fn reads_reviewed_scripts() {
    let (pairs, findings) = parse_reviewed_scripts(
      "scripts.tsv",
      "# name\tversion\truns\temits\nserde\t1.0.229\trustc --version\tcfg flags\nbroken\n",
    );
    assert_eq!(pairs, vec![("serde".to_owned(), "1.0.229".to_owned())]);
    assert_eq!(findings.len(), 1);
  }

  #[test]
  fn reads_allowed_libraries() {
    let (pairs, findings) = parse_allowed_libraries(
      "libraries.tsv",
      "macos\t/usr/lib/libSystem.B.dylib\n\nwindows\tapi-ms-win-crt-*\nlinux\tlibc.so.6\textra\n",
    );
    assert_eq!(pairs.len(), 2);
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].line, 4);
  }
}
