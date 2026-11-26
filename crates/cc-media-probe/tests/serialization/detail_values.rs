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

//! The values of the typed details and the owners that locate them (`json-output.md` sections 5.5, 5.7, 7.11 and 8),
//! after the review of the third slice's first part: a stored 64-bit integer is a decimal string whatever its value;
//! an exact value's parts are canonical decimal strings of any length, with a denominator exactly for a rational
//! field of the field guide; an entry's `stream` is its owner's index exactly when the owner is a stream; a chapter's
//! location has a path and no other location has one; and an external reference's owner is an entity location with
//! its sample description's index beside it, for a stream only, from 1. After the second review of that part: a
//! stored default of text or floating point is in its type's form; an exact rational is reduced, with parts of at
//! most 10000 digits; and the angles, luminances, estimates, depths and light levels of the report model are rational
//! or integer fields too. After its third review: the value of a listed field lies beyond the range that its
//! normalized field holds, and an integer field, a count or a size, is never negative.

use cc_media_probe::details::{Conflict, ExactValue, ObservedBound, StoredValue};
use cc_media_probe::model::{EntityLocation, ExternalReference, FloatingPointValue};
use serde::de::DeserializeOwned;
use serde_json::{Value, json};

/// Whether the value reads as that type, and, when it does, writes back as read.
fn reads_back<Type: DeserializeOwned + serde::Serialize>(value: &Value) -> bool {
  match serde_json::from_value::<Type>(value.clone()) {
    Ok(read) => {
      assert_eq!(&serde_json::to_value(&read).unwrap(), value);
      true
    }
    Err(_) => false,
  }
}

#[test]
fn a_stored_64_bit_integer_is_a_decimal_string_whatever_its_value() {
  for value in ["1000000", "0", "18446744073709551615"] {
    let stored = json!({ "value": value, "stored": true });
    assert!(reads_back::<StoredValue<u64>>(&stored), "{stored}");
  }
  let read: StoredValue<u64> = serde_json::from_value(json!({ "value": "1000000", "stored": false })).unwrap();
  assert_eq!((*read.value(), read.is_stored()), (1_000_000, false));
  for value in ["-9223372036854775808", "-1", "9223372036854775807"] {
    assert!(
      reads_back::<StoredValue<i64>>(&json!({ "value": value, "stored": true })),
      "{value}"
    );
  }
  for malformed in [
    json!({ "value": 1_000_000, "stored": true }),
    json!({ "value": "01", "stored": true }),
    json!({ "value": "+1", "stored": true }),
    json!({ "value": "-1", "stored": true }),
    json!({ "value": "18446744073709551616", "stored": true }),
  ] {
    assert!(
      serde_json::from_value::<StoredValue<u64>>(malformed.clone()).is_err(),
      "{malformed}"
    );
  }
  assert!(serde_json::from_value::<StoredValue<i64>>(json!({ "value": "-0", "stored": true })).is_err());
  assert!(reads_back::<StoredValue<bool>>(
    &json!({ "value": true, "stored": false })
  ));
}

#[test]
fn a_stored_default_of_text_or_floating_point_is_in_its_types_form() {
  // The review's cases: a Matroska track's `Language`, `eng`, and its `SamplingFrequency`, 8000 in double precision.
  assert!(reads_back::<StoredValue<String>>(
    &json!({ "value": "eng", "stored": false })
  ));
  let rate = json!({ "value": { "width": "double", "bits": "40bf400000000000" }, "stored": false });
  assert!(reads_back::<StoredValue<FloatingPointValue>>(&rate));
  assert!(serde_json::from_value::<StoredValue<String>>(json!({ "value": 7, "stored": false })).is_err());
  let uppercase = json!({ "value": { "width": "double", "bits": "40BF400000000000" }, "stored": false });
  assert!(serde_json::from_value::<StoredValue<FloatingPointValue>>(uppercase).is_err());
}

fn exact(field: &str, numerator: &str, denominator: Option<&str>) -> Value {
  let mut value = json!({
    "field": field,
    "numerator": numerator,
    "inputs": [{ "name": "input", "value": numerator }],
  });
  if let Some(denominator) = denominator {
    value["denominator"] = json!(denominator);
  }
  value
}

