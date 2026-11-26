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

//! The parts that a limit can stop and the limits that can stop a companion (`api.md` sections 4.5, 5.3, 5.5 and 13;
//! plan section 7.5), as the maintainer chose at the seventh review of the third slice's first part, which read the
//! companion requests paired with the tags, a report whose stream limit stopped detection, a companion refused before
//! its request by the stream limit, and an insufficient detection whose reason named the companion requests, a cause
//! that the error of the same detection refuses:
//!
//! - **A limit that only one activity charges** stops only the parts of that activity, wherever it stands against
//!   one: the stream limit discovery, the per-stream header limit the headers and the attachments, the payload limit
//!   the attachments, and the companion requests and sources opened the companions. A limit or a part of a newer
//!   version is not checked.
//! - **A companion refused by a limit** names the request limit, the source limit or the retained data before its
//!   request, and the request, source or read limit, or the working memory, after it.

use cc_media_probe::completeness::{HeaderStatus, LimitRecord, PartReason};
use cc_media_probe::error::{CandidateCheck, ErrorRecord};
use cc_media_probe::source::SourceEntry;
use cc_media_probe::{Detection, MediaReport};
use serde::de::DeserializeOwned;
use serde_json::{Value, json};

use crate::fixtures::{detection, detection_stopped, error_record, record, report, with, without};

/// The parts that this version knows.
const PARTS: [&str; 8] = [
  "detection",
  "discovery",
  "headers",
  "tags",
  "chapters",
  "attachments",
  "timing",
  "companions",
];

/// The limits that only one activity charges, with the parts that each can stop.
const SCOPES: [(&str, &[&str]); 5] = [
  ("streams", &["discovery"]),
  ("header_bytes_per_stream", &["headers", "attachments"]),
  ("retained_payloads", &["attachments"]),
  ("companion_requests", &["companions"]),
  ("sources_opened", &["companions"]),
];

/// The limits that every activity can charge, and one of a newer version.
const GENERAL_LIMITS: [&str; 7] = [
  "bytes_read",
  "working_memory",
  "retained_data",
  "decompressed_output",
  "work_units",
  "nesting_depth",
  "future_limit",
];

fn reads<Record: DeserializeOwned>(value: &Value) -> bool {
  serde_json::from_value::<Record>(value.clone()).is_ok()
}

/// A reason of that kind that names that limit.
fn naming(kind: &str, limit: &str) -> Value {
  json!({ "kind": kind, "limit": limit })
}

/// A record whose limit stopped that part first, whose first reason the input's end gave.
fn paired(limit: &str, part: &str) -> Value {
  let mut record = with(record(), "exhausted", json!([limit]));
  record["first_stopped"] = json!([{ "limit": limit, "part": part }]);
  record["first_reasons"] = json!([{ "part": part, "reason": { "kind": "truncated_input" } }]);
  record
}

/// The check of a candidate of that state whose cause is that limit exhausted.
fn checked(state: &str, limit: &str) -> Value {
  let check = json!({ "format": "mp4", "confidence": 60, "state": state });
  with(check, "reason", naming("limit_exhausted", limit))
}

/// A companion's reason that a limit gave, with its source where its request was made.
fn limited(source: Option<&str>, limit: &str) -> Value {
  let mut reason = json!({ "part": "companions", "reason": naming("companion_limited", limit) });
  if let Some(source) = source {
    reason["source_identifier"] = json!(source);
  }
  reason
}

#[test]
fn a_limit_that_one_activity_charges_is_paired_only_with_a_part_of_that_activity() {
  // The review's cases among them: the companion requests paired with the tags, and the stream limit with detection.
  for (limit, scope) in SCOPES {
    for part in PARTS {
      assert_eq!(
        reads::<LimitRecord>(&paired(limit, part)),
        scope.contains(&part),
        "{limit} {part}"
      );
    }
    assert!(reads::<LimitRecord>(&paired(limit, "future_part")), "{limit}");
  }
  for limit in GENERAL_LIMITS {
    for part in PARTS {
      assert!(reads::<LimitRecord>(&paired(limit, part)), "{limit} {part}");
    }
  }
  // The review's case in a report: the stream limit said to have stopped detection.
  let stopped = |limit: &str| {
    let mut stopped = report("1.0");
    stopped["completeness"]["detection"] = json!("incomplete");
    stopped["completeness"]["record"] = paired(limit, "detection");
    stopped["completeness"]["reasons"] = stopped["completeness"]["record"]["first_reasons"].clone();
    stopped
  };
  assert!(reads::<MediaReport>(&stopped("nesting_depth")));
  assert!(!reads::<MediaReport>(&stopped("streams")));
}

