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

//! The cases of an enum that carries data, `api.md` section 12, on `TagValue` and `ChromaFormat`: the kind alone
//! decides how the other members are read, a member is present when its name is, whatever it holds, a variant
//! without a value is its kind alone, a binary size beyond 2^53 - 1 is its twin, and a kind of a newer version keeps
//! its members by value and writes them back in their order.

use cc_media_probe::model::{ChromaFormat, TagValue};
use serde_json::{Value, json};

fn read(value: Value) -> Result<TagValue, serde_json::Error> {
  serde_json::from_value(value)
}

#[test]
fn a_kind_of_a_newer_version_is_read_whatever_its_members_hold() {
  // Its members are not read, so a member that a known kind names may hold any value.
  let newer = read(json!({ "kind": "geolocation", "text": 5, "integer": "north" })).unwrap();
  assert!(matches!(&newer, TagValue::Unrecognized(kind) if kind.as_str() == "geolocation"));
}

#[test]
fn a_kind_of_a_newer_version_writes_its_members_back_unchanged() {
  // The reader that knows `date` reads its value again from what this version wrote back.
  let written = r#"{"kind":"date","date":{"year":2026,"month":11,"day":23},"precision":"day","text":"x"}"#;
  let newer: TagValue = serde_json::from_str(written).unwrap();
  assert_eq!(newer.as_str(), "date");
  assert_eq!(
    serde_json::to_string(&newer).unwrap(),
    written,
    "the members, in the order written"
  );
  // Two values of a newer kind are equal exactly when their members are.
  let other: TagValue = serde_json::from_str(&written.replace("23", "24")).unwrap();
  assert_ne!(newer, other);
}

#[test]
fn a_known_kind_ignores_an_unknown_member() {
  // A member that a newer version added beside a known kind's own is ignored, as any unknown member is.
  let value = read(json!({ "language": "en", "kind": "text", "text": "Title" })).unwrap();
  assert_eq!(value, TagValue::Text("Title".to_owned()));
  assert_eq!(
    serde_json::to_value(&value).unwrap(),
    json!({ "kind": "text", "text": "Title" })
  );
}

#[test]
fn a_member_that_holds_null_is_present() {
  for malformed in [
    json!({ "kind": "integer", "integer": 1, "integer_exact": null }),
    json!({ "kind": "text", "text": "x", "boolean": null }),
    json!({ "kind": "text", "text": null }),
    json!({ "kind": "geolocation", "latitude": null }),
    json!({ "kind": "binary", "binary": { "size": 2, "data": null } }),
  ] {
    assert!(read(malformed.clone()).is_err(), "{malformed}");
  }
}

#[test]
fn a_member_named_twice_is_malformed() {
  let twice = r#"{"kind":"text","text":"a","text":"b"}"#;
  assert!(serde_json::from_str::<TagValue>(twice).is_err());
}

#[test]
fn a_binary_size_beyond_the_exact_range_is_its_twin() {
  // `BinaryValue::size()` is a quantity that the value cannot omit, so beyond 2^53 - 1 it is the twin `size_exact`.
  let large = read(json!({ "kind": "binary", "binary": { "size_exact": "9007199254740993" } })).unwrap();
  let TagValue::Binary(binary) = &large else {
    panic!("a binary value");
  };
  assert_eq!(binary.size(), 9_007_199_254_740_993);
  assert_eq!(
    serde_json::to_value(&large).unwrap(),
    json!({ "kind": "binary", "binary": { "size_exact": "9007199254740993" } })
  );
  let largest = json!({ "kind": "binary", "binary": { "size": 9_007_199_254_740_991_u64 } });
  assert_eq!(serde_json::to_value(read(largest.clone()).unwrap()).unwrap(), largest);
  for malformed in [
    json!({ "kind": "binary", "binary": { "size": 9_007_199_254_740_992_u64 } }),
    json!({ "kind": "binary", "binary": { "size_exact": "9007199254740991" } }),
    json!({ "kind": "binary", "binary": {} }),
  ] {
    assert!(read(malformed.clone()).is_err(), "{malformed}");
  }
}

#[test]
fn a_known_kind_without_a_value_is_its_kind_alone() {
  // The eighth review's case: `ChromaFormat` mixes variants without a value and one with a value.
  let read_format = |value: Value| serde_json::from_value::<ChromaFormat>(value);
  let monochrome = read_format(json!({ "kind": "monochrome" })).unwrap();
  assert_eq!(monochrome, ChromaFormat::Monochrome);
  assert_eq!(
    serde_json::to_value(&monochrome).unwrap(),
    json!({ "kind": "monochrome" })
  );
  // A member named after a kind without a value is no member that a known kind reads, so it is ignored.
  assert_eq!(
    read_format(json!({ "kind": "chroma_420", "chroma_420": true })).unwrap(),
    ChromaFormat::Chroma420
  );
  let other = read_format(json!({ "kind": "other", "other": { "horizontal": 1, "vertical": 0 } })).unwrap();
  let ChromaFormat::Other(subsampling) = &other else {
    panic!("a subsampling kept as written");
  };
  assert_eq!((subsampling.horizontal(), subsampling.vertical()), (1, 0));
  for malformed in [
    json!({ "kind": "other" }),
    json!({ "kind": "monochrome", "other": { "horizontal": 1, "vertical": 1 } }),
    json!({ "kind": "monochrome", "other": null }),
    json!({ "other": { "horizontal": 1, "vertical": 1 } }),
  ] {
    assert!(read_format(malformed.clone()).is_err(), "{malformed}");
  }
  let newer = read_format(json!({ "kind": "chroma_440", "planes": 3 })).unwrap();
  assert_eq!(newer.as_str(), "chroma_440");
  assert_eq!(
    serde_json::to_value(&newer).unwrap(),
    json!({ "kind": "chroma_440", "planes": 3 })
  );
}

