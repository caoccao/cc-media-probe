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

//! What a report names exists in it, as the maintainer chose at the review of the third slice's first part
//! (`json-output.md` section 10; `api.md` section 6.15): the source table names each source once and holds the
//! primary input; every stream index, of a carrier, a program, a diagnostic and a part's reason, names a stream of
//! the report; every source identifier, of a diagnostic, a reason, an offset, an alias and an error, names an entry
//! of its source table; and every resolved location, of a tag target and of a reference's owner, names an entity
//! of the report. Observations before an inventory hold the same source table, and no stream.

use cc_media_probe::MediaReport;
use cc_media_probe::error::ErrorRecord;
use serde_json::{Value, json};

use crate::fixtures::{
  error_record, failed_primary, partial_report, primary_source, report, stream, with, with_partial,
};

fn reads(report: &Value) -> bool {
  serde_json::from_value::<MediaReport>(report.clone()).is_ok()
}

fn companion(identifier: &str) -> Value {
  json!({ "identifier": identifier, "role": "subtitle_index", "outcome": "opened" })
}

/// A report with two streams and the sources given, whose companions are complete when a companion was requested.
fn two_streams(sources: Vec<Value>) -> Value {
  let mut report = report("1.0");
  report["streams"] = json!([stream(0, "video"), stream(1, "audio")]);
  if sources.len() > 1 {
    report["completeness"]["companions"] = json!("complete");
  }
  report["sources"] = Value::Array(sources);
  report
}

#[test]
fn the_source_table_names_each_source_once_and_holds_the_primary_input() {
  assert!(reads(&two_streams(vec![primary_source()])));
  assert!(reads(&two_streams(vec![primary_source(), companion("1")])));
  assert!(reads(&two_streams(vec![
    primary_source(),
    with(companion("1"), "alias_of", json!("0")),
  ])));
  for sources in [
    Vec::new(),
    vec![companion("1")],
    vec![primary_source(), primary_source()],
    vec![primary_source(), companion("1"), companion("1")],
    vec![primary_source(), with(companion("1"), "alias_of", json!("9"))],
  ] {
    let report = two_streams(sources);
    assert!(!reads(&report), "{}", report["sources"]);
  }
}

fn diagnostic(source: &str, stream: Option<usize>) -> Value {
  let mut diagnostic = json!({ "severity": "warning", "code": "test", "message": "m", "source_identifier": source });
  if let Some(stream) = stream {
    diagnostic["stream"] = json!(stream);
  }
  diagnostic
}

#[test]
fn every_stream_index_names_a_stream_of_the_report() {
  let with_stream_member = |position: usize, name: &str, value: Value| {
    let mut report = two_streams(vec![primary_source()]);
    report["streams"][position][name] = value;
    report
  };
  assert!(reads(&with_stream_member(1, "embedded_in", json!(0))));
  assert!(!reads(&with_stream_member(1, "embedded_in", json!(9))));
  assert!(!reads(&with_stream_member(1, "embedded_in", json!(1))));
  let program = |streams: Value| {
    with(
      two_streams(vec![primary_source()]),
      "programs",
      json!([{ "streams": streams, "map_state": "complete" }]),
    )
  };
  assert!(reads(&program(json!([0, 1]))));
  assert!(!reads(&program(json!([0, 2]))));
  let diagnostics = |diagnostic: Value| with(two_streams(vec![primary_source()]), "diagnostics", json!([diagnostic]));
  assert!(reads(&diagnostics(diagnostic("0", Some(1)))));
  assert!(!reads(&diagnostics(diagnostic("0", Some(2)))));
  assert!(!reads(&diagnostics(diagnostic("4", None))));
}

/// A report whose discovery is incomplete with that reason, kept and first.
fn discovery_reason(reason: Value) -> Value {
  let mut report = two_streams(vec![primary_source()]);
  report["completeness"]["discovery"] = json!("incomplete");
  report["completeness"]["reasons"] = Value::Array(vec![reason.clone()]);
  report["completeness"]["record"]["first_reasons"] = Value::Array(vec![reason]);
  report
}

#[test]
fn every_reason_names_a_stream_and_a_source_of_the_report() {
  let reason = |stream: usize, source: &str| {
    json!({
      "part": "discovery",
      "stream": stream,
      "source_identifier": source,
      "reason": { "kind": "truncated_input" },
    })
  };
  assert!(reads(&discovery_reason(reason(1, "0"))));
  assert!(!reads(&discovery_reason(reason(2, "0"))));
  assert!(!reads(&discovery_reason(reason(1, "3"))));
  let mut kept_only = discovery_reason(reason(1, "0"));
  kept_only["completeness"]["reasons"][0]["stream"] = json!(5);
  assert!(!reads(&kept_only));
  let range = |source: &str| {
    let mut report = two_streams(vec![primary_source()]);
    report["streams"][0]["declaration_range"] = json!({ "first_offset": { "source_identifier": source, "offset": 0 } });
    report
  };
  assert!(reads(&range("0")));
  assert!(!reads(&range("2")));
}

