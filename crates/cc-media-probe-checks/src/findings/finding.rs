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

//! One problem found by a check.

use std::fmt;

use crate::findings::rule::Rule;

/// One problem: the file and line, the rule it breaks and what is wrong.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Finding {
  /// The path of the file, relative to the repository root, with `/`.
  pub path: String,
  /// The line, counted from one; zero when the finding has no line.
  pub line: usize,
  /// The rule it breaks.
  pub rule: Rule,
  /// What is wrong.
  pub message: String,
}

impl Finding {
  /// A finding for `path` at `line`.
  #[must_use]
  pub fn new(path: &str, line: usize, rule: Rule, message: impl Into<String>) -> Self {
    Self {
      path: path.to_owned(),
      line,
      rule,
      message: message.into(),
    }
  }
}

impl fmt::Display for Finding {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    let Self {
      path,
      line,
      rule,
      message,
    } = self;
    if *line == 0 {
      write!(formatter, "{path}: {rule}: {message}")
    } else {
      write!(formatter, "{path}:{line}: {rule}: {message}")
    }
  }
}

#[cfg(test)]
mod tests {
  use super::Finding;
  use crate::findings::rule::Rule;

  #[test]
  fn displays_path_line_rule_and_message() {
    let finding = Finding::new("src/lib.rs", 7, Rule::Names, "abbreviation `buf`");
    assert_eq!(finding.to_string(), "src/lib.rs:7: names: abbreviation `buf`");
    let whole_file = Finding::new("Cargo.toml", 0, Rule::Targets, "missing target");
    assert_eq!(whole_file.to_string(), "Cargo.toml: targets: missing target");
  }
}
