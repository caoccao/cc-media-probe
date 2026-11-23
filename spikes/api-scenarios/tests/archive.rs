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

//! The archive cases of the sixth and seventh reviews of `api.md`: records that a reader must refuse or read exactly.
//! An entry keeps the outcome constraints that its known statuses decide, a portable path describes one path, a root
//! record checks its version before anything else, the numbers of section 12 keep their invariants, a member never
//! holds null, and a standalone chapter or tag is the flat form of its subtree.

use std::path::Path;

use cc_media_probe::MediaReport;
use cc_media_probe::document::{Document, DocumentEntry, EntryOutcome, Generator};
use cc_media_probe::error::ErrorRecord;
use cc_media_probe::model::{Chapter, Rational, Tag, TagValue};
use cc_media_probe::probe_bytes;
use cc_media_probe::source::{PortablePath, SourceEntry};
use serde_json::{Value, json};

/// A report without streams, of that version and with that completeness.
fn report_text(version: &str, completeness: Value) -> Value {
  json!({
    "schema_version": version,
    "format": "Matroska",
    "streams": [],
    "chapter_editions": [],
    "tag_sets": [],
    "attachments": [],
    "diagnostics": [],
    "completeness": completeness,
    "usage": { "bytes_read": 0, "peak_working_memory": 0 },
    "sources": [],
  })
}

/// A completeness record with that discovery status, those reasons and that tags status.
fn completeness(discovery: &str, reasons: Value, tags: Value) -> Value {
  json!({
    "detection": "Complete",
    "discovery": discovery,
    "headers": "Complete",
    "tags": tags,
    "chapters": "Complete",
    "attachments": "Complete",
    "reasons": reasons,
    "record": { "exhausted": ["Streams"], "first_stopped": [["Streams", "Discovery"]] },
  })
}

/// A document of one entry that holds that report under that outcome.
fn report_document(report: Value, outcome: &str) -> Value {
  json!({
    "schema_version": "1.0",
    "generator": { "name": "catalog", "version": "1" },
    "results": [{ "index": 0, "input": "movie.mkv", "outcome": outcome, "report": report }],
  })
}

/// The report of the review: discovery explicitly incomplete, a recorded discovery-limit reason, and a tags status
/// from a newer version.
fn known_shortfall_with_a_newer_status() -> Value {
  let reasons = json!([{
    "part": "Discovery",
    "reason": { "kind": "LimitExhausted", "limit": "Streams" },
  }]);
  report_text(
    "1.0",
    completeness("Incomplete", reasons, json!({ "Unrecognized": "partly_skipped" })),
  )
}

#[test]
fn a_known_shortfall_rules_out_ok_beside_a_newer_status() {
  let report = known_shortfall_with_a_newer_status();
  assert!(serde_json::from_value::<Document>(report_document(report.clone(), "partial")).is_ok());
  assert!(
    serde_json::from_value::<Document>(report_document(report.clone(), "ok")).is_err(),
    "a known shortfall rules out ok"
  );
  assert!(
    serde_json::from_value::<Document>(report_document(report.clone(), "error")).is_err(),
    "a report rules out error"
  );
  // The constructor derives the same outcome: the newer status cannot make an incomplete report complete.
  let read: MediaReport = serde_json::from_value(report).unwrap();
  assert!(!read.completeness().is_complete());
  let input = PortablePath::from_path(Path::new("movie.mkv"));
  let entry = DocumentEntry::from_report(input, read).expect("the outcome is derivable");
  assert_eq!(entry.outcome(), &EntryOutcome::Partial);
}

