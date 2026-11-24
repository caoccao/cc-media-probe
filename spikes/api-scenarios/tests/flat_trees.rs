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

//! The ninth review's case of a flat tree out of depth-first order, `api.md` section 12: reading takes the trees of
//! chapters and tags only in the order that writing gives.

use cc_media_probe::MediaReport;
use serde_json::{Value, json};

/// A report without streams whose one tag set holds these tags and whose one edition holds these chapters.
fn report_with(tags: Vec<Value>, chapters: Vec<Value>) -> Value {
  json!({
    "schema_version": "1.0",
    "format": "Matroska",
    "streams": [],
    "chapter_editions": [{ "chapters": chapters }],
    "tag_sets": [{
      "stream_targets": [],
      "edition_targets": [],
      "chapter_targets": [],
      "attachment_targets": [],
      "target_scope": "WholeFile",
      "tags": tags,
      "superseded": false,
    }],
    "attachments": [],
    "diagnostics": [],
    "completeness": {
      "detection": "Complete",
      "discovery": "Complete",
      "headers": "Complete",
      "tags": "Complete",
      "chapters": "Complete",
      "attachments": "Complete",
      "reasons": [],
      "record": { "exhausted": [], "first_stopped": [], "first_reasons": [], "omitted_reasons": "0", "omitted_references": "0" },
    },
    "usage": { "bytes_read": 0, "peak_working_memory": 0 },
    "sources": [],
  })
}

/// One tag named so, with that parent.
fn tag(name: &str, parent: Option<usize>) -> Value {
  let mut tag = json!({ "name": { "as_written": name }, "languages": [] });
  if let Some(parent) = parent {
    tag["parent"] = json!(parent);
  }
  tag
}

/// One chapter of that textual identifier, with that parent.
fn chapter(identifier: &str, parent: Option<usize>) -> Value {
  let mut chapter = json!({ "string_identifier": identifier, "titles": [] });
  if let Some(parent) = parent {
    chapter["parent"] = json!(parent);
  }
  chapter
}

/// The parents of a forest in depth-first order: a root with two children, the first with a child of its own, and a
/// second root with one child.
const DEPTH_FIRST: [Option<usize>; 6] = [None, Some(0), Some(1), Some(0), None, Some(4)];

/// The parents of the review's case: a root, a second root, then a child of the first root, after its subtree closed.
const RETURNS_TO_A_CLOSED_SUBTREE: [Option<usize>; 3] = [None, None, Some(0)];

/// A child of a node whose subtree has closed, after a sibling of that node.
const RETURNS_PAST_A_SIBLING: [Option<usize>; 4] = [None, Some(0), Some(0), Some(1)];

fn tags_of(parents: &[Option<usize>]) -> Vec<Value> {
  parents
    .iter()
    .enumerate()
    .map(|(position, parent)| tag(&position.to_string(), *parent))
    .collect()
}

fn chapters_of(parents: &[Option<usize>]) -> Vec<Value> {
  parents
    .iter()
    .enumerate()
    .map(|(position, parent)| chapter(&position.to_string(), *parent))
    .collect()
}

/// The parent of each node of a flat tree as written.
fn parents_of(nodes: &Value) -> Vec<Option<u64>> {
  nodes
    .as_array()
    .unwrap()
    .iter()
    .map(|node| node.get("parent").and_then(Value::as_u64))
    .collect()
}

#[test]
fn a_flat_tree_in_depth_first_order_reads_back_in_its_order() {
  let text = report_with(tags_of(&DEPTH_FIRST), chapters_of(&DEPTH_FIRST));
  let report: MediaReport = serde_json::from_value(text.clone()).unwrap();
  let written = serde_json::to_value(&report).unwrap();
  let tags = &written["tag_sets"][0]["tags"];
  let chapters = &written["chapter_editions"][0]["chapters"];
  for nodes in [tags, chapters] {
    assert_eq!(parents_of(nodes), parents_of(&text["tag_sets"][0]["tags"]));
  }
  let names: Vec<_> = (0..DEPTH_FIRST.len())
    .map(|position| tags[position]["name"]["as_written"].clone())
    .collect();
  let identifiers: Vec<_> = (0..DEPTH_FIRST.len())
    .map(|position| chapters[position]["string_identifier"].clone())
    .collect();
  let expected: Vec<_> = (0..DEPTH_FIRST.len())
    .map(|position| json!(position.to_string()))
    .collect();
  assert_eq!(names, expected);
  assert_eq!(identifiers, expected);
}

#[test]
fn a_flat_tree_that_returns_to_a_closed_subtree_is_malformed() {
  for parents in [&RETURNS_TO_A_CLOSED_SUBTREE[..], &RETURNS_PAST_A_SIBLING[..]] {
    let tags = report_with(tags_of(parents), Vec::new());
    assert!(
      serde_json::from_value::<MediaReport>(tags).is_err(),
      "tags with the parents {parents:?}"
    );
    let chapters = report_with(Vec::new(), chapters_of(parents));
    assert!(
      serde_json::from_value::<MediaReport>(chapters).is_err(),
      "chapters with the parents {parents:?}"
    );
  }
}
