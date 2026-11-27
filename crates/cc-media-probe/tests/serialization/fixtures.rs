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

//! The smallest records of the serialized form that this version reads, as JSON values that a test changes member
//! by member (`json-output.md` sections 4 and 7).

use serde_json::{Value, json};

/// A usage whose counters are all 0.
pub fn usage() -> Value {
  json!({
    "bytes_read": 0,
    "peak_working_memory": 0,
    "retained_data": 0,
    "retained_payloads": 0,
    "decompressed_output": 0,
    "work_units": 0,
    "companion_requests": 0,
    "sources_opened": 0,
  })
}

/// A fixed-size record that records no stop.
pub fn record() -> Value {
  json!({
    "exhausted": [],
    "first_stopped": [],
    "first_reasons": [],
    "omitted_diagnostics": "0",
    "omitted_declarations": "0",
    "omitted_reasons": "0",
    "omitted_references": "0",
  })
}

/// A record with those first reasons and that count of omitted reasons. A reason is omitted only when the retained
/// data cannot hold it (`api.md` section 5.3), so a count above 0 beside a first reason exhausts the retained data,
/// which stopped the part of the first of them first.
pub fn record_omitting(first_reasons: Value, omitted: &str) -> Value {
  let part = first_reasons.get(0).map(|first| first["part"].clone());
  let mut record = with(
    with(record(), "first_reasons", first_reasons),
    "omitted_reasons",
    json!(omitted),
  );
  if let (true, Some(part)) = (omitted != "0", part) {
    record["exhausted"] = json!(["retained_data"]);
    record["first_stopped"] = json!([{ "limit": "retained_data", "part": part }]);
  }
  record
}

/// A completeness of a report that is complete.
pub fn completeness() -> Value {
  json!({
    "detection": "complete",
    "discovery": "complete",
    "headers": "complete",
    "tags": "complete",
    "chapters": "complete",
    "attachments": "complete",
    "timing": "established",
    "companions": "not_applicable",
    "reasons": [],
    "record": record(),
  })
}

/// A complete report of that version without streams.
pub fn report(version: &str) -> Value {
  json!({
    "schema_version": version,
    "format": "matroska",
    "streams": [],
    "programs": [],
    "chapter_editions": [],
    "tag_sets": [],
    "attachments": [],
    "external_references": [],
    "diagnostics": [],
    "completeness": completeness(),
    "inventory_class": "declared",
    "usage": usage(),
    "sources": [primary_source()],
  })
}

/// The source entry of the primary input, opened.
pub fn primary_source() -> Value {
  json!({ "identifier": "0", "role": "primary", "outcome": "opened" })
}

/// The source entry of the primary input, which failed, as the source table of an error of the primary input
/// names it (`api.md` section 10.1).
pub fn failed_primary() -> Value {
  json!({ "identifier": "0", "role": "primary", "outcome": "failed" })
}

/// A report of that version that a failure of the primary input cut short, as the partial report of
/// `error_record`, whose primary input failed.
pub fn partial_report(version: &str) -> Value {
  with(report(version), "sources", json!([failed_primary()]))
}

/// The error record with that partial report, whose format, which detection found, the error names too.
pub fn with_partial(record: Value, partial: Value) -> Value {
  let format = partial["format"].clone();
  with(with(record, "partial_report", partial), "format", format)
}

/// A stream of that index and kind, with the information that its kind needs and a parsed header.
pub fn stream(index: usize, kind: &str) -> Value {
  let mut stream = json!({
    "index": index,
    "kind": kind,
    "codec": { "candidates": [], "has_every_candidate": true, "raw_identifiers": [] },
    "header_status": { "state": "parsed" },
    "languages": [],
    "titles": [],
    "disposition": {},
  });
  match kind {
    "video" => stream["video"] = json!({ "frame_rate_mode": "constant" }),
    "audio" => stream["audio"] = json!({}),
    "subtitle" => stream["subtitle"] = json!({}),
    _ => {}
  }
  stream
}

/// A detection of that version that identified Matroska.
pub fn detection(version: &str) -> Value {
  json!({
    "schema_version": version,
    "outcome": "identified",
    "format": "matroska",
    "confidence": 100,
    "candidates": [],
    "usage": usage(),
  })
}

