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

//! The fifteenth review's cases of `json-output.md` sections 2, 4.3, 4.4, 5.2 and 9. A detection of a newer outcome
//! and an error record of a newer kind keep every member that they do not read and write them back after their own,
//! as an entry of a newer outcome does, as the maintainer chose: the review read a detection `deferred` of version 1.1
//! with `retry_after: 3`, and its written form had lost `retry_after`. A record of a known outcome or kind still
//! ignores a member that it does not know. A number that serde_json's `arbitrary_precision` feature hands over as its
//! private one-member map reads as the number, as the maintainer chose: the review saw `1.25` come back as
//! `{"$serde_json::private::Number": "1.25"}`. The spike reads that map from text here, which reaches the reader
//! through the same calls as the feature's numbers do.

use cc_media_probe::document::Document;
use cc_media_probe::error::ErrorRecord;
use cc_media_probe::model::Detection;
use cc_media_probe::probe_bytes;
use serde_json::{Value, json};

fn usage() -> Value {
  json!({ "bytes_read": 0, "peak_working_memory": 0 })
}

/// The review's detection: an outcome of version 1.1 with a member that only that version knows.
fn deferred() -> Value {
  json!({ "schema_version": "1.1", "outcome": "deferred", "retry_after": 3, "candidates": [], "usage": usage() })
}

#[test]
fn a_detection_of_a_newer_outcome_writes_back_the_members_it_does_not_read() {
  let read: Detection = serde_json::from_value(deferred()).unwrap();
  let written = serde_json::to_value(&read).unwrap();
  assert_eq!(written["schema_version"], json!("1.1"));
  assert_eq!(written["retry_after"], json!(3));
  // Inside a document of version 1.1, rewritten by this version, the member stays too.
  let entry = json!({ "index": 0, "input": "a", "outcome": "partial", "detection": deferred() });
  let document = json!({ "schema_version": "1.1", "generator": { "name": "g", "version": "1" }, "results": [entry] });
  let read: Document = serde_json::from_value(document).unwrap();
  let written = serde_json::to_value(&read).unwrap();
  assert_eq!(written["results"][0]["detection"]["retry_after"], json!(3));
}

#[test]
fn a_detection_of_a_known_outcome_ignores_a_member_it_does_not_know() {
  let mut unknown = deferred();
  unknown["schema_version"] = json!("1.0");
  unknown["outcome"] = json!("unknown");
  let written = serde_json::to_value(serde_json::from_value::<Detection>(unknown).unwrap()).unwrap();
  assert!(written.get("retry_after").is_none(), "{written}");
}

#[test]
fn an_error_record_of_a_newer_kind_writes_back_the_members_it_does_not_read() {
  let mut error = serde_json::to_value(probe_bytes(b"").unwrap_err().to_record()).unwrap();
  error["schema_version"] = json!("1.1");
  error["kind"] = json!("quota_exceeded");
  error["quota"] = json!({ "name": "daily", "remaining": 0 });
  let written = serde_json::to_value(serde_json::from_value::<ErrorRecord>(error.clone()).unwrap()).unwrap();
  assert_eq!(written["quota"], error["quota"]);
  // An error of a known kind ignores it.
  error["kind"] = json!("unsupported_format");
  let written = serde_json::to_value(serde_json::from_value::<ErrorRecord>(error).unwrap()).unwrap();
  assert!(written.get("quota").is_none(), "{written}");
}

