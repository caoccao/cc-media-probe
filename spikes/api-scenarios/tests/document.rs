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

//! The read side of `api.md` section 12: a document has the wire form of that section, a document read back gives
//! its entries through their accessors, and reading refuses a document that breaks its rules. A usage error never
//! stands beside results, an entry of a known outcome holds exactly one record that warrants that outcome, and every
//! nested record carries the document's version.

use std::path::Path;

use cc_media_probe::document::{Document, DocumentEntry, EntryOutcome, Generator, SchemaVersion, UsageError};
use cc_media_probe::probe_bytes;
use cc_media_probe::source::PortablePath;
use serde_json::{Value, json};

fn error_entries() -> Vec<DocumentEntry> {
  let mut entries = Vec::new();
  for (index, bytes) in [&b""[..], &b"not media"[..]].into_iter().enumerate() {
    let record = probe_bytes(bytes).unwrap_err().to_record();
    let input = PortablePath::from_path(Path::new("input.bin"));
    entries.push(DocumentEntry::from_error(index, input, record));
  }
  entries
}

#[test]
fn a_document_read_back_gives_its_entries() {
  let document = Document::from_results(Generator::new("catalog", "1.2.3"), error_entries());
  let text = serde_json::to_string(&document).unwrap();
  let read: Document = serde_json::from_str(&text).unwrap();
  assert_eq!(read, document);
  assert_eq!(read.schema_version(), SchemaVersion::CURRENT);
  assert_eq!(read.generator().name(), "catalog");
  assert_eq!(read.generator().version(), "1.2.3");
  assert_eq!(read.usage_error(), None);
  let mut kinds = Vec::new();
  for (index, entry) in read.results().iter().enumerate() {
    assert_eq!(entry.index(), index);
    assert_eq!(entry.input().display(), "input.bin");
    assert_eq!(entry.outcome(), &EntryOutcome::Error);
    assert!(entry.report().is_none());
    assert!(entry.detection().is_none());
    kinds.push(entry.error().unwrap().kind().as_str().to_owned());
  }
  assert_eq!(kinds, ["format_not_determined", "unsupported_format"]);
}

#[test]
fn a_usage_error_document_reads_back_without_results() {
  let error = UsageError::new("standard input (-) was named twice");
  let document = Document::from_usage_error(Generator::new("catalog", "1.2.3"), error);
  let text = serde_json::to_string(&document).unwrap();
  let read: Document = serde_json::from_str(&text).unwrap();
  assert_eq!(read, document);
  assert_eq!(
    read.usage_error().unwrap().message(),
    "standard input (-) was named twice"
  );
  assert!(read.results().is_empty());
}

#[test]
fn reading_refuses_a_usage_error_beside_results() {
  let with_results = serde_json::to_value(Document::from_results(Generator::new("catalog", "1"), error_entries()));
  let mut text = serde_json::to_value(Document::from_usage_error(
    Generator::new("catalog", "1"),
    UsageError::new("x"),
  ))
  .unwrap();
  text["results"] = with_results.unwrap()["results"].clone();
  let error = serde_json::from_value::<Document>(text).unwrap_err();
  assert_eq!(error.to_string(), "a document with a usage error holds no results");
}

#[test]
fn reading_refuses_an_entry_of_a_known_outcome_without_its_record() {
  let mut text = serde_json::to_value(Document::from_results(Generator::new("catalog", "1"), error_entries())).unwrap();
  text["results"][0]["error"] = Value::Null;
  assert!(serde_json::from_value::<Document>(text).is_err());
}

fn results_document() -> Value {
  serde_json::to_value(Document::from_results(Generator::new("catalog", "1"), error_entries())).unwrap()
}

fn read_error(text: Value) -> String {
  serde_json::from_value::<Document>(text).unwrap_err().to_string()
}

#[test]
fn the_document_has_the_wire_form_of_section_12() {
  let text = results_document();
  assert_eq!(text["schema_version"], json!("1.0"));
  assert!(text.get("usage_error").is_none(), "an absent member is omitted");
  let entry = &text["results"][0];
  assert_eq!(entry["outcome"], json!("error"));
  assert!(entry.get("report").is_none() && entry.get("detection").is_none());
  assert_eq!(entry["error"]["schema_version"], json!("1.0"));
  assert_eq!(entry["error"]["kind"], json!("format_not_determined"));
  assert!(entry["error"].get("partial_report").is_none());
}

#[test]
fn reading_refuses_an_outcome_that_the_record_does_not_warrant() {
  let mut text = results_document();
  text["results"][0]["outcome"] = json!("ok");
  assert_eq!(
    read_error(text),
    "an entry's outcome disagrees with the record it holds"
  );
}

#[test]
fn a_detection_entry_reads_back_only_with_the_outcome_it_warrants() {
  let detection = |outcome: &str, entry_outcome: &str| {
    let mut text = results_document();
    text["results"][0] = json!({
      "index": 0,
      "input": { "display": "input.bin", "lossy": false },
      "outcome": entry_outcome,
      "detection": { "schema_version": "1.0", "outcome": outcome },
    });
    serde_json::from_value::<Document>(text)
  };
  let read = detection("family_only", "partial").unwrap();
  let entry = &read.results()[0];
  assert_eq!(entry.outcome(), &EntryOutcome::Partial);
  assert_eq!(entry.detection().unwrap().outcome().as_str(), "family_only");
  assert!(detection("identified", "partial").is_err());
  assert!(detection("insufficient_data", "ok").is_err());
  // A detection outcome of a newer version keeps the outcome that was written.
  let newer = detection("identified_by_content", "partial").unwrap();
  assert_eq!(
    newer.results()[0].detection().unwrap().outcome().as_str(),
    "identified_by_content"
  );
}

#[test]
fn reading_refuses_a_nested_record_of_another_version() {
  let mut text = results_document();
  text["results"][1]["error"]["schema_version"] = json!("1.1");
  assert_eq!(
    read_error(text),
    "a nested record's schema version differs from the document's"
  );
}

#[test]
fn reading_refuses_a_document_of_another_major_version() {
  let mut text = results_document();
  text["schema_version"] = json!("2.0");
  assert_eq!(
    read_error(text),
    "the document has a schema major version that this version does not read"
  );
}

#[test]
fn an_outcome_of_a_newer_version_reads_back_as_written() {
  let mut text = results_document();
  text["results"][0]["outcome"] = json!("skipped");
  let read: Document = serde_json::from_value(text).unwrap();
  assert_eq!(read.results()[0].outcome().as_str(), "skipped");
}
