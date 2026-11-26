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

//! Each root record reads its version before anything else is decoded, wherever the member stands, through plain
//! Serde on a bare record (plan section 8.8; `json-output.md` section 2). A member that holds null fails at any depth,
//! and so does a map that names a member twice; neither fails before the version error, which names both versions.
//! Only the members before the version are held, so an error in a member after it keeps its place in the input.

use cc_media_probe::document::SchemaVersion;
use cc_media_probe::error::ErrorRecord;
use cc_media_probe::{Detection, MediaReport};
use serde_json::{Value, json};

use crate::fixtures::{detection, error_record, members_of, report, text_of, with_member};

/// The version error of version 2.0, which names both versions.
pub const OTHER_MAJOR: &str = "the version error: the root record has version: 2.0; this reader reads 1.0";
/// The version error of a record without a version.
pub const MISSING: &str = "the version error: the root record has no schema_version";
/// The message of a member that holds null.
pub const NULL: &str = "a member holds null";
/// The message of a member named twice.
pub const DUPLICATE: &str = "a member is named twice";
/// The version error of a nested root record of another version than its holder's.
pub const NESTED: &str = "the version error: a nested root record has version";

/// One of the root records: its members other than the version, as JSON text, two of them that a broken record
/// replaces with a value that does not decode, and how to read it.
struct RootRecord {
  name: &'static str,
  members: Vec<(String, String)>,
  broken: [&'static str; 2],
  read: fn(&str) -> Result<SchemaVersion, serde_json::Error>,
}

/// The three root records that stand alone; the document's version comes with the document.
fn root_records() -> Vec<RootRecord> {
  vec![
    RootRecord {
      name: "MediaReport",
      members: members_of(&report("1.0")),
      broken: ["streams", "usage"],
      read: |text| serde_json::from_str::<MediaReport>(text).map(|record| record.schema_version()),
    },
    RootRecord {
      name: "Detection",
      members: members_of(&detection("1.0")),
      broken: ["outcome", "format"],
      read: |text| serde_json::from_str::<Detection>(text).map(|record| record.schema_version()),
    },
    RootRecord {
      name: "ErrorRecord",
      members: members_of(&error_record("1.0")),
      broken: ["kind", "message"],
      read: |text| serde_json::from_str::<ErrorRecord>(text).map(|record| record.schema_version()),
    },
  ]
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
    for position in [1, broken.len()] {
      let message = failure(&record, &text_of(&broken, Some((position, "2.0"))));
      assert!(message.starts_with(OTHER_MAJOR), "{}: {message}", record.name);
    }
    let message = failure(&record, &text_of(&broken, None));
    assert!(message.starts_with(MISSING), "{}: {message}", record.name);
    let message = failure(&record, &text_of(&broken, Some((1, "1.0"))));
    assert!(
      !message.starts_with(OTHER_MAJOR) && !message.starts_with(MISSING),
      "{}",
      record.name
    );
  }
}