#[test]
fn a_newer_status_alone_leaves_ok_and_partial_open() {
  let newer = report_text(
    "1.0",
    completeness("Complete", json!([]), json!({ "Unrecognized": "partly_skipped" })),
  );
  for outcome in ["ok", "partial"] {
    assert!(serde_json::from_value::<Document>(report_document(newer.clone(), outcome)).is_ok());
  }
  assert!(serde_json::from_value::<Document>(report_document(newer.clone(), "error")).is_err());
  // The constructor refuses such a report, and the refusal takes `ok` or `partial` but never `error`, so every
  // entry it builds reads back.
  let read: MediaReport = serde_json::from_value(newer).unwrap();
  let refused = DocumentEntry::from_report(PortablePath::from_path(Path::new("movie.mkv")), read).unwrap_err();
  let refused = refused.with_outcome(EntryOutcome::Error).unwrap_err();
  assert_eq!(
    refused.with_outcome(EntryOutcome::Ok).unwrap().outcome(),
    &EntryOutcome::Ok
  );
}

/// A document of one entry with these input members.
fn input_document(members: Value) -> Value {
  let record = serde_json::to_value(probe_bytes(b"").unwrap_err().to_record()).unwrap();
  let mut entry = json!({ "index": 0, "outcome": "error", "error": record });
  for (name, value) in members.as_object().unwrap() {
    entry[name] = value.clone();
  }
  json!({
    "schema_version": "1.0",
    "generator": { "name": "catalog", "version": "1" },
    "results": [entry],
  })
}

#[test]
fn a_portable_path_describes_one_path() {
  // The raw bytes spell `other`, which is valid UTF-8, so the path was not lossy, and its display disagrees.
  let other = input_document(json!({
    "input": "input.bin",
    "input_lossy": true,
    "input_raw": { "encoding": "unix_bytes", "data": "6f74686572" },
  }));
  assert!(
    serde_json::from_value::<Document>(other).is_err(),
    "valid UTF-8 is never lossy"
  );
  // A lossy display that is not the lossy decoding of the raw bytes describes another path.
  let disagreeing = input_document(json!({
    "input": "input.bin",
    "input_lossy": true,
    "input_raw": { "encoding": "unix_bytes", "data": "ff2e62696e" },
  }));
  assert!(
    serde_json::from_value::<Document>(disagreeing).is_err(),
    "the display agrees with the raw form"
  );
  let agreeing = input_document(json!({
    "input": "\u{fffd}.bin",
    "input_lossy": true,
    "input_raw": { "encoding": "unix_bytes", "data": "ff2e62696e" },
  }));
  assert!(serde_json::from_value::<Document>(agreeing).is_ok());
  // A Windows path whose units are valid UTF-16 was not lossy either.
  let wide = input_document(json!({
    "input": "a",
    "input_lossy": true,
    "input_raw": { "encoding": "windows_wide", "data": "0061" },
  }));
  assert!(
    serde_json::from_value::<Document>(wide).is_err(),
    "valid UTF-16 is never lossy"
  );
  let surrogate = input_document(json!({
    "input": "\u{fffd}",
    "input_lossy": true,
    "input_raw": { "encoding": "windows_wide", "data": "d800" },
  }));
  assert!(serde_json::from_value::<Document>(surrogate).is_ok());
  // The flag is written only as true, and only with the raw form.
  let false_flag = input_document(json!({ "input": "input.bin", "input_lossy": false }));
  assert!(
    serde_json::from_value::<Document>(false_flag).is_err(),
    "a flag of false is never written"
  );
}

/// One tag of a flat form, with that parent and that value; a value of `Value::Null` leaves the member out, since the
/// form writes no null.
fn flat_tag(parent: Option<usize>, value: Value) -> Value {
  let mut tag = json!({ "name": { "as_written": "NODE" }, "languages": [] });
  if !value.is_null() {
    tag["value"] = value;
  }
  if let Some(parent) = parent {
    tag["parent"] = json!(parent);
  }
  tag
}

/// A report whose one tag set holds these tags in its flat form.
fn report_with_tags(tags: Vec<Value>) -> Value {
  let mut report = report_text("1.0", completeness("Complete", json!([]), json!("Complete")));
  report["tag_sets"] = json!([{
    "stream_targets": [],
    "edition_targets": [],
    "chapter_targets": [],
    "attachment_targets": [],
    "target_scope": "WholeFile",
    "tags": tags,
  }]);
  report
}

