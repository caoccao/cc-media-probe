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

//! A small self-describing reader of the tests' own, which gives values that JSON does not carry through the same
//! Serde calls as ciborium 0.2.2 gives them (`json-output.md` section 2): a map whose names are integers, a tagged
//! value as an enum of one value, an integer beyond 64 bits as 128 bits, a byte string and a non-finite number, and a
//! character, which ciborium never gives but a format of characters does. As
//! ciborium does, it drops a tag when a type asks for a value of its own, and gives it only to `deserialize_any`.
//! The maintainer chose these simulated values over a dependency on ciborium (third slice of Phase 2, task 11).

use serde::de::value::{
  Error as ValueError, MapAccessDeserializer, MapDeserializer, SeqDeserializer, StringDeserializer,
};
use serde::de::{DeserializeSeed, EnumAccess, Error, IntoDeserializer, MapAccess, VariantAccess, Visitor};
use serde::{Deserializer, forward_to_deserialize_any};
use serde_json::Value;

/// A value of a self-describing format whose map names may be any value.
#[derive(Clone, Debug)]
pub enum Tree {
  Text(String),
  Integer(u64),
  Negative(i64),
  Number(f64),
  Boolean(bool),
  Sequence(Vec<Tree>),
  Map(Vec<(Tree, Tree)>),
  /// A tagged value, which ciborium gives as an enum of one value.
  Tagged(Box<Tree>),
  /// An integer that the reader gives as 128 bits, such as one that ciborium gives from a bignum.
  Wide(u128),
  /// A byte string.
  Bytes(Vec<u8>),
  /// A character, which a format of characters gives apart from a string.
  Character(char),
  /// A unit variant of that name, which Serde's own deserializer of a string gives as an enum, as its
  /// `EnumAccessDeserializer` does.
  UnitVariant(String),
  /// Input that the reader fails to read with that message, as ciborium fails on text that is not UTF-8.
  Broken(String),
  /// A map of one name whose reader fails on the missing colon before it gives the name's value, as `serde_json`
  /// fails on `{"name"}`, given as an enum through Serde's `MapAccessDeserializer`, as the fourth review of the third
  /// slice's first part gave it.
  Colonless(String),
}

impl Tree {
  /// The tree of a JSON value.
  pub fn of(value: &Value) -> Tree {
    match value {
      Value::Null => Tree::Sequence(Vec::new()),
      Value::Bool(value) => Tree::Boolean(*value),
      Value::Number(number) => match (number.as_u64(), number.as_i64()) {
        (Some(value), _) => Tree::Integer(value),
        (None, Some(value)) => Tree::Negative(value),
        _ => Tree::Number(number.as_f64().unwrap_or_default()),
      },
      Value::String(text) => Tree::Text(text.clone()),
      Value::Array(items) => Tree::Sequence(items.iter().map(Tree::of).collect()),
      Value::Object(members) => Tree::Map(
        members
          .iter()
          .map(|(name, value)| (Tree::Text(name.clone()), Tree::of(value)))
          .collect(),
      ),
    }
  }

  /// The map with one more member before its others or after them.
  pub fn with(self, name: Tree, value: Tree, first: bool) -> Tree {
    match self {
      Tree::Map(mut members) => {
        members.retain(|(kept, _)| !matches!((kept, &name), (Tree::Text(kept), Tree::Text(name)) if kept == name));
        if first {
          members.insert(0, (name, value));
        } else {
          members.push((name, value));
        }
        Tree::Map(members)
      }
      other => other,
    }
  }
}

/// A member name that is text.
pub fn name(text: &str) -> Tree {
  Tree::Text(text.to_owned())
}

/// Requests of a value of a known type, which drop a tag around the value, as ciborium does for every request but
/// `deserialize_any`.
macro_rules! untagged {
  ($($method:ident),* $(,)?) => {
    $(
      fn $method<Read: Visitor<'input>>(self, visitor: Read) -> Result<Read::Value, ValueError> {
        match self {
          Tree::Tagged(content) => content.$method(visitor),
          other => other.deserialize_any(visitor),
        }
      }
    )*
  };
}

impl<'input> Deserializer<'input> for Tree {
  type Error = ValueError;

