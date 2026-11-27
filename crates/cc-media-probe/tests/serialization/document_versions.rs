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

//! The versions of the records that a document holds (plan section 8.8; `api.md` section 12; `json-output.md`
//! sections 4.1 and 4.5). Every record that an entry holds, and the partial report of an error record, has the
//! document's version: reading compares it with the document's as soon as it has read it, before it decodes the
//! record's body, whatever the order of the members of either record, and a document that this version builds or
//! rebuilds gives every record that it holds its own version. The record that holds a nested one reads strictly, so
//! a null or a repeated name before the nested version is malformed input.

use std::io::Read;
use std::path::Path;

use cc_media_probe::document::{Document, DocumentEntry, Generator, SchemaVersion};
use cc_media_probe::error::ErrorRecord;
use cc_media_probe::source::PortablePath;
use cc_media_probe::{Detection, MediaReport, Rational};
use serde_json::{Value, json};

use crate::fixtures::{
  detection, document, entry, error_record, generator, members_of, partial_report, report, text_of, with, with_partial,
};

/// The version error of a nested record of that version in a document of version 1.0.
fn nested(version: &str) -> String {
  format!("the version error: a nested root record has version: {version}; the record that holds it has 1.0")
}

fn read_error(text: &str) -> String {
  serde_json::from_str::<Document>(text).unwrap_err().to_string()
}

