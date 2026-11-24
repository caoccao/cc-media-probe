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

//! The fourteenth review's cases of a report, its record and an error record that contradict themselves, which
//! reading refuses (`json-output.md` sections 7.2, 7.3 and 10; `api.md` section 5.3). A known header state that is
//! not final records a shortfall of the headers whatever the headers status says; every part that a limit stopped
//! first has a first reason, and a reason that names an exhausted limit finds it in the record; the record's arrays
//! keep the declared order and its first reasons carry no detail; a stream's index is its position; and an error
//! holds a partial report or observations, never both, with a partial report whose record is the error's.

use std::path::Path;

use cc_media_probe::document::{Document, DocumentEntry, EntryOutcome};
use cc_media_probe::error::ErrorRecord;
use cc_media_probe::source::PortablePath;
use cc_media_probe::{MediaReport, probe_bytes};
use serde_json::{Value, json};

/// A report without streams whose parts are all complete, with that record.
fn report_with_record(record: Value) -> Value {
  json!({
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
      "record": record,
    },
    "usage": { "bytes_read": 0, "peak_working_memory": 0 },
    "sources": [],
  })
}

fn empty_record() -> Value {
  json!({ "exhausted": [], "first_stopped": [] })
}

/// A stream of that index whose header was cut short at the input's end.
fn cut_stream(index: usize) -> Value {
  json!({
    "index": index,
    "kind": "Video",
    "codec": { "candidates": [], "has_every_candidate": true, "raw_identifiers": [] },
    "header_status": { "state": "NotFoundWithinBudget", "reason": { "kind": "TruncatedInput" } },
    "titles": [],
    "disposition": {},
  })
}

fn document_holding(outcome: &str, report: Value) -> Result<Document, serde_json::Error> {
  serde_json::from_value(json!({
    "schema_version": "1.0",
    "generator": { "name": "catalog", "version": "1" },
    "results": [{ "index": 0, "input": "a.mkv", "outcome": outcome, "report": report }],
  }))
}

#[test]
fn a_known_header_state_that_is_not_final_is_a_shortfall_beside_a_newer_headers_status() {
  // The review's case: the headers status is of a newer version, the stream's state is known and not final, and the
  // record has no first reason, so an entry labeled `ok` read.
  let mut report = report_with_record(empty_record());
  report["completeness"]["headers"] = json!({ "Unrecognized": "partially_final" });
  report["streams"] = json!([cut_stream(0)]);
  assert!(document_holding("ok", report.clone()).is_err());
  assert!(serde_json::from_value::<MediaReport>(report.clone()).is_err());
  // With the first reason of the headers, the report reads, and only `partial` describes it.
  report["completeness"]["record"]["first_reasons"] =
    json!([{ "part": "Headers", "stream": 0, "reason": { "kind": "TruncatedInput" } }]);
  let read: MediaReport = serde_json::from_value(report.clone()).unwrap();
  let entry = DocumentEntry::from_report(PortablePath::from_path(Path::new("a.mkv")), read).unwrap();
  assert_eq!(entry.outcome(), &EntryOutcome::Partial);
  assert!(document_holding("ok", report.clone()).is_err());
  assert!(document_holding("partial", report).is_ok());
}

#[test]
fn a_part_that_a_limit_stopped_first_has_a_first_reason() {
  // The review's case: every status complete and no first reason, while the record says that the read limit
  // stopped discovery; `is_complete()` was true.
  let stopped = json!({ "exhausted": ["BytesRead"], "first_stopped": [{ "limit": "BytesRead", "part": "Discovery" }] });
  assert!(serde_json::from_value::<MediaReport>(report_with_record(stopped)).is_err());
  let mut report = report_with_record(json!({
    "exhausted": ["BytesRead"],
    "first_stopped": [{ "limit": "BytesRead", "part": "Discovery" }],
    "first_reasons": [{ "part": "Discovery", "reason": { "kind": "LimitExhausted", "limit": "BytesRead" } }],
  }));
  report["completeness"]["discovery"] = json!("Incomplete");
  assert!(serde_json::from_value::<MediaReport>(report).is_ok());
}

#[test]
fn a_reason_that_names_an_exhausted_limit_finds_it_in_the_record() {
  // The review's case: a first reason of the tags naming the read limit, with no counter exhausted.
  let reason = json!({ "part": "Tags", "reason": { "kind": "LimitExhausted", "limit": "BytesRead" } });
  let mut report = report_with_record(json!({ "exhausted": [], "first_stopped": [], "first_reasons": [reason] }));
  report["completeness"]["tags"] = json!("Incomplete");
  assert!(serde_json::from_value::<MediaReport>(report.clone()).is_err());
  // A kept reason is checked the same way.
  report["completeness"]["reasons"] = json!([reason]);
  assert!(serde_json::from_value::<MediaReport>(report.clone()).is_err());
  report["completeness"]["record"]["exhausted"] = json!(["BytesRead"]);
  report["completeness"]["record"]["first_stopped"] = json!([{ "limit": "BytesRead", "part": "Tags" }]);
  assert!(serde_json::from_value::<MediaReport>(report).is_ok());
}

