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

//! Reading a kept value into its type, once the kind has said which type that is. A kept value holds no null, so an
//! optional member read from it is present.

use std::marker::PhantomData;

use serde::de::value::{MapAccessDeserializer, MapDeserializer, SeqDeserializer};
use serde::de::{Error, IntoDeserializer, Visitor};
use serde::{Deserializer, forward_to_deserialize_any};

use super::value::KeptValue;
use crate::constants::POSITIONAL_RECORD_MESSAGE;

/// A kept value as a Serde deserializer, with the error type of the reader that holds it.
pub(crate) struct KeptValueReader<Failure> {
  value: KeptValue,
  failure: PhantomData<Failure>,
}

impl<'input, Failure: Error> IntoDeserializer<'input, Failure> for KeptValue {
  type Deserializer = KeptValueReader<Failure>;

  fn into_deserializer(self) -> KeptValueReader<Failure> {
    KeptValueReader {
      value: self,
      failure: PhantomData,
    }
  }
}

impl<'input, Failure: Error> Deserializer<'input> for KeptValueReader<Failure> {
  type Error = Failure;

  fn deserialize_any<Read: Visitor<'input>>(self, visitor: Read) -> Result<Read::Value, Failure> {
    match self.value {
      KeptValue::Boolean(value) => visitor.visit_bool(value),
      KeptValue::Unsigned(value) => visitor.visit_u64(value),
      KeptValue::Signed(value) => visitor.visit_i64(value),
      KeptValue::FloatingPoint(bits) => visitor.visit_f64(f64::from_bits(bits)),
      KeptValue::Text(text) => visitor.visit_string(text),
      KeptValue::Bytes(bytes) => visitor.visit_byte_buf(bytes),
      KeptValue::Sequence(items) => {
        let mut sequence = SeqDeserializer::new(items.into_iter());
        let value = visitor.visit_seq(&mut sequence)?;
        sequence.end()?;
        Ok(value)
      }
      KeptValue::Map(members) => {
        let mut map = MapDeserializer::new(members.into_iter());
        let value = visitor.visit_map(&mut map)?;
        map.end()?;
        Ok(value)
      }
    }
  }

  fn deserialize_option<Read: Visitor<'input>>(self, visitor: Read) -> Result<Read::Value, Failure> {
    visitor.visit_some(self)
  }

  fn deserialize_newtype_struct<Read: Visitor<'input>>(
    self,
    _name: &'static str,
    visitor: Read,
  ) -> Result<Read::Value, Failure> {
    visitor.visit_newtype_struct(self)
  }

  /// A record is a map; Serde's derive would read one from a sequence too, which the form never writes.
  fn deserialize_struct<Read: Visitor<'input>>(
    self,
    _name: &'static str,
    _fields: &'static [&'static str],
    visitor: Read,
  ) -> Result<Read::Value, Failure> {
    match self.value {
      KeptValue::Sequence(_) => Err(Failure::custom(POSITIONAL_RECORD_MESSAGE)),
      value => KeptValueReader::<Failure> {
        value,
        failure: PhantomData,
      }
      .deserialize_any(visitor),
    }
  }

  fn deserialize_enum<Read: Visitor<'input>>(
    self,
    _name: &'static str,
    _variants: &'static [&'static str],
    visitor: Read,
  ) -> Result<Read::Value, Failure> {
    match self.value {
      KeptValue::Text(text) => visitor.visit_enum(text.into_deserializer()),
      // A variant that holds a value, as Serde's derive writes it by default: a map of the variant's name.
      KeptValue::Map(members) => {
        visitor.visit_enum(MapAccessDeserializer::new(MapDeserializer::new(members.into_iter())))
      }
      value => KeptValueReader::<Failure> {
        value,
        failure: PhantomData,
      }
      .deserialize_any(visitor),
    }
  }

  forward_to_deserialize_any! {
    <Read: Visitor<'input>>
    bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string bytes byte_buf unit unit_struct seq tuple
    tuple_struct map identifier ignored_any
  }
}
