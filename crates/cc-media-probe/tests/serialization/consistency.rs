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

//! A report, its record and an error record that contradict themselves, which reading refuses (`json-output.md`
//! sections 4.4, 7.2, 7.3 and 10): a known header state that is not final records a shortfall of the headers
//! whatever the headers status says; every part that a limit stopped first has a first reason, and a reason that
//! names an exhausted limit finds it in the record; the record keeps the declared order and its first reasons carry
//! no detail; a stream's index is its position; and an error holds a partial report or observations, never both, with
//! a partial report whose record is the error's.

use cc_media_probe::MediaReport;
use cc_media_probe::completeness::LimitRecord;
use cc_media_probe::error::ErrorRecord;
use serde_json::{Value, json};

use crate::fixtures::{error_record, record, record_omitting, report, stream, with, with_partial};

/// A report without streams whose parts are all complete, with that record.
fn report_with_record(record: Value) -> Value {
  let mut report = report("1.0");
  report["completeness"]["record"] = record;
  report
}

/// A stream of that index whose header was cut short at the input's end.
fn cut_stream(index: usize) -> Value {
  with(
    stream(index, "video"),
    "header_status",
    json!({ "state": "not_found_within_budget", "reason": { "kind": "truncated_input" } }),
  )
}

/// A record whose first reason of the headers was not kept.
fn headers_record() -> Value {
  record_omitting(
    json!([{ "part": "headers", "stream": 0, "reason": { "kind": "truncated_input" } }]),
    "1",
  )
}

#[test]
fn a_known_header_state_that_is_not_final_is_a_shortfall_beside_a_newer_headers_status() {
  let mut report = report_with_record(record());
  report["completeness"]["headers"] = json!("partially_final");
  report["streams"] = json!([cut_stream(0)]);
  assert!(serde_json::from_value::<MediaReport>(report.clone()).is_err());
  report["completeness"]["record"] = headers_record();
  let read: MediaReport = serde_json::from_value(report).unwrap();
  assert!(!read.completeness().is_complete());
}

#[test]
fn the_headers_status_agrees_with_the_header_states_of_the_streams() {
  let mut report = report_with_record(record());
  report["streams"] = json!([cut_stream(0)]);
  assert!(
    serde_json::from_value::<MediaReport>(report.clone()).is_err(),
    "complete beside a cut header"
  );
  report["completeness"]["headers"] = json!("incomplete");
  report["completeness"]["record"] = headers_record();
  assert!(serde_json::from_value::<MediaReport>(report.clone()).is_ok());
  report["streams"] = json!([stream(0, "video")]);
  assert!(
    serde_json::from_value::<MediaReport>(report.clone()).is_err(),
    "incomplete beside final states"
  );
  // Beside a state of a newer version, the status is the verdict of the version that wrote it.
  report["streams"][0]["header_status"] = json!({ "state": "decoded" });
  assert!(serde_json::from_value::<MediaReport>(report.clone()).is_ok());
  report["completeness"]["headers"] = json!("complete");
  report["completeness"]["record"] = record();
  assert!(serde_json::from_value::<MediaReport>(report).is_ok());
}

#[test]
fn a_part_that_a_limit_stopped_first_has_a_first_reason() {
  let mut stopped = record();
  stopped["exhausted"] = json!(["bytes_read"]);
  stopped["first_stopped"] = json!([{ "limit": "bytes_read", "part": "discovery" }]);
  assert!(serde_json::from_value::<MediaReport>(report_with_record(stopped.clone())).is_err());
  stopped["first_reasons"] =
    json!([{ "part": "discovery", "reason": { "kind": "limit_exhausted", "limit": "bytes_read" } }]);
  // The first reason was not kept, which only the exhausted retained data explains.
  stopped["omitted_reasons"] = json!("1");
  stopped["exhausted"] = json!(["bytes_read", "retained_data"]);
  stopped["first_stopped"] = json!([
    { "limit": "bytes_read", "part": "discovery" },
    { "limit": "retained_data", "part": "discovery" }
  ]);
  let mut report = report_with_record(stopped);
  report["completeness"]["discovery"] = json!("incomplete");
  assert!(serde_json::from_value::<MediaReport>(report).is_ok());
}

#[test]
fn a_reason_that_names_an_exhausted_limit_finds_it_in_the_record() {
  let reason = json!({ "part": "tags", "reason": { "kind": "limit_exhausted", "limit": "bytes_read" } });
  let mut report = report_with_record(with(record(), "first_reasons", json!([reason])));
  report["completeness"]["tags"] = json!("incomplete");
  assert!(serde_json::from_value::<MediaReport>(report.clone()).is_err());
  report["completeness"]["reasons"] = json!([reason]);
  assert!(serde_json::from_value::<MediaReport>(report.clone()).is_err());
  report["completeness"]["record"]["exhausted"] = json!(["bytes_read"]);
  report["completeness"]["record"]["first_stopped"] = json!([{ "limit": "bytes_read", "part": "tags" }]);
  assert!(serde_json::from_value::<MediaReport>(report).is_ok());
}

