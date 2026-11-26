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

//! Round trips of the model values that the crate's test constructors build, which no reader of a document gives
//! before the format phases: each is written to JSON and read back as itself, so the writer gives only what the
//! reader takes (`api.md` section 12). Values that only a constructor can build show the writing side of the twins
//! and of a time beyond the years that the form writes.

use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::json;

use crate::error::{Error, ErrorRecord};
use crate::model::{BinaryValue, Detection, IntegerValue, Tag, TagKey, TagSet, TagValue, TargetScope, UtcTime};
use crate::{Format, MediaReport};

/// The value written and read back.
fn round_trip<Value: Serialize + DeserializeOwned + PartialEq + std::fmt::Debug>(value: &Value) {
  let text = serde_json::to_string(value).unwrap();
  assert!(!text.contains("null"), "{text}");
  let read: Value = serde_json::from_str(&text).unwrap();
  assert_eq!(&read, value, "{text}");
}

#[test]
fn reads_back_every_sample_report() {
  round_trip(&MediaReport::sample());
  round_trip(&MediaReport::cut_by_a_source_failure());
  let nested = Tag::sample(
    "ORIGINAL",
    None,
    None,
    vec![Tag::sample("TITLE", Some(TagKey::Title), Some("Original"), Vec::new())],
  );
  let sets = vec![
    TagSet::sample(TargetScope::WholeFile, Vec::new(), vec![nested.clone()]).superseded(),
    TagSet::sample(TargetScope::Incomplete, Vec::new(), Vec::new()),
  ];
  round_trip(&MediaReport::cut_by_a_source_failure().with_tag_sets(sets));
  round_trip(&nested);
}

#[test]
fn reads_back_every_sample_detection_and_error_record() {
  round_trip(&Detection::identified(Format::Matroska));
  round_trip(&Detection::unmatched(None, crate::model::Usage::sample()));
  for error in [
    Error::failed_source(),
    Error::failed_companion(),
    Error::unopened_source(),
    Error::stopped_detection(),
    Error::unchecked_candidate(),
    Error::hinted_format(),
    Error::stopped_before_the_inventory(),
    Error::unsupported_format(),
  ] {
    round_trip(&error.to_record());
  }
  let record = ErrorRecord::from(Error::failed_source());
  let written = serde_json::to_value(&record).unwrap();
  assert_eq!(written["partial_report"]["schema_version"], json!("1.0"));
}

#[test]
fn writes_a_twin_beyond_the_exact_range() {
  let integer = TagValue::Integer(IntegerValue::unsigned(u64::MAX));
  assert_eq!(
    serde_json::to_value(&integer).unwrap(),
    json!({ "kind": "integer", "integer_exact": "18446744073709551615" })
  );
  round_trip(&integer);
  let negative = TagValue::Integer(IntegerValue::signed(i64::MIN));
  assert_eq!(
    serde_json::to_value(&negative).unwrap(),
    json!({ "kind": "integer", "integer_exact": "-9223372036854775808" })
  );
  round_trip(&negative);
  round_trip(&TagValue::Binary(BinaryValue::sample(&[0x89, 0x50])));
}

#[test]
fn refuses_to_write_a_time_beyond_the_years_of_the_form() {
  assert!(serde_json::to_string(&UtcTime::sample(253_402_300_800, 0)).is_err());
  assert_eq!(
    serde_json::to_value(UtcTime::sample(0, 0)).unwrap(),
    json!("1970-01-01T00:00:00Z")
  );
}
