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

//! The private map in which `serde_json`'s `arbitrary_precision` feature hands over a number that is not an integer
//! within 64 bits: one member, `$serde_json::private::Number`, holding the number's text (`json-output.md` section
//! 2). The tests write the map as literal JSON, which reaches the reader through the same calls as the feature's
//! numbers do. Where no type is known, in a catch-all, a kept member and a member that no type reads, the map is the
//! number it spells, the nearest double correctly rounded, refused when it is not finite. Where a type is known, the
//! map is the object it is, so a member that requires a number refuses it, and an object of a known type refuses a
//! member of that name, in either order of the members.

use cc_media_probe::error::ErrorRecord;
use cc_media_probe::model::{Disposition, TagValue};
use cc_media_probe::{Detection, MediaReport};
use serde_json::{Value, json};

use crate::fixtures::{detection, error_record, members_of, report, stream, text_of, usage, with, with_member};
use crate::version_first::NESTED;

const PRIVATE_NAME: &str = "$serde_json::private::Number";
const NOT_JSON: &str = "a value that JSON does not carry";
const RESERVED: &str = "an object of a known type has a member named like serde_json's private number";

fn private(text: &str) -> Value {
  json!({ PRIVATE_NAME: text })
}

/// A detection's text with that member, before its version or after it.
fn detection_text(base: &Value, name: &str, value: &Value, version_first: bool) -> String {
  let members = with_member(&members_of(base), name, &value.to_string(), 0);
  let position = if version_first { 0 } else { members.len() };
  text_of(&members, Some((position, "1.0")))
}

#[test]
fn a_member_of_a_known_type_takes_the_map_as_an_object_in_either_order() {
  for version_first in [false, true] {
    let text = detection_text(&detection("1.0"), "confidence", &private("60"), version_first);
    let error = serde_json::from_str::<Detection>(&text).unwrap_err().to_string();
    assert!(error.starts_with("invalid type: map"), "{version_first}: {error}");
    let text = detection_text(&detection("1.0"), "confidence", &json!(60), version_first);
    assert!(serde_json::from_str::<Detection>(&text).is_ok());
  }
}

/// A detection of a newer outcome, which keeps the members that it does not read.
fn newer_detection() -> Value {
  json!({ "schema_version": "1.0", "outcome": "deferred", "candidates": [], "usage": usage() })
}

fn kept_ratio(ratio: &Value, version_first: bool) -> Result<Value, serde_json::Error> {
  let text = detection_text(&newer_detection(), "ratio", ratio, version_first);
  let read: Detection = serde_json::from_str(&text)?;
  Ok(serde_json::to_value(&read).unwrap()["ratio"].clone())
}

#[test]
fn a_kept_member_takes_the_map_as_its_number_in_either_order() {
  for version_first in [false, true] {
    assert_eq!(kept_ratio(&private("60"), version_first).unwrap(), json!(60));
    assert_eq!(kept_ratio(&private("-60"), version_first).unwrap(), json!(-60));
    assert_eq!(kept_ratio(&private("1.25"), version_first).unwrap(), json!(1.25));
    assert_eq!(
      kept_ratio(&json!([private("2.5")]), version_first).unwrap(),
      json!([2.5])
    );
    let error = kept_ratio(&private("1e400"), version_first).unwrap_err().to_string();
    assert!(error.starts_with(NOT_JSON), "{error}");
  }
}

#[test]
fn a_member_that_no_type_reads_checks_the_number_in_either_order() {
  for version_first in [false, true] {
    for future in [private("1e400"), json!({ "values": [1, private("-1e400")] })] {
      let text = detection_text(&detection("1.0"), "future", &future, version_first);
      let error = serde_json::from_str::<Detection>(&text).unwrap_err().to_string();
      assert!(error.starts_with(NOT_JSON), "{version_first}: {error}");
    }
    let text = detection_text(&detection("1.0"), "future", &private("1.25"), version_first);
    assert!(serde_json::from_str::<Detection>(&text).is_ok());
  }
}

