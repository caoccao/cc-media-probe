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

//! The endings of `api.md` section 5.1 that an error record states, and what it names (`json-output.md` sections 4.4
//! and 10), as the second review of the third slice's first part asked: an unknown format's candidates were all
//! rejected and an undetermined format's include none validated; only a candidate that could not be checked has a
//! cause; a disabled feature names its feature, a probe stopped before its inventory its reason and a source failure
//! its source; the first reasons of a record before an inventory name no stream and only sources of its table; a
//! partial report is of the error's format; and every limit that a reason names is exhausted.

use cc_media_probe::error::{CandidateCheck, ErrorRecord};
use serde_json::{Value, json};

use crate::fixtures::{detection_stopped, error_record, failed_primary, partial_report, with, without};

fn reads(record: &Value) -> bool {
  serde_json::from_value::<ErrorRecord>(record.clone()).is_ok()
}

/// An error record of that kind and format, without the members of a source failure.
fn error_of(kind: &str, format: Option<&str>) -> Value {
  let mut record = without(without(error_record("1.0"), "source_failure_kind"), "source_identifier");
  record["kind"] = json!(kind);
  if let Some(format) = format {
    record["format"] = json!(format);
  }
  record
}

fn check(state: &str, reason: Option<Value>) -> Value {
  let mut check = json!({ "format": "matroska", "confidence": 60, "state": state });
  if let Some(reason) = reason {
    check["reason"] = reason;
  }
  check
}

fn truncated() -> Value {
  json!({ "kind": "truncated_input" })
}

#[test]
fn an_unknown_format_rejected_every_candidate_and_an_undetermined_one_validated_none() {
  let candidates = |record: Value, checks: Vec<Value>| with(record, "candidates", Value::Array(checks));
  assert!(reads(&candidates(
    error_of("unknown_format", None),
    vec![check("rejected", None)]
  )));
  assert!(reads(&candidates(
    with(
      error_of("format_not_determined", None),
      "record",
      detection_stopped(&truncated())
    ),
    vec![check("not_checkable", Some(truncated()))]
  )));
  assert!(reads(&candidates(
    error_of("format_not_determined", None),
    vec![check("not_checkable", None)]
  )));
  assert!(reads(&with(
    with(
      error_of("format_not_determined", None),
      "record",
      detection_stopped(&truncated())
    ),
    "reason",
    truncated()
  )));
  for record in [
    // The review's cases: a validated candidate beside an unknown and an undetermined format.
    candidates(error_of("unknown_format", None), vec![check("validated", None)]),
    candidates(error_of("format_not_determined", None), vec![check("validated", None)]),
    candidates(error_of("unknown_format", None), vec![check("not_checkable", None)]),
    // An undetermined format with neither a reason nor a candidate that could not be checked.
    error_of("format_not_determined", None),
    candidates(error_of("format_not_determined", None), vec![check("rejected", None)]),
  ] {
    assert!(!reads(&record), "{record}");
  }
}

#[test]
fn only_a_candidate_that_could_not_be_checked_has_a_cause() {
  let reads_check = |check: &Value| serde_json::from_value::<CandidateCheck>(check.clone()).is_ok();
  for cause in [
    truncated(),
    json!({ "kind": "range_unavailable" }),
    json!({ "kind": "limit_exhausted", "limit": "bytes_read" }),
    json!({ "kind": "future_kind" }),
  ] {
    assert!(reads_check(&check("not_checkable", Some(cause.clone()))), "{cause}");
  }
  assert!(reads_check(&check("future_state", Some(truncated()))));
  for state in ["validated", "rejected", "outranked"] {
    // The review's case: a validated candidate whose cause is a truncated input.
    assert!(!reads_check(&check(state, Some(truncated()))), "{state}");
    assert!(reads_check(&check(state, None)), "{state}");
  }
  assert!(!reads_check(&check(
    "not_checkable",
    Some(json!({ "kind": "companion_declined" }))
  )));
}

#[test]
fn a_disabled_feature_a_stop_before_the_inventory_and_a_source_failure_name_what_ended_them() {
  let feature = json!({ "kind": "feature_disabled", "feature": "zlib" });
  assert!(reads(&with(
    error_of("feature_disabled", Some("matroska")),
    "reason",
    feature
  )));
  assert!(reads(&with(
    error_of("no_stream_inventory", Some("matroska")),
    "reason",
    truncated()
  )));
  assert!(reads(&error_record("1.0")));
  for record in [
    // The review's cases: a disabled feature without its feature, and a source failure without its source.
    error_of("feature_disabled", Some("matroska")),
    with(error_of("feature_disabled", Some("matroska")), "reason", truncated()),
    error_of("no_stream_inventory", Some("matroska")),
    without(error_record("1.0"), "source_identifier"),
  ] {
    assert!(!reads(&record), "{record}");
  }
}

