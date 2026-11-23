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

//! Task 5's neutral form, plan section 8.8: each of the four root records reads its version before anything else is
//! decoded, wherever the member stands, through plain Serde on bare records too. A member that holds null fails at
//! any depth, and so does a map that names a member twice; neither fails before the version error, which names both
//! versions. As the maintainer chose on 2026-11-23, only the members before the version are held, so an error in a
//! member after it keeps its place in the input.

use cc_media_probe::document::{Document, SchemaVersion};
use cc_media_probe::error::ErrorRecord;
use cc_media_probe::model::Detection;
use cc_media_probe::{MediaReport, probe_bytes};
use serde_json::{Value, json};

/// The version error of version 2.0, which names both versions.
const OTHER_MAJOR: &str =
  "a root record has a schema major version that this version does not read: 2.0; this version reads 1.0";
const MISSING: &str = "a root record has no schema_version";
const NULL: &str = "a member holds null, which the form never writes";
const DUPLICATE: &str = "a map names each member once";
/// The version error of a nested root record, which compares its version with that of the record that holds it.
const NESTED: &str = "a nested root record has a schema version other than that of the record that holds it";

/// One of the four root records: its members other than the version, as JSON text, two of them that a broken
/// record replaces with a value that does not decode, and how to read it.
struct RootRecord {
  name: &'static str,
  members: Vec<(String, String)>,
  broken: [&'static str; 2],
  read: fn(&str) -> Result<SchemaVersion, serde_json::Error>,
}

/// The members of a record written by this version, other than its version, as JSON text.
fn members_of(record: Value) -> Vec<(String, String)> {
  let Value::Object(members) = record else { unreachable!() };
  members
    .into_iter()
    .filter(|(name, _)| name != "schema_version")
    .map(|(name, value)| (name, value.to_string()))
    .collect()
}

fn report_value(version: &str) -> Value {
  json!({
    "schema_version": version,
    "format": "Matroska",
    "streams": [],
    "chapter_editions": [],
    "tag_sets": [],
    "attachments": [],
    "diagnostics": [],
    "completeness": {
      "detection": "Complete",
      "discovery": "Complete",
      "headers": "Complete",
      "tags": "Complete",
      "chapters": "Complete",
      "attachments": "Complete",
      "reasons": [],
      "record": { "exhausted": [], "first_stopped": [] },
    },
    "usage": { "bytes_read": 0, "peak_working_memory": 0 },
    "sources": [],
  })
}

fn root_records() -> Vec<RootRecord> {
  let error = serde_json::to_value(probe_bytes(b"").unwrap_err().to_record()).unwrap();
  vec![
    RootRecord {
      name: "Document",
      members: members_of(json!({ "generator": { "name": "catalog", "version": "1" }, "results": [] })),
      broken: ["generator", "results"],
      read: |text| serde_json::from_str::<Document>(text).map(|record| record.schema_version()),
    },
    RootRecord {
      name: "MediaReport",
      members: members_of(report_value("1.0")),
      broken: ["streams", "usage"],
      read: |text| serde_json::from_str::<MediaReport>(text).map(|record| record.schema_version()),
    },
    RootRecord {
      name: "Detection",
      members: members_of(json!({ "outcome": "identified", "format": "Matroska" })),
      broken: ["outcome", "format"],
      read: |text| serde_json::from_str::<Detection>(text).map(|record| record.schema_version()),
    },
    RootRecord {
      name: "ErrorRecord",
      members: members_of(error),
      broken: ["kind", "message"],
      read: |text| serde_json::from_str::<ErrorRecord>(text).map(|record| record.schema_version()),
    },
  ]
}

/// The record's text, one member a line, with the version at that position when one is given.
fn text_of(members: &[(String, String)], version: Option<(usize, &str)>) -> String {
  let mut lines: Vec<String> = members
    .iter()
    .map(|(name, value)| format!("{}: {value}", json!(name)))
    .collect();
  if let Some((position, version)) = version {
    lines.insert(position, format!("\"schema_version\": {}", json!(version)));
  }
  format!("{{{}}}", lines.join(",\n"))
}

/// The members with one replaced or, when it is absent, added at that position.
fn with_member(members: &[(String, String)], name: &str, value: &str, position: usize) -> Vec<(String, String)> {
  let mut members: Vec<(String, String)> = members.iter().filter(|(kept, _)| kept != name).cloned().collect();
  members.insert(position.min(members.len()), (name.to_owned(), value.to_owned()));
  members
}

fn failure(record: &RootRecord, text: &str) -> String {
  match (record.read)(text) {
    Ok(_) => panic!("{} read: {text}", record.name),
    Err(failure) => failure.to_string(),
  }
}

#[test]
fn each_root_record_reads_its_version_wherever_it_stands() {
  for record in root_records() {
    for position in 0..=record.members.len() {
      let version = (record.read)(&text_of(&record.members, Some((position, "1.1")))).unwrap();
      assert_eq!(
        (version.major(), version.minor()),
        (1, 1),
        "{} at {position}",
        record.name
      );
    }
  }
}

#[test]
fn the_version_error_comes_before_any_member_that_would_not_decode() {
  for record in root_records() {
    let [first, second] = record.broken;
    let broken = with_member(
      &with_member(&record.members, first, "5", 0),
      second,
      "5",
      record.members.len(),
    );
    // One broken member before the version and one after it, then both before it.
    for position in [1, broken.len()] {
      let message = failure(&record, &text_of(&broken, Some((position, "2.0"))));
      assert!(message.starts_with(OTHER_MAJOR), "{}: {message}", record.name);
    }
    let message = failure(&record, &text_of(&broken, None));
    assert!(message.starts_with(MISSING), "{}: {message}", record.name);
    // The same members with the right major fail on the member that does not decode.
    let message = failure(&record, &text_of(&broken, Some((1, "1.0"))));
    assert!(
      !message.starts_with(OTHER_MAJOR) && !message.starts_with(MISSING),
      "{}",
      record.name
    );
  }
}

#[test]
fn a_null_fails_in_every_member_but_never_before_the_version_error() {
  let nulls = ["null", r#"{"inner": [1, null]}"#, r#"[{"deeper": {"deepest": null}}]"#];
  for record in root_records() {
    for value in nulls {
      for (member_position, version_position) in [(0, 1), (1, 0)] {
        let members = with_member(&record.members, "future", value, member_position);
        let message = failure(&record, &text_of(&members, Some((version_position, "1.0"))));
        assert!(message.starts_with(NULL), "{} {value}: {message}", record.name);
        // A newer major can write what this one refuses: the version error comes first.
        let message = failure(&record, &text_of(&members, Some((members.len(), "2.0"))));
        assert!(message.starts_with(OTHER_MAJOR), "{} {value}: {message}", record.name);
      }
      let members = with_member(&record.members, "future", &value.replace("null", "1"), 0);
      assert!(
        (record.read)(&text_of(&members, Some((1, "1.0")))).is_ok(),
        "{}",
        record.name
      );
    }
  }
}

/// A null in a known member fails: where a struct is expected the reader refuses it as a value of the wrong type
/// itself, as serde_json does, and elsewhere `Strict` refuses it.
#[test]
fn a_null_fails_in_a_known_member_of_a_report_at_any_depth() {
  let report = &root_records()[1];
  let cases = [
    ("size", "null"),
    ("streams", "[null]"),
    ("usage", r#"{"bytes_read": 0, "peak_working_memory": 0, "later": null}"#),
    ("format_details", "null"),
  ];
  for (name, value) in cases {
    for (member_position, version_position) in [(0, 1), (1, 0)] {
      let members = with_member(&report.members, name, value, member_position);
      let message = failure(report, &text_of(&members, Some((version_position, "1.0"))));
      assert!(
        message.starts_with(NULL) || message.starts_with("invalid type: null"),
        "{name}: {message}"
      );
    }
  }
}

#[test]
fn a_member_named_twice_fails_at_any_depth_but_never_before_the_version_error() {
  for record in root_records() {
    let twice = |positions: [usize; 2], value: &str| {
      let once = with_member(&record.members, "future", value, positions[0]);
      let mut members = once.clone();
      members.insert(positions[1], ("future".to_owned(), value.to_owned()));
      members
    };
    let length = record.members.len();
    // Both before the version, one on each side, and both after it.
    for (positions, version_position) in [([0, 1], 2), ([0, 2], 1), ([1, 2], 0)] {
      let message = failure(
        &record,
        &text_of(&twice(positions, "1"), Some((version_position, "1.0"))),
      );
      assert!(message.starts_with(DUPLICATE), "{}: {message}", record.name);
      let message = failure(&record, &text_of(&twice(positions, "1"), Some((length + 2, "2.0"))));
      assert!(message.starts_with(OTHER_MAJOR), "{}: {message}", record.name);
    }
    for (member_position, version_position) in [(0, 1), (1, 0)] {
      let members = with_member(
        &record.members,
        "future",
        r#"{"inner": 1, "inner": 2}"#,
        member_position,
      );
      let message = failure(&record, &text_of(&members, Some((version_position, "1.0"))));
      assert!(message.starts_with(DUPLICATE), "{}: {message}", record.name);
    }
    let mut members = record.members.clone();
    members.push(("schema_version".to_owned(), "\"1.0\"".to_owned()));
    let message = failure(&record, &text_of(&members, Some((0, "1.0"))));
    assert!(message.starts_with(DUPLICATE), "{}: {message}", record.name);
  }
}

#[test]
fn a_bare_error_record_reads_the_version_of_its_partial_report_first_too() {
  let record = &root_records()[3];
  for version in ["1.1", "2.0"] {
    let mut report = report_value(version);
    // The partial report's version last, and the partial report before the record's version.
    let report_members = members_of(report.clone());
    report = Value::String(text_of(&report_members, Some((report_members.len(), version))));
    let Value::String(report_text) = report else {
      unreachable!()
    };
    let members = with_member(&record.members, "partial_report", &report_text, 0);
    let message = failure(record, &text_of(&members, Some((members.len(), "1.0"))));
    assert!(message.starts_with(NESTED), "{version}: {message}");
  }
  let report_members = members_of(report_value("1.0"));
  let members = with_member(
    &record.members,
    "partial_report",
    &text_of(&report_members, Some((9, "1.0"))),
    0,
  );
  assert!((record.read)(&text_of(&members, Some((members.len(), "1.0")))).is_ok());
}

#[test]
fn a_document_reads_its_nested_records_against_the_version_it_finds_last() {
  let document = &root_records()[0];
  let report = text_of(&members_of(report_value("1.1")), Some((0, "1.1")));
  let entry = format!(r#"[{{"index": 0, "input": "movie.mkv", "outcome": "ok", "report": {report}}}]"#);
  let members = with_member(&document.members, "results", &entry, 0);
  assert_eq!(
    (document.read)(&text_of(&members, Some((members.len(), "1.1"))))
      .unwrap()
      .minor(),
    1
  );
  let message = failure(document, &text_of(&members, Some((members.len(), "1.0"))));
  assert!(message.starts_with(NESTED), "{message}");
}

#[test]
fn an_error_in_a_member_after_the_version_keeps_its_place_in_the_input() {
  let report = &root_records()[1];
  let position = report.members.iter().position(|(name, _)| name == "usage").unwrap();
  let members = with_member(&report.members, "usage", "5", position);
  // The version first, then one member a line: the member at `position` is on line `position + 2`.
  let failure = serde_json::from_str::<MediaReport>(&text_of(&members, Some((0, "1.0")))).unwrap_err();
  assert_eq!(failure.line(), position + 2);
}

#[test]
fn a_record_written_by_this_version_holds_no_null_and_reads_back() {
  let report: MediaReport = serde_json::from_value(report_value("1.0")).unwrap();
  let written = serde_json::to_string(&report).unwrap();
  assert!(!written.contains("null"), "{written}");
  assert_eq!(serde_json::from_str::<MediaReport>(&written).unwrap(), report);
  let record = probe_bytes(b"").unwrap_err().to_record();
  let written = serde_json::to_string(&record).unwrap();
  assert!(!written.contains("null"), "{written}");
  assert_eq!(serde_json::from_str::<ErrorRecord>(&written).unwrap(), record);
}

#[test]
fn the_version_comes_first_only_among_values_that_the_reader_can_represent() {
  // The thirteenth review's case: serde_json cannot hold 1e400, so its own error comes before the version error.
  let message = serde_json::from_str::<ErrorRecord>(r#"{"future": 1e400, "schema_version": "2.0"}"#)
    .unwrap_err()
    .to_string();
  assert!(message.starts_with("number out of range"), "{message}");
  // A value that serde_json has already parsed keeps one member for each name, so a repeated name is gone before
  // the record is read: only the text shows it.
  let repeated =
    r#"{"future": 1, "future": 2, "schema_version": "1.0", "outcome": "identified", "format": "Matroska"}"#;
  let message = serde_json::from_str::<Detection>(repeated).unwrap_err().to_string();
  assert!(message.starts_with(DUPLICATE), "{message}");
  let parsed: Value = serde_json::from_str(repeated).unwrap();
  assert!(serde_json::from_value::<Detection>(parsed).is_ok());
}