/// An error record of that version of a source that could not be opened.
pub fn error_record(version: &str) -> Value {
  json!({
    "schema_version": version,
    "kind": "source_failed",
    "message": "the source could not be opened",
    "candidates": [],
    "source_identifier": "0",
    "source_failure_kind": "not_found",
    "cause_messages": [],
    "usage": usage(),
    "record": record(),
  })
}

/// The value with one member set.
pub fn with(mut value: Value, name: &str, member: Value) -> Value {
  value[name] = member;
  value
}

/// The value without one member.
pub fn without(mut value: Value, name: &str) -> Value {
  if let Value::Object(members) = &mut value {
    members.remove(name);
  }
  value
}

/// The members of a record other than its version, as JSON text, in the order written.
pub fn members_of(record: &Value) -> Vec<(String, String)> {
  let Value::Object(members) = record else {
    return Vec::new();
  };
  members
    .iter()
    .filter(|(name, _)| *name != "schema_version")
    .map(|(name, value)| (name.clone(), value.to_string()))
    .collect()
}

/// The record's text, one member a line, with the version at that position when one is given.
pub fn text_of(members: &[(String, String)], version: Option<(usize, &str)>) -> String {
  let mut lines: Vec<String> = members
    .iter()
    .map(|(name, value)| format!("{}: {value}", json!(name)))
    .collect();
  if let Some((position, version)) = version {
    lines.insert(
      position.min(lines.len()),
      format!("\"schema_version\": {}", json!(version)),
    );
  }
  format!("{{{}}}", lines.join(",\n"))
}

/// The members with one replaced or, when it is absent, added at that position.
pub fn with_member(members: &[(String, String)], name: &str, value: &str, position: usize) -> Vec<(String, String)> {
  let mut members: Vec<(String, String)> = members.iter().filter(|(kept, _)| kept != name).cloned().collect();
  members.insert(position.min(members.len()), (name.to_owned(), value.to_owned()));
  members
}

/// The members that an error of that known kind needs for its ending (`api.md` section 5.1): the source of a source
/// failure, the reason of a disabled feature, which names its feature, and the reason of a probe stopped before its
/// inventory or of too few bytes to determine the format, which the record keeps as the first reason of detection
/// (section 5.3).
pub fn ending_members(kind: &str) -> Vec<(&'static str, Value)> {
  let truncated = json!({ "kind": "truncated_input" });
  match kind {
    "source_failed" => vec![("source_identifier", json!("0"))],
    "feature_disabled" => vec![("reason", json!({ "kind": "feature_disabled", "feature": "zlib" }))],
    "no_stream_inventory" => vec![("reason", truncated)],
    "format_not_determined" => vec![("reason", truncated.clone()), ("record", detection_stopped(&truncated))],
    _ => Vec::new(),
  }
}

/// A fixed-size record whose one first reason, of detection, is `reason`.
pub fn detection_stopped(reason: &Value) -> Value {
  with(
    record(),
    "first_reasons",
    json!([{ "part": "detection", "reason": reason }]),
  )
}

/// The generator of the documents of these tests.
pub fn generator() -> Value {
  json!({ "name": "catalog", "version": "1" })
}

/// A document of that version, by the generator of these tests, with those entries.
pub fn document(version: &str, results: Value) -> Value {
  let mut document = json!({ "schema_version": version, "generator": generator() });
  document["results"] = results;
  document
}

/// An entry at that index for the input `input.bin`, of that outcome, that holds the record under that member.
pub fn entry(index: usize, outcome: &str, member: &str, record: Value) -> Value {
  let mut entry = json!({ "index": index, "input": "input.bin", "outcome": outcome });
  entry[member] = record;
  entry
}

/// A document of version 1.0 whose one entry, of that outcome, holds the record under that member.
pub fn document_holding(outcome: &str, member: &str, record: Value) -> Value {
  document("1.0", json!([entry(0, outcome, member, record)]))
}

/// Whether the value reads as a document.
pub fn reads_document(document: &Value) -> bool {
  serde_json::from_value::<cc_media_probe::document::Document>(document.clone()).is_ok()
}
