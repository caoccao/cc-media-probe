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

//! The checks that a record makes on its own members, alone as inside a report (`json-output.md` sections 2, 4.4,
//! 7.1 and 7.2): a candidate keeps its confidence within 100, a completeness agrees with its record, a reason holds
//! the members of its kind, only a source failure carries a partial report, an error's limit reason finds its
//! counter, and a value and its provenance are present together.

use cc_media_probe::completeness::{Completeness, Reason};
use cc_media_probe::error::ErrorRecord;
use cc_media_probe::model::{AudioInformation, Candidate, Chromaticity, VideoInformation};
use cc_media_probe::{MediaReport, Stream};
use serde_json::{Value, json};

use crate::fixtures::{
  completeness as complete, error_record, record as empty_record, record_omitting as record, report, stream, with,
};

#[test]
fn a_standalone_candidate_keeps_its_confidence_within_100() {
  assert!(serde_json::from_value::<Candidate>(json!({ "format": "mp4", "confidence": 100 })).is_ok());
  assert!(serde_json::from_value::<Candidate>(json!({ "format": "mp4", "confidence": 101 })).is_err());
  assert!(serde_json::from_value::<Candidate>(json!({ "format": "mp4", "confidence": -1 })).is_err());
}

fn completeness(tags: &str, reasons: Value, record: Value) -> Value {
  with(
    with(with(complete(), "tags", json!(tags)), "reasons", reasons),
    "record",
    record,
  )
}

/// Whether the completeness reads, alone and inside a report alike.
fn reads(completeness: Value) -> bool {
  let alone = serde_json::from_value::<Completeness>(completeness.clone()).is_ok();
  let nested = serde_json::from_value::<MediaReport>(with(report("1.0"), "completeness", completeness)).is_ok();
  assert_eq!(alone, nested, "the two entry points disagree");
  alone
}

fn tags_reason(kind: &str) -> Value {
  json!({ "part": "tags", "reason": { "kind": kind } })
}

#[test]
fn a_standalone_completeness_agrees_with_its_record() {
  assert!(!reads(completeness("incomplete", json!([]), record(json!([]), "0"))));
  let first = json!([tags_reason("truncated_input")]);
  assert!(reads(completeness(
    "incomplete",
    json!([tags_reason("truncated_input")]),
    record(first.clone(), "0")
  )));
  assert!(!reads(completeness(
    "incomplete",
    json!([tags_reason("truncated_input")]),
    record(json!([]), "0")
  )));
  assert!(!reads(completeness("complete", json!([]), record(first, "1"))));
}

#[test]
fn the_omitted_count_covers_every_first_reason_that_the_kept_reasons_lack() {
  let first = json!([tags_reason("truncated_input")]);
  assert!(!reads(completeness(
    "incomplete",
    json!([]),
    record(first.clone(), "0")
  )));
  assert!(reads(completeness("incomplete", json!([]), record(first.clone(), "1"))));
  let other = json!([tags_reason("range_unavailable")]);
  assert!(!reads(completeness(
    "incomplete",
    other.clone(),
    record(first.clone(), "0")
  )));
  assert!(reads(completeness("incomplete", other, record(first.clone(), "1"))));
  let detailed = json!([{ "part": "tags", "reason": { "kind": "truncated_input", "detail": "the file ends" } }]);
  assert!(reads(completeness("incomplete", detailed, record(first, "0"))));
}

#[test]
fn a_reason_of_a_known_kind_holds_the_members_of_its_kind() {
  let read = |reason: Value| serde_json::from_value::<Reason>(reason).is_ok();
  assert!(read(json!({ "kind": "limit_exhausted", "limit": "bytes_read" })));
  assert!(read(json!({ "kind": "truncated_input" })));
  assert!(read(json!({ "kind": "feature_disabled", "feature": "zlib" })));
  assert!(!read(json!({ "kind": "limit_exhausted" })));
  assert!(!read(
    json!({ "kind": "limit_exhausted", "limit": "bytes_read", "feature": "zlib" })
  ));
  assert!(!read(json!({ "kind": "truncated_input", "limit": "bytes_read" })));
  assert!(!read(json!({ "kind": "range_unavailable", "limit": "bytes_read" })));
  assert!(read(json!({ "kind": "quota_reached", "limit": "bytes_read" })));
  let bare = json!([{ "part": "tags", "reason": { "kind": "limit_exhausted" } }]);
  assert!(!reads(completeness("incomplete", bare.clone(), record(bare, "0"))));
}

