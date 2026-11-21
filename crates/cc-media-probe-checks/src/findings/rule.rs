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

//! The rules that findings belong to.

use std::fmt;

use crate::findings::constants::{
  BUILD_OUTPUT_LABEL, CONSTANTS_LABEL, COVERAGE_LABEL, HEADER_LABEL, INCLUSION_LABEL, INVENTORY_LABEL,
  LINE_LIMIT_LABEL, MODULE_FILE_LABEL, MODULE_ROOT_LABEL, NAMES_LABEL, SYNTAX_LABEL, TARGETS_LABEL,
};

/// The rule a finding breaks.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Rule {
  /// The copyright header of every Rust file (G1.2).
  Header,
  /// Module roots only declare (R30).
  ModuleRoot,
  /// Directory modules use `mod.rs` (R30).
  ModuleFile,
  /// At most 500 lines before the embedded tests (R31).
  LineLimit,
  /// No `#[path]` and no `include!` of Rust source (R31).
  Inclusion,
  /// Names are whole words (R32).
  Names,
  /// Literals live in constants modules (R33).
  Constants,
  /// A source file that cannot be analyzed.
  Syntax,
  /// Line coverage above 90% (R28).
  Coverage,
  /// Every target belongs to its expected package.
  Targets,
  /// The pure-Rust inventory (D4, first layer).
  Inventory,
  /// The outputs of build scripts (D4, third layer).
  BuildOutput,
}

impl Rule {
  /// The short label printed with a finding.
  #[must_use]
  pub fn label(self) -> &'static str {
    match self {
      Self::Header => HEADER_LABEL,
      Self::ModuleRoot => MODULE_ROOT_LABEL,
      Self::ModuleFile => MODULE_FILE_LABEL,
      Self::LineLimit => LINE_LIMIT_LABEL,
      Self::Inclusion => INCLUSION_LABEL,
      Self::Names => NAMES_LABEL,
      Self::Constants => CONSTANTS_LABEL,
      Self::Syntax => SYNTAX_LABEL,
      Self::Coverage => COVERAGE_LABEL,
      Self::Targets => TARGETS_LABEL,
      Self::Inventory => INVENTORY_LABEL,
      Self::BuildOutput => BUILD_OUTPUT_LABEL,
    }
  }
}

impl fmt::Display for Rule {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(self.label())
  }
}

#[cfg(test)]
mod tests {
  use super::Rule;

  #[test]
  fn every_rule_has_a_distinct_label() {
    let rules = [
      Rule::Header,
      Rule::ModuleRoot,
      Rule::ModuleFile,
      Rule::LineLimit,
      Rule::Inclusion,
      Rule::Names,
      Rule::Constants,
      Rule::Syntax,
      Rule::Coverage,
      Rule::Targets,
      Rule::Inventory,
      Rule::BuildOutput,
    ];
    let mut labels: Vec<String> = rules.iter().map(ToString::to_string).collect();
    labels.sort();
    labels.dedup();
    assert_eq!(labels.len(), rules.len());
  }
}