/// A chain of tags that deep, each the only child of the one before, with a text value at the end.
fn deep_tags(depth: usize) -> Value {
  let tags = (0..depth)
    .map(|position| {
      let value = if position + 1 == depth {
        json!({ "kind": "text", "text": "x" })
      } else {
        Value::Null
      };
      flat_tag(position.checked_sub(1), value)
    })
    .collect();
  report_with_tags(tags)
}

#[test]
fn a_tree_of_any_depth_reads_back_with_the_default_recursion_limit() {
  // Far beyond the 60 levels that a nested form allowed, and beyond what a recursive drop survives on a test thread.
  let depth = 200_000;
  let report: MediaReport = serde_json::from_value(deep_tags(depth)).unwrap();
  let mut tag = &report.tag_sets()[0].tags()[0];
  let mut levels = 1;
  while let Some(child) = tag.children().first() {
    tag = child;
    levels += 1;
  }
  assert_eq!(levels, depth);
  let copy = report.clone();
  assert_eq!(copy, report);
  let input = PortablePath::from_path(Path::new("movie.mkv"));
  let document = Document::from_results(
    Generator::new("catalog", "1"),
    vec![DocumentEntry::from_report(input, report).unwrap()],
  );
  let text = serde_json::to_string(&document).unwrap();
  let read: Document = serde_json::from_str(&text).unwrap();
  assert_eq!(read, document);
  // The flat form lists every node with its parent, and no `children`.
  let written: Value = serde_json::from_str(&text).unwrap();
  let tags = &written["results"][0]["report"]["tag_sets"][0]["tags"];
  assert_eq!(tags.as_array().unwrap().len(), depth);
  assert!(tags[0].get("parent").is_none() && tags[0].get("children").is_none());
  assert_eq!(tags[1]["parent"], json!(0));
}

#[test]
fn a_flat_form_keeps_the_order_of_siblings_and_needs_each_parent_first() {
  let text = |value: &str| json!({ "kind": "text", "text": value });
  // A root with two children, the first of which has a child of its own, in depth-first order.
  let tags = vec![
    flat_tag(None, Value::Null),
    flat_tag(Some(0), text("first")),
    flat_tag(Some(1), text("grandchild")),
    flat_tag(Some(0), text("second")),
  ];
  let report: MediaReport = serde_json::from_value(report_with_tags(tags)).unwrap();
  let root = &report.tag_sets()[0].tags()[0];
  let values: Vec<_> = root.children().iter().map(|tag| tag.value().cloned()).collect();
  assert_eq!(values.len(), 2);
  assert_eq!(root.children()[0].children().len(), 1);
  let round = serde_json::to_value(&report).unwrap();
  assert_eq!(round["tag_sets"][0]["tags"][3]["parent"], json!(0));
  // A parent that does not come before its node is malformed.
  for parent in [1, 2] {
    let tags = vec![flat_tag(None, Value::Null), flat_tag(Some(parent), text("x"))];
    assert!(serde_json::from_value::<MediaReport>(report_with_tags(tags)).is_err());
  }
}