#[test]
fn only_a_source_failure_carries_a_partial_report() {
  let read = |kind: &str| {
    let mut error = crate::fixtures::with_partial(error_record("1.0"), crate::fixtures::partial_report("1.0"));
    error["kind"] = json!(kind);
    if kind != "source_failed" {
      error = crate::fixtures::without(error, "source_failure_kind");
    }
    serde_json::from_value::<ErrorRecord>(error)
  };
  assert!(read("source_failed").is_ok());
  for kind in [
    "unknown_format",
    "format_not_determined",
    "unsupported_format",
    "feature_disabled",
    "no_stream_inventory",
  ] {
    assert!(read(kind).is_err(), "{kind}");
  }
  assert!(read("quota_exceeded").is_ok());
}

#[test]
fn an_errors_limit_reason_finds_its_counter() {
  let reason = json!({ "kind": "limit_exhausted", "limit": "bytes_read" });
  let mut stopped = record(json!([{ "part": "detection", "reason": reason }]), "0");
  stopped["exhausted"] = json!(["bytes_read"]);
  stopped["first_stopped"] = json!([{ "limit": "bytes_read", "part": "detection" }]);
  let read = |record: Value| {
    let error = with(with(error_record("1.0"), "reason", reason.clone()), "record", record);
    serde_json::from_value::<ErrorRecord>(error)
  };
  assert!(read(stopped).is_ok());
  assert!(read(empty_record()).is_err());
  // The read limit exhausted but not paired with a part, as a limit that a reason names is (fifth review of the
  // third slice's first part).
  let unpaired = with(empty_record(), "exhausted", json!(["bytes_read"]));
  assert!(read(unpaired).is_err());
}

const PAIR: &str = "a value and its provenance are not present together";

fn pair_holds<Type: serde::de::DeserializeOwned + std::fmt::Debug>(
  name: &str,
  base: &Value,
  member: &str,
  value: &Value,
) {
  let provenance = format!("{member}_provenance");
  let read = |members: &[(&str, Value)]| {
    let mut record = base.clone();
    for (name, value) in members {
      record[*name] = value.clone();
    }
    serde_json::from_value::<Type>(record)
  };
  assert!(read(&[]).is_ok(), "{name}");
  assert!(
    read(&[(member, value.clone()), (&provenance, json!("declared"))]).is_ok(),
    "{name} {member}"
  );
  for members in [
    vec![(member, value.clone())],
    vec![(&provenance[..], json!("declared"))],
  ] {
    let error = read(&members).unwrap_err().to_string();
    assert!(error.starts_with(PAIR), "{name} {members:?}: {error}");
  }
}

#[test]
fn a_value_and_its_provenance_are_present_together() {
  let rate = json!({ "numerator": 25, "denominator": 1 });
  let video = json!({ "frame_rate_mode": "constant" });
  pair_holds::<VideoInformation>("video", &video, "frame_rate", &rate);
  pair_holds::<VideoInformation>("video", &video, "coded_size", &json!({ "width": 2, "height": 2 }));
  pair_holds::<AudioInformation>("audio", &json!({}), "channel_count", &json!(2));
  pair_holds::<AudioInformation>("audio", &json!({}), "padding", &json!(2));
  pair_holds::<Chromaticity>("chromaticity", &json!({}), "x_coordinate", &rate);
  let video_stream = stream(0, "video");
  for member in ["duration", "bit_rate", "start_time"] {
    pair_holds::<Stream>("stream", &video_stream, member, &rate);
  }
  pair_holds::<Stream>("stream", &video_stream, "frame_count", &json!(9));
  pair_holds::<MediaReport>("report", &report("1.0"), "duration", &rate);
  pair_holds::<MediaReport>("report", &report("1.0"), "bit_rate", &rate);
}
