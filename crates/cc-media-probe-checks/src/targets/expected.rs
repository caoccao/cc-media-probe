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

//! The committed list of expected targets.

use crate::constants::data_files::{COLUMN_SEPARATOR, COMMENT_PREFIX};
use crate::findings::{Finding, Rule};
use crate::targets::constants::{EXPECTED_COLUMNS, MALFORMED_EXPECTATION};

/// One target: its package, its kind and its name.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TargetEntry {
  /// The package that owns the target.
  pub package: String,
  /// The target kind, such as `lib`, `bin` or `test`.
  pub kind: String,
  /// The target name.
  pub name: String,
}

/// Reads the expected targets: one tab-separated line per target, with
/// blank lines and `#` comments ignored. Malformed lines become findings.
#[must_use]
pub fn parse_expected_targets(path: &str, text: &str) -> (Vec<TargetEntry>, Vec<Finding>) {
  let mut entries = Vec::new();
  let mut findings = Vec::new();
  for (offset, line) in text.lines().enumerate() {
    if line.trim().is_empty() || line.starts_with(COMMENT_PREFIX) {
      continue;
    }
    let columns: Vec<&str> = line.split(COLUMN_SEPARATOR).collect();
    match columns.as_slice() {
      [package, kind, name] if columns.len() == EXPECTED_COLUMNS => {
        entries.push(TargetEntry {
          package: (*package).to_owned(),
          kind: (*kind).to_owned(),
          name: (*name).to_owned(),
        });
      }
      _ => findings.push(Finding::new(
        path,
        offset.saturating_add(1),
        Rule::Targets,
        MALFORMED_EXPECTATION,
      )),
    }
  }
  (entries, findings)
}

#[cfg(test)]
mod tests {
  use super::parse_expected_targets;

  #[test]
  fn reads_entries_and_reports_malformed_lines() {
    let (entries, findings) = parse_expected_targets(
      "targets.tsv",
      "# package\tkind\tname\n\ncc-media-probe\tlib\tcc_media_probe\nbroken line\n",
    );
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].kind, "lib");
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].line, 4);
  }
}
