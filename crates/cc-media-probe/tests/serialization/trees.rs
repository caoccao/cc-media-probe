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

//! The flat trees of chapters and tags (`json-output.md` section 5.9): one array of every node in depth-first order,
//! each with the position of its parent, read back in its order and only in depth-first order, so a node that
//! returns to a closed subtree fails. A standalone chapter or tag is the flat array of its subtree, with exactly one
//! root. The trees add no depth, so `serde_json`'s default recursion limit reads a chain of 200,000 tags.

use cc_media_probe::MediaReport;
use cc_media_probe::model::{Chapter, Tag};
use serde_json::{Value, json};

use crate::fixtures::report;

const DEPTH_FIRST: [Option<usize>; 7] = [None, Some(0), Some(1), Some(1), Some(0), None, Some(5)];
const RETURNS_TO_A_CLOSED_SUBTREE: [Option<usize>; 3] = [None, None, Some(0)];
const RETURNS_PAST_A_SIBLING: [Option<usize>; 4] = [None, Some(0), Some(0), Some(1)];
const PARENT_AFTER_ITS_NODE: [Option<usize>; 2] = [Some(1), None];

fn tag(name: &str, parent: Option<usize>) -> Value {
  let mut node = json!({ "name": { "as_written": name }, "languages": [], "countries": [] });
  if let Some(parent) = parent {
    node["parent"] = json!(parent);
  }
  node
}

fn chapter(identifier: &str, parent: Option<usize>) -> Value {
  let mut node = json!({ "string_identifier": identifier, "titles": [] });
  if let Some(parent) = parent {
    node["parent"] = json!(parent);
  }
  node
}

fn nodes(parents: &[Option<usize>], node: fn(&str, Option<usize>) -> Value) -> Vec<Value> {
  (0..parents.len())
    .map(|position| node(&position.to_string(), parents[position]))
    .collect()
}

fn report_with(tags: Vec<Value>, chapters: Vec<Value>) -> Value {
  let mut report = report("1.0");
  report["tag_sets"] = json!([{
    "stream_targets": [],
    "edition_targets": [],
    "chapter_targets": [],
    "attachment_targets": [],
    "target_scope": "whole_file",
    "superseded": false,
  }]);
  report["tag_sets"][0]["tags"] = Value::Array(tags);
  report["chapter_editions"] = json!([{ "origin": "matroska_edition" }]);
  report["chapter_editions"][0]["chapters"] = Value::Array(chapters);
  report
}

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
  let text = report_with(nodes(&DEPTH_FIRST, tag), nodes(&DEPTH_FIRST, chapter));
  let report: MediaReport = serde_json::from_value(text.clone()).unwrap();
  assert_eq!(report.tag_sets()[0].tags().len(), 2);
  assert_eq!(report.chapter_editions()[0].chapters()[0].children().len(), 2);
  let written = serde_json::to_value(&report).unwrap();
  let tags = &written["tag_sets"][0]["tags"];
  let chapters = &written["chapter_editions"][0]["chapters"];
  for nodes in [tags, chapters] {
    assert_eq!(parents_of(nodes), parents_of(&text["tag_sets"][0]["tags"]));
  }
  for position in 0..DEPTH_FIRST.len() {
    assert_eq!(tags[position]["name"]["as_written"], json!(position.to_string()));
    assert_eq!(chapters[position]["string_identifier"], json!(position.to_string()));
  }
}

#[test]
fn a_flat_tree_out_of_depth_first_order_is_malformed() {
  for parents in [
    &RETURNS_TO_A_CLOSED_SUBTREE[..],
    &RETURNS_PAST_A_SIBLING[..],
    &PARENT_AFTER_ITS_NODE[..],
  ] {
    let tags = report_with(nodes(parents, tag), Vec::new());
    assert!(serde_json::from_value::<MediaReport>(tags).is_err(), "tags {parents:?}");
    let chapters = report_with(Vec::new(), nodes(parents, chapter));
    assert!(
      serde_json::from_value::<MediaReport>(chapters).is_err(),
      "chapters {parents:?}"
    );
  }
}

#[test]
fn a_standalone_node_is_the_flat_array_of_its_subtree_with_one_root() {
  let subtree = json!([chapter("0", None), chapter("1", Some(0)), chapter("2", Some(1))]);
  let read: Chapter = serde_json::from_value(subtree.clone()).unwrap();
  assert_eq!(read.string_identifier(), Some("0"));
  assert_eq!(serde_json::to_value(&read).unwrap(), subtree);
  let tags = json!([tag("0", None), tag("1", Some(0))]);
  let read: Tag = serde_json::from_value(tags.clone()).unwrap();
  assert_eq!(read.children().len(), 1);
  assert_eq!(serde_json::to_value(&read).unwrap(), tags);
  for malformed in [
    json!([]),
    json!([tag("0", None), tag("1", None)]),
    json!(tag("0", None)),
  ] {
    assert!(serde_json::from_value::<Tag>(malformed.clone()).is_err(), "{malformed}");
  }
}

#[test]
fn a_node_holds_no_children_and_no_null() {
  let mut node = tag("0", None);
  node["children"] = json!([tag("1", None)]);
  let read: Tag = serde_json::from_value(json!([node])).unwrap();
  assert!(read.children().is_empty(), "an unknown member, ignored");
  let mut null = tag("0", None);
  null["parent"] = Value::Null;
  assert!(serde_json::from_value::<Tag>(json!([null])).is_err());
}

#[test]
fn a_chain_of_two_hundred_thousand_tags_reads_with_the_default_recursion_limit() {
  let depth = 200_000;
  let chain: Vec<Value> = (0_usize..depth)
    .map(|position| tag("TITLE", position.checked_sub(1)))
    .collect();
  let text = serde_json::to_string(&report_with(chain, Vec::new())).unwrap();
  let report: MediaReport = serde_json::from_str(&text).unwrap();
  let written = serde_json::to_string(&report).unwrap();
  assert_eq!(written.len(), text.len());
  drop(report);
}

#[test]
fn a_nodes_members_are_read_as_they_come() {
  // The fourth review of the third slice's first part read an invalid `titles` before a null as the later null, and
  // with a valid extension on a later line as an error at the closing brace: a node's members were kept to its end.
  let message = serde_json::from_str::<Chapter>(r#"[{"titles": false, "extension": null}]"#)
    .unwrap_err()
    .to_string();
  assert!(message.starts_with("invalid type: boolean `false`"), "{message}");
  let text = "[{\"titles\": false,\n\"extension\": [1, 2, 3]\n}]";
  let error = serde_json::from_str::<Chapter>(text).unwrap_err();
  assert_eq!(error.line(), 1, "{error}");
  // `parent` is taken out where it stands, before the node's members or after them, and once only.
  let chapters: Vec<Chapter> = [
    "[{\"titles\": []}, {\"parent\": 0, \"titles\": []}]",
    "[{\"titles\": []}, {\"titles\": [], \"parent\": 0}]",
  ]
  .iter()
  .map(|text| serde_json::from_str::<Chapter>(text).unwrap())
  .collect();
  assert!(chapters.iter().all(|chapter| chapter.children().len() == 1));
  let twice = "[{\"titles\": []}, {\"parent\": 0, \"titles\": [], \"parent\": 0}]";
  assert!(serde_json::from_str::<Chapter>(twice).is_err());
  let null = serde_json::from_str::<Chapter>("[{\"titles\": [], \"extension\": null}]").unwrap_err();
  assert!(null.to_string().starts_with("a member holds null"), "{null}");
}
