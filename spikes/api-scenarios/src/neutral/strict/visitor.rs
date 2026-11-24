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

//! The visitor of `Strict`: it hands every value to the wrapped visitor, wraps what holds further values, and refuses
//! null, whether the reader gives it as an absent option or as a unit. When it reads a member's name, it also notes
//! the name, so that the map can refuse one given twice.

use std::fmt;

use serde::de::{Deserializer, EnumAccess, Error, MapAccess, SeqAccess, Visitor};

use super::access::{StrictMap, StrictSequence};
use super::deserializer::{Holder, Strict};
use super::variant::StrictEnum;
use crate::constants::{NOT_JSON_VALUE_MESSAGE, NULL_MEMBER_MESSAGE, POSITIONAL_RECORD_MESSAGE};

/// Where a member's name is noted while it is read, for a visitor that reads a name.
pub(super) type NameSlot<'name> = Option<&'name mut Option<String>>;

/// A visitor wrapped so that the values below it are read strictly too.
pub(super) struct StrictVisitor<'name, Read> {
  read: Read,
  name: NameSlot<'name>,
  record: bool,
  holder: Holder,
}

impl<'name, Read> StrictVisitor<'name, Read> {
  /// The visitor of a value, or of a member's name when `name` is given.
  pub(super) fn new(read: Read, name: NameSlot<'name>, holder: Holder) -> StrictVisitor<'name, Read> {
    StrictVisitor {
      read,
      name,
      record: false,
      holder,
    }
  }

  /// The visitor of a record, which a positional array never is (thirteenth review): Serde's derive reads a struct
  /// from a sequence too, so the form refuses one here.
  pub(super) fn record(read: Read, holder: Holder) -> StrictVisitor<'name, Read> {
    StrictVisitor {
      read,
      name: None,
      record: true,
      holder,
    }
  }

  /// Notes a member's name, when this visitor reads one.
  fn note(&mut self, text: &str) {
    if let Some(slot) = self.name.as_mut() {
      **slot = Some(text.to_owned());
    }
  }
}

/// Hands a value that holds no further values to the wrapped visitor as it came.
macro_rules! forward_values {
  ($($method:ident: $value:ty),* $(,)?) => {
    $(
      fn $method<Failure: Error>(self, value: $value) -> Result<Read::Value, Failure> {
        self.read.$method(value)
      }
    )*
  };
}

impl<'input, Read: Visitor<'input>> Visitor<'input> for StrictVisitor<'_, Read> {
  type Value = Read::Value;

  fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    self.read.expecting(formatter)
  }

  forward_values! {
    visit_bool: bool, visit_i8: i8, visit_i16: i16, visit_i32: i32, visit_i64: i64, visit_i128: i128,
    visit_u8: u8, visit_u16: u16, visit_u32: u32, visit_u64: u64, visit_u128: u128, visit_char: char,
  }

  /// A finite number passes; NaN and the infinities have no JSON form (fourteenth review).
  fn visit_f32<Failure: Error>(self, value: f32) -> Result<Read::Value, Failure> {
    if !value.is_finite() {
      return Err(Failure::custom(NOT_JSON_VALUE_MESSAGE));
    }
    self.read.visit_f32(value)
  }

  fn visit_f64<Failure: Error>(self, value: f64) -> Result<Read::Value, Failure> {
    if !value.is_finite() {
      return Err(Failure::custom(NOT_JSON_VALUE_MESSAGE));
    }
    self.read.visit_f64(value)
  }

  /// A byte string has no JSON form: the form writes bytes as hexadecimal text (fourteenth review).
  fn visit_bytes<Failure: Error>(self, _value: &[u8]) -> Result<Read::Value, Failure> {
    Err(Failure::custom(NOT_JSON_VALUE_MESSAGE))
  }

  fn visit_borrowed_bytes<Failure: Error>(self, _value: &'input [u8]) -> Result<Read::Value, Failure> {
    Err(Failure::custom(NOT_JSON_VALUE_MESSAGE))
  }

  fn visit_byte_buf<Failure: Error>(self, _value: Vec<u8>) -> Result<Read::Value, Failure> {
    Err(Failure::custom(NOT_JSON_VALUE_MESSAGE))
  }

  fn visit_str<Failure: Error>(mut self, value: &str) -> Result<Read::Value, Failure> {
    self.note(value);
    self.read.visit_str(value)
  }

  fn visit_borrowed_str<Failure: Error>(mut self, value: &'input str) -> Result<Read::Value, Failure> {
    self.note(value);
    self.read.visit_borrowed_str(value)
  }

  fn visit_string<Failure: Error>(mut self, value: String) -> Result<Read::Value, Failure> {
    self.note(&value);
    self.read.visit_string(value)
  }

  fn visit_none<Failure: Error>(self) -> Result<Read::Value, Failure> {
    Err(Failure::custom(NULL_MEMBER_MESSAGE))
  }

  fn visit_unit<Failure: Error>(self) -> Result<Read::Value, Failure> {
    Err(Failure::custom(NULL_MEMBER_MESSAGE))
  }

  fn visit_some<Inner: Deserializer<'input>>(self, deserializer: Inner) -> Result<Read::Value, Inner::Error> {
    self.read.visit_some(Strict::value(deserializer, self.holder))
  }

  fn visit_newtype_struct<Inner: Deserializer<'input>>(self, deserializer: Inner) -> Result<Read::Value, Inner::Error> {
    self.read.visit_newtype_struct(Strict::value(deserializer, self.holder))
  }

  fn visit_seq<Items: SeqAccess<'input>>(self, items: Items) -> Result<Read::Value, Items::Error> {
    if self.record {
      return Err(Items::Error::custom(POSITIONAL_RECORD_MESSAGE));
    }
    self.read.visit_seq(StrictSequence::new(items, self.holder))
  }

  fn visit_map<Members: MapAccess<'input>>(self, members: Members) -> Result<Read::Value, Members::Error> {
    self.read.visit_map(StrictMap::new(members, self.holder))
  }

  fn visit_enum<Access: EnumAccess<'input>>(self, access: Access) -> Result<Read::Value, Access::Error> {
    self.read.visit_enum(StrictEnum::new(access, self.holder))
  }
}
