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
  /// The seed of the value that the record asks for.
  pub(super) seed: Seed,
  /// The member's name.
  pub(super) name: String,
  /// What the outcome has decided so far.
  pub(super) decision: Decision,
  /// The members held while the outcome is not known, or kept for an outcome of a newer version.
  pub(super) held: &'held mut Vec<(String, KeptValue)>,
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

/// Hands every request with arguments of its own to the deserializer below.
macro_rules! forward_with_arguments {
  ($($method:ident($($argument:ident: $type:ty),*)),* $(,)?) => {
    $(
      fn $method<Read: Visitor<'input>>(self, $($argument: $type,)* visitor: Read) -> Result<Read::Value, Inner::Error> {
        self.inner.$method($($argument,)* visitor)
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

  forward_with_arguments! {
    deserialize_unit_struct(name: &'static str),
    deserialize_newtype_struct(name: &'static str),
    deserialize_tuple(length: usize),
    deserialize_tuple_struct(name: &'static str, length: usize),
    deserialize_struct(name: &'static str, fields: &'static [&'static str]),
    deserialize_enum(name: &'static str, variants: &'static [&'static str]),
  }

  fn is_human_readable(&self) -> bool {
    self.inner.is_human_readable()
  }
}

#[cfg(test)]
mod tests {
  use std::marker::PhantomData;

  use serde::Deserialize;
  use serde::de::{DeserializeSeed, IgnoredAny};
  use serde_json::json;

  use super::Passing;
  use crate::form::kept::outcome::Decision;

  /// The value of that text, read through a member that a record of a known outcome reads.
  fn read<Value: for<'input> Deserialize<'input>>(value: serde_json::Value) -> Option<Value> {
    let mut held = Vec::new();
    let seed = Passing {
      seed: PhantomData::<Value>,
      name: "member".to_owned(),
      decision: Decision::Known,
      held: &mut held,
    };
    seed.deserialize(value).ok()
  }

  #[derive(Deserialize, Debug, PartialEq)]
  struct Unit;

  #[derive(Deserialize, Debug, PartialEq)]
  struct Pair(u8, u8);

  #[derive(Deserialize, Debug, PartialEq)]
  struct Record {
    value: u8,
  }

  #[derive(Deserialize, Debug, PartialEq)]
  enum Choice {
    First,
  }

  #[test]
  fn hands_every_request_but_ignoring_to_the_value_below() {
    assert_eq!(read::<(u8, u8)>(json!([1, 2])), Some((1, 2)));
    assert_eq!(read::<Pair>(json!([1, 2])), Some(Pair(1, 2)));
    assert_eq!(read::<Unit>(json!(null)), Some(Unit));
    assert_eq!(read::<Record>(json!({ "value": 1 })), Some(Record { value: 1 }));
    assert_eq!(read::<Choice>(json!("First")), Some(Choice::First));
    assert_eq!(read::<IgnoredAny>(json!([1])), Some(IgnoredAny));
    let mut held = Vec::new();
    let seed = Passing {
      seed: PhantomData::<IgnoredAny>,
      name: "later".to_owned(),
      decision: Decision::Newer,
      held: &mut held,
    };
    let kept = seed.deserialize(json!(7));
    assert!(kept.is_ok());
    assert_eq!(held.len(), 1);
  }
}