#[test]
fn a_map_of_that_name_beside_another_member_or_without_a_number_stays_a_map() {
  for map in [
    json!({ PRIVATE_NAME: "1e400", "other": 1 }),
    private("x"),
    json!({ PRIVATE_NAME: 5 }),
  ] {
    for version_first in [false, true] {
      assert_eq!(kept_ratio(&map, version_first).unwrap(), map, "{map}");
      let text = detection_text(&detection("1.0"), "future", &map, version_first);
      assert!(serde_json::from_str::<Detection>(&text).is_ok(), "{map}");
    }
  }
}

#[test]
fn a_decimal_that_needs_rounding_is_the_nearest_double() {
  assert_eq!(
    kept_ratio(&private("1.2234567890123456789"), true).unwrap(),
    json!(1.223_456_789_012_345_8)
  );
  let wide = kept_ratio(&json!(18_446_744_073_709_551_617_f64), true).unwrap();
  assert!(wide.is_f64(), "{wide}");
}

#[test]
fn a_record_refuses_a_member_of_the_private_name_in_either_order() {
  let error = serde_json::from_value::<Disposition>(private("1.25"))
    .unwrap_err()
    .to_string();
  assert!(error.starts_with(RESERVED), "{error}");
  let beside = json!({ "default": true, PRIVATE_NAME: "1" });
  assert!(
    serde_json::from_value::<Disposition>(beside)
      .unwrap_err()
      .to_string()
      .starts_with(RESERVED)
  );
  for version_first in [false, true] {
    let streams = json!([with(stream(0, "data"), "disposition", private("1.25"))]);
    let members = with_member(&members_of(&report("1.0")), "streams", &streams.to_string(), 0);
    let position = if version_first { 0 } else { members.len() };
    let error = serde_json::from_str::<MediaReport>(&text_of(&members, Some((position, "1.0")))).unwrap_err();
    assert!(error.to_string().starts_with(RESERVED), "{version_first}: {error}");
  }
}

#[test]
fn a_kind_refuses_the_private_name_beside_its_members_alone_and_in_a_report() {
  for value in [
    json!({ "kind": "text", "text": "x", PRIVATE_NAME: "1" }),
    json!({ PRIVATE_NAME: "1", "kind": "text", "text": "x" }),
  ] {
    let error = serde_json::from_value::<TagValue>(value.clone())
      .unwrap_err()
      .to_string();
    assert!(error.starts_with(RESERVED), "{value}: {error}");
  }
  let newer = serde_json::from_value::<TagValue>(json!({ "kind": "rating", "stars": private("4.5") })).unwrap();
  assert_eq!(
    serde_json::to_value(newer).unwrap(),
    json!({ "kind": "rating", "stars": 4.5 })
  );
}

#[test]
fn a_member_of_a_known_integer_type_refuses_the_map_wherever_it_is_read() {
  for value in [
    json!({ "integer": private("60"), "kind": "integer" }),
    json!({ "kind": "integer", "integer": private("60") }),
    json!({ "kind": "binary", "binary": { "size": private("4") } }),
  ] {
    let error = serde_json::from_value::<TagValue>(value.clone())
      .unwrap_err()
      .to_string();
    assert!(error.starts_with("invalid type: map"), "{value}: {error}");
  }
  let chapter = json!([{ "titles": [] }, { "parent": private("0"), "titles": [] }]);
  let error = serde_json::from_value::<cc_media_probe::model::Chapter>(chapter)
    .unwrap_err()
    .to_string();
  assert!(error.starts_with("invalid type: map"), "{error}");
}

#[test]
fn a_nested_record_refuses_a_number_that_is_not_finite_before_its_version_in_either_order() {
  for report_first in [true, false] {
    let partial = with(report("2.0"), "future", private("1e400"));
    let record = with(error_record("1.0"), "partial_report", partial);
    let members = members_of(&record);
    let position = if report_first { members.len() } else { 0 };
    let error = serde_json::from_str::<ErrorRecord>(&text_of(&members, Some((position, "1.0")))).unwrap_err();
    assert!(error.to_string().starts_with(NOT_JSON), "{report_first}: {error}");
    let partial = with(report("2.0"), PRIVATE_NAME, json!("hi"));
    let record = with(error_record("1.0"), "partial_report", partial);
    let members = members_of(&record);
    let error = serde_json::from_str::<ErrorRecord>(&text_of(&members, Some((position, "1.0")))).unwrap_err();
    assert!(error.to_string().starts_with(NESTED), "{report_first}: {error}");
  }
}