#[test]
fn a_tag_value_holds_the_member_named_after_its_kind() {
  let read = |value: Value| serde_json::from_value::<MediaReport>(report_with_tags(vec![flat_tag(None, value)]));
  let value_of = |value: Value| read(value).unwrap().tag_sets()[0].tags()[0].value().cloned().unwrap();
  assert_eq!(
    value_of(json!({ "kind": "text", "text": "Title" })),
    TagValue::Text("Title".to_owned())
  );
  // A kind that a newer version wrote keeps its name, and its payload unread (`tests/data_enums.rs`).
  let newer = value_of(json!({ "kind": "geolocation", "geolocation": { "latitude": "1" } }));
  assert!(matches!(&newer, TagValue::Unrecognized(kind) if kind.as_str() == "geolocation"));
  assert_eq!(newer.as_str(), "geolocation");
  // An integer beyond the exact range is the twin `integer_exact`, and each form is read only in its own range.
  let large = value_of(json!({ "kind": "integer", "integer_exact": "-9223372036854775808" }));
  let TagValue::Integer(integer) = large else {
    panic!("an integer");
  };
  assert_eq!(integer.to_i64(), Some(i64::MIN));
  let written = serde_json::to_value(TagValue::Integer(integer)).unwrap();
  assert_eq!(
    written,
    json!({ "kind": "integer", "integer_exact": "-9223372036854775808" })
  );
  for malformed in [
    json!({ "kind": "text" }),
    json!({ "kind": "text", "text": "x", "boolean": true }),
    json!({ "kind": "integer", "integer": 9_007_199_254_740_992_i64 }),
    json!({ "kind": "integer", "integer_exact": "42" }),
    json!({ "kind": "integer", "integer": 1, "integer_exact": "9007199254740993" }),
  ] {
    assert!(read(malformed.clone()).is_err(), "{malformed}");
  }
}

#[test]
fn every_root_record_checks_its_version() {
  let complete = || completeness("Complete", json!([]), json!("Complete"));
  assert!(
    serde_json::from_value::<MediaReport>(report_text("2.0", complete())).is_err(),
    "a bare report of another major version"
  );
  let mut record = serde_json::to_value(probe_bytes(b"").unwrap_err().to_record()).unwrap();
  for version in ["1.1", "2.0"] {
    record["partial_report"] = report_text(version, complete());
    assert!(
      serde_json::from_value::<ErrorRecord>(record.clone()).is_err(),
      "a bare error record whose partial report has the version {version}"
    );
  }
  // The stub decodes the members in the order written, so the version error comes first here, where the version
  // does. Written last, as `serde_json::Value` orders the members, the generator fails first in the stub; the
  // library reads the version before anything else, wherever it stands (plan section 8.8; task 5).
  let document = r#"{ "schema_version": "2.0", "generator": 5, "results": [] }"#;
  let error = serde_json::from_str::<Document>(document).unwrap_err().to_string();
  assert!(
    error.contains("major version"),
    "the version error comes first, not: {error}"
  );
}

#[test]
fn a_rational_reads_back_only_in_the_form_that_rational_new_gives() {
  let read = |numerator: i64, denominator: u64| {
    serde_json::from_value::<Rational>(json!({ "numerator": numerator, "denominator": denominator }))
  };
  assert_eq!(read(30_000, 1_001).unwrap(), Rational::new(30_000, 1_001).unwrap());
  assert_eq!(
    Rational::new(2, 4).unwrap(),
    Rational::new(1, 2).unwrap(),
    "the constructor reduces"
  );
  for (numerator, denominator) in [(1, 0), (2, 4), (9_007_199_254_740_992, 1), (1, 9_007_199_254_740_992)] {
    assert!(read(numerator, denominator).is_err(), "{numerator}/{denominator}");
  }
}

#[test]
fn a_quantity_is_its_number_or_its_exact_twin() {
  let read = |members: Value| {
    let mut entry = json!({ "identifier": "0" });
    for (name, value) in members.as_object().unwrap() {
      entry[name] = value.clone();
    }
    serde_json::from_value::<SourceEntry>(entry).map(|entry| entry.bytes_read())
  };
  assert_eq!(
    read(json!({ "bytes_read_exact": "9007199254740993" })).unwrap(),
    Some(9_007_199_254_740_993)
  );
  assert_eq!(read(json!({ "bytes_read": 4096 })).unwrap(), Some(4096));
  assert_eq!(read(json!({})).unwrap(), None);
  for malformed in [
    json!({ "bytes_read": 9_007_199_254_740_992_u64 }),
    json!({ "bytes_read_exact": "4096" }),
    json!({ "bytes_read_exact": "09007199254740993" }),
    json!({ "bytes_read_exact": "+9007199254740993" }),
    json!({ "bytes_read": 1, "bytes_read_exact": "9007199254740993" }),
  ] {
    assert!(read(malformed.clone()).is_err(), "{malformed}");
  }
  // The writer uses the twin beyond the range, and only there.
  let entry: SourceEntry =
    serde_json::from_value(json!({ "identifier": "0", "bytes_read_exact": "18446744073709551615" })).unwrap();
  let written = serde_json::to_value(entry).unwrap();
  assert!(written.get("bytes_read").is_none());
  assert_eq!(written["bytes_read_exact"], json!("18446744073709551615"));
}

