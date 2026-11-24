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

//! The sixteenth review's cases of `json-output.md` section 2. The private map of serde_json's `arbitrary_precision`
//! feature, one member named `$serde_json::private::Number` that holds a number's text, cannot be told apart from an
//! object of the same shape in the input, so the form gives it one rule in every order and every build, as the
//! maintainer chose: the map is the number it spells only where the reader does not know the type of the value, in
//! a catch-all, a kept member and a member that no type reads, and a member of a known type takes it as the object it
//! is. The review read such an object as a candidate's confidence of 60 before the version, saw it refused after the
//! version, and, with the feature on, saw `1e400` in a member that no type reads refused before the version and read
//! after it. The spike's build has no such feature, so the map is written here as text, which reaches the reader
//! through the same calls as the feature's numbers do. The review also wrote `1.2234567890123456789` back as two
//! different doubles in the two builds: through the map, the number is the correctly rounded nearest double.
//!
//! The seventeenth review read a disposition of `1.25` with the feature on, refused with the version first and read
//! as an empty disposition with the version last; as the maintainer chose, no object of a known type has a member of
//! the map's name. It also read `1e400` before a nested record's version, which gave the nested version error in
//! either order of the parent's members, and asked to state what becomes of a JSON integer beyond 64 bits.

use cc_media_probe::MediaReport;
use cc_media_probe::document::Document;
use cc_media_probe::model::{Chapter, Detection, Disposition, Tag, TagValue};
use serde_json::{Value, json};

const PRIVATE_NAME: &str = "$serde_json::private::Number";
const NOT_JSON: &str = "a value is one that JSON carries as read";

fn private(text: &str) -> Value {
  json!({ PRIVATE_NAME: text })
}

