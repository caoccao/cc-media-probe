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

//! A detection (`json-output.md` section 4.3; `api.md` section 8): each outcome that this version knows has the
//! members that it needs and none that another outcome needs; `bytes_needed` has the twin `bytes_needed_exact`
//! beyond 2^53 - 1; the candidates name each format once and fall in confidence; insufficient data names the input's
//! end or a limit; a detection keeps its usage; and a detection of a newer outcome keeps the members that it does not
//! read and writes them back after its own, while one of a known outcome passes them by.

use cc_media_probe::Detection;
use serde_json::{Value, json};

use crate::fixtures::{detection as identified, usage, with, without};

fn read(detection: Value) -> Result<Detection, serde_json::Error> {
  serde_json::from_value(complete(detection))
}

/// The detection with its version, and the empty list of candidates and the usage where it has none.
fn complete(mut detection: Value) -> Value {
  detection["schema_version"] = json!("1.0");
  if detection.get("candidates").is_none() {
    detection["candidates"] = json!([]);
  }
  if detection.get("usage").is_none() {
    detection["usage"] = usage();
  }
  detection
}

fn hint() -> Value {
  json!({ "format": "matroska", "origin": "file_name" })
}

fn candidates() -> Value {
  json!([{ "format": "mp4", "confidence": 60 }, { "format": "quicktime", "confidence": 60 }])
}

#[test]
fn each_known_outcome_reads_with_its_own_members() {
  for detection in [
    json!({ "outcome": "identified", "format": "matroska", "confidence": 100 }),
    json!({ "outcome": "family_only", "format": "iso_base_media" }),
    json!({ "outcome": "ambiguous", "candidates": candidates() }),
    json!({ "outcome": "hint_only", "hint": hint() }),
    json!({ "outcome": "insufficient_data", "bytes_needed": 8, "insufficient_reason": { "kind": "truncated_input" } }),
    json!({ "outcome": "insufficient_data" }),
    json!({ "outcome": "unknown" }),
  ] {
    let read_back = read(detection.clone()).unwrap_or_else(|error| panic!("{detection}: {error}"));
    let written = serde_json::to_value(&read_back).unwrap();
    assert_eq!(read(written).unwrap(), read_back);
  }
}

#[test]
fn an_outcome_without_the_member_it_needs_is_malformed() {
  for detection in [
    json!({ "outcome": "identified" }),
    json!({ "outcome": "identified", "format": "matroska" }),
    json!({ "outcome": "identified", "format": "iso_base_media", "confidence": 100 }),
    json!({ "outcome": "family_only" }),
    json!({ "outcome": "family_only", "format": "matroska" }),
    json!({ "outcome": "ambiguous", "candidates": [] }),
    json!({ "outcome": "ambiguous", "candidates": [{ "format": "mp4", "confidence": 60 }] }),
    json!({ "outcome": "hint_only" }),
  ] {
    assert!(read(detection.clone()).is_err(), "{detection}");
  }
}

#[test]
fn an_outcome_beside_a_member_of_another_outcome_is_malformed() {
  for detection in [
    json!({ "outcome": "identified", "format": "matroska", "confidence": 100, "hint": hint() }),
    json!({ "outcome": "identified", "format": "matroska", "confidence": 100, "candidates": candidates() }),
    json!({ "outcome": "family_only", "format": "iso_base_media", "confidence": 90 }),
    json!({ "outcome": "ambiguous", "candidates": candidates(), "format": "mp4" }),
    json!({ "outcome": "hint_only", "hint": hint(), "format": "matroska" }),
    json!({ "outcome": "insufficient_data", "format": "matroska" }),
    json!({ "outcome": "unknown", "bytes_needed": 8 }),
    json!({ "outcome": "unknown", "insufficient_reason": { "kind": "truncated_input" } }),
    json!({ "outcome": "unknown", "hint": hint() }),
  ] {
    assert!(read(detection.clone()).is_err(), "{detection}");
  }
}

#[test]
fn a_confidence_lies_from_0_to_100_and_candidates_fall_with_it() {
  assert!(read(json!({ "outcome": "identified", "format": "matroska", "confidence": 101 })).is_err());
  let rising = json!([{ "format": "mp4", "confidence": 40 }, { "format": "quicktime", "confidence": 60 }]);
  assert!(read(json!({ "outcome": "ambiguous", "candidates": rising })).is_err());
}

