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

//! The thirteenth review's case of `json-output.md` section 4.5: a nested root record is compared with the version
//! of the record that holds it as soon as its own version is read, before its body is decoded, so the version error
//! comes first whatever the order of the parent's members. The review found a type error before the version error of
//! a nested record of version 1.1, and a null error in place of it when the document's `results` came before the
//! document's version. A null or a repeated name before the nested record's own version stays malformed input.

use cc_media_probe::document::Document;
use cc_media_probe::error::ErrorRecord;

/// The version error of a nested record of version 1.1 in a record of version 1.0, which names both versions.
const NESTED_MINOR: &str = "a nested root record has a schema version other than that of the record that holds it: \
                            1.1; the record that holds it has 1.0";
/// The same for a nested record of version 2.0.
const NESTED_MAJOR: &str = "a nested root record has a schema version other than that of the record that holds it: \
                            2.0; the record that holds it has 1.0";
const NULL: &str = "a member holds null, which the form never writes";
const DUPLICATE: &str = "a map names each member once";

/// A document of version 1.0 whose one entry holds the error record `error`, with `results` before or after the
/// document's own version.
fn document(error: &str, results_first: bool) -> String {
  let results = format!(r#""results": [{{"index": 0, "input": "a.mkv", "outcome": "error", "error": {error}}}]"#);
  let generator = r#""generator": {"name": "catalog", "version": "1"}"#;
  let version = r#""schema_version": "1.0""#;
  if results_first {
    format!("{{{results}, {generator}, {version}}}")
  } else {
    format!("{{{version}, {generator}, {results}}}")
  }
}

/// The error of reading the text as a document, in both orders of the document's members.
fn errors_in_both_orders(error: &str) -> [String; 2] {
  [true, false].map(|results_first| {
    serde_json::from_str::<Document>(&document(error, results_first))
      .unwrap_err()
      .to_string()
  })
}

#[test]
fn a_nested_version_comes_before_a_member_that_does_not_decode() {
  // The review's first case: a message of the wrong type, after the nested record's version and before it.
  for error in [
    r#"{"schema_version": "1.1", "kind": "format_not_determined", "message": 42}"#,
    r#"{"message": 42, "kind": "format_not_determined", "schema_version": "1.1"}"#,
  ] {
    for message in errors_in_both_orders(error) {
      assert!(message.starts_with(NESTED_MINOR), "{message}");
    }
  }
}

#[test]
fn a_nested_version_comes_before_a_null_after_it_in_both_orders_of_the_parent() {
  // The review's second case: a nested record of version 2.0 with a null after its version.
  let error = r#"{"schema_version": "2.0", "kind": "format_not_determined", "message": "m", "future": null}"#;
  for message in errors_in_both_orders(error) {
    assert!(message.starts_with(NESTED_MAJOR), "{message}");
  }
}

#[test]
fn a_null_or_a_repeated_name_before_a_nested_version_is_malformed_in_both_orders() {
  let null = r#"{"future": null, "schema_version": "2.0", "kind": "format_not_determined", "message": "m"}"#;
  for message in errors_in_both_orders(null) {
    assert!(message.starts_with(NULL), "{message}");
  }
  let repeated = r#"{"future": 1, "future": 2, "schema_version": "2.0", "kind": "format_not_determined"}"#;
  for message in errors_in_both_orders(repeated) {
    assert!(message.starts_with(DUPLICATE), "{message}");
  }
}

#[test]
fn a_nested_record_of_the_parent_version_reads_in_both_orders() {
  let error = r#"{"kind": "format_not_determined", "schema_version": "1.0", "message": "m"}"#;
  for results_first in [true, false] {
    let read: Document = serde_json::from_str(&document(error, results_first)).unwrap();
    assert_eq!(read.results()[0].error().unwrap().message(), "m");
  }
}

#[test]
fn a_bare_error_record_compares_its_partial_report_before_decoding_it() {
  // The partial report's streams do not decode, and its version differs from the record's.
  let report = r#"{"schema_version": "1.1", "streams": 5}"#;
  let partial = format!(r#""partial_report": {report}"#);
  let members = r#""kind": "format_not_determined", "message": "m""#;
  let version = r#""schema_version": "1.0""#;
  for text in [
    format!("{{{partial}, {members}, {version}}}"),
    format!("{{{version}, {members}, {partial}}}"),
  ] {
    let message = serde_json::from_str::<ErrorRecord>(&text).unwrap_err().to_string();
    assert!(message.starts_with(NESTED_MINOR), "{message}");
  }
}