#[test]
fn a_version_that_is_not_two_canonical_numbers_is_the_version_error() {
  let record = &root_records()[0];
  for version in [
    json!("+1.+0"),
    json!("01.0"),
    json!("1.00"),
    json!("1.4294967296"),
    json!("1"),
    json!(17),
  ] {
    let members = with_member(&record.members, "schema_version", &version.to_string(), 0);
    let message = failure(record, &text_of(&members, None));
    assert!(message.starts_with("the version error"), "{version}: {message}");
  }
  let message = failure(
    record,
    &text_of(&with_member(&record.members, "schema_version", "null", 0), None),
  );
  assert!(message.starts_with("the version error"), "{message}");
  let members = with_member(&record.members, "schema_version", "\"1.4294967295\"", 0);
  assert_eq!((record.read)(&text_of(&members, None)).unwrap().minor(), u32::MAX);
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

#[test]
fn a_null_fails_in_a_known_member_of_a_report_at_any_depth() {
  let report = &root_records()[0];
  let cases = [
    ("size", "null"),
    ("streams", "[null]"),
    ("usage", r#"{"bytes_read": 0, "later": null}"#),
    ("format_details", "null"),
  ];
  for (name, value) in cases {
    for (member_position, version_position) in [(0, 1), (1, 0)] {
      let members = with_member(&report.members, name, value, member_position);
      let message = failure(report, &text_of(&members, Some((version_position, "1.0"))));
      assert!(message.starts_with(NULL), "{name}: {message}");
    }
  }
}

#[test]
fn a_member_named_twice_fails_at_any_depth_but_never_before_the_version_error() {
  for record in root_records() {
    let twice = |positions: [usize; 2], value: &str| {
      let mut members = with_member(&record.members, "future", value, positions[0]);
      members.insert(positions[1], ("future".to_owned(), value.to_owned()));
      members
    };
    let length = record.members.len();
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
  let record = &root_records()[2];
  for version in ["1.1", "2.0"] {
    let report_members = members_of(&report(version));
    let partial = text_of(&report_members, Some((report_members.len(), version)));
    let members = with_member(&record.members, "partial_report", &partial, 0);
    let message = failure(record, &text_of(&members, Some((members.len(), "1.0"))));
    assert!(message.starts_with(NESTED), "{version}: {message}");
  }
  let report_members = members_of(&crate::fixtures::partial_report("1.0"));
  let members = with_member(
    &with_member(&record.members, "format", "\"matroska\"", 0),
    "partial_report",
    &text_of(&report_members, Some((9, "1.0"))),
    0,
  );
  let read: ErrorRecord = serde_json::from_str(&text_of(&members, Some((members.len(), "1.0")))).unwrap();
  assert_eq!(
    read.partial_report().map(MediaReport::schema_version),
    Some(SchemaVersion::CURRENT)
  );
}

#[test]
fn a_nested_record_reads_strictly_before_its_version() {
  // The record that holds it already has a version of this major, so a null before the nested version is malformed
  // input, in either order of the holder's members, while a member of the wrong type fails after the version error.
  let record = &root_records()[2];
  let report_members = with_member(&members_of(&report("2.0")), "future", "null", 0);
  let partial = text_of(&report_members, Some((report_members.len(), "2.0")));
  for (member_position, version_position) in [(0, 9), (9, 0)] {
    let members = with_member(&record.members, "partial_report", &partial, member_position);
    let message = failure(record, &text_of(&members, Some((version_position, "1.0"))));
    assert!(message.starts_with(NULL), "{message}");
  }
  let report_members = with_member(&members_of(&report("2.0")), "streams", "5", 0);
  let partial = text_of(&report_members, Some((report_members.len(), "2.0")));
  let members = with_member(&record.members, "partial_report", &partial, 0);
  let message = failure(record, &text_of(&members, Some((members.len(), "1.0"))));
  assert!(message.starts_with(NESTED), "{message}");
}

#[test]
fn an_error_in_a_member_after_the_version_keeps_its_place_in_the_input() {
  let report = &root_records()[0];
  let position = report.members.iter().position(|(name, _)| name == "usage").unwrap();
  let members = with_member(&report.members, "usage", "5", position);
  let failure = serde_json::from_str::<MediaReport>(&text_of(&members, Some((0, "1.0")))).unwrap_err();
  assert_eq!(failure.line(), position + 2);
}

#[test]
fn a_record_written_by_this_version_holds_no_null_and_reads_back() {
  let report: MediaReport = serde_json::from_value(report("1.0")).unwrap();
  let written = serde_json::to_string(&report).unwrap();
  assert!(
    !written.contains("null") && written.starts_with(r#"{"schema_version":"1.0","#),
    "{written}"
  );
  assert_eq!(serde_json::from_str::<MediaReport>(&written).unwrap(), report);
  let record = cc_media_probe::probe_bytes(b"").unwrap_err().to_record();
  let written = serde_json::to_string(&record).unwrap();
  assert!(
    !written.contains("null") && written.starts_with(r#"{"schema_version":"1.0","#),
    "{written}"
  );
  assert_eq!(serde_json::from_str::<ErrorRecord>(&written).unwrap(), record);
  let detection = cc_media_probe::detect_bytes(b"").unwrap();
  let written = serde_json::to_string(&detection).unwrap();
  assert_eq!(serde_json::from_str::<Detection>(&written).unwrap(), detection);
}

#[test]
fn a_bare_record_read_at_a_newer_minor_is_written_with_that_version() {
  let read: MediaReport = serde_json::from_value(report("1.7")).unwrap();
  let written: Value = serde_json::to_value(&read).unwrap();
  assert_eq!(written["schema_version"], json!("1.7"));
}

#[test]
fn the_version_comes_first_only_among_values_that_the_reader_can_represent() {
  // `serde_json` represents `1e400` only with `arbitrary_precision`, which another crate of a build can turn on, so
  // the expected error is the parser's: the version error where it represents the number, and its own otherwise
  // (the review of the third slice's first part).
  let represented = serde_json::from_str::<Value>("1e400").is_ok();
  let message = serde_json::from_str::<ErrorRecord>(r#"{"future": 1e400, "schema_version": "2.0"}"#)
    .unwrap_err()
    .to_string();
  let expected = if represented {
    OTHER_MAJOR
  } else {
    "number out of range"
  };
  assert!(message.starts_with(expected), "{message}");
  let mut repeated = members_of(&detection("1.0"));
  repeated.insert(0, ("future".to_owned(), "1".to_owned()));
  repeated.insert(0, ("future".to_owned(), "2".to_owned()));
  let text = text_of(&repeated, Some((repeated.len(), "1.0")));
  let message = serde_json::from_str::<Detection>(&text).unwrap_err().to_string();
  assert!(message.starts_with(DUPLICATE), "{message}");
  let parsed: Value = serde_json::from_str(&text).unwrap();
  assert!(serde_json::from_value::<Detection>(parsed).is_ok());
}

#[test]
fn a_root_that_is_no_object_fails_before_its_version() {
  for text in [
    "[1, 2]",
    "1.25",
    "\"1.0\"",
    "true",
    r#"{"$serde_json::private::Number": "1.25"}"#,
  ] {
    let message = serde_json::from_str::<MediaReport>(text).unwrap_err().to_string();
    assert!(!message.starts_with("the version error"), "{text}: {message}");
  }
}
