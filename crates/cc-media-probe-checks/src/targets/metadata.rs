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

//! The targets that `cargo metadata` reports.

use crate::constants::metadata::{KIND_MEMBER, NAME_MEMBER, PACKAGES_MEMBER, TARGETS_MEMBER};
use crate::json::JsonValue;
use crate::targets::expected::TargetEntry;

/// Every target of every package in the metadata, one entry per kind.
#[must_use]
pub fn workspace_targets(metadata: &JsonValue) -> Vec<TargetEntry> {
  let mut entries = Vec::new();
  let packages = metadata.get(PACKAGES_MEMBER).map_or_default(JsonValue::elements);
  for package in packages {
    let package_name = package
      .get(NAME_MEMBER)
      .and_then(JsonValue::as_text)
      .unwrap_or_default();
    for target in package.get(TARGETS_MEMBER).map_or_default(JsonValue::elements) {
      let name = target.get(NAME_MEMBER).and_then(JsonValue::as_text).unwrap_or_default();
      for kind in target.get(KIND_MEMBER).map_or_default(JsonValue::elements) {
        entries.push(TargetEntry {
          package: package_name.to_owned(),
          kind: kind.as_text().unwrap_or_default().to_owned(),
          name: name.to_owned(),
        });
      }
    }
  }
  entries.sort();
  entries
}

#[cfg(test)]
mod tests {
  use super::workspace_targets;
  use crate::json::parse_json;

  #[test]
  fn lists_targets_per_kind() {
    let metadata = parse_json(r#"{"packages": [{"name": "tool", "targets": [{"name": "tool", "kind": ["bin"]}, {"name": "tool", "kind": ["lib", "rlib"]}]}]}"#).unwrap();
    let entries: Vec<String> = workspace_targets(&metadata)
      .into_iter()
      .map(|entry| format!("{} {} {}", entry.package, entry.kind, entry.name))
      .collect();
    assert_eq!(entries, vec!["tool bin tool", "tool lib tool", "tool rlib tool"]);
    assert_eq!(workspace_targets(&parse_json("{}").unwrap()), Vec::new());
  }
}
