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

//! The rules of the model's other records (`json-output.md` sections 6.1, 7.3, 8 and 10): a protection's sample
//! descriptions agree with its flag, its count and the order of their indexes; a declaration holds the members of its
//! scope; a timecode's nominal rate is at least 1; an exact value's parts are canonical decimal strings; a string
//! enum and a typed detail of a newer version keep what they read and write it back; and an error record of a newer
//! kind keeps the members that it does not read.

use cc_media_probe::details::{ExactValue, FormatDetails};
use cc_media_probe::error::ErrorRecord;
use cc_media_probe::model::{Protection, ProtectionDeclaration, SampleDescriptionState, Timecode};
use cc_media_probe::{Format, StreamKind};
use serde_json::{Value, json};

use crate::fixtures::{error_record, with, without};

fn protection(has: bool, count: Option<u64>, entries: Value, declarations: Value) -> Value {
  let mut protection = json!({ "fully_examined": false, "has_sample_descriptions": has });
  protection["declarations"] = declarations;
  protection["sample_descriptions"] = entries;
  if let Some(count) = count {
    protection["sample_description_count"] = json!(count);
  }
  protection
}

fn entry(index: u64, state: &str) -> Value {
  json!({ "index": index, "state": state })
}

fn declared(index: u64) -> Value {
  json!({ "scope": "sample_description", "sample_description_index": index })
}

#[test]
fn a_protection_agrees_with_its_flag_its_count_and_its_indexes() {
  let read = |value: Value| serde_json::from_value::<Protection>(value);
  let valid = protection(
    true,
    Some(3),
    json!([entry(1, "clear"), entry(3, "declared")]),
    json!([declared(3)]),
  );
  let read_back = read(valid.clone()).unwrap();
  assert_eq!(
    read_back.sample_description_state(2),
    Some(&SampleDescriptionState::Unknown)
  );
  assert_eq!(serde_json::to_value(&read_back).unwrap(), valid);
  // A `declared` entry stands without a declaration that names its index, and reads back as written (the review of
  // revision 57, plan Phase 2, task 13).
  let dropped = protection(true, None, json!([entry(2, "declared")]), json!([]));
  let read_back = read(dropped.clone()).unwrap();
  assert_eq!(
    read_back.sample_description_state(2),
    Some(&SampleDescriptionState::Declared)
  );
  assert_eq!(serde_json::to_value(&read_back).unwrap(), dropped);
  assert!(read(protection(true, None, json!([entry(2, "future_state")]), json!([]))).is_ok());
  for malformed in [
    protection(false, Some(1), json!([]), json!([])),
    protection(false, None, json!([entry(1, "clear")]), json!([])),
    protection(false, None, json!([]), json!([declared(1)])),
    protection(true, None, json!([entry(0, "clear")]), json!([])),
    protection(true, None, json!([entry(2, "clear"), entry(1, "clear")]), json!([])),
    protection(true, None, json!([entry(1, "clear"), entry(1, "clear")]), json!([])),
    protection(true, Some(2), json!([entry(3, "clear")]), json!([])),
    protection(true, None, json!([entry(1, "unknown")]), json!([])),
    protection(true, Some(2), json!([]), json!([declared(3)])),
    protection(true, None, json!([]), json!([declared(0)])),
  ] {
    assert!(read(malformed.clone()).is_err(), "{malformed}");
  }
}

#[test]
fn a_declaration_holds_the_members_of_its_scope() {
  let read = |value: Value| serde_json::from_value::<ProtectionDeclaration>(value);
  let scheme = json!({ "namespace": "iso_scheme_type", "value": "63656e63" });
  assert!(read(json!({ "scheme": scheme, "scope": "packets" })).is_ok());
  assert!(read(json!({ "scope": "encoding_settings", "encoding_order": 0 })).is_ok());
  assert!(read(declared(1)).is_ok());
  assert!(read(json!({ "scope": "future_scope", "sample_description_index": 0, "encoding_order": 1 })).is_ok());
  for malformed in [
    json!({ "scope": "sample_description" }),
    json!({ "scope": "packets", "sample_description_index": 1 }),
    json!({ "scope": "packets", "encoding_order": 1 }),
    json!({ "scope": "sample_description", "sample_description_index": 1, "encoding_order": 1 }),
  ] {
    assert!(read(malformed.clone()).is_err(), "{malformed}");
  }
}