/// An ambiguous detection whose first candidate has that confidence, with the version before or after the candidates.
fn ambiguous(confidence: Value, candidates_first: bool) -> String {
  let candidates =
    json!([{ "format": "Mp4", "confidence": confidence }, { "format": "QuickTime", "confidence": 60 }]).to_string();
  let rest = r#""schema_version": "1.0", "outcome": "ambiguous", "usage": {"bytes_read": 0, "peak_working_memory": 0}"#;
  if candidates_first {
    format!(r#"{{"candidates": {candidates}, {rest}}}"#)
  } else {
    format!(r#"{{{rest}, "candidates": {candidates}}}"#)
  }
}

#[test]
fn a_member_of_a_known_type_takes_the_map_as_an_object_in_either_order() {
  // The review's case: before the version, the map read as a confidence of 60; after it, it was refused.
  for candidates_first in [false, true] {
    let error = serde_json::from_str::<Detection>(&ambiguous(private("60"), candidates_first)).unwrap_err();
    assert!(
      error.to_string().starts_with("invalid type: map"),
      "{candidates_first}: {error}"
    );
    assert!(serde_json::from_str::<Detection>(&ambiguous(json!(60), candidates_first)).is_ok());
  }
}

/// A document whose one entry, of a newer outcome, keeps `ratio`, and that also holds `future`, a member that no
/// type reads, before the version or after it.
fn document(ratio: Value, future: Value, version_first: bool) -> String {
  let results = json!([{ "index": 0, "input": "a", "outcome": "skipped", "ratio": ratio }]).to_string();
  let rest = format!(r#""schema_version": "1.0", "generator": {{"name": "g", "version": "1"}}, "results": {results}"#);
  let future = format!(r#""future": {future}"#);
  match version_first {
    true => format!("{{{rest}, {future}}}"),
    false => format!("{{{future}, {rest}}}"),
  }
}

fn written_ratio(text: &str) -> Value {
  let read: Document = serde_json::from_str(text).unwrap();
  serde_json::to_value(&read).unwrap()["results"][0]["ratio"].clone()
}

#[test]
fn a_kept_member_takes_the_map_as_its_number_in_either_order() {
  // The restriction that the form states: an object of that shape in a member whose type is not known is a number.
  for version_first in [false, true] {
    assert_eq!(
      written_ratio(&document(private("60"), json!(1), version_first)),
      json!(60)
    );
  }
}

#[test]
fn a_member_that_no_type_reads_checks_the_number_in_either_order() {
  // The review's case, with the feature on: `1e400` was refused before the version and read after it.
  for version_first in [false, true] {
    let error = serde_json::from_str::<Document>(&document(json!(1), private("1e400"), version_first)).unwrap_err();
    assert!(error.to_string().starts_with(NOT_JSON), "{version_first}: {error}");
    assert!(serde_json::from_str::<Document>(&document(json!(1), private("1.25"), version_first)).is_ok());
    // Deeper inside such a member too.
    let deep = json!({ "values": [1, private("-1e400")] });
    assert!(serde_json::from_str::<Document>(&document(json!(1), deep, version_first)).is_err());
  }
}

#[test]
fn a_map_of_that_name_beside_another_member_is_an_ordinary_map() {
  let two = json!({ PRIVATE_NAME: "1e400", "other": 1 });
  for version_first in [false, true] {
    assert!(serde_json::from_str::<Document>(&document(json!(1), two.clone(), version_first)).is_ok());
    assert_eq!(written_ratio(&document(two.clone(), json!(1), version_first)), two);
  }
}

#[test]
fn a_decimal_that_needs_rounding_is_the_nearest_double() {
  // The review's decimal: the default parser of serde_json wrote 1.2234567890123456, a build with the feature
  // 1.2234567890123458, which is the nearest double, as serde_json's `float_roundtrip` feature also reads it.
  let nearest: f64 = "1.2234567890123456789".parse().unwrap();
  for version_first in [false, true] {
    let ratio = written_ratio(&document(private("1.2234567890123456789"), json!(1), version_first));
    assert_eq!(ratio.as_f64().map(f64::to_bits), Some(nearest.to_bits()), "{ratio}");
    assert_eq!(ratio.to_string(), "1.2234567890123458");
  }
}

const RESERVED: &str = "no object of a known type has a member named `$serde_json::private::Number`";
const NESTED: &str = "a nested root record has a schema version other than that of the record that holds it";

/// A report whose one stream has that disposition, written as raw text with the version first or last.
fn report_with_disposition(disposition: &str, version_first: bool) -> String {
  let stream = format!(
    r#"{{"index": 0, "kind": "Video", "codec": {{"candidates": [], "has_every_candidate": true, "raw_identifiers": []}}, "header_status": {{"state": "Parsed"}}, "titles": [], "disposition": {disposition}}}"#
  );
  let completeness = r#"{"detection": "Complete", "discovery": "Complete", "headers": "Complete", "tags": "Complete", "chapters": "Complete", "attachments": "Complete", "reasons": [], "record": {"exhausted": [], "first_stopped": [], "first_reasons": [], "omitted_reasons": "0", "omitted_references": "0"}}"#;
  let body = format!(
    r#""format": "Matroska", "streams": [{stream}], "chapter_editions": [], "tag_sets": [], "attachments": [], "diagnostics": [], "completeness": {completeness}, "usage": {{"bytes_read": 0, "peak_working_memory": 0}}, "sources": []"#
  );
  match version_first {
    true => format!(r#"{{"schema_version": "1.0", {body}}}"#),
    false => format!(r#"{{{body}, "schema_version": "1.0"}}"#),
  }
}

#[test]
fn a_record_refuses_a_member_of_the_private_name_in_either_order() {
  // The seventeenth review's case: with the feature on, `"disposition": 1.25` was refused with the version first,
  // and read as an empty disposition with the version last, where the held number reached `Disposition`, whose
  // members are all optional, as the map. The map is written here as raw text, which reaches the reader through the
  // same calls as the feature's number does, so the literal object fails as well, as the maintainer chose.
  for version_first in [true, false] {
    for disposition in [
      r#"{"$serde_json::private::Number": "1.25"}"#,
      r#"{"default": true, "$serde_json::private::Number": "1.25"}"#,
    ] {
      let error =
        serde_json::from_str::<MediaReport>(&report_with_disposition(disposition, version_first)).unwrap_err();
      assert!(error.to_string().starts_with(RESERVED), "{version_first}: {error}");
    }
    assert!(serde_json::from_str::<MediaReport>(&report_with_disposition("{}", version_first)).is_ok());
    assert!(serde_json::from_str::<MediaReport>(&report_with_disposition("1.25", version_first)).is_err());
  }
  // Read alone too.
  let error = serde_json::from_str::<Disposition>(r#"{"$serde_json::private::Number": "1.25"}"#).unwrap_err();
  assert!(error.to_string().starts_with(RESERVED), "{error}");
}

/// A document of 1.0 whose one entry holds a detection with `future` before its version of 2.0, with the document's
/// version first or its results first.
fn nested_detection(future: Value, version_first: bool) -> String {
  let detection = json!({ "future": future, "schema_version": "2.0" }).to_string();
  let results = format!(r#""results": [{{"index": 0, "input": "a", "outcome": "ok", "detection": {detection}}}]"#);
  let head = r#""schema_version": "1.0", "generator": {"name": "g", "version": "1"}"#;
  match version_first {
    true => format!("{{{head}, {results}}}"),
    false => format!("{{{results}, {head}}}"),
  }
}

#[test]
fn a_nested_record_refuses_a_number_that_is_not_finite_before_its_version_in_either_order() {
  // The seventeenth review's case, with the feature on: `1e400` before the nested version gave the nested version
  // error in both orders, where the record that holds it reads strictly and the value is malformed input.
  for version_first in [true, false] {
    let error = serde_json::from_str::<Document>(&nested_detection(private("1e400"), version_first)).unwrap_err();
    assert!(error.to_string().starts_with(NOT_JSON), "{version_first}: {error}");
    let error = serde_json::from_str::<Document>(&nested_detection(private("1.25"), version_first)).unwrap_err();
    assert!(error.to_string().starts_with(NESTED), "{version_first}: {error}");
  }
}

#[test]
fn an_integer_beyond_64_bits_where_no_type_is_known_is_the_nearest_double() {
  // The seventeenth review's case: JSON `18446744073709551617` in a member of a future entry came back as the
  // nearest double in either build, without the feature from serde_json's own parser and with it through the map.
  for version_first in [true, false] {
    for ratio in [json!(18446744073709551617.0_f64), private("18446744073709551617")] {
      let text = document(ratio, json!(1), version_first).replace("1.8446744073709552e19", "18446744073709551617");
      assert_eq!(written_ratio(&text).to_string(), "1.8446744073709552e+19");
    }
  }
  // Where a type is known, the member refuses it: an entry's index is an integer within 64 bits.
  let text = document(json!(1), json!(1), true).replacen(r#""index":0"#, r#""index":18446744073709551617"#, 1);
  assert!(serde_json::from_str::<Document>(&text).is_err());
}

/// A report without streams whose one tag set holds these tags and whose one edition holds these chapters.
fn report_with_trees(tags: Vec<Value>, chapters: Vec<Value>) -> Value {
  let mut report: Value = serde_json::from_str(&report_with_disposition("{}", true)).unwrap();
  report["streams"] = json!([]);
  report["chapter_editions"] = json!([{ "chapters": chapters }]);
  report["tag_sets"] = json!([{
    "stream_targets": [],
    "edition_targets": [],
    "chapter_targets": [],
    "attachment_targets": [],
    "target_scope": "WholeFile",
    "tags": tags,
    "superseded": false,
  }]);
  report
}

fn tag_of(value: Value) -> Value {
  json!({ "name": { "as_written": "TAG" }, "languages": [], "value": value })
}

const OBJECT: &str = "invalid type: map";

#[test]
fn a_member_of_a_known_integer_type_refuses_the_map_wherever_it_is_read() {
  // The eighteenth review's cases, in both builds of serde_json: a tag's integer of 60, a chapter's parent and a
  // binary value's size read from the map as numbers, since the members of a kind and of a tree's node were kept as
  // numbers before their types were known, and the tag value was written back with `"integer": 60`.
  let integer = json!({ "kind": "integer", "integer": private("60") });
  let error = serde_json::from_value::<TagValue>(integer.clone()).unwrap_err();
  assert!(error.to_string().starts_with(OBJECT), "{error}");
  let error = serde_json::from_value::<MediaReport>(report_with_trees(vec![tag_of(integer)], Vec::new())).unwrap_err();
  assert!(error.to_string().starts_with(OBJECT), "{error}");
  let chapters = vec![
    json!({ "string_identifier": "a", "titles": [] }),
    json!({ "string_identifier": "b", "titles": [], "parent": private("0") }),
  ];
  let error = serde_json::from_value::<Chapter>(Value::Array(chapters.clone())).unwrap_err();
  assert!(error.to_string().starts_with(OBJECT), "{error}");
  let error = serde_json::from_value::<MediaReport>(report_with_trees(Vec::new(), chapters)).unwrap_err();
  assert!(error.to_string().starts_with(OBJECT), "{error}");
  let binary = json!({ "kind": "binary", "binary": { "size": private("4"), "data": "89504e47" } });
  let error = serde_json::from_value::<TagValue>(binary.clone()).unwrap_err();
  assert!(error.to_string().starts_with(OBJECT), "{error}");
  let error = serde_json::from_value::<Tag>(json!([tag_of(binary)])).unwrap_err();
  assert!(error.to_string().starts_with(OBJECT), "{error}");
  // The same members as numbers read.
  let numbers = json!({ "kind": "binary", "binary": { "size": 4, "data": "89504e47" } });
  assert!(serde_json::from_value::<Tag>(json!([tag_of(numbers)])).is_ok());
  assert!(serde_json::from_value::<TagValue>(json!({ "kind": "integer", "integer": 60 })).is_ok());
}

#[test]
fn a_kind_refuses_the_private_name_beside_its_members_alone_and_in_a_report() {
  // The eighteenth review's case: read alone, the tag value dropped the member and wrote `{"kind": "text", "text":
  // "Title"}` back. A report's tag kept its value's members as written too, and dropped it the same way.
  let beside = json!({ "kind": "text", "text": "Title", PRIVATE_NAME: "x" });
  let error = serde_json::from_value::<TagValue>(beside.clone()).unwrap_err();
  assert!(error.to_string().starts_with(RESERVED), "{error}");
  let error = serde_json::from_value::<MediaReport>(report_with_trees(vec![tag_of(beside)], Vec::new())).unwrap_err();
  assert!(error.to_string().starts_with(RESERVED), "{error}");
  // A kind of a newer version keeps the map as its number, where no type is known.
  let newer: TagValue = serde_json::from_value(json!({ "kind": "rating", "rating": private("1.25") })).unwrap();
  assert_eq!(
    serde_json::to_value(&newer).unwrap(),
    json!({ "kind": "rating", "rating": 1.25 })
  );
}

#[test]
fn a_member_of_the_private_name_before_a_nested_version_comes_after_the_version_error_in_either_order() {
  // The eighteenth review's case: the nested detection's member of that name gave the name's error in both orders of
  // the document's members, where the table of `json-output.md` section 2 puts the version error first. The member
  // has a name given once and a value that JSON carries, and only a type that is known refuses the name.
  for version_first in [true, false] {
    let text = nested_detection(json!("hi"), version_first).replace(r#""future""#, &format!(r#""{PRIVATE_NAME}""#));
    let error = serde_json::from_str::<Document>(&text).unwrap_err();
    assert!(error.to_string().starts_with(NESTED), "{version_first}: {error}");
  }
}
