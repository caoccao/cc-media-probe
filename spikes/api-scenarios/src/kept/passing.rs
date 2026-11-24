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

//! The value of a member that a record of an outcome may pass by (`super::outcome`): every request goes to the
//! deserializer below, except the one for Serde's `IgnoredAny`, by which the record's derive passes by a member that
//! it does not know. That member is held while the outcome is not known yet and kept for an outcome of a newer
//! version, and it is walked as it comes for an outcome that this version knows, so it is never held then.

use serde::de::{DeserializeSeed, Deserializer, Visitor};

use super::outcome::Decision;
use super::value::KeptValue;

/// The seed of such a member's value.
pub(super) struct Passing<'held, Seed> {
  seed: Seed,
  name: String,
  decision: Decision,
  held: &'held mut Vec<(String, KeptValue)>,
}

impl<'held, Seed> Passing<'held, Seed> {
  pub(super) fn new(
    seed: Seed,
    name: String,
    decision: Decision,
    held: &'held mut Vec<(String, KeptValue)>,
  ) -> Passing<'held, Seed> {
    Passing {
      seed,
      name,
      decision,
      held,
    }
  }
}

impl<'input, Seed: DeserializeSeed<'input>> DeserializeSeed<'input> for Passing<'_, Seed> {
  type Value = Seed::Value;

  fn deserialize<Inner: Deserializer<'input>>(self, deserializer: Inner) -> Result<Seed::Value, Inner::Error> {
    self.seed.deserialize(PassingValue {
      inner: deserializer,
      name: self.name,
      decision: self.decision,
      held: self.held,
    })
  }
}

/// The member's value as a deserializer.
struct PassingValue<'held, Inner> {
  inner: Inner,
  name: String,
  decision: Decision,
  held: &'held mut Vec<(String, KeptValue)>,
}

/// Hands every request other than `IgnoredAny`'s to the deserializer below.
macro_rules! forward_requests {
  ($($method:ident),* $(,)?) => {
    $(
      fn $method<Read: Visitor<'input>>(self, visitor: Read) -> Result<Read::Value, Inner::Error> {
        self.inner.$method(visitor)
      }
    )*
  };
}

impl<'input, Inner: Deserializer<'input>> Deserializer<'input> for PassingValue<'_, Inner> {
  type Error = Inner::Error;

  forward_requests! {
    deserialize_any, deserialize_bool, deserialize_i8, deserialize_i16, deserialize_i32, deserialize_i64,
    deserialize_i128, deserialize_u8, deserialize_u16, deserialize_u32, deserialize_u64, deserialize_u128,
    deserialize_f32, deserialize_f64, deserialize_char, deserialize_str, deserialize_string, deserialize_bytes,
    deserialize_byte_buf, deserialize_option, deserialize_unit, deserialize_seq, deserialize_map,
    deserialize_identifier,
  }

  /// A member that the record does not know: passed by for a known outcome, held or kept otherwise.
  fn deserialize_ignored_any<Read: Visitor<'input>>(self, visitor: Read) -> Result<Read::Value, Inner::Error> {
    if self.decision == Decision::Known {
      return self.inner.deserialize_ignored_any(visitor);
    }
    let value = <KeptValue as serde::Deserialize>::deserialize(self.inner)?;
    self.held.push((self.name, value));
    visitor.visit_unit()
  }

  fn deserialize_unit_struct<Read: Visitor<'input>>(
    self,
    name: &'static str,
    visitor: Read,
  ) -> Result<Read::Value, Inner::Error> {
    self.inner.deserialize_unit_struct(name, visitor)
  }

  fn deserialize_newtype_struct<Read: Visitor<'input>>(
    self,
    name: &'static str,
    visitor: Read,
  ) -> Result<Read::Value, Inner::Error> {
    self.inner.deserialize_newtype_struct(name, visitor)
  }

  fn deserialize_tuple<Read: Visitor<'input>>(self, length: usize, visitor: Read) -> Result<Read::Value, Inner::Error> {
    self.inner.deserialize_tuple(length, visitor)
  }

  fn deserialize_tuple_struct<Read: Visitor<'input>>(
    self,
    name: &'static str,
    length: usize,
    visitor: Read,
  ) -> Result<Read::Value, Inner::Error> {
    self.inner.deserialize_tuple_struct(name, length, visitor)
  }

  fn deserialize_struct<Read: Visitor<'input>>(
    self,
    name: &'static str,
    fields: &'static [&'static str],
    visitor: Read,
  ) -> Result<Read::Value, Inner::Error> {
    self.inner.deserialize_struct(name, fields, visitor)
  }

  fn deserialize_enum<Read: Visitor<'input>>(
    self,
    name: &'static str,
    variants: &'static [&'static str],
    visitor: Read,
  ) -> Result<Read::Value, Inner::Error> {
    self.inner.deserialize_enum(name, variants, visitor)
  }

  fn is_human_readable(&self) -> bool {
    self.inner.is_human_readable()
  }
}
