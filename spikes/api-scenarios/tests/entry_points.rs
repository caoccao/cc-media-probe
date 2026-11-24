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

//! The nineteenth review's cases of `api.md` section 3.3 and `json-output.md` section 2. A type of the form has one
//! way to be read: the review called the public inherent `Generator::deserialize` that Serde's derive gave with
//! `serde(remote = "Self")`, which read the positional array `["catalog", "1"]` that the trait refuses. The strict
//! reader's answer to the request for the scope was a sequence of two, so through Serde's `SeqDeserializer` the
//! review read `["unused", {"name": "catalog", "version": "1"}]` as a generator, and refused the valid chapter array
//! `[{"titles": []}]`. An integer within 64 bits that a reader gives as 128 bits failed as one beyond them: the
//! review read the rational `1/2` from a standard map with `u64` parts and refused it with `u128` parts. And a number
//! at the root failed as no object without serde_json's `arbitrary_precision` and as a root record without its
//! version with it, since the feature gives `1.25` as a private map; the maintainer chose the number in every build.

use cc_media_probe::Rational;
use cc_media_probe::completeness::DetectionStatus;
use cc_media_probe::document::{Document, Generator};
use cc_media_probe::model::Chapter;
use serde::Deserialize;
use serde::de::value::{
  EnumAccessDeserializer, Error as ValueError, MapDeserializer, SeqDeserializer, StrDeserializer,
};
use serde_json::{Value, json};

const POSITIONAL: &str = "a record is written as an object of named members, never as an array";
const NOT_JSON: &str = "a value is one that JSON carries as read";
const NUMBER_AT_THE_ROOT: &str = "invalid type: floating point `1.25`, expected a root record";

#[test]
fn a_generator_read_by_its_own_name_meets_the_rules_of_the_trait() {
  // The review's case: with the trait in scope, the name reaches the trait's function, and no inherent function of
  // the derive stands before it; a doc test of `Generator` shows that, without the trait, the name reaches nothing.
  let positional = r#"["catalog", "1"]"#;
  let read = Generator::deserialize(&mut serde_json::Deserializer::from_str(positional));
  assert!(read.unwrap_err().to_string().starts_with(POSITIONAL));
  let valid = r#"{"name": "catalog", "version": "1"}"#;
  assert!(Generator::deserialize(&mut serde_json::Deserializer::from_str(valid)).is_ok());
}

fn sequence(items: Vec<Value>) -> SeqDeserializer<std::vec::IntoIter<Value>, serde_json::Error> {
  SeqDeserializer::new(items.into_iter())
}

#[test]
fn a_sequence_that_a_value_deserializer_gives_is_a_value_read_alone() {
  // The review's two cases: the valid chapter array, and a sequence shaped like the strict reader's former answer.
  let chapter = Chapter::deserialize(sequence(vec![json!({ "titles": [] })])).unwrap();
  assert!(chapter.children().is_empty());
  let forged = sequence(vec![json!("unused"), json!({ "name": "catalog", "version": "1" })]);
  assert!(
    Generator::deserialize(forged)
      .unwrap_err()
      .to_string()
      .starts_with(POSITIONAL)
  );
  // The text of the strict reader's identifier is a string like any other: only the static itself is the answer.
  let named = sequence(vec![
    json!("$cc_media_probe::read_scope"),
    json!({ "name": "catalog", "version": "1" }),
  ]);
  assert!(
    Generator::deserialize(named)
      .unwrap_err()
      .to_string()
      .starts_with(POSITIONAL)
  );
}

#[test]
fn an_enum_that_a_value_deserializer_gives_is_no_value_of_json() {
  // An enum given to a request for the scope is the strict reader's answer only when its identifier is the reader's
  // own static; any other is refused as a tagged value of ciborium is, wherever the strict reader meets one.
  let given = EnumAccessDeserializer::new(StrDeserializer::<ValueError>::new("Incomplete"));
  assert!(
    DetectionStatus::deserialize(given)
      .unwrap_err()
      .to_string()
      .starts_with(NOT_JSON)
  );
  // The variant's name as a string is the status.
  let named = StrDeserializer::<ValueError>::new("Incomplete");
  assert_eq!(
    DetectionStatus::deserialize(named).unwrap(),
    DetectionStatus::Incomplete
  );
}

#[test]
fn an_integer_within_64_bits_reads_whatever_width_the_reader_gives_it_in() {
  let narrow = MapDeserializer::<_, ValueError>::new(vec![("numerator", 1_u64), ("denominator", 2_u64)].into_iter());
  assert_eq!(Rational::deserialize(narrow).unwrap(), Rational::new(1, 2).unwrap());
  // The review's case: the same parts as 128 bits.
  let wide = MapDeserializer::<_, ValueError>::new(vec![("numerator", 1_u128), ("denominator", 2_u128)].into_iter());
  assert_eq!(Rational::deserialize(wide).unwrap(), Rational::new(1, 2).unwrap());
  let signed = MapDeserializer::<_, ValueError>::new(vec![("numerator", -1_i128), ("denominator", 2_i128)].into_iter());
  assert_eq!(Rational::deserialize(signed).unwrap(), Rational::new(-1, 2).unwrap());
  // Beyond 64 bits, such as a CBOR bignum, the value is not one that JSON carries.
  let beyond = 1_u128 << 64;
  let too_wide =
    MapDeserializer::<_, ValueError>::new(vec![("numerator", 1_u128), ("denominator", beyond)].into_iter());
  assert!(
    Rational::deserialize(too_wide)
      .unwrap_err()
      .to_string()
      .starts_with(NOT_JSON)
  );
}

/// A document holding one entry whose report is that value.
fn entry_with_report(report: Value) -> String {
  let entry = json!({ "index": 0, "input": "a", "outcome": "ok", "report": report });
  format!(r#"{{"schema_version": "1.0", "generator": {{"name": "g", "version": "1"}}, "results": [{entry}]}}"#)
}

#[test]
fn a_number_at_the_root_is_no_object_in_every_build() {
  // The spike's build has no `arbitrary_precision`, so the map that the feature gives for `1.25` is written as text,
  // which reaches the reader through the same calls; the plain number fails the same way.
  let private = r#"{"$serde_json::private::Number": "1.25"}"#;
  for text in ["1.25", private] {
    let error = serde_json::from_str::<Document>(text).unwrap_err().to_string();
    assert!(error.starts_with(NUMBER_AT_THE_ROOT), "{text}: {error}");
    let nested = serde_json::from_str::<Document>(&entry_with_report(serde_json::from_str(text).unwrap()));
    let error = nested.unwrap_err().to_string();
    assert!(error.starts_with(NUMBER_AT_THE_ROOT), "nested {text}: {error}");
  }
  // A map of that name that holds no number's text, or holds another member beside it, is an object without a
  // version.
  for text in [
    r#"{"$serde_json::private::Number": "hi"}"#,
    r#"{"$serde_json::private::Number": "1.25", "other": 1}"#,
  ] {
    let error = serde_json::from_str::<Document>(text).unwrap_err().to_string();
    assert!(
      error.starts_with("a root record has no schema_version"),
      "{text}: {error}"
    );
  }
}