/// A source failure before the inventory whose observations hold the primary input, with that first reason.
fn observed(first_reason: Value) -> Value {
  let mut record = with(
    error_record("1.0"),
    "observations",
    json!({ "diagnostics": [], "sources": [failed_primary()] }),
  );
  record["record"]["first_reasons"] = Value::Array(vec![first_reason]);
  record
}

#[test]
fn the_first_reasons_before_an_inventory_name_no_stream_and_only_sources_of_the_table() {
  assert!(reads(&observed(json!({ "part": "discovery", "source_identifier": "0",
    "reason": { "kind": "source_failed" } }))));
  // The review's cases: the source 9 and the stream 9 beside observations of the primary input alone.
  assert!(!reads(&observed(
    json!({ "part": "discovery", "source_identifier": "9",
    "reason": { "kind": "source_failed" } })
  )));
  assert!(!reads(&observed(
    json!({ "part": "headers", "stream": 9, "reason": { "kind": "truncated_input" } })
  )));
  let mut unobserved = error_record("1.0");
  unobserved["record"]["first_reasons"] =
    json!([{ "part": "discovery", "source_identifier": "1", "reason": { "kind": "source_failed" } }]);
  assert!(!reads(&unobserved));
}

#[test]
fn a_partial_report_is_of_the_errors_format() {
  let partial = |format: Option<&str>, report_format: &str| {
    let mut partial = partial_report("1.0");
    partial["format"] = json!(report_format);
    let record = with(error_record("1.0"), "partial_report", partial);
    match format {
      Some(format) => with(record, "format", json!(format)),
      None => record,
    }
  };
  assert!(reads(&partial(Some("matroska"), "matroska")));
  assert!(reads(&partial(Some("future_format"), "matroska")));
  // The second review's case: a source failure of an AVI file with a Matroska partial report.
  assert!(!reads(&partial(Some("avi"), "matroska")));
  // The third review's case: a Matroska partial report beside no format, which detection found (`api.md` section
  // 5.1); a kind of a newer version may leave it out.
  assert!(!reads(&partial(None, "matroska")));
  assert!(reads(&with(partial(None, "matroska"), "kind", json!("quota_exceeded"))));
}

#[test]
fn every_limit_that_a_candidate_or_an_observed_entry_names_is_exhausted() {
  let stopped = |record: Value, part: &str| {
    let mut record = record;
    record["record"]["exhausted"] = json!(["bytes_read"]);
    record["record"]["first_stopped"] = json!([{ "limit": "bytes_read", "part": part }]);
    record["record"]["first_reasons"] =
      json!([{ "part": part, "reason": { "kind": "limit_exhausted", "limit": "bytes_read" } }]);
    record
  };
  let limited = json!({ "kind": "limit_exhausted", "limit": "bytes_read" });
  let undetermined = with(
    error_of("format_not_determined", None),
    "candidates",
    json!([check("not_checkable", Some(limited))]),
  );
  assert!(reads(&stopped(undetermined.clone(), "detection")));
  assert!(!reads(&undetermined));
  let refused = json!({
    "identifier": "1",
    "role": "subtitle_index",
    "outcome": "limit_reached",
    "reason": { "kind": "companion_limited", "limit": "bytes_read" },
  });
  let entries = with(
    error_record("1.0"),
    "observations",
    json!({ "diagnostics": [], "sources": [failed_primary(), refused] }),
  );
  assert!(reads(&stopped(entries.clone(), "companions")));
  assert!(!reads(&entries));
}

#[test]
fn too_few_bytes_have_their_reason_and_an_ambiguous_result_has_candidates_that_carry_theirs() {
  let reasoned =
    |record: Value, reason: Value| with(with(record, "record", detection_stopped(&reason)), "reason", reason);
  let declined = json!({ "kind": "companion_declined" });
  let newer = json!({ "kind": "future_kind" });
  assert!(reads(&reasoned(error_of("format_not_determined", None), newer)));
  // The review's cases: a companion's decline as the reason of too few bytes, and a reason beside candidates that
  // could not be checked, which carry their own causes.
  assert!(!reads(&reasoned(error_of("format_not_determined", None), declined)));
  let uncheckable = vec![check("not_checkable", Some(truncated()))];
  let both = with(
    reasoned(error_of("format_not_determined", None), truncated()),
    "candidates",
    Value::Array(uncheckable),
  );
  assert!(!reads(&both));
  let beside_rejected = with(
    reasoned(error_of("format_not_determined", None), truncated()),
    "candidates",
    json!([check("rejected", None)]),
  );
  assert!(!reads(&beside_rejected));
  // The review's case: a record of version 1.1 whose one candidate is in a state of a newer version, which only
  // its version can tell, without a reason.
  let newer_state = with(
    with(
      error_of("format_not_determined", None),
      "candidates",
      json!([check("future_state", None)]),
    ),
    "schema_version",
    json!("1.1"),
  );
  assert!(reads(&newer_state), "{newer_state}");
}