#[test]
fn the_record_keeps_the_declared_order_and_no_detail_in_a_first_reason() {
  let read = |record: Value| serde_json::from_value::<cc_media_probe::completeness::LimitRecord>(record);
  let reason = |limit: &str| json!({ "part": "Discovery", "reason": { "kind": "LimitExhausted", "limit": limit } });
  let pair = |limit: &str| json!({ "limit": limit, "part": "Discovery" });
  // The order of `api.md` section 4.5: the read limit before working memory.
  let ordered = json!({
    "exhausted": ["BytesRead", "WorkingMemory"],
    "first_stopped": [pair("BytesRead"), pair("WorkingMemory")],
    "first_reasons": [reason("BytesRead")],
  });
  assert!(read(ordered).is_ok());
  // The review's case: the two limits the other way round in both arrays, and a first reason with a detail.
  let reversed = json!({
    "exhausted": ["WorkingMemory", "BytesRead"],
    "first_stopped": [pair("WorkingMemory"), pair("BytesRead")],
    "first_reasons": [reason("WorkingMemory")],
  });
  assert!(read(reversed).is_err());
  let mut detailed = reason("BytesRead");
  detailed["reason"]["detail"] = json!("any text at all");
  let record = json!({ "exhausted": ["BytesRead"], "first_stopped": [pair("BytesRead")], "first_reasons": [detailed] });
  assert!(read(record).is_err());
  // The first reasons in the order of the parts; a limit of a newer version may stand anywhere.
  let parts = json!({ "exhausted": [], "first_stopped": [], "first_reasons": [
    { "part": "Tags", "reason": { "kind": "TruncatedInput" } },
    { "part": "Discovery", "reason": { "kind": "TruncatedInput" } } ] });
  assert!(read(parts).is_err());
  let newer = json!({
    "exhausted": [{ "Unrecognized": "future_limit" }, "BytesRead"],
    "first_stopped": [{ "limit": { "Unrecognized": "future_limit" }, "part": "Discovery" }, pair("BytesRead")],
    "first_reasons": [reason("BytesRead")],
  });
  assert!(read(newer).is_ok());
}

#[test]
fn a_streams_index_is_its_position() {
  let mut report = report_with_record(json!({
    "exhausted": [],
    "first_stopped": [],
    "first_reasons": [{ "part": "Headers", "stream": 0, "reason": { "kind": "TruncatedInput" } }],
  }));
  report["completeness"]["headers"] = json!("Incomplete");
  report["streams"] = json!([cut_stream(0)]);
  let read: MediaReport = serde_json::from_value(report.clone()).unwrap();
  assert_eq!(read.streams()[0].index(), 0);
  // The review's case: one stream that claims index 7.
  report["streams"] = json!([cut_stream(7)]);
  assert!(serde_json::from_value::<MediaReport>(report).is_err());
}

/// An error record of a source failure, with the members given.
fn source_failure(members: &[(&str, Value)]) -> Value {
  let mut error = serde_json::to_value(probe_bytes(b"").unwrap_err().to_record()).unwrap();
  error["kind"] = json!("source_failed");
  for (name, value) in members {
    error[*name] = value.clone();
  }
  error
}

#[test]
fn an_error_holds_a_partial_report_or_observations_never_both() {
  let partial = report_with_record(empty_record());
  let observations = json!({ "diagnostics": [] });
  let both = source_failure(&[
    ("partial_report", partial.clone()),
    ("observations", observations.clone()),
  ]);
  assert!(serde_json::from_value::<ErrorRecord>(both).is_err());
  assert!(serde_json::from_value::<ErrorRecord>(source_failure(&[("partial_report", partial)])).is_ok());
  assert!(serde_json::from_value::<ErrorRecord>(source_failure(&[("observations", observations)])).is_ok());
}

#[test]
fn an_errors_record_is_its_partial_reports_record() {
  // `api.md` section 5.3: an error that carries a partial report has that report's record.
  let stopped = json!({
    "exhausted": ["BytesRead"],
    "first_stopped": [{ "limit": "BytesRead", "part": "Discovery" }],
    "first_reasons": [{ "part": "Discovery", "reason": { "kind": "LimitExhausted", "limit": "BytesRead" } }],
  });
  let mut partial = report_with_record(stopped.clone());
  partial["completeness"]["discovery"] = json!("Incomplete");
  let agreeing = source_failure(&[("partial_report", partial.clone()), ("record", stopped)]);
  assert!(serde_json::from_value::<ErrorRecord>(agreeing).is_ok());
  let disagreeing = source_failure(&[("partial_report", partial), ("record", empty_record())]);
  assert!(serde_json::from_value::<ErrorRecord>(disagreeing).is_err());
}
