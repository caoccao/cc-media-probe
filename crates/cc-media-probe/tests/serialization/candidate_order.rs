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

//! The candidates of an error record as a list (`api.md` sections 5.1 and 5.3), as the third review of the third
//! slice's first part asked, which read rejected candidates of the confidences 10 and then 90, an outranked candidate
//! ahead of the validated one, and a candidate caused by a truncated input beside no first reason of detection: the
//! candidates fall in confidence and stand in the order of their checks, and the record's first reason of detection
//! is the first cause in their rank, for every known kind, as the maintainer chose.

use cc_media_probe::error::ErrorRecord;
use serde_json::{Value, json};

use crate::fixtures::{detection_stopped, error_record, with, without};

fn reads(record: &Value) -> bool {
  serde_json::from_value::<ErrorRecord>(record.clone()).is_ok()
}

/// A candidate of that format, confidence and state, with that cause when one is given.
fn candidate(format: &str, confidence: u8, state: &str, cause: Option<&Value>) -> Value {
  let mut candidate = json!({ "format": format, "confidence": confidence, "state": state });
  if let Some(cause) = cause {
    candidate["reason"] = cause.clone();
  }
  candidate
}

/// An error record of that kind and format with those candidates, without the members of a source failure.
fn error(kind: &str, format: Option<&str>, candidates: Vec<Value>) -> Value {
  let mut record = without(without(error_record("1.0"), "source_failure_kind"), "source_identifier");
  record["kind"] = json!(kind);
  record["candidates"] = Value::Array(candidates);
  match format {
    Some(format) => with(record, "format", json!(format)),
    None => record,
  }
}

#[test]
fn the_candidates_fall_in_confidence() {
  let rejected = |confidences: [u8; 2]| {
    error(
      "unknown_format",
      None,
      vec![
        candidate("mp4", confidences[0], "rejected", None),
        candidate("matroska", confidences[1], "rejected", None),
      ],
    )
  };
  assert!(reads(&rejected([90, 10])));
  assert!(reads(&rejected([50, 50])));
  // The review's case: rejected candidates of the confidences 10 and then 90.
  assert!(!reads(&rejected([10, 90])));
}

/// An unsupported format whose candidates, in falling confidence, are in those states. The error names the format of
/// the candidate that validated, where one did.
fn unsupported(states: &[&str]) -> Value {
  let formats = ["avi", "mp4", "matroska", "ogg"];
  let confidences = [90, 80, 70, 60];
  let candidates = states
    .iter()
    .zip(formats.iter().zip(confidences))
    .map(|(state, (format, confidence))| candidate(format, confidence, state, None))
    .collect();
  let format = states
    .iter()
    .position(|state| *state == "validated")
    .map_or("avi", |position| formats[position]);
  error("unsupported_format", Some(format), candidates)
}

#[test]
fn the_candidates_stand_in_the_order_of_their_checks() {
  for states in [
    &["validated"][..],
    &["rejected", "validated", "outranked"],
    &["not_checkable", "rejected", "validated", "outranked"],
    &["future_state", "outranked"],
    &["validated", "future_state", "outranked"],
  ] {
    assert!(reads(&unsupported(states)), "{states:?}");
  }
  for states in [
    // The review's case: an outranked candidate ahead of the validated one.
    &["outranked", "validated"][..],
    &["outranked"],
    &["validated", "validated"],
    &["validated", "rejected"],
    &["validated", "not_checkable"],
  ] {
    assert!(!reads(&unsupported(states)), "{states:?}");
  }
}