#[test]
fn an_exact_value_has_no_width_and_its_field_decides_its_denominator() {
  let wide = "170141183460469231731687303715884105728";
  assert!(reads_back::<ExactValue>(&exact("duration", wide, Some("1"))));
  let wider = format!("-{wide}{wide}");
  // 2^127 - 1 is prime and divides neither factor of the numerator, 2^127 and 10^39 + 1.
  assert!(reads_back::<ExactValue>(&exact(
    "start",
    &wider,
    Some("170141183460469231731687303715884105727")
  )));
  assert!(reads_back::<ExactValue>(&exact("frame_count", wide, None)));
  assert!(reads_back::<ExactValue>(&exact("future_field", wide, None)));
  assert!(reads_back::<ExactValue>(&exact("future_field", wide, Some("3"))));
  for (field, numerator, denominator) in [
    ("duration", wide, None),
    ("frame_rate", "9007199254740993", None),
    ("frame_count", wide, Some("1")),
    ("size", "9007199254740993", Some("1")),
    ("duration", "-0", Some("1")),
    ("duration", "007", Some("1")),
    ("duration", "7", Some("-1")),
    ("duration", "7", Some("00")),
  ] {
    let value = exact(field, numerator, denominator);
    assert!(serde_json::from_value::<ExactValue>(value.clone()).is_err(), "{value}");
  }
}

#[test]
fn an_exact_rational_is_reduced_with_parts_of_at_most_10000_digits() {
  assert!(reads_back::<ExactValue>(&exact(
    "start",
    "3689348814741910323",
    Some("200000000")
  )));
  assert!(reads_back::<ExactValue>(&exact("future_field", "0", Some("1"))));
  assert!(reads_back::<ExactValue>(&exact("future_field", "-9", Some("4"))));
  let longest = format!("1{}", "0".repeat(9_999));
  assert!(reads_back::<ExactValue>(&exact("duration", &longest, Some("1"))));
  assert!(reads_back::<ExactValue>(&exact(
    "duration",
    &format!("-{longest}"),
    Some("3")
  )));
  assert!(reads_back::<ExactValue>(&exact("frame_count", &longest, None)));
  let longer = format!("{longest}0");
  for (field, numerator, denominator) in [
    // The review's case: 9007199254740993 seconds written over 2.
    ("duration", "18014398509481986", Some("2")),
    ("duration", "-6", Some("4")),
    ("duration", "0", Some("2")),
    ("future_field", "9", Some("3")),
    ("duration", longer.as_str(), Some("1")),
    ("frame_count", longer.as_str(), None),
    ("duration", "1", Some(longer.as_str())),
  ] {
    let value = exact(field, numerator, denominator);
    assert!(
      serde_json::from_value::<ExactValue>(value.clone()).is_err(),
      "{field} {}",
      numerator.len()
    );
  }
}

#[test]
fn every_rational_and_integer_quantity_of_the_model_decides_its_denominator() {
  let beyond = "9007199254740993";
  for field in [
    "yaw",
    "pitch",
    "roll",
    "azimuth",
    "elevation",
    "maximum_luminance",
    "minimum_luminance",
    "x_coordinate",
    "y_coordinate",
    "estimated_duration",
    "estimated_bit_rate",
  ] {
    assert!(reads_back::<ExactValue>(&exact(field, beyond, Some("1"))), "{field}");
    // The review's case: a projection's yaw without a denominator.
    assert!(
      serde_json::from_value::<ExactValue>(exact(field, beyond, None)).is_err(),
      "{field}"
    );
  }
  for field in [
    "bit_depth",
    "ambisonic_component",
    "maximum_content",
    "maximum_frame_average",
  ] {
    assert!(reads_back::<ExactValue>(&exact(field, beyond, None)), "{field}");
    // The review's case: an audio bit depth with a denominator.
    assert!(
      serde_json::from_value::<ExactValue>(exact(field, beyond, Some("1"))).is_err(),
      "{field}"
    );
  }
}

fn location(kind: &str, index: usize, path: &[usize]) -> Value {
  json!({ "kind": kind, "index": index, "chapter_path": path })
}

#[test]
fn a_chapters_location_has_a_path_and_no_other_location_has_one() {
  assert!(reads_back::<EntityLocation>(&location("chapter", 0, &[0, 2])));
  assert!(reads_back::<EntityLocation>(&location("stream", 1, &[])));
  assert!(reads_back::<EntityLocation>(&location("future_entity", 1, &[3])));
  for malformed in [
    location("chapter", 0, &[]),
    location("stream", 1, &[0]),
    location("edition", 0, &[1]),
  ] {
    assert!(
      serde_json::from_value::<EntityLocation>(malformed.clone()).is_err(),
      "{malformed}"
    );
  }
}

/// An entry of a list of the typed details, with the test of whether it reads back.
type Entry = (Value, fn(&Value) -> bool);