#[test]
fn bytes_needed_beyond_the_exact_range_is_its_twin() {
  let large = read(json!({ "outcome": "insufficient_data", "bytes_needed_exact": "9007199254740993" })).unwrap();
  assert_eq!(large.bytes_needed(), Some(9_007_199_254_740_993));
  assert_eq!(
    serde_json::to_value(&large).unwrap()["bytes_needed_exact"],
    json!("9007199254740993")
  );
  for wrong in [
    json!({ "outcome": "insufficient_data", "bytes_needed": 9_007_199_254_740_993_u64 }),
    json!({ "outcome": "insufficient_data", "bytes_needed_exact": "8" }),
    json!({ "outcome": "insufficient_data", "bytes_needed": 8, "bytes_needed_exact": "9007199254740993" }),
  ] {
    assert!(read(wrong.clone()).is_err(), "{wrong}");
  }
}

#[test]
fn the_candidates_name_each_format_once() {
  let twice = json!([{ "format": "mp4", "confidence": 60 }, { "format": "mp4", "confidence": 60 }]);
  assert!(read(json!({ "outcome": "ambiguous", "candidates": twice.clone() })).is_err());
  let three = json!([
    { "format": "mp4", "confidence": 60 },
    { "format": "quicktime", "confidence": 60 },
    { "format": "mp4", "confidence": 50 },
  ]);
  assert!(read(json!({ "outcome": "ambiguous", "candidates": three })).is_err());
  assert!(read(json!({ "outcome": "identified_by_content", "candidates": twice })).is_err());
}

#[test]
fn insufficient_data_names_the_inputs_end_or_a_limit() {
  for (reason, valid) in [
    (json!({ "kind": "truncated_input" }), true),
    (json!({ "kind": "limit_exhausted", "limit": "bytes_read" }), true),
    (json!({ "kind": "window_closed" }), true),
    (json!({ "kind": "range_unavailable" }), false),
  ] {
    let detection = json!({ "outcome": "insufficient_data", "insufficient_reason": reason });
    assert_eq!(read(detection.clone()).is_ok(), valid, "{detection}");
  }
}

#[test]
fn a_detection_keeps_its_usage() {
  let usage = with(usage(), "bytes_read", json!(4096));
  let read_back = read(json!({ "outcome": "unknown", "usage": usage.clone() })).unwrap();
  assert_eq!(read_back.usage().bytes_read(), 4096);
  assert_eq!(serde_json::to_value(&read_back).unwrap()["usage"], usage);
  assert!(serde_json::from_value::<Detection>(without(complete(json!({ "outcome": "unknown" })), "usage")).is_err());
}

#[test]
fn a_detection_of_a_newer_outcome_keeps_the_members_that_it_does_not_read() {
  let text =
    r#"{"schema_version":"1.1","outcome":"deferred","retry_after":3,"candidates":[],"usage":USAGE,"later":[1]}"#
      .replace("USAGE", &usage().to_string());
  let newer: Detection = serde_json::from_str(&text).unwrap();
  let written = serde_json::to_string(&newer).unwrap();
  assert!(written.ends_with(r#""retry_after":3,"later":[1]}"#), "{written}");
  assert_eq!(serde_json::from_str::<Detection>(&written).unwrap(), newer);
  // Members before the outcome are held until it is known, and kept for a newer outcome.
  let before = r#"{"retry_after":3,"schema_version":"1.1","candidates":[],"usage":USAGE,"outcome":"deferred"}"#
    .replace("USAGE", &usage().to_string());
  let held = serde_json::to_string(&serde_json::from_str::<Detection>(&before).unwrap()).unwrap();
  assert!(held.ends_with(r#""retry_after":3}"#), "{held}");
  // A detection of a known outcome passes such a member by and does not write it back.
  let known = with(identified("1.1"), "retry_after", json!(3));
  let written = serde_json::to_value(serde_json::from_value::<Detection>(known).unwrap()).unwrap();
  assert!(written.get("retry_after").is_none(), "{written}");
  // A member that it keeps holds no null.
  let null = text.replace(r#""retry_after":3"#, r#""retry_after":null"#);
  assert!(serde_json::from_str::<Detection>(&null).is_err());
}