/// A document of one entry of a newer outcome that keeps `ratio`, with the version before or after `results`.
fn skipped(ratio: Value, results_first: bool) -> String {
  let results = json!([{ "index": 0, "input": "a", "outcome": "skipped", "ratio": ratio }]).to_string();
  let rest = r#""schema_version": "1.0", "generator": {"name": "g", "version": "1"}"#;
  if results_first {
    format!(r#"{{"results": {results}, {rest}}}"#)
  } else {
    format!(r#"{{{rest}, "results": {results}}}"#)
  }
}

#[test]
fn a_number_in_the_private_map_of_arbitrary_precision_reads_as_the_number() {
  // The review's case, `1.25` in an entry of a newer outcome, held before the version or kept after it.
  for results_first in [false, true] {
    for (number, expected) in [
      ("1.25", json!(1.25)),
      ("18446744073709551616", json!(18_446_744_073_709_551_616.0_f64)),
      ("-0", json!(-0.0)),
    ] {
      let private = json!({ "$serde_json::private::Number": number });
      let read: Document = serde_json::from_str(&skipped(private, results_first)).unwrap();
      let written = serde_json::to_value(&read).unwrap();
      assert_eq!(written["results"][0]["ratio"], expected, "{number}");
    }
  }
}

#[test]
fn a_number_beyond_every_float_in_the_private_map_is_refused() {
  // `1e400`, which the feature keeps as text, is no finite number, so it fails as a value that JSON does not carry.
  let private = json!({ "$serde_json::private::Number": "1e400" });
  for results_first in [false, true] {
    let error = serde_json::from_str::<Document>(&skipped(private.clone(), results_first)).unwrap_err();
    assert!(error.to_string().starts_with(NOT_JSON), "{error}");
  }
}

#[test]
fn a_map_of_that_name_that_holds_no_number_stays_a_map() {
  let private = json!({ "$serde_json::private::Number": "not a number" });
  let read: Document = serde_json::from_str(&skipped(private.clone(), false)).unwrap();
  assert_eq!(serde_json::to_value(&read).unwrap()["results"][0]["ratio"], private);
}

const NOT_JSON: &str = "a value is one that JSON carries as read";

/// A detection of 1.1 whose members that this version does not read stand before its outcome and after it.
fn around_the_outcome(outcome: &str) -> String {
  format!(
    r#"{{"schema_version": "1.1", "before": [1, 2], "outcome": "{outcome}", "candidates": [], "after": {{"x": 1}}, "usage": {{"bytes_read": 0, "peak_working_memory": 0}}}}"#
  )
}

#[test]
fn a_record_holds_what_it_does_not_read_only_until_its_outcome_is_known() {
  // As the maintainer chose at the eighteenth review, where Serde's `flatten` held every such member to the end: a
  // newer outcome keeps them, before its outcome and after it, in the order written, and a known one passes them by.
  let newer: Detection = serde_json::from_str(&around_the_outcome("deferred")).unwrap();
  let written = serde_json::to_string(&newer).unwrap();
  assert!(written.ends_with(r#""before":[1,2],"after":{"x":1}}"#), "{written}");
  let known = serde_json::to_value(serde_json::from_str::<Detection>(&around_the_outcome("unknown")).unwrap()).unwrap();
  assert!(known.get("before").is_none() && known.get("after").is_none(), "{known}");
  // A member passed by is read strictly all the same.
  let null = around_the_outcome("unknown").replace(r#"{"x": 1}"#, r#"{"x": null}"#);
  assert!(serde_json::from_str::<Detection>(&null).is_err());
  // An entry decides by its outcome, and an error record by its kind.
  let document = |entry: &str| {
    let text =
      format!(r#"{{"schema_version": "1.1", "generator": {{"name": "g", "version": "1"}}, "results": [{entry}]}}"#);
    serde_json::to_string(&serde_json::from_str::<Document>(&text).unwrap()).unwrap()
  };
  let written = document(r#"{"index": 0, "input": "a", "before": 1, "outcome": "skipped", "after": 2}"#);
  assert!(
    written.contains(r#""outcome":"skipped","before":1,"after":2}"#),
    "{written}"
  );
  let detection = serde_json::to_string(&deferred()).unwrap();
  let written = document(&format!(
    r#"{{"index": 0, "input": "a", "before": 1, "outcome": "partial", "detection": {detection}, "after": 2}}"#
  ));
  assert!(
    !written.contains(r#""before""#) && !written.contains(r#""after""#),
    "{written}"
  );
  let mut error = serde_json::to_value(probe_bytes(b"").unwrap_err().to_record()).unwrap();
  error["quota"] = json!(1);
  for (kind, kept) in [("quota_exceeded", true), ("unsupported_format", false)] {
    error["kind"] = json!(kind);
    let written = serde_json::to_value(serde_json::from_value::<ErrorRecord>(error.clone()).unwrap()).unwrap();
    assert_eq!(written.get("quota").is_some(), kept, "{kind}");
  }
}