/// An entry of each list of the typed details, with that owner and stream.
fn entries(owner: Option<&Value>, stream: Option<usize>) -> [Entry; 3] {
  let mut conflict = json!({ "field": "duration", "values": [] });
  let mut bound = json!({ "field": "frame_count", "value": { "kind": "count", "count": "12" } });
  let mut exact = exact("frame_count", "9007199254740993", None);
  for entry in [&mut conflict, &mut bound, &mut exact] {
    if let Some(owner) = owner {
      entry["owner"] = owner.clone();
    }
    if let Some(stream) = stream {
      entry["stream"] = json!(stream);
    }
  }
  [
    (conflict, reads_back::<Conflict>),
    (bound, reads_back::<ObservedBound>),
    (exact, reads_back::<ExactValue>),
  ]
}

#[test]
fn an_entrys_stream_is_its_owners_index_exactly_when_the_owner_is_a_stream() {
  for (owner, stream) in [
    (None, None),
    (Some(location("stream", 1, &[])), Some(1)),
    (Some(location("chapter", 0, &[1])), None),
    (Some(location("future_entity", 1, &[])), None),
  ] {
    for (entry, reads) in entries(owner.as_ref(), stream) {
      assert!(reads(&entry), "{entry}");
    }
  }
  for (owner, stream) in [
    (None, Some(0)),
    (Some(location("stream", 1, &[])), Some(0)),
    (Some(location("stream", 1, &[])), None),
    (Some(location("chapter", 0, &[1])), Some(0)),
    (Some(location("future_entity", 1, &[])), Some(1)),
  ] {
    for (entry, reads) in entries(owner.as_ref(), stream) {
      assert!(!reads(&entry), "{entry}");
    }
  }
}

fn reference(owner: &Value) -> Value {
  json!({ "kind": "data_reference", "location": "media.bin", "owner": owner })
}

#[test]
fn a_reference_owner_is_an_entity_location_with_its_sample_description_beside_it() {
  let mut owner = location("stream", 0, &[]);
  owner["sample_description_index"] = json!(2);
  assert!(reads_back::<ExternalReference>(&reference(&owner)));
  let read: ExternalReference = serde_json::from_value(reference(&owner)).unwrap();
  let read_owner = read.owner().unwrap();
  assert_eq!(
    (read_owner.location().index(), read_owner.sample_description_index()),
    (0, Some(2))
  );
  assert!(reads_back::<ExternalReference>(&reference(&location(
    "chapter",
    1,
    &[0, 3]
  ))));
  assert!(reads_back::<ExternalReference>(&reference(&location(
    "attachment",
    2,
    &[]
  ))));
  let mut chapter = location("chapter", 1, &[0]);
  chapter["sample_description_index"] = json!(1);
  let mut zero = location("stream", 0, &[]);
  zero["sample_description_index"] = json!(0);
  let nested = json!({ "location": location("stream", 0, &[]), "sample_description_index": 2 });
  for malformed in [
    chapter,
    zero,
    nested,
    location("chapter", 1, &[]),
    location("edition", 0, &[]),
    location("program", 0, &[]),
  ] {
    let value = reference(&malformed);
    assert!(
      serde_json::from_value::<ExternalReference>(value.clone()).is_err(),
      "{value}"
    );
  }
}

#[test]
fn an_exact_value_lies_beyond_the_range_of_its_normalized_field() {
  let beyond = "9007199254740992";
  for (field, numerator, denominator) in [
    ("duration", beyond, Some("1")),
    ("duration", "1", Some(beyond)),
    ("start_time", "-9007199254740993", Some("1")),
    ("yaw", "-1", Some(beyond)),
    ("frame_count", beyond, None),
    ("size", "18446744073709551616", None),
    ("future_field", "1", None),
    ("future_field", "-1", Some("2")),
  ] {
    let value = exact(field, numerator, denominator);
    assert!(reads_back::<ExactValue>(&value), "{value}");
  }
  for (field, numerator, denominator) in [
    // The review's cases: a duration of 0/1, a frame count of 1, and a negative frame count.
    ("duration", "0", Some("1")),
    ("frame_count", "1", None),
    ("frame_count", "-9007199254740993", None),
    ("duration", "9007199254740991", Some("9007199254740990")),
    ("channel_count", "9007199254740991", None),
  ] {
    let value = exact(field, numerator, denominator);
    assert!(serde_json::from_value::<ExactValue>(value.clone()).is_err(), "{value}");
  }
}
