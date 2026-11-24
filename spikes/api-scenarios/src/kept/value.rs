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
//! chose at the fourteenth review, so a catch-all forwards from any format only what JSON writes back as read.

use std::fmt;

use serde::de::{Error, MapAccess, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::members::KeptMembers;
use crate::constants::{KEPT_VALUE_EXPECTING, NOT_JSON_VALUE_MESSAGE};

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

  fn visit_i64<Failure>(self, value: i64) -> Result<KeptValue, Failure> {
    Ok(match u64::try_from(value) {
      Ok(value) => KeptValue::Unsigned(value),
      Err(_) => KeptValue::Signed(value),
    })
  }

  fn visit_u64<Failure>(self, value: u64) -> Result<KeptValue, Failure> {
    Ok(KeptValue::Unsigned(value))
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

  fn visit_seq<Items: SeqAccess<'input>>(self, mut items: Items) -> Result<KeptValue, Items::Error> {
    let mut values = Vec::new();
    while let Some(value) = items.next_element()? {
      values.push(value);
    }
    Ok(KeptValue::Sequence(values))
  }

  fn visit_map<Members: MapAccess<'input>>(self, members: Members) -> Result<KeptValue, Members::Error> {
    KeptMembers::from_map(members).map(KeptValue::Map)
  }
}
