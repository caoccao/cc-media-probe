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

//! A member held before the version is known. It takes any value that a self-describing reader gives, null
//! included, and a map keeps every member, a repeated name included, so that nothing can fail to decode before the
//! version error (plan section 8.8). Once the version is checked, a held value is read through `Strict`, in the
//! order written, as a later member is read from the input: a null or a repeated name fails where it stands, so the
//! errors of a record do not depend on where its version stood (thirteenth review). Its depth is bounded where the
//! reader's is, as serde_json's recursion limit bounds it.

use std::fmt;

use serde::de::{MapAccess, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer};

use crate::constants::HELD_VALUE_EXPECTING;

/// A value held as read.
pub(super) enum HeldValue {
  /// Null, refused once the version is checked.
  Null,
  /// A Boolean.
  Boolean(bool),
  /// An integer that is not negative.
  Unsigned(u64),
  /// A negative integer.
  Signed(i64),
  /// A floating-point number.
  FloatingPoint(f64),
  /// A string.
  Text(String),
  /// Bytes, from a format that has them.
  Bytes(Vec<u8>),
  /// A sequence.
  Sequence(Vec<HeldValue>),
  /// A map, its members in the order written.
  Map(Vec<(String, HeldValue)>),
}

impl<'input> Deserialize<'input> for HeldValue {
  fn deserialize<Input: Deserializer<'input>>(deserializer: Input) -> Result<HeldValue, Input::Error> {
    deserializer.deserialize_any(HeldValueVisitor)
  }
}

/// Reads any value, null included.
struct HeldValueVisitor;

impl<'input> Visitor<'input> for HeldValueVisitor {
  type Value = HeldValue;

  fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(HELD_VALUE_EXPECTING)
  }

  fn visit_bool<Failure>(self, value: bool) -> Result<HeldValue, Failure> {
    Ok(HeldValue::Boolean(value))
  }

  fn visit_i64<Failure>(self, value: i64) -> Result<HeldValue, Failure> {
    Ok(match u64::try_from(value) {
      Ok(value) => HeldValue::Unsigned(value),
      Err(_) => HeldValue::Signed(value),
    })
  }

  fn visit_u64<Failure>(self, value: u64) -> Result<HeldValue, Failure> {
    Ok(HeldValue::Unsigned(value))
  }

  fn visit_f64<Failure>(self, value: f64) -> Result<HeldValue, Failure> {
    Ok(HeldValue::FloatingPoint(value))
  }

  fn visit_str<Failure>(self, value: &str) -> Result<HeldValue, Failure> {
    Ok(HeldValue::Text(value.to_owned()))
  }

  fn visit_string<Failure>(self, value: String) -> Result<HeldValue, Failure> {
    Ok(HeldValue::Text(value))
  }

  fn visit_bytes<Failure>(self, value: &[u8]) -> Result<HeldValue, Failure> {
    Ok(HeldValue::Bytes(value.to_vec()))
  }

  fn visit_none<Failure>(self) -> Result<HeldValue, Failure> {
    Ok(HeldValue::Null)
  }

  fn visit_unit<Failure>(self) -> Result<HeldValue, Failure> {
    Ok(HeldValue::Null)
  }

  fn visit_some<Inner: Deserializer<'input>>(self, deserializer: Inner) -> Result<HeldValue, Inner::Error> {
    HeldValue::deserialize(deserializer)
  }

  fn visit_seq<Items: SeqAccess<'input>>(self, mut items: Items) -> Result<HeldValue, Items::Error> {
    let mut values = Vec::new();
    while let Some(value) = items.next_element()? {
      values.push(value);
    }
    Ok(HeldValue::Sequence(values))
  }

  fn visit_map<Members: MapAccess<'input>>(self, mut members: Members) -> Result<HeldValue, Members::Error> {
    let mut held = Vec::new();
    while let Some(member) = members.next_entry()? {
      held.push(member);
    }
    Ok(HeldValue::Map(held))
  }
}
