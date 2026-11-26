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

//! Reading a kept value into its type, once the kind has said which type that is. The value is read through `Strict`
//! (`super::reading`), which asks for every value with `deserialize_any` and keeps the request of the type, so the
//! reader gives every request what it holds, as the reader of a held value does.

use std::marker::PhantomData;

use serde::de::value::{MapDeserializer, SeqDeserializer};
use serde::de::{Error, IntoDeserializer, Visitor};
use serde::{Deserializer, forward_to_deserialize_any};

use super::value::KeptValue;
/// A kept value as a Serde deserializer, with the error type of the reader that holds it.
pub(crate) struct KeptValueReader<Failure>(KeptValue, PhantomData<Failure>);

impl<Failure: Error> IntoDeserializer<'_, Failure> for KeptValue {
  type Deserializer = KeptValueReader<Failure>;

  fn into_deserializer(self) -> KeptValueReader<Failure> {
    KeptValueReader(self, PhantomData)
  }
}

impl<'input, Failure: Error> Deserializer<'input> for KeptValueReader<Failure> {
  type Error = Failure;

  fn deserialize_any<Read: Visitor<'input>>(self, visitor: Read) -> Result<Read::Value, Failure> {
    match self.0 {
      KeptValue::Boolean(value) => visitor.visit_bool(value),
      KeptValue::Unsigned(value) => visitor.visit_u64(value),
      KeptValue::Signed(value) => visitor.visit_i64(value),
      KeptValue::FloatingPoint(bits) => visitor.visit_f64(f64::from_bits(bits)),
      KeptValue::Text(text) => visitor.visit_string(text),
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

  forward_to_deserialize_any! {
    <Read: Visitor<'input>>
    bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string bytes byte_buf option unit unit_struct
    newtype_struct seq tuple tuple_struct map struct enum identifier ignored_any
  }
}