#[test]
fn a_member_that_holds_null_is_malformed() {
  // The form writes no null, so no member reads null as absent (the maintainer's choice at the seventh review).
  for members in [
    json!({ "input": "movie.mkv", "input_lossy": null }),
    json!({ "input": "movie.mkv", "input_raw": null }),
    json!({ "input": "movie.mkv", "report": null }),
  ] {
    let document = input_document(members.clone());
    assert!(serde_json::from_value::<Document>(document).is_err(), "{members}");
  }
  for entry in [
    json!({ "identifier": "0", "bytes_read": 1, "bytes_read_exact": null }),
    json!({ "identifier": "0", "path": null }),
  ] {
    assert!(serde_json::from_value::<SourceEntry>(entry.clone()).is_err(), "{entry}");
  }
}

#[test]
fn chapters_are_flat_and_a_textual_identifier_stays_its_text() {
  let chapter = |parent: Option<usize>, string_identifier: &str| {
    let mut chapter = json!({ "unique_identifier": "1", "string_identifier": string_identifier, "titles": [] });
    if let Some(parent) = parent {
      chapter["parent"] = json!(parent);
    }
    chapter
  };
  let mut report = report_text("1.0", completeness("Complete", json!([]), json!("Complete")));
  report["chapter_editions"] = json!([{
    "chapters": [chapter(None, "chapter-one"), chapter(Some(0), "chapter-one-a")],
  }]);
  let read: MediaReport = serde_json::from_value(report).unwrap();
  let edition = &read.chapter_editions()[0];
  assert_eq!(edition.chapters()[0].string_identifier(), Some("chapter-one"));
  assert_eq!(
    edition.chapter_at(&[0, 0]).unwrap().string_identifier(),
    Some("chapter-one-a")
  );
  let written = serde_json::to_value(&read).unwrap();
  assert_eq!(written["chapter_editions"][0]["chapters"][1]["parent"], json!(0));
  assert_eq!(
    written["chapter_editions"][0]["chapters"][0]["string_identifier"],
    json!("chapter-one")
  );
  // A standalone chapter is the flat form of its subtree, the chapter itself first and without a parent.
  let alone = serde_json::to_value(&edition.chapters()[0]).unwrap();
  assert!(alone[0].get("parent").is_none());
  assert_eq!(alone[1]["parent"], json!(0));
  assert_eq!(serde_json::from_value::<Chapter>(alone).unwrap(), edition.chapters()[0]);
}

#[test]
fn a_standalone_tag_is_the_flat_form_of_its_subtree_with_one_root() {
  let text = |value: &str| json!({ "kind": "text", "text": value });
  let tags = vec![
    flat_tag(None, Value::Null),
    flat_tag(Some(0), text("first")),
    flat_tag(Some(1), text("grandchild")),
  ];
  let report: MediaReport = serde_json::from_value(report_with_tags(tags)).unwrap();
  let child = &report.tag_sets()[0].tags()[0].children()[0];
  let written = serde_json::to_value(child).unwrap();
  assert_eq!(written.as_array().unwrap().len(), 2, "the child and the grandchild");
  assert!(written[0].get("parent").is_none());
  assert_eq!(written[1]["parent"], json!(0));
  assert_eq!(&serde_json::from_value::<Tag>(written).unwrap(), child);
  // An array with no root, or with two, is not one subtree.
  for malformed in [json!([]), json!([flat_tag(None, text("a")), flat_tag(None, text("b"))])] {
    assert!(serde_json::from_value::<Tag>(malformed.clone()).is_err(), "{malformed}");
  }
}
