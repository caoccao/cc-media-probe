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

//! The fifteenth review's cases of `api.md` sections 3.3, 5.3, 5.5 and 10.2 and `json-output.md` sections 4.4, 7.2
//! and 10. A check that needs only a record's own members runs wherever the record is read, alone too: the review
//! read a standalone candidate of confidence 101 and a standalone completeness whose tags were incomplete without a
//! first reason. A reason of a known kind holds the members of its kind, so `limit_exhausted` names its limit. The
//! omitted count covers each first reason that the kept reasons lack, and when nothing was omitted, the first kept
//! reason of each part is the record's first reason apart from its detail. Only a source failure carries a partial
//! report, and an error's own limit reason finds its counter in the record.

use cc_media_probe::completeness::{Completeness, Reason};
use cc_media_probe::error::ErrorRecord;
use cc_media_probe::model::Candidate;
use cc_media_probe::{MediaReport, probe_bytes};
use serde_json::{Value, json};

#[test]
fn a_standalone_candidate_keeps_its_confidence_within_100() {
  assert!(serde_json::from_value::<Candidate>(json!({ "format": "Mp4", "confidence": 100 })).is_ok());
  assert!(serde_json::from_value::<Candidate>(json!({ "format": "Mp4", "confidence": 101 })).is_err());
}

/// A completeness whose parts are complete but the tags, with those kept reasons and that record.
fn completeness(tags: &str, reasons: Value, record: Value) -> Value {
  json!({
    "detection": "Complete",
    "discovery": "Complete",
    "headers": "Complete",
    "tags": tags,
    "chapters": "Complete",
    "attachments": "Complete",
    "reasons": reasons,
    "record": record,
  })
}

/// A report without streams with that completeness.
fn report(completeness: Value) -> Value {
  json!({
    "schema_version": "1.0",
    "format": "Matroska",
    "streams": [],
    "chapter_editions": [],
    "tag_sets": [],
    "attachments": [],
    "diagnostics": [],
    "completeness": completeness,
    "usage": { "bytes_read": 0, "peak_working_memory": 0 },
    "sources": [],
  })
}

/// Whether the completeness reads alone and inside a report, which must agree.
fn reads(completeness: Value) -> bool {
  let alone = serde_json::from_value::<Completeness>(completeness.clone()).is_ok();
  let nested = serde_json::from_value::<MediaReport>(report(completeness)).is_ok();
  assert_eq!(alone, nested, "the two entry points disagree");
  alone
}

fn tags_reason(kind: &str) -> Value {
  json!({ "part": "Tags", "reason": { "kind": kind } })
}

fn record(first_reasons: Value, omitted_reasons: &str) -> Value {
  json!({ "exhausted": [], "first_stopped": [], "first_reasons": first_reasons, "omitted_reasons": omitted_reasons })
}

#[test]
fn a_standalone_completeness_agrees_with_its_record() {
  // The review's case: incomplete tags without a first reason, which a report refused and the type alone read.
  assert!(!reads(completeness("Incomplete", json!([]), record(json!([]), "0"))));
  let first = json!([tags_reason("TruncatedInput")]);
  assert!(reads(completeness(
    "Incomplete",
    json!([tags_reason("TruncatedInput")]),
    record(first.clone(), "0")
  )));
  // A kept reason without a first reason of its part, and complete tags with a first reason.
  assert!(!reads(completeness(
    "Incomplete",
    json!([tags_reason("TruncatedInput")]),
    record(json!([]), "0")
  )));
  assert!(!reads(completeness("Complete", json!([]), record(first, "1"))));
}

#[test]
fn the_omitted_count_covers_every_first_reason_that_the_kept_reasons_lack() {
  let first = json!([tags_reason("TruncatedInput")]);
  // The review's first case: no reason kept and none omitted.
  assert!(!reads(completeness(
    "Incomplete",
    json!([]),
    record(first.clone(), "0")
  )));
  assert!(reads(completeness("Incomplete", json!([]), record(first.clone(), "1"))));
  // The review's second case: the only kept reason of the tags differs from the first, and none was omitted.
  let other = json!([tags_reason("RangeUnavailable")]);
  assert!(!reads(completeness(
    "Incomplete",
    other.clone(),
    record(first.clone(), "0")
  )));
  assert!(reads(completeness("Incomplete", other, record(first.clone(), "1"))));
  // The first kept reason may carry the detail that the record leaves out.
  let detailed = json!([{ "part": "Tags", "reason": { "kind": "TruncatedInput", "detail": "the file ends" } }]);
  assert!(reads(completeness("Incomplete", detailed, record(first, "0"))));
}

#[test]
fn a_reason_of_a_known_kind_holds_the_members_of_its_kind() {
  let read = |reason: Value| serde_json::from_value::<Reason>(reason).is_ok();
  assert!(read(json!({ "kind": "LimitExhausted", "limit": "BytesRead" })));
  assert!(read(json!({ "kind": "TruncatedInput" })));
  // The review's case: a limit reason that names no limit, which the report took with an empty `exhausted`.
  assert!(!read(json!({ "kind": "LimitExhausted" })));
  assert!(!read(json!({ "kind": "TruncatedInput", "limit": "BytesRead" })));
  assert!(!read(json!({ "kind": "RangeUnavailable", "limit": "BytesRead" })));
  // A kind of a newer version may hold whatever members it needs.
  assert!(read(
    json!({ "kind": { "Unrecognized": "quota_reached" }, "limit": "BytesRead" })
  ));
  let bare = json!([{ "part": "Tags", "reason": { "kind": "LimitExhausted" } }]);
  assert!(!reads(completeness("Incomplete", bare.clone(), record(bare, "0"))));
}

/// An error record of that kind, with the members given.
fn error_record(kind: &str, members: &[(&str, Value)]) -> Value {
  let mut error = serde_json::to_value(probe_bytes(b"").unwrap_err().to_record()).unwrap();
  error["kind"] = json!(kind);
  for (name, value) in members {
    error[*name] = value.clone();
  }
  error
}

#[test]
fn only_a_source_failure_carries_a_partial_report() {
  let partial = report(completeness("Complete", json!([]), record(json!([]), "0")));
  let read =
    |kind: &str| serde_json::from_value::<ErrorRecord>(error_record(kind, &[("partial_report", partial.clone())]));
  assert!(read("source_failed").is_ok());
  // The review's case, and every other kind that this version knows.
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
  let reason = json!({ "kind": "LimitExhausted", "limit": "BytesRead" });
  let empty = json!({ "exhausted": [], "first_stopped": [] });
  let stopped = json!({
    "exhausted": ["BytesRead"],
    "first_stopped": [{ "limit": "BytesRead", "part": "Detection" }],
    "first_reasons": [{ "part": "Detection", "reason": reason }],
  });
  let read = |record: &Value| {
    let members = [("reason", reason.clone()), ("record", record.clone())];
    serde_json::from_value::<ErrorRecord>(error_record("format_not_determined", &members))
  };
  assert!(read(&stopped).is_ok());
  assert!(read(&empty).is_err());
}