#[test]
fn the_record_keeps_the_declared_order_and_no_detail_in_a_first_reason() {
  let read = |value: Value| serde_json::from_value::<LimitRecord>(value);
  let reason = |limit: &str| json!({ "part": "discovery", "reason": { "kind": "limit_exhausted", "limit": limit } });
  let pair = |limit: &str| json!({ "limit": limit, "part": "discovery" });
  let ordered = with(
    with(record(), "exhausted", json!(["bytes_read", "working_memory"])),
    "first_stopped",
    json!([pair("bytes_read"), pair("working_memory")]),
  );
  assert!(read(with(ordered, "first_reasons", json!([reason("bytes_read")]))).is_ok());
  let reversed = with(
    with(record(), "exhausted", json!(["working_memory", "bytes_read"])),
    "first_stopped",
    json!([pair("working_memory"), pair("bytes_read")]),
  );
  assert!(read(with(reversed, "first_reasons", json!([reason("working_memory")]))).is_err());
  let mut detailed = reason("bytes_read");
  detailed["reason"]["detail"] = json!("any text at all");
  let one = with(
    with(record(), "exhausted", json!(["bytes_read"])),
    "first_stopped",
    json!([pair("bytes_read")]),
  );
  assert!(read(with(one, "first_reasons", json!([detailed]))).is_err());
  let parts = json!([
    { "part": "tags", "reason": { "kind": "truncated_input" } },
    { "part": "discovery", "reason": { "kind": "truncated_input" } },
  ]);
  assert!(read(with(record(), "first_reasons", parts)).is_err());
  let newer = with(
    with(record(), "exhausted", json!(["future_limit", "bytes_read"])),
    "first_stopped",
    json!([{ "limit": "future_limit", "part": "discovery" }, pair("bytes_read")]),
  );
  assert!(read(with(newer, "first_reasons", json!([reason("bytes_read")]))).is_ok());
}

#[test]
fn a_streams_index_is_its_position() {
  let mut report = report_with_record(headers_record());
  report["completeness"]["headers"] = json!("incomplete");
  report["streams"] = json!([cut_stream(0)]);
  let read: MediaReport = serde_json::from_value(report.clone()).unwrap();
  assert_eq!(read.stream(0).map(cc_media_probe::Stream::index), Some(0));
  report["streams"] = json!([cut_stream(7)]);
  assert!(serde_json::from_value::<MediaReport>(report).is_err());
}

#[test]
fn a_streams_information_stands_beside_its_kind() {
  let mut report = report("1.0");
  report["streams"] = json!([
    stream(0, "video"),
    stream(1, "audio"),
    stream(2, "subtitle"),
    stream(3, "data")
  ]);
  assert!(serde_json::from_value::<MediaReport>(report.clone()).is_ok());
  for (position, member, value) in [
    (0, "audio", json!({})),
    (1, "video", json!({ "frame_rate_mode": "constant" })),
    (3, "subtitle", json!({})),
  ] {
    let mut malformed = report.clone();
    malformed["streams"][position][member] = value;
    assert!(
      serde_json::from_value::<MediaReport>(malformed).is_err(),
      "{position} {member}"
    );
  }
  let mut missing = report;
  missing["streams"][0] = crate::fixtures::without(stream(0, "video"), "video");
  assert!(serde_json::from_value::<MediaReport>(missing).is_err());
}

#[test]
fn an_error_holds_a_partial_report_or_observations_never_both() {
  let partial = crate::fixtures::partial_report("1.0");
  let observations = json!({ "diagnostics": [], "sources": [crate::fixtures::failed_primary()] });
  let both = with(
    with_partial(error_record("1.0"), partial.clone()),
    "observations",
    observations.clone(),
  );
  assert!(serde_json::from_value::<ErrorRecord>(both).is_err());
  assert!(serde_json::from_value::<ErrorRecord>(with_partial(error_record("1.0"), partial)).is_ok());
  assert!(serde_json::from_value::<ErrorRecord>(with(error_record("1.0"), "observations", observations)).is_ok());
}

#[test]
fn an_errors_record_is_its_partial_reports_record() {
  let mut stopped = record();
  stopped["exhausted"] = json!(["bytes_read", "retained_data"]);
  stopped["first_stopped"] = json!([
    { "limit": "bytes_read", "part": "discovery" },
    { "limit": "retained_data", "part": "discovery" }
  ]);
  stopped["first_reasons"] =
    json!([{ "part": "discovery", "reason": { "kind": "limit_exhausted", "limit": "bytes_read" } }]);
  stopped["omitted_reasons"] = json!("1");
  let mut partial = report_with_record(stopped.clone());
  partial["completeness"]["discovery"] = json!("incomplete");
  partial["sources"] = json!([crate::fixtures::failed_primary()]);
  let agreeing = with(with_partial(error_record("1.0"), partial.clone()), "record", stopped);
  assert!(serde_json::from_value::<ErrorRecord>(agreeing).is_ok());
  let disagreeing = with_partial(error_record("1.0"), partial);
  assert!(serde_json::from_value::<ErrorRecord>(disagreeing).is_err());
}

#[test]
fn a_protection_has_sample_descriptions_exactly_in_the_iso_family() {
  let protected = |has: bool| {
    with(
      stream(0, "audio"),
      "protection",
      json!({ "declarations": [], "fully_examined": false, "has_sample_descriptions": has, "sample_descriptions": [] }),
    )
  };
  for (format, has, reads) in [
    ("matroska", false, true),
    ("matroska", true, false),
    ("mp4", true, true),
    ("iso_base_media", true, true),
    ("quicktime", false, false),
    ("future_format", true, true),
    ("future_format", false, true),
  ] {
    let report = with(
      with(report("1.0"), "format", json!(format)),
      "streams",
      json!([protected(has)]),
    );
    assert_eq!(
      serde_json::from_value::<MediaReport>(report).is_ok(),
      reads,
      "{format} {has}"
    );
  }
}
