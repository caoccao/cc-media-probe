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

//! The first layer of D4: what the graph contains, and what it must not.

use crate::findings::{Finding, Rule};
use crate::inventory::constants::{
  BUILD_SCRIPT_MARK, DECLARES_LINKS, LINKS_MARK, NATIVE_HELPER_DEPENDENCY, NATIVE_HELPERS, UNREVIEWED_BUILD_SCRIPT,
};
use crate::inventory::graph::CrateNode;

/// The summary lines and the findings for the graph. `reviewed` lists the
/// `(name, version)` of every reviewed build script.
#[must_use]
pub fn check_inventory(graph: &[CrateNode], reviewed: &[(String, String)]) -> (Vec<String>, Vec<Finding>) {
  let mut summary = Vec::new();
  let mut findings = Vec::new();
  for node in graph {
    let CrateNode {
      name,
      version,
      has_build_script,
      links,
      build_dependencies,
    } = node;
    let label = format!("{name} {version}");
    let mut line = label.clone();
    if *has_build_script {
      line = format!("{line}, {BUILD_SCRIPT_MARK}");
      let is_reviewed = reviewed
        .iter()
        .any(|(reviewed_name, reviewed_version)| reviewed_name == name && reviewed_version == version);
      if !is_reviewed {
        findings.push(Finding::new(&label, 0, Rule::Inventory, UNREVIEWED_BUILD_SCRIPT));
      }
    }
    if let Some(value) = links {
      line = format!("{line}, {LINKS_MARK}={value}");
      findings.push(Finding::new(&label, 0, Rule::Inventory, DECLARES_LINKS));
    }
    for helper in build_dependencies
      .iter()
      .filter(|dependency| NATIVE_HELPERS.contains(&dependency.as_str()))
    {
      findings.push(Finding::new(
        &label,
        0,
        Rule::Inventory,
        format!("{NATIVE_HELPER_DEPENDENCY}: {helper}"),
      ));
    }
    summary.push(line);
  }
  (summary, findings)
}

#[cfg(test)]
mod tests {
  use super::check_inventory;
  use crate::inventory::graph::CrateNode;

  fn node(name: &str, has_build_script: bool, links: Option<&str>, build_dependencies: &[&str]) -> CrateNode {
    CrateNode {
      name: name.to_owned(),
      version: "1.0.0".to_owned(),
      has_build_script,
      links: links.map(str::to_owned),
      build_dependencies: build_dependencies.iter().map(|name| (*name).to_owned()).collect(),
    }
  }

  #[test]
  fn reports_unreviewed_scripts_links_and_native_helpers() {
    let graph = vec![
      node("plain", false, None, &[]),
      node("reviewed", true, None, &["version_check"]),
      node("native", true, Some("native"), &["cc"]),
    ];
    let reviewed = vec![("reviewed".to_owned(), "1.0.0".to_owned())];
    let (summary, findings) = check_inventory(&graph, &reviewed);
    assert_eq!(
      summary,
      vec![
        "plain 1.0.0",
        "reviewed 1.0.0, build script",
        "native 1.0.0, build script, links=native"
      ]
    );
    let messages: Vec<String> = findings.into_iter().map(|finding| finding.to_string()).collect();
    assert_eq!(messages.len(), 3);
    assert!(messages[0].starts_with("native 1.0.0: inventory: build script not in the reviewed list"));
    assert!(messages[2].ends_with(": cc"));
  }
}
