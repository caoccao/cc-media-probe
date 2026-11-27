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

//! The document of `api.md` section 12 and `json-output.md` section 3: its members in their order, its entries
//! read back through their accessors with each entry's position as its index, and the rules that reading checks. A
//! usage error never stands beside results, an entry of a known outcome holds exactly one record, and each entry is
//! checked as soon as it is read, as the maintainer chose for the second part of the third slice, so the read stops
//! at the first broken entry and its error keeps the place where the read stopped, just after that entry. An entry
//! held before the document's version is checked once the version is read, so its error has the place there.

use std::path::Path;

use cc_media_probe::document::{Document, DocumentEntry, EntryOutcome, Generator, SchemaVersion, UsageError};
use cc_media_probe::source::PortablePath;
use serde_json::{Value, json};

use crate::fixtures::{document, entry, error_record, generator, reads_document, report, usage, with, without};

/// The entries of an empty input and of bytes that match nothing, each an error record of this version's probe.
fn error_entries() -> Vec<DocumentEntry> {
  [&b""[..], &b"not media"[..]]
    .into_iter()
    .map(|bytes| {
      let record = cc_media_probe::probe_bytes(bytes).unwrap_err().to_record();
      DocumentEntry::from_error(PortablePath::from_path(Path::new("input.bin")), record)
    })
    .collect()
}

fn read_error(document: Value) -> String {
  serde_json::from_value::<Document>(document).unwrap_err().to_string()
}

#[test]
fn a_document_read_back_gives_its_entries() {
  let document = Document::from_results(Generator::new("catalog", "1.2.3"), error_entries());
  let text = serde_json::to_string(&document).unwrap();
  let read: Document = serde_json::from_str(&text).unwrap();
  assert_eq!(read, document);
  assert_eq!(read.schema_version(), SchemaVersion::CURRENT);
  assert_eq!(
    (read.generator().name(), read.generator().version()),
    ("catalog", "1.2.3")
  );
  assert_eq!(read.usage_error(), None);
  let written: Value = serde_json::from_str(&text).unwrap();
  assert_eq!(read.results().len(), 2);
  for (index, entry) in read.results().iter().enumerate() {
    assert_eq!(
      written["results"][index]["index"],
      json!(index),
      "an entry's index is its position"
    );
    assert_eq!(entry.input().display(), "input.bin");
    assert_eq!(entry.outcome(), &EntryOutcome::Error);
    assert!(entry.report().is_none() && entry.detection().is_none());
    assert_eq!(entry.error().unwrap().kind().as_str(), "unknown_format");
  }
}

#[test]
fn a_document_has_its_members_in_the_order_of_section_3() {
  // Written as text, since `serde_json::Value` orders a map's members by name. An absent member is omitted.
  let entry = error_entries().swap_remove(0);
  let text = serde_json::to_string(&Document::from_results(Generator::new("catalog", "1"), vec![entry])).unwrap();
  let start = concat!(
    r#"{"schema_version":"1.0","generator":{"name":"catalog","version":"1"},"results":["#,
    r#"{"index":0,"input":"input.bin","outcome":"error","error":{"schema_version":"1.0","kind":"unknown_format""#,
  );
  assert!(text.starts_with(start), "{text}");
  assert!(
    !text.contains("usage_error") && !text.contains("input_lossy") && !text.contains("null"),
    "{text}"
  );
  // The document of `--json --version` has no entries.
  let version = Document::from_results(Generator::new("cc-media-probe-cli", "0.1.0"), Vec::new());
  assert_eq!(
    serde_json::to_string(&version).unwrap(),
    r#"{"schema_version":"1.0","generator":{"name":"cc-media-probe-cli","version":"0.1.0"},"results":[]}"#
  );
}

