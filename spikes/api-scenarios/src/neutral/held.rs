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
//!
//! A value that JSON does not carry is held too, as the fifteenth review asked, so that it fails after the version
//! error rather than before it: a byte string, and a map with a name that is not a string, such as the integer keys
//! of a CBOR map, which ciborium reads. Once the version is checked, it fails as malformed input where it stands. The
//! private map of serde_json's `arbitrary_precision` feature is held as the number it stands for (`number_text`).

use std::fmt;

use serde::de::{MapAccess, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer};

use crate::constants::{ARBITRARY_PRECISION_NAME, HELD_VALUE_EXPECTING};
use crate::number_text::{self, NumberText};

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
  /// A sequence.
  Sequence(Vec<HeldValue>),
  /// A map, its members in the order written.
  Map(Vec<(String, HeldValue)>),
  /// A value that JSON does not carry, refused once the version is checked: a byte string, or a map with a name that
  /// is not a string.
  NotJson,
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

  fn visit_bytes<Failure>(self, _value: &[u8]) -> Result<HeldValue, Failure> {
    Ok(HeldValue::NotJson)
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

  /// A map, whose names are read as any value, so that a name that is not a string is held as well.
  fn visit_map<Members: MapAccess<'input>>(self, mut members: Members) -> Result<HeldValue, Members::Error> {
    let mut held = Vec::new();
    let mut textual = true;
    while let Some((name, value)) = members.next_entry::<HeldValue, HeldValue>()? {
      match name {
        HeldValue::Text(name) => held.push((name, value)),
        _ => textual = false,
      }
    }
    if !textual {
      return Ok(HeldValue::NotJson);
    }
    Ok(HeldValue::number_of(&held).unwrap_or(HeldValue::Map(held)))
  }
}

impl HeldValue {
  /// The number that the private map of serde_json's `arbitrary_precision` feature stands for, if it is one.
  fn number_of(members: &[(String, HeldValue)]) -> Option<HeldValue> {
    let [(name, HeldValue::Text(text))] = members else {
      return None;
    };
    if name != ARBITRARY_PRECISION_NAME {
      return None;
    }
    number_text::parse(text).map(|number| match number {
      NumberText::Unsigned(value) => HeldValue::Unsigned(value),
      NumberText::Signed(value) => HeldValue::Signed(value),
      NumberText::FloatingPoint(value) => HeldValue::FloatingPoint(value),
    })
  }
}