/// A report with two streams, one edition of one chapter that has one child, one attachment, and a tag set of the
/// scope `entities` with that target.
fn targeted(list: &str, location: Value) -> Value {
  let mut report = two_streams(vec![primary_source()]);
  report["chapter_editions"] = json!([{
    "origin": "matroska_edition",
    "chapters": [{ "string_identifier": "a", "titles": [] }, { "parent": 0, "string_identifier": "b", "titles": [] }],
  }]);
  report["attachments"] = json!([{}]);
  let mut set = json!({
    "stream_targets": [],
    "edition_targets": [],
    "chapter_targets": [],
    "attachment_targets": [],
    "target_scope": "entities",
    "superseded": false,
    "tags": [],
  });
  let mut target = json!({ "kind": "resolved" });
  target["location"] = location;
  set[list] = Value::Array(vec![target]);
  report["tag_sets"] = json!([set]);
  report
}

fn location(kind: &str, index: usize, path: &[usize]) -> Value {
  json!({ "kind": kind, "index": index, "chapter_path": path })
}

#[test]
fn every_resolved_location_names_an_entity_of_the_report() {
  for (list, target) in [
    ("stream_targets", location("stream", 1, &[])),
    ("edition_targets", location("edition", 0, &[])),
    ("chapter_targets", location("chapter", 0, &[0])),
    ("chapter_targets", location("chapter", 0, &[0, 0])),
    ("attachment_targets", location("attachment", 0, &[])),
  ] {
    assert!(reads(&targeted(list, target.clone())), "{target}");
  }
  for (list, target) in [
    ("stream_targets", location("stream", 2, &[])),
    ("edition_targets", location("edition", 1, &[])),
    ("chapter_targets", location("chapter", 1, &[0])),
    ("chapter_targets", location("chapter", 0, &[1])),
    ("chapter_targets", location("chapter", 0, &[0, 1])),
    ("chapter_targets", location("chapter", 0, &[0, 0, 0])),
    ("attachment_targets", location("attachment", 1, &[])),
  ] {
    assert!(!reads(&targeted(list, target.clone())), "{target}");
  }
  let owned = |owner: Value| {
    with(
      targeted("stream_targets", location("stream", 0, &[])),
      "external_references",
      json!([{ "kind": "data_reference", "owner": owner }]),
    )
  };
  assert!(reads(&owned(location("stream", 1, &[]))));
  assert!(reads(&owned(location("chapter", 0, &[0, 0]))));
  assert!(!reads(&owned(location("stream", 2, &[]))));
  assert!(!reads(&owned(location("attachment", 3, &[]))));
  assert!(!reads(&owned(location("chapter", 0, &[2]))));
}

#[test]
fn an_error_and_its_observations_name_the_sources_of_their_table() {
  let observations = |sources: Vec<Value>, diagnostics: Vec<Value>| {
    with(
      error_record("1.0"),
      "observations",
      json!({ "diagnostics": diagnostics, "sources": sources }),
    )
  };
  let read = |record: Value| serde_json::from_value::<ErrorRecord>(record).is_ok();
  assert!(read(observations(vec![failed_primary()], vec![diagnostic("0", None)])));
  assert!(!read(observations(Vec::new(), Vec::new())));
  assert!(!read(observations(vec![failed_primary()], vec![diagnostic("1", None)])));
  assert!(!read(observations(
    vec![failed_primary()],
    vec![diagnostic("0", Some(0))]
  )));
  assert!(!read(with(
    observations(vec![failed_primary()], Vec::new()),
    "source_identifier",
    json!("2")
  )));
  let partial = with_partial(error_record("1.0"), partial_report("1.0"));
  assert!(read(partial.clone()));
  assert!(!read(with(partial, "source_identifier", json!("1"))));
  // A record without a table names only the primary input, since only a probe that requests a companion holds a
  // table that names one (second review of the third slice's first part).
  assert!(!read(with(error_record("1.0"), "source_identifier", json!("1"))));
  let failed = with(companion("1"), "outcome", json!("failed"));
  assert!(read(with(
    observations(vec![primary_source(), failed], Vec::new()),
    "source_identifier",
    json!("1")
  )));
}
