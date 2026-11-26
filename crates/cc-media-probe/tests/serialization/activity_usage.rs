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

//! The usage and the observed sources of a result whose activities never ran (`api.md` sections 5.3, 6.14, 8 and 10.2;
//! plan section 7.5), as the maintainer chose at the seventh review of the third slice's first part, which read an
//! unknown format and a report whose companions were not applicable, each beside companion requests and sources
//! opened counted in its usage, and an unknown format whose observations held an opened companion: lookup that never
//! ran counted no request and opened no companion, and a probe that never ran its prober, as a detection never does,
//! retained no payload either.

use cc_media_probe::error::ErrorRecord;
use cc_media_probe::{Detection, MediaReport};
use serde_json::{Value, json};

use crate::fixtures::{detection, error_record, primary_source, report, with, without};

/// The counters that only companion lookup charges, which a result whose lookup never ran leaves at 0.
const LOOKUP_COUNTERS: [&str; 2] = ["companion_requests", "sources_opened"];

fn reads_error(error: &Value) -> bool {
  serde_json::from_value::<ErrorRecord>(error.clone()).is_ok()
}

fn reads_report(report: &Value) -> bool {
  serde_json::from_value::<MediaReport>(report.clone()).is_ok()
}

fn reads_detection(detection: &Value) -> bool {
  serde_json::from_value::<Detection>(detection.clone()).is_ok()
}

/// The value whose usage counts 1 for that counter.
fn counting(mut value: Value, counter: &str) -> Value {
  value["usage"][counter] = json!(1);
  value
}

/// An unknown format, which ended at detection.
fn unknown() -> Value {
  let mut error = without(without(error_record("1.0"), "source_failure_kind"), "source_identifier");
  error["kind"] = json!("unknown_format");
  error
}

/// A probe stopped before its inventory, which ran its prober.
fn stopped_before_its_inventory() -> Value {
  let mut error = with(unknown(), "kind", json!("no_stream_inventory"));
  error["format"] = json!("matroska");
  error["reason"] = json!({ "kind": "truncated_input" });
  error
}

/// The value with observations of these sources.
fn observing(error: Value, sources: &Value) -> Value {
  with(error, "observations", json!({ "diagnostics": [], "sources": sources }))
}

#[test]
fn a_probe_that_never_ran_its_prober_counts_nothing_of_a_later_activity() {
  for counter in ["companion_requests", "sources_opened", "retained_payloads"] {
    // The review's cases: an unknown format whose usage counts companion requests or sources opened.
    assert!(!reads_error(&counting(unknown(), counter)), "{counter}");
    // A primary file that could not be opened never ran its prober either.
    assert!(!reads_error(&counting(error_record("1.0"), counter)), "{counter}");
    assert!(
      reads_error(&counting(stopped_before_its_inventory(), counter)),
      "{counter}"
    );
    let detected = with(error_record("1.0"), "format", json!("matroska"));
    assert!(reads_error(&counting(detected, counter)), "{counter}");
  }
  for counter in ["bytes_read", "peak_working_memory", "retained_data", "work_units"] {
    assert!(reads_error(&counting(unknown(), counter)), "{counter}");
  }
  assert!(reads_error(&counting(
    with(unknown(), "kind", json!("future_kind")),
    "companion_requests"
  )));
}

#[test]
fn a_probe_that_ended_at_detection_observes_no_companion() {
  let companion = json!({ "identifier": "1", "role": "subtitle_index", "outcome": "opened" });
  assert!(reads_error(&observing(unknown(), &json!([primary_source()]))));
  // The review's case: an unknown format whose observations hold an opened companion.
  assert!(!reads_error(&observing(
    unknown(),
    &json!([primary_source(), companion.clone()])
  )));
  let mut ran = observing(stopped_before_its_inventory(), &json!([primary_source(), companion]));
  ran["usage"]["companion_requests"] = json!(1);
  ran["usage"]["sources_opened"] = json!(1);
  assert!(reads_error(&ran));
}

#[test]
fn companions_that_never_ran_stand_beside_a_usage_without_lookup() {
  let companions = |status: &str| {
    let mut companions = report("1.0");
    companions["completeness"]["companions"] = json!(status);
    if status == "not_enabled" {
      let off = json!({ "part": "companions", "reason": { "kind": "companions_not_enabled" } });
      companions["completeness"]["reasons"] = json!([off]);
      companions["completeness"]["record"]["first_reasons"] = json!([off]);
    }
    companions
  };
  for status in ["not_applicable", "not_enabled"] {
    assert!(reads_report(&companions(status)), "{status}");
    // The review's case: companions not applicable beside companion requests or sources opened counted.
    for counter in LOOKUP_COUNTERS {
      assert!(
        !reads_report(&counting(companions(status), counter)),
        "{status} {counter}"
      );
    }
    // A report retains payloads whatever its companions, and the prober ran.
    assert!(
      reads_report(&counting(companions(status), "retained_payloads")),
      "{status}"
    );
  }
  for status in ["complete", "future_status"] {
    for counter in LOOKUP_COUNTERS {
      assert!(
        reads_report(&counting(companions(status), counter)),
        "{status} {counter}"
      );
    }
  }
}

#[test]
fn a_detection_counts_no_companion_and_no_payload() {
  for counter in ["companion_requests", "sources_opened", "retained_payloads"] {
    assert!(!reads_detection(&counting(detection("1.0"), counter)), "{counter}");
    let newer = with(detection("1.0"), "outcome", json!("future_outcome"));
    assert!(reads_detection(&counting(newer, counter)), "{counter}");
  }
  for counter in ["bytes_read", "peak_working_memory", "decompressed_output", "work_units"] {
    assert!(reads_detection(&counting(detection("1.0"), counter)), "{counter}");
  }
}
