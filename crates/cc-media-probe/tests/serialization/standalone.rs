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

//! A record read alone obeys the rules that it obeys inside a root record (`api.md` section 3.3; `json-output.md`
//! section 2): a null, a member named twice and a record written as an array fail at both entry points. A value that
//! one of Serde's own value deserializers gives is read alone from that value, so a sequence never passes for a read
//! under way, and an enum that a reader gives as an enum is a value that JSON does not carry.

use cc_media_probe::document::{Document, Generator, UsageError};
use cc_media_probe::error::ErrorRecord;
use cc_media_probe::model::{Chapter, Disposition, TagValue};
use cc_media_probe::{MediaReport, Rational};
use serde::Deserialize;
use serde::de::value::{Error as ValueError, MapDeserializer, SeqDeserializer, StrDeserializer};
use serde_json::{Value, json};

use crate::fixtures::{document, error_record, reads_document, report, with};

#[test]
fn a_record_obeys_the_same_rules_alone_and_inside_a_root_record() {
  let nested = |value: Value| {
    let mut report = report("1.0");
    report["duration"] = value;
    report["duration_provenance"] = json!("declared");
    serde_json::from_value::<MediaReport>(report).is_ok()
  };
  let alone = |value: Value| serde_json::from_value::<Rational>(value).is_ok();
  for value in [
    json!({ "numerator": 1, "denominator": 2 }),
    json!({ "numerator": 1, "denominator": null }),
    json!([1, 2]),
    json!({ "numerator": 1, "denominator": 2, "extra": null }),
  ] {
    assert_eq!(alone(value.clone()), nested(value.clone()), "{value}");
  }
  let twice = r#"{"numerator": 1, "numerator": 1, "denominator": 2}"#;
  assert!(serde_json::from_str::<Rational>(twice).is_err());
}

#[test]
fn a_record_is_never_a_positional_array() {
  assert!(serde_json::from_value::<MediaReport>(json!(["1.0", "matroska"])).is_err());
  assert!(serde_json::from_value::<Disposition>(json!([true])).is_err());
  let binary = json!({ "kind": "binary", "binary": [1, "ff"] });
  assert!(serde_json::from_value::<TagValue>(binary).is_err());
  let record = with(error_record("1.0"), "usage", json!([0, 0, 0, 0, 0, 0, 0, 0]));
  assert!(serde_json::from_value::<ErrorRecord>(record).is_err());
}

#[test]
fn a_sequence_that_a_value_deserializer_gives_is_a_value_read_alone() {
  let chapter = vec![json!({ "titles": [] })];
  let read = Chapter::deserialize(SeqDeserializer::<_, serde_json::Error>::new(chapter.into_iter()));
  assert!(read.is_ok(), "{read:?}");
  let positional = vec![json!(1), json!(2)];
  assert!(Rational::deserialize(SeqDeserializer::<_, serde_json::Error>::new(positional.into_iter())).is_err());
  let members = vec![("numerator", 1_u64), ("denominator", 2)];
  let read = Rational::deserialize(MapDeserializer::<_, ValueError>::new(members.into_iter()));
  assert_eq!(read.ok(), Rational::new(1, 2));
  let status =
    cc_media_probe::completeness::DetectionStatus::deserialize(StrDeserializer::<ValueError>::new("complete"));
  assert!(status.is_ok());
}

#[test]
fn an_enum_that_a_value_deserializer_gives_is_no_value_of_json() {
  let access = StrDeserializer::<ValueError>::new("complete");
  let enumerated = serde::de::value::EnumAccessDeserializer::new(access);
  let error = cc_media_probe::completeness::DetectionStatus::deserialize(enumerated)
    .unwrap_err()
    .to_string();
  assert!(error.starts_with("a value that JSON does not carry"), "{error}");
}

#[test]
fn an_error_record_read_alone_obeys_the_rules_of_its_members() {
  let mut record = error_record("1.0");
  record["usage"]["bytes_read"] = Value::Null;
  assert!(serde_json::from_value::<ErrorRecord>(record).is_err());
  let record = with(error_record("1.0"), "cause_messages", json!(["a", null]));
  assert!(serde_json::from_value::<ErrorRecord>(record).is_err());
}

#[test]
fn a_generator_and_a_usage_error_obey_the_same_rules_alone_and_in_a_document() {
  let generator = json!({ "name": "catalog", "version": "1" });
  let usage_error = json!({ "message": "no input was named" });
  for (member, valid) in [("generator", generator), ("usage_error", usage_error)] {
    let positional: Vec<Value> = valid.as_object().unwrap().values().cloned().collect();
    for value in [
      valid.clone(),
      with(valid.clone(), "future", Value::Null),
      with(valid.clone(), "future", json!({ "inner": [1, null] })),
      json!(positional),
    ] {
      let alone = match member {
        "generator" => serde_json::from_value::<Generator>(value.clone()).is_ok(),
        _ => serde_json::from_value::<UsageError>(value.clone()).is_ok(),
      };
      let nested = reads_document(&with(document("1.0", json!([])), member, value.clone()));
      assert_eq!(alone, nested, "{member} {value}");
      assert_eq!(alone, value == valid, "{member} {value}");
    }
  }
  // A name given twice, which only the text shows; and the trait's own function, which a type read through its own
  // name reaches, as the nineteenth review asked.
  let twice = r#"{"name": "catalog", "name": "catalog", "version": "1"}"#;
  assert!(serde_json::from_str::<Generator>(twice).is_err());
  let positional = r#"["catalog", "1"]"#;
  let read = Generator::deserialize(&mut serde_json::Deserializer::from_str(positional));
  assert!(
    read
      .unwrap_err()
      .to_string()
      .starts_with("a record is written as an array of its values")
  );
}

#[test]
fn a_document_is_never_a_positional_array() {
  let positional = json!(["1.0", { "name": "catalog", "version": "1" }, []]);
  let message = serde_json::from_value::<Document>(positional).unwrap_err().to_string();
  assert!(!message.starts_with("the version error"), "{message}");
  let entry = json!([0, "input.bin", "error", error_record("1.0")]);
  assert!(!reads_document(&document("1.0", json!([entry]))));
}
