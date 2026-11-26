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

//! The limits that stopped detection in an error record (`api.md` sections 5.1 and 5.3), as the maintainer chose at
//! the fourth review of the third slice's first part, which read an undetermined format of too few bytes whose read
//! limit the record said had stopped discovery first: a limit that a cause of detection names stopped detection
//! first, and a probe that ended at detection names no other part in its record.

use cc_media_probe::error::ErrorRecord;
use serde_json::{Value, json};

use crate::fixtures::{error_record, record, with, without};

fn reads(record: &Value) -> bool {
  serde_json::from_value::<ErrorRecord>(record.clone()).is_ok()
}

fn limit(limit: &str) -> Value {
  json!({ "kind": "limit_exhausted", "limit": limit })
}

/// An error of that kind, without the members of a source failure, with those members.
fn error(kind: &str, members: &[(&str, Value)]) -> Value {
  let mut error = without(without(error_record("1.0"), "source_failure_kind"), "source_identifier");
  error["kind"] = json!(kind);
  for (name, value) in members {
    error[*name] = value.clone();
  }
  error
}

/// A record with those first reasons, each of a part and a reason, and those limits, each with the part it stopped
/// first.
fn stops(first_reasons: &[(&str, Value)], limits: &[(&str, &str)]) -> Value {
  let mut stops = record();
  stops["first_reasons"] = first_reasons
    .iter()
    .map(|(part, reason)| json!({ "part": part, "reason": reason }))
    .collect();
  stops["exhausted"] = limits.iter().map(|(limit, _)| json!(limit)).collect();
  stops["first_stopped"] = limits
    .iter()
    .map(|(limit, part)| json!({ "limit": limit, "part": part }))
    .collect();
  stops
}

#[test]
fn a_limit_that_ended_detection_stopped_detection_first() {
  let read = limit("bytes_read");
  let few = |stopped: Value| {
    error(
      "format_not_determined",
      &[("reason", read.clone()), ("record", stopped)],
    )
  };
  assert!(reads(&few(stops(
    &[("detection", read.clone())],
    &[("bytes_read", "detection")]
  ))));
  // The review's case: the read limit said to have stopped discovery first, which never began.
  let truncated = json!({ "kind": "truncated_input" });
  let discovery = stops(
    &[("detection", read.clone()), ("discovery", truncated)],
    &[("bytes_read", "discovery")],
  );
  assert!(!reads(&few(discovery)));
}

#[test]
fn a_limit_that_stopped_a_candidates_check_stopped_detection_first() {
  let range = json!({ "kind": "range_unavailable" });
  let depth = limit("nesting_depth");
  let candidates = json!([
    { "format": "matroska", "confidence": 90, "state": "not_checkable", "reason": range },
    { "format": "avi", "confidence": 80, "state": "not_checkable", "reason": depth },
    { "format": "mp4", "confidence": 70, "state": "validated" },
  ]);
  let stopped_first = |part: &str| {
    error(
      "no_stream_inventory",
      &[
        ("format", json!("mp4")),
        ("reason", json!({ "kind": "truncated_input" })),
        ("candidates", candidates.clone()),
        (
          "record",
          stops(
            &[("detection", range.clone()), ("discovery", depth.clone())],
            &[("nesting_depth", part)],
          ),
        ),
      ],
    )
  };
  assert!(reads(&stopped_first("detection")));
  assert!(!reads(&stopped_first("discovery")));
}

#[test]
fn a_probe_that_ended_at_detection_names_no_other_part() {
  let truncated = json!({ "kind": "truncated_input" });
  let rejected = json!([{ "format": "mp4", "confidence": 90, "state": "rejected" }]);
  let unknown = |stopped: Value| {
    error(
      "unknown_format",
      &[("candidates", rejected.clone()), ("record", stopped)],
    )
  };
  assert!(reads(&unknown(record())));
  assert!(!reads(&unknown(stops(&[("discovery", truncated.clone())], &[]))));
  let unsupported = |stopped: Value| error("unsupported_format", &[("format", json!("riff")), ("record", stopped)]);
  let work = limit("work_units");
  assert!(!reads(&unsupported(stops(
    &[("tags", work.clone())],
    &[("work_units", "tags")]
  ))));
  // A probe stopped before its inventory began discovery, and a kind of a newer version is not checked.
  let inventory = error(
    "no_stream_inventory",
    &[
      ("format", json!("mp4")),
      ("reason", truncated.clone()),
      ("record", stops(&[("discovery", truncated.clone())], &[])),
    ],
  );
  assert!(reads(&inventory));
  assert!(reads(&with(inventory, "kind", json!("quota_exceeded"))));
}