/// A document of version 1.0 whose one entry, of that outcome, holds the record written as `record` under that
/// member, with the document's `results` before or after its version.
fn holding_text(outcome: &str, member: &str, record: &str, results_first: bool) -> String {
  let results =
    format!(r#""results": [{{"index": 0, "input": "a.mkv", "outcome": "{outcome}", "{member}": {record}}}]"#);
  let head = format!(r#""schema_version": "1.0", "generator": {}"#, generator());
  if results_first {
    format!("{{{results}, {head}}}")
  } else {
    format!("{{{head}, {results}}}")
  }
}

/// The error of each order of the document's members around an error record written as `record`.
fn errors_in_both_orders(record: &str) -> [String; 2] {
  [true, false].map(|results_first| read_error(&holding_text("error", "error", record, results_first)))
}

/// The members of an error record of that version as text, with the version first or last.
fn error_text(version: &str, version_first: bool, extra: &[(&str, &str)]) -> String {
  let mut members = members_of(&error_record(version));
  for (name, value) in extra {
    members.retain(|(kept, _)| kept != name);
    members.push(((*name).to_owned(), (*value).to_owned()));
  }
  let position = if version_first { 0 } else { members.len() };
  text_of(&members, Some((position, version)))
}

#[test]
fn reading_refuses_a_nested_record_of_another_version() {
  for (outcome, member, record) in [
    ("ok", "report", report as fn(&str) -> Value),
    ("ok", "detection", detection),
    ("error", "error", error_record),
  ] {
    for version in ["1.1", "2.0"] {
      let text = document("1.0", json!([entry(0, outcome, member, record(version))])).to_string();
      assert!(read_error(&text).starts_with(&nested(version)), "{member} {version}");
    }
    // A document of a newer minor version holds records of that version, which each give it.
    let newer = document("1.1", json!([entry(0, outcome, member, record("1.1"))]));
    let read: Document = serde_json::from_value(newer).unwrap();
    assert_eq!(read.schema_version().minor(), 1);
    let held = &read.results()[0];
    let versions = [
      held.report().map(MediaReport::schema_version),
      held.detection().map(Detection::schema_version),
      held.error().map(ErrorRecord::schema_version),
    ];
    assert_eq!(
      versions.into_iter().flatten().collect::<Vec<_>>(),
      [read.schema_version()],
      "{member}"
    );
    let older = document("1.1", json!([entry(0, outcome, member, record("1.0"))])).to_string();
    assert!(
      read_error(&older).starts_with("the version error: a nested root record has version: 1.0"),
      "{member}"
    );
  }
}

#[test]
fn a_document_reads_any_minor_version_of_its_major() {
  let read: Document = serde_json::from_value(document("1.4294967295", json!([]))).unwrap();
  assert_eq!(
    (read.schema_version().major(), read.schema_version().minor()),
    (1, u32::MAX)
  );
  for version in ["1.4294967296", "01.0", "2.0"] {
    let failure = read_error(&document(version, json!([])).to_string());
    assert!(failure.starts_with("the version error"), "{version}: {failure}");
  }
}

#[test]
fn reading_checks_the_partial_report_inside_an_error_record() {
  let failed = |version: &str| with_partial(error_record("1.0"), partial_report(version));
  let read: Document =
    serde_json::from_value(document("1.0", json!([entry(0, "error", "error", failed("1.0"))]))).unwrap();
  let partial = read.results()[0].error().unwrap().partial_report().unwrap();
  assert_eq!(partial.schema_version(), SchemaVersion::CURRENT);
  for version in ["1.1", "2.0"] {
    let text = document("1.0", json!([entry(0, "error", "error", failed(version))])).to_string();
    assert!(read_error(&text).starts_with(&nested(version)), "{version}");
  }
}

#[test]
fn reading_checks_every_record_of_an_entry_of_a_newer_outcome() {
  // An entry of a newer outcome may hold several records, as one that holds records of two versions shows; each
  // must have the document's version.
  let skipped = |detection_version: &str| {
    let entry = with(
      entry(0, "skipped", "error", error_record("1.0")),
      "detection",
      detection(detection_version),
    );
    document("1.0", json!([entry]))
  };
  assert!(crate::fixtures::reads_document(&skipped("1.0")));
  assert!(read_error(&skipped("1.1").to_string()).starts_with(&nested("1.1")));
}

/// A document of version 1.1 whose entries hold records of that version: an error record with its partial report, a
/// report and a detection.
fn newer_document() -> Value {
  let failed = with_partial(error_record("1.1"), partial_report("1.1"));
  document(
    "1.1",
    json!([
      entry(0, "error", "error", failed),
      entry(1, "ok", "report", report("1.1")),
      entry(2, "ok", "detection", detection("1.1")),
    ]),
  )
}

#[test]
fn a_document_rebuilt_from_newer_entries_gives_them_its_version() {
  let read: Document = serde_json::from_value(newer_document()).unwrap();
  assert_eq!(read.schema_version().minor(), 1);
  let entries = read.results().to_vec();
  let rebuilt = Document::from_results(Generator::new("catalog", "1"), entries.clone());
  let mut streamed = Vec::new();
  Document::serialize_results(
    &Generator::new("catalog", "1"),
    entries,
    &mut serde_json::Serializer::new(&mut streamed),
  )
  .unwrap();
  // Both writers write the same text, at version 1.0 throughout, down to the partial report.
  let text = serde_json::to_string(&rebuilt).unwrap();
  assert_eq!(String::from_utf8(streamed).unwrap(), text);
  let written: Value = serde_json::from_str(&text).unwrap();
  assert_eq!(written["schema_version"], json!("1.0"));
  for (index, member) in [(0, "error"), (1, "report"), (2, "detection")] {
    assert_eq!(
      written["results"][index][member]["schema_version"],
      json!("1.0"),
      "{member}"
    );
  }
  assert_eq!(
    written["results"][0]["error"]["partial_report"]["schema_version"],
    json!("1.0")
  );
  let read_back: Document = serde_json::from_str(&text).unwrap();
  assert_eq!(read_back, rebuilt);
  assert_eq!(
    read_back.results()[0]
      .error()
      .unwrap()
      .partial_report()
      .map(MediaReport::schema_version),
    Some(SchemaVersion::CURRENT)
  );
}

#[test]
fn a_record_that_a_document_holds_has_the_documents_version() {
  // A bare record read at 1.1 keeps that version and writes it again; a document gives it its own.
  let report: MediaReport = serde_json::from_value(report("1.1")).unwrap();
  assert_eq!(serde_json::to_value(&report).unwrap()["schema_version"], json!("1.1"));
  let entry = DocumentEntry::from_report(PortablePath::from_path(Path::new("movie.mkv")), report).unwrap();
  assert_eq!(
    entry
      .report()
      .map(MediaReport::schema_version)
      .map(|version| version.minor()),
    Some(1)
  );
  let document = Document::from_results(Generator::new("catalog", "1"), vec![entry]);
  let held = document.results()[0].report().unwrap();
  assert_eq!(held.schema_version(), SchemaVersion::CURRENT);
  assert_eq!(held.schema_version(), document.schema_version());
  let written = serde_json::to_value(&document).unwrap();
  assert_eq!(written["results"][0]["report"]["schema_version"], json!("1.0"));
}

#[test]
fn a_document_gives_its_version_to_each_record_where_it_stands() {
  // The review of the third slice's second part found the partial report of an error record allocated again when a
  // document gave it its version, even the version that it already had. The record and its partial report now keep
  // their places, read at this version and at a newer minor one.
  for version in ["1.0", "1.1"] {
    let failed = with_partial(error_record(version), partial_report(version));
    let record: ErrorRecord = serde_json::from_value(failed).unwrap();
    let entry = DocumentEntry::from_error(PortablePath::from_path(Path::new("input.bin")), record);
    let held = std::ptr::from_ref(entry.error().unwrap());
    let partial = std::ptr::from_ref(entry.error().unwrap().partial_report().unwrap());
    let document = Document::from_results(Generator::new("catalog", "1"), vec![entry]);
    let record = document.results()[0].error().unwrap();
    assert!(std::ptr::eq(record, held), "{version}");
    assert!(std::ptr::eq(record.partial_report().unwrap(), partial), "{version}");
    assert_eq!(record.schema_version(), SchemaVersion::CURRENT);
    assert_eq!(
      record.partial_report().unwrap().schema_version(),
      SchemaVersion::CURRENT
    );
  }
}

#[test]
fn a_nested_version_comes_before_a_member_that_does_not_decode() {
  // The thirteenth review's first case: a message of the wrong type, after the nested record's version and before
  // it, with the document's `results` before and after the document's version.
  for version_first in [true, false] {
    let record = error_text("1.1", version_first, &[("message", "42")]);
    for message in errors_in_both_orders(&record) {
      assert!(message.starts_with(&nested("1.1")), "{record}: {message}");
    }
  }
}

#[test]
fn a_nested_version_comes_before_a_null_after_it_in_both_orders_of_the_document() {
  let record = error_text("2.0", true, &[("future", "null")]);
  for message in errors_in_both_orders(&record) {
    assert!(message.starts_with(&nested("2.0")), "{message}");
  }
}

#[test]
fn a_null_or_a_repeated_name_before_a_nested_version_is_malformed_in_both_orders() {
  let null = error_text("2.0", false, &[("future", "null")]);
  for message in errors_in_both_orders(&null) {
    assert!(message.starts_with("a member holds null"), "{message}");
  }
  let repeated = format!(r#"{{"future": 1, "future": 2, {}"#, &error_text("2.0", false, &[])[1..]);
  for message in errors_in_both_orders(&repeated) {
    assert!(message.starts_with("a member is named twice"), "{message}");
  }
}

#[test]
fn a_nested_record_of_the_documents_version_reads_in_both_orders() {
  for version_first in [true, false] {
    let record = error_text("1.0", version_first, &[]);
    for results_first in [true, false] {
      let read: Document = serde_json::from_str(&holding_text("error", "error", &record, results_first)).unwrap();
      assert_eq!(
        read.results()[0].error().unwrap().message(),
        "the source could not be opened"
      );
    }
  }
}

#[test]
fn a_document_reads_its_nested_records_against_the_version_that_it_finds_last() {
  let report_members = members_of(&report("1.1"));
  let nested_report = text_of(&report_members, Some((0, "1.1")));
  let results =
    format!(r#""results": [{{"index": 0, "input": "movie.mkv", "outcome": "ok", "report": {nested_report}}}]"#);
  let document = |version: &str| {
    format!(
      r#"{{{results}, "generator": {}, "schema_version": "{version}"}}"#,
      generator()
    )
  };
  let read: Document = serde_json::from_str(&document("1.1")).unwrap();
  assert_eq!(read.schema_version().minor(), 1);
  assert!(read_error(&document("1.0")).starts_with(&nested("1.1")));
}

/// A reader that gives the bytes of one document, one at a time, and once it has given `trigger` of them, reads two
/// other values on its own, as a callback may: a document of version 1.1 and a rational written as an array. Each
/// is a read of its own, not one inside the document that waits for its bytes (fourteenth review).
struct CallbackReader {
  outer: Vec<u8>,
  position: usize,
  trigger: usize,
  inner: Option<(Result<Document, String>, Result<Rational, String>)>,
}

impl Read for CallbackReader {
  fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {
    if self.position == self.trigger && self.inner.is_none() {
      let document = document("1.1", json!([])).to_string();
      let read_document = serde_json::from_str::<Document>(&document).map_err(|error| error.to_string());
      let read_rational = serde_json::from_str::<Rational>("[1, 2]").map_err(|error| error.to_string());
      self.inner = Some((read_document, read_rational));
    }
    match (self.outer.get(self.position), buffer.first_mut()) {
      (Some(byte), Some(slot)) => {
        *slot = *byte;
        self.position = self.position.saturating_add(1);
        Ok(1)
      }
      _ => Ok(0),
    }
  }
}

#[test]
fn an_independent_read_inside_a_readers_callback_has_a_scope_of_its_own() {
  let outer = holding_text("error", "error", &error_text("1.0", true, &[]), false);
  let mut reader = CallbackReader {
    trigger: outer.find("\"kind\"").unwrap(),
    outer: outer.into_bytes(),
    position: 0,
    inner: None,
  };
  let read: Document = serde_json::from_reader(&mut reader).unwrap();
  assert_eq!(read.generator().name(), "catalog");
  let (document, rational) = reader.inner.unwrap();
  assert_eq!(document.unwrap().schema_version().minor(), 1);
  assert!(
    rational
      .unwrap_err()
      .starts_with("a record is written as an array of its values")
  );
}