#[test]
fn a_kind_of_a_newer_version_keeps_its_members_by_value() {
  // The promise covers each member's decoded value and the order of the members other than `kind`, which is written
  // first. The spelling of a number or a string is the writer's own: `1e0` comes back as `1.0` and an escaped solidus
  // as `/`, while an integer stays an integer and a floating-point number a floating-point number.
  let written = r#"{"payload":1e0,"kind":"future","count":7,"path":"a\/b"}"#;
  let newer: TagValue = serde_json::from_str(written).unwrap();
  assert_eq!(
    serde_json::to_string(&newer).unwrap(),
    r#"{"kind":"future","payload":1.0,"count":7,"path":"a/b"}"#
  );
}

/// A value of a format other than JSON, which a catch-all may meet: a string, a finite or non-finite number, and a
/// byte string, as CBOR has them.
enum OtherValue {
  Text(&'static str),
  Number(f64),
  Bytes(&'static [u8]),
}

impl<'input> serde::de::IntoDeserializer<'input, serde::de::value::Error> for OtherValue {
  type Deserializer = OtherValueReader;

  fn into_deserializer(self) -> OtherValueReader {
    OtherValueReader(self)
  }
}

struct OtherValueReader(OtherValue);

impl<'input> serde::Deserializer<'input> for OtherValueReader {
  type Error = serde::de::value::Error;

  fn deserialize_any<Read: serde::de::Visitor<'input>>(self, visitor: Read) -> Result<Read::Value, Self::Error> {
    match self.0 {
      OtherValue::Text(text) => visitor.visit_str(text),
      OtherValue::Number(number) => visitor.visit_f64(number),
      OtherValue::Bytes(bytes) => visitor.visit_bytes(bytes),
    }
  }

  serde::forward_to_deserialize_any! {
    <Read: Visitor<'input>>
    bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string bytes byte_buf option unit unit_struct
    newtype_struct seq tuple tuple_struct map struct enum identifier ignored_any
  }
}

#[test]
fn a_catch_all_refuses_a_value_that_json_cannot_carry() {
  // The fourteenth review's case, as the maintainer chose: a kind of a newer version read from CBOR with NaN or a
  // byte string, which serde_json would then write as null or as an array of numbers.
  let read = |value: OtherValue| {
    let members = vec![("kind", OtherValue::Text("future")), ("member", value)];
    serde::Deserialize::deserialize(serde::de::value::MapDeserializer::new(members.into_iter()))
      .map(|value: TagValue| value)
  };
  assert!(read(OtherValue::Number(1.5)).is_ok());
  for value in [
    OtherValue::Number(f64::NAN),
    OtherValue::Number(f64::INFINITY),
    OtherValue::Bytes(b"\x01\x02"),
  ] {
    assert!(read(value).is_err());
  }
}

#[test]
fn a_known_kind_reads_its_members_as_they_come() {
  // The nineteenth review's case: with the kind first, the integer's error comes where it stands, before the later
  // null, which it came after while the map was kept whole.
  let invalid = "invalid type: string \"oops\"";
  let error = serde_json::from_str::<TagValue>(r#"{"kind":"integer","integer":"oops","extension":null}"#).unwrap_err();
  assert!(error.to_string().starts_with(invalid), "{error}");
  assert_eq!((error.line(), error.column()), (1, 34));
  // Without the null, the error keeps its place, where the review saw line 0, column 0.
  let error = serde_json::from_str::<TagValue>(r#"{"kind":"integer","integer":"oops"}"#).unwrap_err();
  assert_eq!((error.line(), error.column()), (1, 34));
  // A member before the kind is kept until the kind says how to read it, so its error comes there.
  let error = serde_json::from_str::<TagValue>(r#"{"integer":"oops","kind":"integer"}"#).unwrap_err();
  assert!(error.to_string().starts_with(invalid), "{error}");
  assert_eq!((error.line(), error.column()), (1, 35));
  // A member that the kind does not read is passed by, before the kind as after it; one of another known kind fails.
  let passed = r#"{"extension":[1],"kind":"boolean","boolean":true,"later":{"a":1}}"#;
  assert!(matches!(
    serde_json::from_str::<TagValue>(passed).unwrap(),
    TagValue::Boolean(true)
  ));
  assert!(serde_json::from_str::<TagValue>(r#"{"kind":"boolean","boolean":true,"text":"x"}"#).is_err());
  assert!(serde_json::from_str::<TagValue>(r#"{"text":"x","kind":"boolean","boolean":true}"#).is_err());
  // A chroma format reads `other` as it comes too.
  let other = r#"{"kind":"other","other":{"horizontal":"two","vertical":1},"later":null}"#;
  let error = serde_json::from_str::<ChromaFormat>(other).unwrap_err();
  assert!(error.to_string().starts_with("invalid type: string \"two\""), "{error}");
}

#[test]
fn a_kind_of_a_newer_version_keeps_its_members_before_and_after_its_kind() {
  let value = serde_json::from_str::<TagValue>(r#"{"before":1,"kind":"geolocation","after":2}"#).unwrap();
  assert_eq!(
    serde_json::to_string(&value).unwrap(),
    r#"{"kind":"geolocation","before":1,"after":2}"#
  );
}
