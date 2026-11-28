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

//! The smallest records of the library's JSON form that the cases' tests change, as the library's serialization
//! tests build theirs (`crates/cc-media-probe/tests/serialization/fixtures.rs`). Each is read back by the library's
//! strict reader, so that a test exercises a record that this version reads, as the seventh review of the proposal
//! asked of a test that put an error's sources at its root. No probe of this version opens a companion, so a
//! companion's entry has a role of a later version, as those tests give it.

use cc_media_probe::MediaReport;
use cc_media_probe::error::ErrorRecord;
use serde_json::{Map, Value, json};

/// A usage whose counters are all 0 but its retained data, written in its canonical form, and its companions'
/// requests and sources opened.
fn usage(retained: u64, companions: usize) -> Value {
  let mut usage = json!({
    "bytes_read": 0, "peak_working_memory": 0, "retained_payloads": 0, "decompressed_output": 0, "work_units": 0,
    "companion_requests": companions, "sources_opened": companions,
  });
  if retained > 9_007_199_254_740_991 {
    usage["retained_data_exact"] = json!(retained.to_string());
  } else {
    usage["retained_data"] = json!(retained);
  }
  usage
}

/// A fixed-size record that records no stop.
fn limit_record() -> Value {
  json!({
    "exhausted": [], "first_stopped": [], "first_reasons": [], "omitted_diagnostics": "0",
    "omitted_declarations": "0", "omitted_reasons": "0", "omitted_references": "0",
  })
}

/// The source entry `identifier` with that outcome and the members of its path: the primary input for 0, and a
/// companion otherwise.
pub fn entry(identifier: &str, outcome: &str, path: &Map<String, Value>) -> Value {
  let role = if identifier == "0" { "primary" } else { "subtitle_index" };
  let mut entry = json!({ "identifier": identifier, "role": role, "outcome": outcome });
  entry.as_object_mut().unwrap().extend(path.clone());
  entry
}

/// A complete Matroska report without streams whose usage retains `retained` bytes and whose source table is
/// `sources`, each companion in it requested and opened, read strictly.
pub fn report(retained: u64, sources: &[Value]) -> Value {
  let companions = sources.len().saturating_sub(1);
  let report = json!({
    "schema_version": "1.0",
    "format": "matroska",
    "streams": [], "programs": [], "chapter_editions": [], "tag_sets": [], "attachments": [],
    "external_references": [], "diagnostics": [],
    "completeness": {
      "detection": "complete", "discovery": "complete", "headers": "complete", "tags": "complete",
      "chapters": "complete", "attachments": "complete", "timing": "established",
      "companions": if companions == 0 { "not_applicable" } else { "complete" },
      "reasons": [], "record": limit_record(),
    },
    "inventory_class": "declared",
    "usage": usage(retained, companions),
    "sources": sources,
  });
  if let Err(error) = serde_json::from_value::<MediaReport>(report.clone()) {
    panic!("{report}: {error}");
  }
  report
}

/// The error record of `kind` with that usage and `members` added, read strictly.
fn error(kind: &str, usage: &Value, members: &Value) -> Value {
  let mut error = json!({
    "schema_version": "1.0",
    "kind": kind,
    "message": "m",
    "candidates": [],
    "cause_messages": [],
    "usage": usage,
    "record": limit_record(),
  });
  error
    .as_object_mut()
    .unwrap()
    .extend(members.as_object().unwrap().clone());
  if let Err(failure) = serde_json::from_value::<ErrorRecord>(error.clone()) {
    panic!("{error}: {failure}");
  }
  error
}

/// The failure of the primary input after its report was established, with the partial report whose table is
/// `sources`, which lists the primary input as failed, and whose usage the error's covers.
pub fn failed(retained: u64, sources: &[Value]) -> Value {
  let partial = report(retained, sources);
  error(
    "source_failed",
    &partial["usage"],
    &json!({
      "format": "matroska", "source_identifier": "0", "source_failure_kind": "unexpected_eof",
      "partial_report": partial,
    }),
  )
}

/// A Matroska input cut short before its stream inventory, whose observations list `sources`.
pub fn stopped(retained: u64, sources: &[Value]) -> Value {
  error(
    "no_stream_inventory",
    &usage(retained, sources.len().saturating_sub(1)),
    &json!({
      "format": "matroska", "reason": { "kind": "truncated_input" },
      "observations": { "diagnostics": [], "sources": sources },
    }),
  )
}

/// An input of an unknown format, whose probe ended at detection and lists no source.
pub fn unknown(retained: u64) -> Value {
  error("unknown_format", &usage(retained, 0), &json!({}))
}