#[test]
fn the_first_reason_of_detection_is_the_first_cause_in_the_rank() {
  let truncated = json!({ "kind": "truncated_input" });
  let range = json!({ "kind": "range_unavailable" });
  let undetermined = error(
    "format_not_determined",
    None,
    vec![
      candidate("mp4", 90, "not_checkable", None),
      candidate("matroska", 80, "not_checkable", Some(&truncated)),
      candidate("ogg", 70, "not_checkable", Some(&range)),
    ],
  );
  assert!(reads(&with(
    undetermined.clone(),
    "record",
    detection_stopped(&truncated)
  )));
  // The review's case: a candidate caused by a truncated input beside no first reason of detection.
  assert!(!reads(&undetermined));
  assert!(!reads(&with(undetermined, "record", detection_stopped(&range))));
  // Candidates without a cause give none.
  let unchecked = error(
    "format_not_determined",
    None,
    vec![candidate("mp4", 90, "not_checkable", None)],
  );
  assert!(reads(&unchecked));
  assert!(!reads(&with(unchecked, "record", detection_stopped(&truncated))));
  // Too few bytes: the error's reason is the first reason of detection.
  let few = with(
    error("format_not_determined", None, Vec::new()),
    "reason",
    truncated.clone(),
  );
  assert!(reads(&with(few.clone(), "record", detection_stopped(&truncated))));
  assert!(!reads(&few));
  assert!(!reads(&with(few, "record", detection_stopped(&range))));
  // Every known kind: a candidate ranked ahead of the one validated could not be checked.
  let ahead = |kind: &str| {
    let mut record = error(
      kind,
      Some("mp4"),
      vec![
        candidate("matroska", 90, "not_checkable", Some(&range)),
        candidate("mp4", 80, "validated", None),
      ],
    );
    record["reason"] = truncated.clone();
    record
  };
  assert!(reads(&with(
    ahead("no_stream_inventory"),
    "record",
    detection_stopped(&range)
  )));
  assert!(!reads(&ahead("no_stream_inventory")));
  // A kind of a newer version is not held to it.
  assert!(reads(&ahead("quota_exceeded")));
}

#[test]
fn a_candidate_of_a_newer_state_won_only_where_no_known_check_follows_it() {
  // As the fourth review of the third slice's first part asked: checking never resumes after a winner, and no
  // candidate wins behind a known check that followed a candidate of a newer state.
  for states in [
    &["future_state", "validated"][..],
    &["future_state", "future_state", "outranked"],
    &["rejected", "future_state", "outranked", "future_state"],
  ] {
    assert!(reads(&unsupported(states)), "{states:?}");
  }
  for states in [
    // The review's cases.
    &["future_state", "outranked", "validated"][..],
    &["future_state", "rejected", "outranked"],
    &["future_state", "outranked", "not_checkable"],
  ] {
    assert!(!reads(&unsupported(states)), "{states:?}");
  }
}

#[test]
fn the_errors_format_is_the_format_of_the_candidate_that_validated() {
  let validated = |format: &str, winner: &str| {
    error(
      "unsupported_format",
      Some(format),
      vec![candidate(winner, 90, "validated", None)],
    )
  };
  assert!(reads(&validated("mp4", "mp4")));
  // The review's case: an unsupported Matroska file whose one validated candidate is MP4.
  assert!(!reads(&validated("matroska", "mp4")));
  // A format of a newer version, on either side, is not compared.
  assert!(reads(&validated("future_format", "mp4")));
  assert!(reads(&validated("matroska", "future_format")));
}

#[test]
fn a_known_kind_whose_candidates_have_no_cause_has_no_first_reason_of_detection() {
  let truncated = json!({ "kind": "truncated_input" });
  let rejected = error(
    "unknown_format",
    None,
    vec![
      candidate("mp4", 90, "rejected", None),
      candidate("matroska", 80, "rejected", None),
    ],
  );
  assert!(reads(&rejected));
  // The fourth review's case: an unknown format whose every candidate was rejected beside a first reason of
  // detection, which section 16 of `api.md` rules out.
  assert!(!reads(&with(rejected, "record", detection_stopped(&truncated))));
  let validated = with(unsupported(&["validated"]), "record", detection_stopped(&truncated));
  assert!(!reads(&validated));
  // A source can fail while detection reads, so a source failure is not held to it.
  let failed = json!({ "kind": "source_failed" });
  assert!(reads(&with(error_record("1.0"), "record", detection_stopped(&failed))));
}
