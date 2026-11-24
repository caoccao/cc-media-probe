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

//! The fourteenth review's cases of a detection (`json-output.md` section 4.3; `api.md` section 8): each outcome that
//! this version knows has the members it needs and none that another outcome needs, so an entry's outcome is read
//! from a detection that describes it. The review read an `ok` entry whose identified detection had neither format
//! nor confidence. `bytes_needed` has the twin `bytes_needed_exact` beyond 2^53 - 1, as the maintainer chose.

use cc_media_probe::document::Document;
use cc_media_probe::model::Detection;
use serde_json::{Value, json};

fn read(detection: Value) -> Result<Detection, serde_json::Error> {
  serde_json::from_value(complete(detection))
}

/// The detection with its version and, where it has none, the empty list of candidates that every detection holds.
fn complete(detection: Value) -> Value {
  let mut text = detection;
  text["schema_version"] = json!("1.0");
  if text.get("candidates").is_none() {
    text["candidates"] = json!([]);
  }
  text
}

fn hint() -> Value {
  json!({ "format": "Matroska", "origin": "FileName" })
}

fn candidates() -> Value {
  json!([{ "format": "Mp4", "confidence": 60 }, { "format": "QuickTime", "confidence": 60 }])
}

/// One valid detection of each outcome that this version knows, and the outcome of its entry.
fn valid() -> Vec<(Value, &'static str)> {
  vec![
    (
      json!({ "outcome": "identified", "format": "Matroska", "confidence": 100 }),
      "ok",
    ),
    (json!({ "outcome": "family_only", "format": "IsoBaseMedia" }), "partial"),
    (json!({ "outcome": "ambiguous", "candidates": candidates() }), "partial"),
    (json!({ "outcome": "hint_only", "hint": hint() }), "partial"),
    (
      json!({ "outcome": "insufficient_data", "bytes_needed": 8, "insufficient_reason": { "kind": "TruncatedInput" } }),
      "error",
    ),
    (json!({ "outcome": "insufficient_data" }), "error"),
    (json!({ "outcome": "unknown" }), "error"),
  ]
}

#[test]
fn each_known_outcome_reads_with_its_own_members() {
  for (detection, outcome) in valid() {
    let read_back = read(detection.clone()).unwrap_or_else(|error| panic!("{detection}: {error}"));
    let written = serde_json::to_value(&read_back).unwrap();
    assert_eq!(read(written).unwrap(), read_back);
    let entry = json!({ "index": 0, "input": "a", "outcome": outcome, "detection": complete(detection.clone()) });
    let document = json!({ "schema_version": "1.0", "generator": { "name": "g", "version": "1" }, "results": [entry] });
    assert!(serde_json::from_value::<Document>(document).is_ok(), "{detection}");
  }
}

#[test]
fn an_outcome_without_the_member_it_needs_is_malformed() {
  for detection in [
    // The review's case, and an identification without its confidence or with a family.
    json!({ "outcome": "identified" }),
    json!({ "outcome": "identified", "format": "Matroska" }),
    json!({ "outcome": "identified", "format": "IsoBaseMedia", "confidence": 100 }),
    json!({ "outcome": "family_only" }),
    json!({ "outcome": "family_only", "format": "Matroska" }),
    json!({ "outcome": "ambiguous", "candidates": [] }),
    json!({ "outcome": "ambiguous", "candidates": [{ "format": "Mp4", "confidence": 60 }] }),
    json!({ "outcome": "hint_only" }),
  ] {
    assert!(read(detection.clone()).is_err(), "{detection}");
  }
  let entry =
    json!({ "index": 0, "input": "a", "outcome": "ok", "detection": complete(json!({ "outcome": "identified" })) });
  let document = json!({ "schema_version": "1.0", "generator": { "name": "g", "version": "1" }, "results": [entry] });
  assert!(serde_json::from_value::<Document>(document).is_err());
}

#[test]
fn an_outcome_beside_a_member_of_another_outcome_is_malformed() {
  for detection in [
    json!({ "outcome": "identified", "format": "Matroska", "confidence": 100, "hint": hint() }),
    json!({ "outcome": "identified", "format": "Matroska", "confidence": 100, "candidates": candidates() }),
    json!({ "outcome": "family_only", "format": "IsoBaseMedia", "confidence": 90 }),
    json!({ "outcome": "ambiguous", "candidates": candidates(), "format": "Mp4" }),
    json!({ "outcome": "hint_only", "hint": hint(), "format": "Matroska" }),
    json!({ "outcome": "insufficient_data", "format": "Matroska" }),
    json!({ "outcome": "unknown", "bytes_needed": 8 }),
    json!({ "outcome": "unknown", "insufficient_reason": { "kind": "TruncatedInput" } }),
    json!({ "outcome": "unknown", "hint": hint() }),
  ] {
    assert!(read(detection.clone()).is_err(), "{detection}");
  }
}

#[test]
fn a_confidence_lies_from_0_to_100_and_candidates_fall_with_it() {
  assert!(read(json!({ "outcome": "identified", "format": "Matroska", "confidence": 101 })).is_err());
  let rising = json!([{ "format": "Mp4", "confidence": 40 }, { "format": "QuickTime", "confidence": 60 }]);
  assert!(read(json!({ "outcome": "ambiguous", "candidates": rising })).is_err());
}

#[test]
fn an_outcome_of_a_newer_version_reads_whatever_members_it_holds() {
  let newer = json!({ "outcome": "identified_by_content", "format": "Matroska", "bytes_needed": 8, "hint": hint() });
  assert!(read(newer).is_ok());
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
