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

//! The dependency graph of the published crates, from `cargo metadata`.

use std::collections::{BTreeMap, BTreeSet};

use crate::constants::metadata::{KIND_MEMBER, NAME_MEMBER, PACKAGES_MEMBER, TARGETS_MEMBER};
use crate::inventory::constants::{
  BUILD_DEPENDENCY_KIND, BUILD_SCRIPT_KIND, DEPENDENCIES_MEMBER, DEPENDENCY_KINDS_MEMBER, IDENTIFIER_MEMBER,
  LINKS_MEMBER, NODES_MEMBER, PACKAGE_MEMBER, RESOLVE_MEMBER, VERSION_MEMBER,
};
use crate::json::JsonValue;

/// One crate of the graph.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CrateNode {
  /// The package name.
  pub name: String,
  /// The package version.
  pub version: String,
  /// Whether it has a build script.
  pub has_build_script: bool,
  /// Its `links` key, if any.
  pub links: Option<String>,
  /// The names of its direct build dependencies.
  pub build_dependencies: Vec<String>,
}

/// Every crate that the packages named in `roots` need at build or run
/// time: normal and build dependencies, on every platform, with the
/// features the metadata was resolved with.
#[must_use]
pub fn published_graph(metadata: &JsonValue, roots: &[&str]) -> Vec<CrateNode> {
  let packages: BTreeMap<&str, &JsonValue> = metadata
    .get(PACKAGES_MEMBER)
    .map_or_default(JsonValue::elements)
    .iter()
    .filter_map(|package| Some((package.get(IDENTIFIER_MEMBER)?.as_text()?, package)))
    .collect();
  let nodes: BTreeMap<&str, &JsonValue> = metadata
    .get(RESOLVE_MEMBER)
    .and_then(|resolve| resolve.get(NODES_MEMBER))
    .map_or_default(JsonValue::elements)
    .iter()
    .filter_map(|node| Some((node.get(IDENTIFIER_MEMBER)?.as_text()?, node)))
    .collect();
  let mut pending: Vec<&str> = packages
    .iter()
    .filter(|(_, package)| roots.contains(&text_of(package, NAME_MEMBER)))
    .map(|(identifier, _)| *identifier)
    .collect();
  let mut visited: BTreeSet<&str> = BTreeSet::new();
  let mut graph = Vec::new();
  while let Some(identifier) = pending.pop() {
    if !visited.insert(identifier) {
      continue;
    }
    let Some(package) = packages.get(identifier) else {
      continue;
    };
    let dependencies = nodes
      .get(identifier)
      .and_then(|node| node.get(DEPENDENCIES_MEMBER))
      .map_or_default(JsonValue::elements);
    let mut build_dependencies = Vec::new();
    for dependency in dependencies {
      let kinds: Vec<Option<&str>> = dependency
        .get(DEPENDENCY_KINDS_MEMBER)
        .map_or_default(JsonValue::elements)
        .iter()
        .map(|kind| kind.get(KIND_MEMBER).and_then(JsonValue::as_text))
        .collect();
      let is_needed = kinds
        .iter()
        .any(|kind| kind.is_none_or(|text| text == BUILD_DEPENDENCY_KIND));
      let Some(target) = dependency.get(PACKAGE_MEMBER).and_then(JsonValue::as_text) else {
        continue;
      };
      if kinds.contains(&Some(BUILD_DEPENDENCY_KIND)) {
        let name = packages.get(target).map_or_default(|found| text_of(found, NAME_MEMBER));
        build_dependencies.push(name.to_owned());
      }
      if is_needed {
        pending.push(target);
      }
    }
    graph.push(node_of(package, build_dependencies));
  }
  graph.sort();
  graph
}

fn node_of(package: &JsonValue, build_dependencies: Vec<String>) -> CrateNode {
  let has_build_script = package
    .get(TARGETS_MEMBER)
    .map_or_default(JsonValue::elements)
    .iter()
    .flat_map(|target| target.get(KIND_MEMBER).map_or_default(JsonValue::elements))
    .any(|kind| kind.as_text() == Some(BUILD_SCRIPT_KIND));
  CrateNode {
    name: text_of(package, NAME_MEMBER).to_owned(),
    version: text_of(package, VERSION_MEMBER).to_owned(),
    has_build_script,
    links: package
      .get(LINKS_MEMBER)
      .and_then(JsonValue::as_text)
      .map(str::to_owned),
    build_dependencies,
  }
}

fn text_of<'value>(value: &'value JsonValue, member: &str) -> &'value str {
  value.get(member).and_then(JsonValue::as_text).unwrap_or_default()
}

#[cfg(test)]
mod tests {
  use super::published_graph;
  use crate::json::parse_json;

  const METADATA: &str = r#"{
        "packages": [
            {"id": "library", "name": "cc-media-probe", "version": "0.1.0", "links": null, "targets": [{"kind": ["lib"]}]},
            {"id": "parser", "name": "parser", "version": "1.0.0", "links": "native", "targets": [{"kind": ["lib"]}, {"kind": ["custom-build"]}]},
            {"id": "helper", "name": "cc", "version": "1.2.0", "targets": [{"kind": ["lib"]}]},
            {"id": "tester", "name": "tester", "version": "2.0.0", "targets": [{"kind": ["lib"]}]},
            {"id": "unrelated", "name": "unrelated", "version": "3.0.0", "targets": []}
        ],
        "resolve": {"nodes": [
            {"id": "library", "deps": [
                {"pkg": "parser", "dep_kinds": [{"kind": null, "target": null}]},
                {"pkg": "tester", "dep_kinds": [{"kind": "dev", "target": null}]}
            ]},
            {"id": "parser", "deps": [{"pkg": "helper", "dep_kinds": [{"kind": "build", "target": null}]}, {"pkg": "library", "dep_kinds": [{"kind": null}]}, {"dep_kinds": []}]},
            {"id": "helper", "deps": []}
        ]}
    }"#;

  #[test]
  fn follows_normal_and_build_dependencies_only() {
    let metadata = parse_json(METADATA).unwrap();
    let graph = published_graph(&metadata, &["cc-media-probe"]);
    let names: Vec<&str> = graph.iter().map(|node| node.name.as_str()).collect();
    assert_eq!(names, vec!["cc", "cc-media-probe", "parser"]);
    let parser = &graph[2];
    assert!(parser.has_build_script);
    assert_eq!(parser.links.as_deref(), Some("native"));
    assert_eq!(parser.build_dependencies, vec!["cc"]);
    assert!(!graph[1].has_build_script);
  }

  #[test]
  fn tolerates_missing_parts() {
    let metadata = parse_json(r#"{"packages": [{"id": "library", "name": "cc-media-probe"}]}"#).unwrap();
    assert_eq!(published_graph(&metadata, &["cc-media-probe"]).len(), 1);
    assert_eq!(
      published_graph(&parse_json("{}").unwrap(), &["cc-media-probe"]),
      Vec::new()
    );
  }
}
