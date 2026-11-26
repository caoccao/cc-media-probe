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

//! One value kept as written, read through the self-describing side of Serde. Null is not among the values: the form
//! never writes it, so a member that holds null fails as a value of the wrong type, kept or not. Nor is any value that
//! JSON cannot carry faithfully, a non-finite number or a byte string from a format that has them, as the maintainer
//! chose at the fourteenth review, so a catch-all forwards from any format only what JSON writes back as read; since
//! the sixteenth review, nor a tagged value or an integer beyond 64 bits, as ciborium gives them. A map is kept as
//! written, the private map of `serde_json`'s `arbitrary_precision` feature included, until the type that reads it is
//! known, as the eighteenth review asked: the members of a known kind and of a tree's node are then read into their
//! types, and the members that a catch-all keeps become its payload, where that map is the number it stands for
//! (`super::payload`).

use std::fmt;

use serde::de::{EnumAccess, Error, MapAccess, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::members::KeptMembers;
use crate::form::constants::{KEPT_VALUE_EXPECTING, NOT_JSON_VALUE_MESSAGE};
use crate::form::numbers::{Narrowed, narrow_signed, narrow_unsigned};

/// A value kept as written. A floating-point number is held by its bits, so equality and hashing are exact.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) enum KeptValue {
  /// A Boolean.
  Boolean(bool),
  /// An integer that is not negative.
  Unsigned(u64),
  /// A negative integer.
  Signed(i64),
  /// A floating-point number, by the bits of its `f64`.
  FloatingPoint(u64),
  /// A string.
  Text(String),
  /// A sequence.
  Sequence(Vec<KeptValue>),
  /// A map, its members in the order written.
  Map(KeptMembers),
}

impl Serialize for KeptValue {
  fn serialize<Output: Serializer>(&self, serializer: Output) -> Result<Output::Ok, Output::Error> {
    match self {
      KeptValue::Boolean(value) => serializer.serialize_bool(*value),
      KeptValue::Unsigned(value) => serializer.serialize_u64(*value),
      KeptValue::Signed(value) => serializer.serialize_i64(*value),
      KeptValue::FloatingPoint(bits) => serializer.serialize_f64(f64::from_bits(*bits)),
      KeptValue::Text(text) => serializer.serialize_str(text),
      KeptValue::Sequence(items) => serializer.collect_seq(items),
      KeptValue::Map(members) => members.serialize(serializer),
    }
  }
}

impl<'input> Deserialize<'input> for KeptValue {
  fn deserialize<Input: Deserializer<'input>>(deserializer: Input) -> Result<KeptValue, Input::Error> {
    deserializer.deserialize_any(KeptValueVisitor)
  }
}

/// Reads any value but null. Neither `visit_unit` nor `visit_none` is implemented, so null is the wrong type.
struct KeptValueVisitor;

impl<'input> Visitor<'input> for KeptValueVisitor {
  type Value = KeptValue;

  fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(KEPT_VALUE_EXPECTING)
  }

  fn visit_bool<Failure>(self, value: bool) -> Result<KeptValue, Failure> {
    Ok(KeptValue::Boolean(value))
  }

  fn visit_i64<Failure: Error>(self, value: i64) -> Result<KeptValue, Failure> {
    kept_integer(narrow_signed(i128::from(value)))
  }

  fn visit_u64<Failure>(self, value: u64) -> Result<KeptValue, Failure> {
    Ok(KeptValue::Unsigned(value))
  }

  /// An integer given as 128 bits is the integer within 64 bits; beyond them, such as a CBOR bignum, it has no JSON
  /// number within the form (sixteenth and nineteenth reviews).
  fn visit_i128<Failure: Error>(self, value: i128) -> Result<KeptValue, Failure> {
    kept_integer(narrow_signed(value))
  }

  fn visit_u128<Failure: Error>(self, value: u128) -> Result<KeptValue, Failure> {
    kept_integer(narrow_unsigned(value))
  }

  /// A finite number; NaN and the infinities have no JSON form, as the maintainer chose at the fourteenth review.
  fn visit_f64<Failure: Error>(self, value: f64) -> Result<KeptValue, Failure> {
    if !value.is_finite() {
      return Err(Failure::custom(NOT_JSON_VALUE_MESSAGE));
    }
    Ok(KeptValue::FloatingPoint(value.to_bits()))
  }

  fn visit_str<Failure>(self, value: &str) -> Result<KeptValue, Failure> {
    Ok(KeptValue::Text(value.to_owned()))
  }

  fn visit_string<Failure>(self, value: String) -> Result<KeptValue, Failure> {
    Ok(KeptValue::Text(value))
  }

  /// A byte string has no JSON form: the form writes bytes as hexadecimal text (fourteenth review).
  fn visit_bytes<Failure: Error>(self, _value: &[u8]) -> Result<KeptValue, Failure> {
    Err(Failure::custom(NOT_JSON_VALUE_MESSAGE))
  }

  fn visit_seq<Items: SeqAccess<'input>>(self, items: Items) -> Result<KeptValue, Items::Error> {
    crate::form::items::collect_items(items).map(KeptValue::Sequence)
  }

  /// A tagged value, which ciborium gives as an enum of one value, has no JSON form (sixteenth review).
  fn visit_enum<Access: EnumAccess<'input>>(self, _access: Access) -> Result<KeptValue, Access::Error> {
    Err(Access::Error::custom(NOT_JSON_VALUE_MESSAGE))
  }

  /// A map, as written: the private map of `serde_json`'s `arbitrary_precision` feature too, until a type reads it.
  fn visit_map<Members: MapAccess<'input>>(self, members: Members) -> Result<KeptValue, Members::Error> {
    KeptMembers::from_map(members).map(KeptValue::Map)
  }
}

/// An integer within 64 bits, or, beyond them, a value that JSON does not carry.
fn kept_integer<Failure: Error>(narrowed: Option<Narrowed>) -> Result<KeptValue, Failure> {
  match narrowed {
    Some(Narrowed::Unsigned(value)) => Ok(KeptValue::Unsigned(value)),
    Some(Narrowed::Signed(value)) => Ok(KeptValue::Signed(value)),
    None => Err(Failure::custom(NOT_JSON_VALUE_MESSAGE)),
  }
}

#[cfg(test)]
mod tests {
  use serde::Deserialize;
  use serde::de::IntoDeserializer;
  use serde::de::value::{BytesDeserializer, EnumAccessDeserializer, Error, StrDeserializer};

  use super::KeptValue;

  fn kept<'input, Input: serde::Deserializer<'input, Error = Error>>(input: Input) -> Option<KeptValue> {
    KeptValue::deserialize(input).ok()
  }

  #[test]
  fn keeps_every_value_that_json_carries_and_no_other() {
    assert_eq!(kept(true.into_deserializer()), Some(KeptValue::Boolean(true)));
    assert_eq!(kept((-1_i64).into_deserializer()), Some(KeptValue::Signed(-1)));
    assert_eq!(kept(1_i64.into_deserializer()), Some(KeptValue::Unsigned(1)));
    assert_eq!(kept((-1_i128).into_deserializer()), Some(KeptValue::Signed(-1)));
    assert_eq!(kept(1_u128.into_deserializer()), Some(KeptValue::Unsigned(1)));
    assert_eq!(kept((1_u128 << 64).into_deserializer()), None);
    assert_eq!(kept(f64::NAN.into_deserializer()), None);
    assert_eq!(kept(StrDeserializer::new("x")), Some(KeptValue::Text("x".to_owned())));
    assert_eq!(kept(BytesDeserializer::new(b"x")), None);
    assert_eq!(kept(EnumAccessDeserializer::new(StrDeserializer::new("x"))), None);
    let floating = kept(1.5_f64.into_deserializer());
    assert_eq!(floating, Some(KeptValue::FloatingPoint(1.5_f64.to_bits())));
    assert_eq!(
      serde_json::to_value(&floating).ok(),
      Some(serde_json::json!([1.5])).and_then(|value| value.get(0).cloned())
    );
    let signed = serde_json::to_value(KeptValue::Signed(-3)).ok();
    assert_eq!(signed, Some(serde_json::json!(-3)));
  }
}
