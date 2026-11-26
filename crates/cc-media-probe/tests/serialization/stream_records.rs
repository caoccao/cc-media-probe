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

//! A stream's header state against its codec, its protection and the report's record (`api.md` sections 5.4, 6.3
//! and 6.4), as the fourth review of the third slice's first part asked, which read an MP4 stream without every
//! candidate beside a parsed header, a protected header without a protection, a header stopped by the read limit
//! beside no exhausted limit, and codec candidates of descriptions that their streams cannot have.

use cc_media_probe::{MediaReport, Stream};
use serde_json::{Value, json};

use crate::fixtures::{report, stream, with};

fn reads(report: &Value) -> bool {
  serde_json::from_value::<MediaReport>(report.clone()).is_ok()
}

fn reads_stream(stream: &Value) -> bool {
  serde_json::from_value::<Stream>(stream.clone()).is_ok()
}

/// A report of that format with those streams.
fn report_of(format: &str, streams: &[&Value]) -> Value {
  let streams = streams.iter().map(|stream| (*stream).clone()).collect();
  with(with(report("1.0"), "format", json!(format)), "streams", streams)
}

/// A reason of the headers of that stream.
fn headers(index: usize, reason: &Value) -> Value {
  json!({ "part": "headers", "stream": index, "reason": reason })
}

/// The report with its headers cut short at stream 0 by the input's end, kept and first.
fn headers_cut(mut report: Value) -> Value {
  let truncated = headers(0, &json!({ "kind": "truncated_input" }));
  report["completeness"]["headers"] = json!("incomplete");
  report["completeness"]["reasons"] = json!([truncated]);
  report["completeness"]["record"]["first_reasons"] = json!([truncated]);
  report
}

/// That stream, with its header cut short by that reason before any field was read.
fn cut(stream: Value, reason: &Value) -> Value {
  with(
    stream,
    "header_status",
    json!({ "state": "not_found_within_budget", "reason": reason }),
  )
}

/// The information of an H.264 codec whose candidates are of those descriptions.
fn candidates(indexes: &[u64], every: bool) -> Value {
  let identifier = json!({ "namespace": "iso_sample_entry", "value": "61766331" });
  let candidates: Vec<Value> = indexes
    .iter()
    .map(|index| json!({ "sample_description_index": index, "codec": "h264", "raw_identifiers": [identifier] }))
    .collect();
  let mut codec = json!({ "has_every_candidate": every, "raw_identifiers": vec![identifier; indexes.len()] });
  codec["candidates"] = Value::Array(candidates);
  if every && !indexes.is_empty() {
    codec["codec"] = json!("h264");
  }
  codec
}

#[test]
fn a_missing_description_leaves_the_header_state_not_final() {
  // The review's case: an MP4 stream without every candidate and without one, beside a parsed header.
  let missing = with(stream(0, "video"), "codec", candidates(&[], false));
  assert!(!reads_stream(&missing));
  assert!(!reads(&report_of("mp4", &[&missing])));
  let truncated = json!({ "kind": "truncated_input" });
  let missing = cut(missing, &truncated);
  assert!(reads_stream(&missing));
  assert!(reads(&headers_cut(report_of("mp4", &[&missing]))));
  // A state of a newer version is not final.
  assert!(reads_stream(&with(
    missing,
    "header_status",
    json!({ "state": "decoded" })
  )));
}

#[test]
fn a_protected_header_needs_a_protection() {
  let protected = with(stream(0, "audio"), "header_status", json!({ "state": "protected" }));
  assert!(!reads_stream(&protected));
  let protection =
    json!({ "declarations": [], "fully_examined": false, "has_sample_descriptions": false, "sample_descriptions": [] });
  assert!(reads_stream(&with(protected, "protection", protection)));
}

#[test]
fn a_header_states_reason_finds_its_counter_and_is_kept_first_or_counted() {
  let truncated = json!({ "kind": "truncated_input" });
  let limited = json!({ "kind": "limit_exhausted", "limit": "bytes_read" });
  let cut_streams = report_of(
    "matroska",
    &[&cut(stream(0, "video"), &truncated), &cut(stream(1, "audio"), &limited)],
  );
  let with_record = |kept: Value, exhausted: &[&str], omitted: &str| {
    let mut report = headers_cut(cut_streams.clone());
    report["completeness"]["reasons"] = kept;
    let record = &mut report["completeness"]["record"];
    record["exhausted"] = exhausted.iter().map(|limit| json!(limit)).collect();
    record["first_stopped"] = exhausted
      .iter()
      .map(|limit| json!({ "limit": limit, "part": "headers" }))
      .collect();
    record["omitted_reasons"] = json!(omitted);
    report
  };
  let first = || headers(0, &truncated);
  let both = || json!([first(), headers(1, &limited)]);
  assert!(reads(&with_record(both(), &["bytes_read"], "0")));
  // The review's case: a header stopped by the read limit beside no exhausted limit, with only the input's end kept.
  assert!(!reads(&with_record(json!([first()]), &[], "0")));
  // The limit is exhausted, and the reason was neither kept nor counted.
  assert!(!reads(&with_record(json!([first()]), &["bytes_read"], "0")));
  assert!(reads(&with_record(
    json!([first()]),
    &["bytes_read", "retained_data"],
    "1"
  )));
  // Neither kept: the first is counted among the first reasons that the kept ones lack, and the second once more.
  assert!(!reads(&with_record(json!([]), &["bytes_read", "retained_data"], "1")));
  assert!(reads(&with_record(json!([]), &["bytes_read", "retained_data"], "2")));
  // A state of a newer version is not checked.
  let mut newer = with_record(json!([first()]), &[], "0");
  newer["streams"][1]["header_status"] = json!({ "state": "decoded", "reason": limited });
  assert!(reads(&newer));
}

#[test]
fn a_codec_candidate_names_a_description_that_its_stream_can_have() {
  let protection = |count: u64| {
    json!({
      "declarations": [],
      "fully_examined": false,
      "has_sample_descriptions": true,
      "sample_description_count": count,
      "sample_descriptions": [],
    })
  };
  let video = |codec: Value| with(stream(0, "video"), "codec", codec);
  let counted = |codec: Value, count: u64| with(video(codec), "protection", protection(count));
  assert!(reads(&report_of("mp4", &[&video(candidates(&[1, 2], true))])));
  assert!(reads(&report_of("mp4", &[&counted(candidates(&[1, 2], true), 2)])));
  // The review's cases: a candidate of description 2 in a Matroska stream, and in an MP4 stream that declares one.
  assert!(!reads(&report_of("matroska", &[&video(candidates(&[2], true))])));
  assert!(!reads(&report_of("mp4", &[&counted(candidates(&[2], true), 1)])));
  // A stream of a format without sample descriptions misses no candidate, since it has none.
  let missing = cut(video(candidates(&[], false)), &json!({ "kind": "truncated_input" }));
  assert!(reads(&headers_cut(report_of("mp4", &[&missing]))));
  assert!(!reads(&headers_cut(report_of("matroska", &[&missing]))));
  // A format of a newer version is not checked.
  assert!(reads(&report_of("future_format", &[&video(candidates(&[2], true))])));
}
