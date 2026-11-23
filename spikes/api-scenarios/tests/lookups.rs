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

//! The tag lookups of `api.md` section 6.9, as the maintainer chose at the tenth review: every lookup sees the top
//! level only. A nested tag is an attribute of its parent (Matroska tags draft, section 3), so a `TITLE` under
//! `ORIGINAL` is the title of the original work and never the file's.

use cc_media_probe::MediaReport;
use cc_media_probe::model::{Tag, TagKey, TagValue};
use serde_json::{Value, json};

/// A tag in the flat form of a tree, with that name, that key where it has one, that text value where it has one,
/// and that parent where it has one.
fn flat_tag(name: &str, key: Option<&str>, text: Option<&str>, parent: Option<usize>) -> Value {
  let mut tag = json!({ "name": { "as_written": name }, "languages": [] });
  if let Some(key) = key {
    tag["key"] = json!(key);
  }
  if let Some(text) = text {
    tag["value"] = json!({ "kind": "text", "text": text });
  }
  if let Some(parent) = parent {
    tag["parent"] = json!(parent);
  }
  tag
}

/// A tag set of that scope, without targets, that holds those tags in their flat form.
fn tag_set(scope: &str, tags: Vec<Value>) -> Value {
  json!({
    "stream_targets": [],
    "edition_targets": [],
    "chapter_targets": [],
    "attachment_targets": [],
    "target_scope": scope,
    "tags": tags,
  })
}

/// A complete report without streams that holds those tag sets.
fn report(tag_sets: Vec<Value>) -> MediaReport {
  serde_json::from_value(json!({
    "schema_version": "1.0",
    "format": "Matroska",
    "streams": [],
    "chapter_editions": [],
    "tag_sets": tag_sets,
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
      "record": { "exhausted": [], "first_stopped": [] },
    },
    "usage": { "bytes_read": 0, "peak_working_memory": 0 },
    "sources": [],
  }))
  .unwrap()
}

fn text(tag: &Tag) -> Option<&str> {
  match tag.value() {
    Some(TagValue::Text(text)) => Some(text),
    _ => None,
  }
}

#[test]
fn a_nested_title_is_never_the_file_title() {
  // The tenth review's case: a whole-file set holds a valueless `ORIGINAL` with a valued `TITLE` child.
  let nested = report(vec![tag_set(
    "WholeFile",
    vec![
      flat_tag("ORIGINAL", None, None, None),
      flat_tag("TITLE", Some("Title"), Some("The original work"), Some(0)),
    ],
  )]);
  assert_eq!(nested.tag(&TagKey::Title), None);
  assert_eq!(nested.tags_with_key(&TagKey::Title).count(), 0);
  assert_eq!(nested.file_tags().count(), 1, "only the top level");
  // The child is reached through its parent.
  let original = nested.file_tags().next().unwrap();
  assert_eq!(text(&original.children()[0]), Some("The original work"));
}

#[test]
fn the_first_match_is_the_first_top_level_tag_in_report_order() {
  let sets = vec![
    // A set cut short before its targets: never among the file's tags, but among every tag with the key.
    tag_set(
      "Incomplete",
      vec![flat_tag("TITLE", Some("Title"), Some("cut short"), None)],
    ),
    tag_set(
      "WholeFile",
      vec![
        // A title without a value, which the value lookup skips.
        flat_tag("TITLE", Some("Title"), None, None),
        flat_tag("ORIGINAL", None, None, None),
        flat_tag("TITLE", Some("Title"), Some("nested"), Some(1)),
        flat_tag("TITLE", Some("Title"), Some("The file"), None),
      ],
    ),
    tag_set("WholeFile", vec![flat_tag("TITLE", Some("Title"), Some("later"), None)]),
  ];
  let titled = report(sets);
  assert_eq!(titled.tag(&TagKey::Title).and_then(text), Some("The file"));
  let every: Vec<Option<&str>> = titled.tags_with_key(&TagKey::Title).map(text).collect();
  assert_eq!(every, vec![Some("cut short"), None, Some("The file"), Some("later")]);
}

#[test]
fn a_superseded_set_of_a_live_stream_is_history_that_no_lookup_sees() {
  // The twelfth review's case, as the maintainer chose: in a live Matroska segment a Tags element between clusters
  // resets the earlier ones (RFC 9559 section 23.2). The first holds a title, and its replacement only an artist.
  let mut first = tag_set(
    "WholeFile",
    vec![flat_tag("TITLE", Some("Title"), Some("Old show"), None)],
  );
  first["superseded"] = json!(true);
  let replacement = tag_set(
    "WholeFile",
    vec![flat_tag("ARTIST", Some("Artist"), Some("Now on air"), None)],
  );
  let live = report(vec![first, replacement]);
  assert_eq!(live.tag(&TagKey::Title), None, "the reset title is gone");
  assert_eq!(live.tags_with_key(&TagKey::Title).count(), 0);
  assert_eq!(live.tag(&TagKey::Artist).and_then(text), Some("Now on air"));
  assert_eq!(live.file_tags().count(), 1);
  // The history stays in the tag sets, each marked.
  let marks: Vec<bool> = live.tag_sets().iter().map(|set| set.is_superseded()).collect();
  assert_eq!(marks, [true, false]);
  assert_eq!(text(&live.tag_sets()[0].tags()[0]), Some("Old show"));
}