#[test]
fn the_document_of_a_usage_error_reads_back_without_results() {
  // The example of `json-output.md` section 3, as written there.
  let written = r#"{
  "schema_version": "1.0",
  "generator": {
    "name": "cc-media-probe-cli",
    "version": "0.1.0"
  },
  "usage_error": {
    "message": "standard input (-) was named twice"
  },
  "results": []
}"#;
  let error = UsageError::new("standard input (-) was named twice");
  let document = Document::from_usage_error(Generator::new("cc-media-probe-cli", "0.1.0"), error);
  assert_eq!(serde_json::to_string_pretty(&document).unwrap(), written);
  let read: Document = serde_json::from_str(written).unwrap();
  assert_eq!(read, document);
  assert_eq!(
    read.usage_error().unwrap().message(),
    "standard input (-) was named twice"
  );
  assert_eq!(read.results(), []);
}

#[test]
fn a_usage_error_never_stands_beside_results() {
  let results = json!([entry(0, "error", "error", error_record("1.0"))]);
  let usage_error = json!({ "message": "x" });
  let message = "a document with a usage error holds no results";
  // In either order of the two members, which a reader sees only once the document has been read.
  let mut before = json!({ "schema_version": "1.0", "generator": generator(), "usage_error": usage_error });
  before["results"] = results.clone();
  assert!(read_error(before).starts_with(message));
  let text = format!(
    r#"{{"schema_version": "1.0", "generator": {}, "results": {results}, "usage_error": {usage_error}}}"#,
    generator()
  );
  let failure = serde_json::from_str::<Document>(&text).unwrap_err().to_string();
  assert!(failure.starts_with(message), "{failure}");
  let empty = with(document("1.0", json!([])), "usage_error", usage_error);
  assert!(reads_document(&empty));
}

#[test]
fn the_entries_of_the_plan_read_back_with_their_input_as_a_string() {
  // The entries of plan section 8.9 that hold an error, at their positions 0 and 1, as this version writes them. A
  // member that this version does not know, `detected`, is ignored and not written back.
  let unread = without(without(error_record("1.0"), "source_failure_kind"), "source_identifier");
  let unread = with(unread, "message", json!("not probed"));
  let unknown = with(unread.clone(), "kind", json!("unknown_format"));
  let unsupported = with(
    with(unread, "kind", json!("unsupported_format")),
    "format",
    json!("mxf"),
  );
  let mut written = document(
    "1.0",
    json!([
      { "index": 0, "input": "notes.txt", "outcome": "error", "error": unknown },
      { "index": 1, "input": "tape.mxf", "outcome": "error", "error": unsupported },
    ]),
  );
  written["results"][1]["error"]["detected"] = json!({});
  let read: Document = serde_json::from_value(written.clone()).unwrap();
  let input = read.results()[0].input();
  assert_eq!(input.display(), "notes.txt");
  assert!(!input.is_lossy() && input.raw().is_none());
  assert_eq!(input.to_native().as_deref(), Some(Path::new("notes.txt")));
  assert_eq!(read.results()[1].error().unwrap().kind().as_str(), "unsupported_format");
  written["results"][1]["error"]
    .as_object_mut()
    .unwrap()
    .remove("detected");
  assert_eq!(serde_json::to_value(&read).unwrap(), written);
}

#[test]
fn an_entry_of_a_known_outcome_holds_exactly_one_record() {
  let message = "an entry of a known outcome holds exactly one of a report, a detection and an error record";
  let error = || error_record("1.0");
  let none = json!({ "index": 0, "input": "input.bin", "outcome": "error" });
  let unknown = json!({ "schema_version": "1.0", "outcome": "unknown", "candidates": [], "usage": usage() });
  let two = with(entry(0, "error", "error", error()), "detection", unknown);
  let report_and_error = with(entry(0, "ok", "report", report("1.0")), "error", error());
  for entry in [none, two, report_and_error] {
    assert!(
      read_error(document("1.0", json!([entry]))).starts_with(message),
      "{entry}"
    );
  }
}

