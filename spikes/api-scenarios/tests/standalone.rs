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

//! The thirteenth review's cases of `api.md` sections 3.3 and 12 and `json-output.md` section 10: a model type read
//! on its own obeys the rules that it obeys inside a root record. A member that holds null, at any depth, a map that
//! names a member twice, and a record written as a positional array fail at both entry points, and each case is
//! read both ways here. The review found a standalone `Rational` that took an unknown null and an unknown name
//! given twice, and a document whose `generator` was the array `["catalog", "1"]`.

use cc_media_probe::Rational;
use cc_media_probe::diagnostics::Diagnostic;
use cc_media_probe::document::{Document, Generator};
use cc_media_probe::model::{LocalizedText, TagValue, Usage};
use cc_media_probe::{MediaReport, probe_bytes};
use serde_json::{Value, json};

const NULL: &str = "a member holds null, which the form never writes";
const DUPLICATE: &str = "a map names each member once";
const POSITIONAL: &str = "a record is written as an object of named members, never as an array";

/// A record of the form, its standalone reader, and the document text with `@` where the record stands.
struct Record {
  name: &'static str,
  valid: Value,
  read_alone: fn(&str) -> Result<(), serde_json::Error>,
  holder: String,
}

/// A report without streams, with `@` at that member, as the text of a document's only entry.
fn report_with(member: &str, at: Value) -> Value {
  let mut report = json!({
    "schema_version": "1.0",
    "format": "Matroska",
    "streams": [],
    "chapter_editions": [],
    "tag_sets": [],
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
  });
  report[member] = at;
  report
}

/// The report with a provenance beside its value, since the two are present together (nineteenth review).
fn with_provenance(mut report: Value, member: &str) -> Value {
  report[member] = json!("Declared");
  report
}

/// A document whose one entry holds that report, as text with the version first.
fn document_holding(report: Value) -> String {
  let entry = json!({ "index": 0, "input": "a.mkv", "outcome": "ok", "report": report });
  format!(r#"{{"schema_version": "1.0", "generator": {{"name": "catalog", "version": "1"}}, "results": [{entry}]}}"#)
}

fn records() -> Vec<Record> {
  let chapter = json!([{ "chapters": [{ "string_identifier": "1", "titles": ["@"] }] }]);
  vec![
    Record {
      name: "Rational",
      valid: json!({ "numerator": 1, "denominator": 2 }),
      read_alone: |text| serde_json::from_str::<Rational>(text).map(drop),
      holder: document_holding(with_provenance(
        report_with("bit_rate", json!("@")),
        "bit_rate_provenance",
      )),
    },
    Record {
      name: "Usage",
      valid: json!({ "bytes_read": 0, "peak_working_memory": 0 }),
      read_alone: |text| serde_json::from_str::<Usage>(text).map(drop),
      holder: document_holding(report_with("usage", json!("@"))),
    },
    Record {
      name: "Diagnostic",
      valid: json!({ "severity": "Warning", "code": "matroska.void", "message": "m", "source_identifier": "0" }),
      read_alone: |text| serde_json::from_str::<Diagnostic>(text).map(drop),
      holder: document_holding(report_with("diagnostics", json!(["@"]))),
    },
    Record {
      name: "LocalizedText, in a node of a tree",
      valid: json!({ "text": "Act one", "languages": [], "countries": [] }),
      read_alone: |text| serde_json::from_str::<LocalizedText>(text).map(drop),
      holder: document_holding(report_with("chapter_editions", chapter)),
    },
    Record {
      name: "Generator",
      valid: json!({ "name": "catalog", "version": "1" }),
      read_alone: |text| serde_json::from_str::<Generator>(text).map(drop),
      holder: r#"{"schema_version": "1.0", "generator": "@", "results": []}"#.to_owned(),
    },
  ]
}

/// The record's members as text, without the braces, so that further members can follow them.
fn members(valid: &Value) -> String {
  let text = valid.to_string();
  text[1..text.len() - 1].to_owned()
}

/// The broken forms of a record, each with the start of the message that reading it gives.
fn broken(valid: &Value) -> Vec<(String, &'static str)> {
  let Value::Object(fields) = valid else { unreachable!() };
  let positional: Vec<&Value> = fields.values().collect();
  vec![
    (format!(r#"{{{}, "future": null}}"#, members(valid)), NULL),
    (
      format!(r#"{{{}, "future": {{"inner": [1, null]}}}}"#, members(valid)),
      NULL,
    ),
    (
      format!(r#"{{{}, "future": 1, "future": 2}}"#, members(valid)),
      DUPLICATE,
    ),
    (json!(positional).to_string(), POSITIONAL),
  ]
}

#[test]
fn a_record_obeys_the_same_rules_alone_and_inside_a_root_record() {
  for record in records() {
    let valid = record.valid.to_string();
    (record.read_alone)(&valid).unwrap_or_else(|error| panic!("{}: {error}", record.name));
    serde_json::from_str::<Document>(&record.holder.replace("\"@\"", &valid))
      .unwrap_or_else(|error| panic!("{} in a document: {error}", record.name));
    for (text, expected) in broken(&record.valid) {
      let alone = (record.read_alone)(&text).unwrap_err().to_string();
      assert!(alone.starts_with(expected), "{} alone, {text}: {alone}", record.name);
      let held = record.holder.replace("\"@\"", &text);
      let nested = serde_json::from_str::<Document>(&held).unwrap_err().to_string();
      assert!(nested.starts_with(expected), "{} nested, {text}: {nested}", record.name);
    }
  }
}

#[test]
fn a_root_record_is_never_a_positional_array() {
  let message = serde_json::from_str::<MediaReport>(r#"["1.0", "Matroska"]"#)
    .unwrap_err()
    .to_string();
  assert!(message.starts_with("invalid type: sequence"), "{message}");
  let message = serde_json::from_str::<Document>(r#"["1.0", {"name": "catalog", "version": "1"}, []]"#)
    .unwrap_err()
    .to_string();
  assert!(message.starts_with("invalid type: sequence"), "{message}");
}

#[test]
fn a_record_that_a_tag_value_holds_is_never_a_positional_array() {
  // A known kind reads its value from the members kept as written, which refuse an array for a record too.
  let positional = r#"{"kind": "binary", "binary": [1, "ff"]}"#;
  let message = serde_json::from_str::<TagValue>(positional).unwrap_err().to_string();
  assert!(message.starts_with(POSITIONAL), "{message}");
}

#[test]
fn an_error_record_read_alone_obeys_the_rules_of_its_members() {
  let mut record = serde_json::to_value(probe_bytes(b"").unwrap_err().to_record()).unwrap();
  record["observations"] = json!({ "diagnostics": [[ "Warning", "matroska.void", "m", "0" ]] });
  let message = serde_json::from_value::<cc_media_probe::error::ErrorRecord>(record)
    .unwrap_err()
    .to_string();
  assert!(message.starts_with(POSITIONAL), "{message}");
}