  fn deserialize_any<Read: Visitor<'input>>(self, visitor: Read) -> Result<Read::Value, ValueError> {
    match self {
      Tree::Text(text) => visitor.visit_string(text),
      Tree::Integer(value) => visitor.visit_u64(value),
      Tree::Negative(value) => visitor.visit_i64(value),
      Tree::Number(value) => visitor.visit_f64(value),
      Tree::Boolean(value) => visitor.visit_bool(value),
      Tree::Sequence(items) => visitor.visit_seq(SeqDeserializer::new(items.into_iter())),
      Tree::Map(members) => visitor.visit_map(MapDeserializer::new(members.into_iter())),
      Tree::Tagged(content) => visitor.visit_enum(Tag(*content)),
      Tree::Wide(value) => visitor.visit_u128(value),
      Tree::Bytes(bytes) => visitor.visit_byte_buf(bytes),
      Tree::Character(value) => visitor.visit_char(value),
      Tree::UnitVariant(name) => visitor.visit_enum(StringDeserializer::<ValueError>::new(name)),
      Tree::Broken(message) => Err(ValueError::custom(message)),
      Tree::Colonless(name) => visitor.visit_enum(MapAccessDeserializer::new(Colonless(Some(name)))),
    }
  }

  fn deserialize_option<Read: Visitor<'input>>(self, visitor: Read) -> Result<Read::Value, ValueError> {
    visitor.visit_some(self)
  }

  fn deserialize_newtype_struct<Read: Visitor<'input>>(
    self,
    _name: &'static str,
    visitor: Read,
  ) -> Result<Read::Value, ValueError> {
    visitor.visit_newtype_struct(self)
  }

  fn deserialize_struct<Read: Visitor<'input>>(
    self,
    name: &'static str,
    fields: &'static [&'static str],
    visitor: Read,
  ) -> Result<Read::Value, ValueError> {
    match self {
      Tree::Tagged(content) => content.deserialize_struct(name, fields, visitor),
      other => other.deserialize_any(visitor),
    }
  }

  untagged! {
    deserialize_bool, deserialize_i8, deserialize_i16, deserialize_i32, deserialize_i64, deserialize_u8,
    deserialize_u16, deserialize_u32, deserialize_u64, deserialize_f32, deserialize_f64, deserialize_char,
    deserialize_str, deserialize_string, deserialize_seq, deserialize_map, deserialize_identifier,
  }

  forward_to_deserialize_any! {
    <Read: Visitor<'input>>
    i128 u128 bytes byte_buf unit unit_struct tuple tuple_struct enum ignored_any
  }
}

/// The enum of one value by which ciborium gives a tagged value: a variant of a reserved name and the value.
struct Tag(Tree);

impl<'input> EnumAccess<'input> for Tag {
  type Error = ValueError;
  type Variant = Tag;

  fn variant_seed<Seed: DeserializeSeed<'input>>(self, seed: Seed) -> Result<(Seed::Value, Tag), ValueError> {
    Ok((seed.deserialize(name("@@TAGGED@@"))?, self))
  }
}

impl<'input> VariantAccess<'input> for Tag {
  type Error = ValueError;

  fn unit_variant(self) -> Result<(), ValueError> {
    Err(ValueError::custom("expected tag"))
  }

  fn newtype_variant_seed<Seed: DeserializeSeed<'input>>(self, seed: Seed) -> Result<Seed::Value, ValueError> {
    seed.deserialize(self.0)
  }

  fn tuple_variant<Read: Visitor<'input>>(self, _length: usize, _visitor: Read) -> Result<Read::Value, ValueError> {
    Err(ValueError::custom("expected tag"))
  }

  fn struct_variant<Read: Visitor<'input>>(
    self,
    _fields: &'static [&'static str],
    _visitor: Read,
  ) -> Result<Read::Value, ValueError> {
    Err(ValueError::custom("expected tag"))
  }
}

/// The map of `Tree::Colonless`: its one name, and then the reader's failure where the colon should stand.
struct Colonless(Option<String>);

impl<'input> MapAccess<'input> for Colonless {
  type Error = ValueError;

  fn next_key_seed<Seed: DeserializeSeed<'input>>(&mut self, seed: Seed) -> Result<Option<Seed::Value>, ValueError> {
    self
      .0
      .take()
      .map(|name| seed.deserialize(StringDeserializer::<ValueError>::new(name)))
      .transpose()
  }

  fn next_value_seed<Seed: DeserializeSeed<'input>>(&mut self, _seed: Seed) -> Result<Seed::Value, ValueError> {
    Err(ValueError::custom("expected `:`"))
  }
}

impl IntoDeserializer<'_, ValueError> for Tree {
  type Deserializer = Tree;

  fn into_deserializer(self) -> Tree {
    self
  }
}