#[test]
fn a_timecode_has_a_nominal_rate_of_at_least_1() {
  let timecode = |nominal: u64| {
    json!({
      "frame_number": -25,
      "rate": { "numerator": 30_000, "denominator": 1_001 },
      "nominal_frames_per_second": nominal,
      "drop_frame": true,
      "wraps_at_24_hours": false,
    })
  };
  let read_back: Timecode = serde_json::from_value(timecode(30)).unwrap();
  assert_eq!(read_back.frame_number(), -25);
  assert_eq!(serde_json::to_value(&read_back).unwrap(), timecode(30));
  assert!(serde_json::from_value::<Timecode>(timecode(0)).is_err());
  assert!(serde_json::from_value::<Timecode>(without(timecode(30), "drop_frame")).is_err());
}

#[test]
fn an_exact_value_holds_canonical_decimal_strings() {
  let exact = |numerator: &str, denominator: Option<&str>, input: &str| {
    let mut value = json!({
      "field": "start",
      "owner": { "kind": "chapter", "index": 0, "chapter_path": [0] },
      "numerator": numerator,
      "inputs": [{ "name": "chapter_time_start", "value": input }],
    });
    if let Some(denominator) = denominator {
      value["denominator"] = json!(denominator);
    }
    value
  };
  let value = exact("3689348814741910323", Some("200000000"), "18446744073709551615");
  let read_back: ExactValue = serde_json::from_value(value.clone()).unwrap();
  assert_eq!(serde_json::to_value(&read_back).unwrap(), value);
  assert!(serde_json::from_value::<ExactValue>(exact("-9007199254740993", Some("1"), "-1")).is_ok());
  for (numerator, denominator, input) in [
    ("+1", Some("1"), "1"),
    ("01", Some("1"), "1"),
    ("1", Some("0"), "1"),
    ("1", Some("01"), "1"),
    ("1", Some("-1"), "1"),
    ("1", Some("1"), "1.5"),
    ("1", Some("1"), ""),
  ] {
    let value = exact(numerator, denominator, input);
    assert!(serde_json::from_value::<ExactValue>(value.clone()).is_err(), "{value}");
  }
}

#[test]
fn a_string_enum_and_a_typed_detail_of_a_newer_version_are_written_back_as_read() {
  let kind: StreamKind = serde_json::from_value(json!("hologram")).unwrap();
  assert_eq!(kind.as_str(), "hologram");
  assert_eq!(serde_json::to_value(&kind).unwrap(), json!("hologram"));
  assert_eq!(serde_json::from_value::<Format>(json!("webm")).unwrap(), Format::WebM);
  assert!(serde_json::from_value::<Format>(json!(5)).is_err());
  let details = r#"{"segment_uid":"00ff","kind":"matroska","date_utc":"-1","tracks":[{"number":1}]}"#;
  let read_back: FormatDetails = serde_json::from_str(details).unwrap();
  assert_eq!(read_back.as_str(), "matroska");
  assert_eq!(
    serde_json::to_string(&read_back).unwrap(),
    r#"{"kind":"matroska","segment_uid":"00ff","date_utc":"-1","tracks":[{"number":1}]}"#
  );
  assert!(serde_json::from_value::<FormatDetails>(json!({ "segment_uid": "00ff" })).is_err());
  assert!(serde_json::from_value::<FormatDetails>(json!({ "kind": "matroska", "x": null })).is_err());
}

#[test]
fn an_error_record_of_a_newer_kind_keeps_the_members_that_it_does_not_read() {
  let mut newer = without(error_record("1.1"), "source_failure_kind");
  newer["kind"] = json!("quota_exceeded");
  newer["quota"] = json!({ "limit": 3 });
  let read_back: ErrorRecord = serde_json::from_value(newer).unwrap();
  let written = serde_json::to_string(&read_back).unwrap();
  assert!(written.ends_with(r#""quota":{"limit":3}}"#), "{written}");
  assert_eq!(serde_json::from_str::<ErrorRecord>(&written).unwrap(), read_back);
  let known = with(error_record("1.1"), "quota", json!({ "limit": 3 }));
  let written = serde_json::to_value(serde_json::from_value::<ErrorRecord>(known).unwrap()).unwrap();
  assert!(written.get("quota").is_none(), "{written}");
}
