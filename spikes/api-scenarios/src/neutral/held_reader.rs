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

//! Reading a held value into its type once the version is checked. The value is given as it was read, null and
//! repeated names included; the root record reads it through `Strict`, which refuses them where they stand, as it
//! does for a member after the version. So a nested root record held before its parent's version reads its own
//! version first, exactly as it would from the input (thirteenth review). A value that JSON does not carry fails as
//! soon as it is decoded (fifteenth review).

use std::marker::PhantomData;

use serde::de::value::{MapAccessDeserializer, MapDeserializer, SeqDeserializer};
use serde::de::{Error, IntoDeserializer, Visitor};
use serde::{Deserializer, forward_to_deserialize_any};

use super::held::HeldValue;
use crate::constants::NOT_JSON_VALUE_MESSAGE;

/// A held value as a Serde deserializer, with the error type of the reader that held it.
pub(super) struct HeldValueReader<Failure> {
  value: HeldValue,
  failure: PhantomData<Failure>,
}

impl<'input, Failure: Error> IntoDeserializer<'input, Failure> for HeldValue {
  type Deserializer = HeldValueReader<Failure>;

  fn into_deserializer(self) -> HeldValueReader<Failure> {
    HeldValueReader {
      value: self,
      failure: PhantomData,
    }
  }
}

impl<'input, Failure: Error> Deserializer<'input> for HeldValueReader<Failure> {
  type Error = Failure;

  fn deserialize_any<Read: Visitor<'input>>(self, visitor: Read) -> Result<Read::Value, Failure> {
    match self.value {
      HeldValue::Null => visitor.visit_unit(),
      HeldValue::Boolean(value) => visitor.visit_bool(value),
      HeldValue::Unsigned(value) => visitor.visit_u64(value),
      HeldValue::Signed(value) => visitor.visit_i64(value),
      HeldValue::FloatingPoint(value) => visitor.visit_f64(value),
      HeldValue::Text(text) => visitor.visit_string(text),
      HeldValue::Sequence(items) => {
        let mut sequence = SeqDeserializer::new(items.into_iter());
        let value = visitor.visit_seq(&mut sequence)?;
        sequence.end()?;
        Ok(value)
      }
      HeldValue::Map(members) => {
        let mut map = MapDeserializer::new(members.into_iter());
        let value = visitor.visit_map(&mut map)?;
        map.end()?;
        Ok(value)
      }
      HeldValue::NotJson => Err(Failure::custom(NOT_JSON_VALUE_MESSAGE)),
    }
  }

  /// Null is an absent option here, as a self-describing reader gives it; `Strict` refuses it either way.
  fn deserialize_option<Read: Visitor<'input>>(self, visitor: Read) -> Result<Read::Value, Failure> {
    match self.value {
      HeldValue::Null => visitor.visit_none(),
      _ => visitor.visit_some(self),
    }
  }

  fn deserialize_newtype_struct<Read: Visitor<'input>>(
    self,
    _name: &'static str,
    visitor: Read,
  ) -> Result<Read::Value, Failure> {
    visitor.visit_newtype_struct(self)
  }

  fn deserialize_enum<Read: Visitor<'input>>(
    self,
    _name: &'static str,
    _variants: &'static [&'static str],
    visitor: Read,
  ) -> Result<Read::Value, Failure> {
    match self.value {
      HeldValue::Text(text) => visitor.visit_enum(text.into_deserializer()),
      // A variant that holds a value, as Serde's derive writes it by default: a map of the variant's name.
      HeldValue::Map(members) => {
        visitor.visit_enum(MapAccessDeserializer::new(MapDeserializer::new(members.into_iter())))
      }
      value => HeldValueReader::<Failure> {
        value,
        failure: PhantomData,
      }
      .deserialize_any(visitor),
    }
  }

  forward_to_deserialize_any! {
    <Read: Visitor<'input>>
    bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string bytes byte_buf unit unit_struct seq tuple
    tuple_struct map struct identifier ignored_any
  }
}
