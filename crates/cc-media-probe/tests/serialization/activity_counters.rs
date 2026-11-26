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

//! The counters that only an activity which ran can have exhausted (`api.md` sections 5.1 and 5.3; plan section
//! 7.5), as the maintainer chose at the sixth review of the third slice's first part, which read an unknown format and
//! a report whose companions were not applicable, each beside the companion requests exhausted: companions that are
//! not applicable or not enabled exhaust no companion counter, and a probe that ended at detection none that only
//! companion lookup or the prober charges. As the maintainer chose at the seventh review of that part, which read the
//! stream limit exhausted beside a primary file that could not be opened, a source failure before detection found a
//! format never ran its prober either, so it exhausts none of them and names detection alone, while a disabled
//! feature and a probe stopped before its inventory ran it, whatever they hold.

use cc_media_probe::MediaReport;
use cc_media_probe::completeness::Completeness;
use cc_media_probe::error::ErrorRecord;
use serde_json::{Value, json};

use crate::fixtures::{completeness, error_record, failed_primary, record, with, without};

/// The limits that only companion lookup or the prober charges once detection is over.
const LATER_LIMITS: [&str; 5] = [
  "companion_requests",
  "sources_opened",
  "header_bytes_per_stream",
  "retained_payloads",
  "streams",
];

fn reads_completeness(completeness: &Value) -> bool {
  serde_json::from_value::<Completeness>(completeness.clone()).is_ok()
}

fn reads_error(error: &Value) -> bool {
  serde_json::from_value::<ErrorRecord>(error.clone()).is_ok()
}

/// A completeness whose companions have that status, with their reason where lookup was off, and with those limits
/// exhausted and paired with no part.
fn companions(status: &str, limits: &[&str]) -> Value {
  let mut companions = with(completeness(), "companions", json!(status));
  if status == "not_enabled" {
    let off = json!({ "part": "companions", "reason": { "kind": "companions_not_enabled" } });
    companions["reasons"] = json!([off]);
    companions["record"]["first_reasons"] = json!([off]);
  }
  companions["record"]["exhausted"] = limits.iter().map(|limit| json!(limit)).collect();
  companions
}

/// An error of a kind other than a source failure, without its members, with the candidates that its kind allows and
/// that limit exhausted and paired with no part: an undetermined format whose one candidate could not be checked, and
/// an unsupported format whose one candidate validated.
fn ended(kind: &str, limit: &str) -> Value {
  let mut error = without(without(error_record("1.0"), "source_failure_kind"), "source_identifier");
  error["kind"] = json!(kind);
  error["record"]["exhausted"] = json!([limit]);
  match kind {
    "format_not_determined" => {
      error["candidates"] = json!([{ "format": "matroska", "confidence": 60, "state": "not_checkable" }]);
    }
    "unsupported_format" => {
      error["format"] = json!("mp4");
      error["candidates"] = json!([{ "format": "mp4", "confidence": 90, "state": "validated" }]);
    }
    _ => {}
  }
  error
}

/// The error record of a primary file that could not be opened, with that record.
fn failed(record: Value) -> Value {
  with(error_record("1.0"), "record", record)
}

/// A record whose one first reason, of that part, is the failure of the primary input.
fn failed_in(part: &str) -> Value {
  let first = json!({ "part": part, "source_identifier": "0", "reason": { "kind": "source_failed" } });
  with(record(), "first_reasons", json!([first]))
}

#[test]
fn companions_that_never_ran_exhaust_no_companion_counter() {
  for status in ["not_applicable", "not_enabled"] {
    assert!(reads_completeness(&companions(status, &[])), "{status}");
    // The review's case: the companion requests exhausted beside companions that are not applicable.
    for limit in ["companion_requests", "sources_opened"] {
      assert!(!reads_completeness(&companions(status, &[limit])), "{status} {limit}");
    }
  }
  for status in ["complete", "future_status"] {
    assert!(
      reads_completeness(&companions(status, &["companion_requests", "sources_opened"])),
      "{status}"
    );
  }
  let mut report = crate::fixtures::report("1.0");
  report["completeness"] = companions("not_applicable", &["companion_requests"]);
  assert!(serde_json::from_value::<MediaReport>(report).is_err());
}

#[test]
fn a_probe_that_ended_at_detection_exhausts_no_counter_of_a_later_activity() {
  // The review's case: an unknown format beside the companion requests exhausted.
  for kind in ["unknown_format", "format_not_determined", "unsupported_format"] {
    for limit in LATER_LIMITS {
      assert!(!reads_error(&ended(kind, limit)), "{kind} {limit}");
    }
    assert!(reads_error(&ended(kind, "working_memory")), "{kind}");
  }
  assert!(reads_error(&ended("future_kind", "companion_requests")));
}

#[test]
fn a_probe_that_failed_before_detection_found_a_format_exhausts_no_counter_of_a_later_activity() {
  let exhausting = |limit: &str| with(record(), "exhausted", json!([limit]));
  // The seventh review's case: the stream limit exhausted beside a primary file that could not be opened.
  for limit in LATER_LIMITS {
    assert!(!reads_error(&failed(exhausting(limit))), "{limit}");
  }
  for limit in ["bytes_read", "working_memory", "work_units", "future_limit"] {
    assert!(reads_error(&failed(exhausting(limit))), "{limit}");
  }
  // A source failure that holds observations, or the format that detection found, ran its prober.
  let observations = json!({ "diagnostics": [], "sources": [failed_primary()] });
  for limit in LATER_LIMITS {
    let observed = with(failed(exhausting(limit)), "observations", observations.clone());
    assert!(reads_error(&observed), "{limit}");
    let detected = with(failed(exhausting(limit)), "format", json!("matroska"));
    assert!(reads_error(&detected), "{limit}");
  }
}

#[test]
fn a_probe_that_never_ran_its_prober_names_detection_alone() {
  // A primary input that failed while detection read names detection, and no later part, which never started.
  assert!(reads_error(&failed(failed_in("detection"))));
  for part in ["discovery", "headers", "companions"] {
    assert!(!reads_error(&failed(failed_in(part))), "{part}");
    assert!(
      reads_error(&with(failed(failed_in(part)), "format", json!("matroska"))),
      "{part}"
    );
  }
  // A limit that stopped detection first is paired with it, and one paired with a later part is refused with it.
  let paired = |part: &str| {
    let mut stopped = failed_in(part);
    stopped["exhausted"] = json!(["bytes_read"]);
    stopped["first_stopped"] = json!([{ "limit": "bytes_read", "part": part }]);
    failed(stopped)
  };
  assert!(reads_error(&paired("detection")));
  assert!(!reads_error(&paired("discovery")));
  assert!(reads_error(&with(paired("discovery"), "format", json!("matroska"))));
}

#[test]
fn a_disabled_feature_or_a_probe_stopped_before_its_inventory_ran_its_prober() {
  // Neither holds observations when it observed nothing, and both reached discovery and could request companions.
  for (kind, reason) in [
    (
      "feature_disabled",
      json!({ "kind": "feature_disabled", "feature": "zlib" }),
    ),
    ("no_stream_inventory", json!({ "kind": "truncated_input" })),
  ] {
    let mut error = without(without(error_record("1.0"), "source_failure_kind"), "source_identifier");
    error["kind"] = json!(kind);
    error["format"] = json!("matroska");
    error["reason"] = reason.clone();
    error["record"] = with(
      record(),
      "first_reasons",
      json!([{ "part": "discovery", "reason": reason }]),
    );
    error["record"]["exhausted"] = json!(["streams", "companion_requests"]);
    assert!(reads_error(&error), "{kind}");
  }
}