#[test]
fn a_reason_names_a_limit_that_can_stop_its_part() {
  for (limit, scope) in SCOPES {
    for part in PARTS {
      let reason = json!({ "part": part, "reason": naming("limit_exhausted", limit) });
      assert_eq!(reads::<PartReason>(&reason), scope.contains(&part), "{limit} {part}");
    }
  }
  let newer = json!({ "part": "future_part", "reason": naming("limit_exhausted", "streams") });
  assert!(reads::<PartReason>(&newer));
  // A known header state's reason stands against the headers, and a candidate's cause against detection.
  let header = |state: &str, limit: &str| json!({ "state": state, "reason": naming("limit_exhausted", limit) });
  for state in ["not_found_within_budget", "partially_parsed_for_lack_of_bytes"] {
    assert!(
      reads::<HeaderStatus>(&header(state, "header_bytes_per_stream")),
      "{state}"
    );
    for limit in ["streams", "retained_payloads", "companion_requests"] {
      assert!(!reads::<HeaderStatus>(&header(state, limit)), "{state} {limit}");
    }
  }
  assert!(reads::<HeaderStatus>(&header("future_state", "streams")));
  assert!(reads::<CandidateCheck>(&checked("not_checkable", "nesting_depth")));
  for (limit, _) in SCOPES {
    assert!(!reads::<CandidateCheck>(&checked("not_checkable", limit)), "{limit}");
  }
  assert!(reads::<CandidateCheck>(&checked("future_state", "streams")));
}

#[test]
fn a_companion_limited_names_a_limit_that_can_stop_the_companion_where_it_stands() {
  // Before its request, which a reason without a source tells.
  for limit in ["companion_requests", "sources_opened", "retained_data", "future_limit"] {
    assert!(reads::<PartReason>(&limited(None, limit)), "{limit}");
  }
  // The review's case: the stream limit refusing a companion before its request.
  for limit in ["streams", "bytes_read", "working_memory", "work_units"] {
    assert!(!reads::<PartReason>(&limited(None, limit)), "{limit}");
  }
  // After its request, which names its source, and in the entry that the request has.
  let entry = |limit: &str| {
    json!({
      "identifier": "1",
      "role": "subtitle_index",
      "outcome": "limit_reached",
      "reason": naming("companion_limited", limit),
    })
  };
  for limit in [
    "companion_requests",
    "sources_opened",
    "bytes_read",
    "working_memory",
    "future_limit",
  ] {
    assert!(reads::<PartReason>(&limited(Some("1"), limit)), "{limit}");
    assert!(reads::<SourceEntry>(&entry(limit)), "{limit}");
  }
  for limit in ["retained_data", "streams", "work_units"] {
    assert!(!reads::<PartReason>(&limited(Some("1"), limit)), "{limit}");
    assert!(!reads::<SourceEntry>(&entry(limit)), "{limit}");
  }
}

#[test]
fn a_detection_and_its_error_refuse_a_cause_that_cannot_stop_detection() {
  let insufficient = |limit: &str| {
    let mut insufficient = with(detection("1.0"), "outcome", json!("insufficient_data"));
    insufficient["insufficient_reason"] = naming("limit_exhausted", limit);
    without(without(insufficient, "format"), "confidence")
  };
  let undetermined = |limit: &str| {
    let cause = naming("limit_exhausted", limit);
    let mut error = without(without(error_record("1.0"), "source_failure_kind"), "source_identifier");
    error["kind"] = json!("format_not_determined");
    error["reason"] = cause.clone();
    error["record"] = with(detection_stopped(&cause), "exhausted", json!([limit]));
    error["record"]["first_stopped"] = json!([{ "limit": limit, "part": "detection" }]);
    error
  };
  // The review's case: the companion requests as the cause of too few bytes, read in both records.
  for (limit, _) in SCOPES {
    assert!(!reads::<Detection>(&insufficient(limit)), "{limit}");
    assert!(!reads::<ErrorRecord>(&undetermined(limit)), "{limit}");
  }
  for limit in GENERAL_LIMITS {
    assert!(reads::<Detection>(&insufficient(limit)), "{limit}");
    assert!(reads::<ErrorRecord>(&undetermined(limit)), "{limit}");
  }
  let newer = with(insufficient("companion_requests"), "outcome", json!("future_outcome"));
  assert!(reads::<Detection>(&newer));
}
