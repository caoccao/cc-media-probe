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

//! Comparing the actual targets with the expected ones.

use std::collections::BTreeSet;

use crate::findings::{Finding, Rule};
use crate::targets::constants::{MISSING_TARGET, UNEXPECTED_TARGET};
use crate::targets::expected::TargetEntry;

/// One finding for every target that exists but is not expected, and for
/// every expected target that does not exist. `path` names the list.
#[must_use]
pub fn compare_targets(path: &str, actual: &[TargetEntry], expected: &[TargetEntry]) -> Vec<Finding> {
  let actual_set: BTreeSet<&TargetEntry> = actual.iter().collect();
  let expected_set: BTreeSet<&TargetEntry> = expected.iter().collect();
  let describe = |entry: &TargetEntry| {
    let TargetEntry { package, kind, name } = entry;
    format!("{package} {kind} {name}")
  };
  let unexpected = actual_set.difference(&expected_set).map(|entry| {
    Finding::new(
      path,
      0,
      Rule::Targets,
      format!("{UNEXPECTED_TARGET}: {}", describe(entry)),
    )
  });
  let missing = expected_set
    .difference(&actual_set)
    .map(|entry| Finding::new(path, 0, Rule::Targets, format!("{MISSING_TARGET}: {}", describe(entry))));
  unexpected.chain(missing).collect()
}

#[cfg(test)]
mod tests {
  use super::compare_targets;
  use crate::targets::expected::TargetEntry;

  fn entry(package: &str, kind: &str, name: &str) -> TargetEntry {
    TargetEntry {
      package: package.to_owned(),
      kind: kind.to_owned(),
      name: name.to_owned(),
    }
  }

  #[test]
  fn reports_unexpected_and_missing_targets() {
    let actual = vec![entry("library", "lib", "library"), entry("library", "test", "stray")];
    let expected = vec![entry("library", "lib", "library"), entry("tool", "bin", "tool")];
    let messages: Vec<String> = compare_targets("targets.tsv", &actual, &expected)
      .into_iter()
      .map(|finding| finding.message)
      .collect();
    assert_eq!(
      messages,
      vec![
        "target not listed in the expected targets: library test stray",
        "expected target is missing: tool bin tool",
      ]
    );
    assert_eq!(compare_targets("targets.tsv", &expected, &expected), Vec::new());
  }
}