#[test]
fn an_entrys_index_is_its_position() {
  let message = "an entry's index is its position in the results";
  let results = |indexes: [u64; 2]| {
    let entries: Vec<Value> = indexes
      .iter()
      .map(|index| with(entry(0, "error", "error", error_record("1.0")), "index", json!(index)))
      .collect();
    document("1.0", json!(entries))
  };
  assert!(reads_document(&results([0, 1])));
  for indexes in [[2, 3], [0, 0], [1, 0], [0, 2]] {
    assert!(read_error(results(indexes)).starts_with(message), "{indexes:?}");
  }
  // An index is a number within 2^53 - 1, never its text.
  let mut text = results([0, 1]);
  text["results"][0]["index"] = json!("0");
  assert!(!reads_document(&text));
}

#[test]
fn an_entry_is_checked_as_soon_as_it_is_read() {
  // The first entry is labeled `ok` beside an error record, and the second holds a null: the read stops at the
  // first, which a check after the whole document would have reached only after the second's null.
  let message = "an entry's outcome disagrees with the record it holds";
  let broken = entry(0, "ok", "error", error_record("1.0")).to_string();
  let holding_null = with(entry(1, "error", "error", error_record("1.0")), "future", Value::Null).to_string();
  let head = format!(
    r#"{{"schema_version": "1.0", "generator": {}, "results": ["#,
    generator()
  );
  let text = format!("{head}{broken},\n{holding_null}]}}");
  let first = serde_json::from_str::<Document>(&text).unwrap_err();
  assert!(first.to_string().starts_with(message), "{first}");
  // Without the broken entry, the null on the second line is what fails.
  let fixed = text.replacen(r#""outcome":"ok""#, r#""outcome":"error""#, 1);
  let null = serde_json::from_str::<Document>(&fixed).unwrap_err();
  assert!(null.to_string().starts_with("a member holds null"), "{null}");
  // The first error has a place in the input, just after its entry, before the null that the read never reached.
  assert!(first.line() > 0, "{first}");
  assert!(
    (first.line(), first.column()) < (null.line(), null.column()),
    "{first}; {null}"
  );
  assert_eq!(null.line(), 2);
}

#[test]
fn an_entry_held_before_the_version_fails_where_the_version_was_read() {
  // Entries that stand before the document's version are held until it is read and checked then, by the same rules,
  // so the error has the place where the read stopped after the version's value, not the place after the entry, as
  // the review of the third slice's second part found. serde_json looks for the end of the document before it
  // gives the place, so it passes the white space after the value, and the closing brace where the document ends.
  let message = "an entry's outcome disagrees with the record it holds";
  let broken = entry(0, "ok", "error", error_record("1.0")).to_string();
  let later = entry(1, "error", "error", error_record("1.0")).to_string();
  let version = r#""schema_version": "1.0""#;
  let before = format!(
    "{{\"results\": [{broken},\n{later}],\n{version}  ,\n\"generator\": {}}}",
    generator()
  );
  let error = serde_json::from_str::<Document>(&before).unwrap_err();
  assert!(error.to_string().starts_with(message), "{error}");
  assert_eq!((error.line(), error.column()), (3, version.len() + 2));
  let last = format!(
    "{{\"generator\": {}, \"results\": [{broken},\n{later}],\n{version}\n}}",
    generator()
  );
  let error = serde_json::from_str::<Document>(&last).unwrap_err();
  assert!(error.to_string().starts_with(message), "{error}");
  assert_eq!((error.line(), error.column()), (4, 1));
  // Read after the version, the same entry fails just after itself: serde_json looks for the end of the results
  // first, so it passes the comma and the line break after the entry, to the start of the next line.
  let after = format!(
    "{{{version}, \"generator\": {}, \"results\": [\n{broken},\n{later}]}}",
    generator()
  );
  let error = serde_json::from_str::<Document>(&after).unwrap_err();
  assert!(error.to_string().starts_with(message), "{error}");
  assert_eq!((error.line(), error.column()), (3, 0));
}
