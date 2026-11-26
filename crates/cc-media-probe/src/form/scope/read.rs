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

//! The scope of a read of the form, as the fourteenth review of `api.md` asked: whether a read is under way, and the
//! version of the root record whose body is being decoded. Serde's derive passes nothing from a record to the records
//! it holds, so a value asks the deserializer that it was given, with a newtype request of a reserved name. The
//! library's strict deserializer answers with an enum whose identifier is a private static, compared by address, and
//! whose variant carries the version and the strict deserializer itself; every format gives the plain newtype, which
//! says that no read of the form is under way. The scope therefore belongs to the deserializer, never to the thread:
//! an independent read, even one started inside a reader's callback while another read waits for its bytes, has a
//! scope of its own.
//!
//! Serde's own value deserializers, such as `MapDeserializer`, `SeqDeserializer` and `StringDeserializer`, answer a
//! newtype request with the value itself, as `deserialize_any` gives it, so every value given in answer is read
//! alone, from that value: a map, a sequence, a string, a number, a Boolean, and a null, which then fails as null
//! does. A sequence is always such a value, and an enum is the strict answer only when its identifier is the static
//! itself; any other enum, such as one that `EnumAccessDeserializer` gives, is a value that JSON does not carry, as it
//! is wherever `Strict` meets one (nineteenth review).

use std::fmt;
use std::marker::PhantomData;

use serde::Deserializer;
use serde::de::value::{
  BorrowedBytesDeserializer, BorrowedStrDeserializer, BytesDeserializer, MapAccessDeserializer, SeqAccessDeserializer,
  StrDeserializer, StringDeserializer, UnitDeserializer,
};
use serde::de::{DeserializeSeed, EnumAccess, Error, IntoDeserializer, MapAccess, SeqAccess, VariantAccess, Visitor};

use super::identifier::IdentifierSeed;
use crate::document::SchemaVersion;
use crate::form::constants::{
  NOT_JSON_VALUE_MESSAGE, READ_SCOPE_EXPECTING, READ_SCOPE_LENGTH, READ_SCOPE_MESSAGE, READ_SCOPE_NAME,
};
use crate::form::strict::Holder;

/// A read that depends on its scope.
pub(crate) trait ScopedRead<'input>: Sized {
  /// What it reads.
  type Value;

  /// Outside a read of the form: the deserializer as the format gave it.
  fn alone<Input: Deserializer<'input>>(self, deserializer: Input) -> Result<Self::Value, Input::Error>;

  /// Inside a read of the form: a strict deserializer, within the body of a root record of that version, or of none.
  fn within<Input: Deserializer<'input>>(
    self,
    deserializer: Input,
    holder: Holder,
  ) -> Result<Self::Value, Input::Error>;
}

/// Asks the deserializer for its scope, and reads in it.
pub(crate) fn read_in_scope<'input, Input: Deserializer<'input>, Read: ScopedRead<'input>>(
  deserializer: Input,
  read: Read,
) -> Result<Read::Value, Input::Error> {
  deserializer.deserialize_newtype_struct(READ_SCOPE_NAME, ScopeVisitor(read, PhantomData))
}

/// Takes either answer to the request for the scope.
struct ScopeVisitor<'input, Read>(Read, PhantomData<&'input ()>);

/// Reads a value that a deserializer gave in answer alone, through Serde's own deserializer of that value.
macro_rules! alone_from {
  ($($method:ident: $value:ty => $deserializer:expr),* $(,)?) => {
    $(
      fn $method<Failure: Error>(self, value: $value) -> Result<Read::Value, Failure> {
        self.0.alone($deserializer(value))
      }
    )*
  };
}

impl<'input, Read: ScopedRead<'input>> Visitor<'input> for ScopeVisitor<'input, Read> {
  type Value = Read::Value;

  fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(READ_SCOPE_EXPECTING)
  }

  /// A format's answer: no read of the form is under way.
  fn visit_newtype_struct<Input: Deserializer<'input>>(self, deserializer: Input) -> Result<Read::Value, Input::Error> {
    self.0.alone(deserializer)
  }

  /// The answer of a deserializer that gives the value itself: a map, read alone.
  fn visit_map<Members: MapAccess<'input>>(self, members: Members) -> Result<Read::Value, Members::Error> {
    self.0.alone(MapAccessDeserializer::new(members))
  }

  /// A sequence, read alone, such as a standalone chapter's flat array (nineteenth review).
  fn visit_seq<Items: SeqAccess<'input>>(self, items: Items) -> Result<Read::Value, Items::Error> {
    self.0.alone(SeqAccessDeserializer::new(items))
  }

  // A string, a Boolean, a number, bytes or a null, read alone, so that each fails as it would in a record.
  alone_from! {
    visit_str: &str => StrDeserializer::new,
    visit_borrowed_str: &'input str => BorrowedStrDeserializer::new,
    visit_string: String => StringDeserializer::new,
    visit_bytes: &[u8] => BytesDeserializer::new,
    visit_borrowed_bytes: &'input [u8] => BorrowedBytesDeserializer::new,
    visit_bool: bool => IntoDeserializer::into_deserializer,
    visit_i64: i64 => IntoDeserializer::into_deserializer,
    visit_u64: u64 => IntoDeserializer::into_deserializer,
    visit_i128: i128 => IntoDeserializer::into_deserializer,
    visit_u128: u128 => IntoDeserializer::into_deserializer,
    visit_f64: f64 => IntoDeserializer::into_deserializer,
    visit_char: char => IntoDeserializer::into_deserializer,
  }

  fn visit_unit<Failure: Error>(self) -> Result<Read::Value, Failure> {
    self.0.alone(UnitDeserializer::new())
  }

  fn visit_none<Failure: Error>(self) -> Result<Read::Value, Failure> {
    self.0.alone(UnitDeserializer::new())
  }

  fn visit_some<Input: Deserializer<'input>>(self, deserializer: Input) -> Result<Read::Value, Input::Error> {
    self.0.alone(deserializer)
  }

  /// The strict deserializer's answer, when the identifier is its static; any other enum is a value that JSON does
  /// not carry.
  fn visit_enum<Answer: EnumAccess<'input>>(self, answer: Answer) -> Result<Read::Value, Answer::Error> {
    match answer.variant_seed(IdentifierSeed)? {
      (true, variant) => variant.tuple_variant(READ_SCOPE_LENGTH, WithinVisitor(self.0, PhantomData)),
      (false, _) => Err(Answer::Error::custom(NOT_JSON_VALUE_MESSAGE)),
    }
  }
}

/// Takes the two elements of the strict deserializer's answer: the version, then the deserializer itself.
struct WithinVisitor<'input, Read>(Read, PhantomData<&'input ()>);

impl<'input, Read: ScopedRead<'input>> Visitor<'input> for WithinVisitor<'input, Read> {
  type Value = Read::Value;

  fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(READ_SCOPE_EXPECTING)
  }

  fn visit_seq<Answer: SeqAccess<'input>>(self, mut answer: Answer) -> Result<Read::Value, Answer::Error> {
    let text: String = answer
      .next_element()?
      .ok_or_else(|| Answer::Error::custom(READ_SCOPE_MESSAGE))?;
    let holder = SchemaVersion::from_text(&text).ok();
    answer
      .next_element_seed(WithinSeed { read: self.0, holder })?
      .ok_or_else(|| Answer::Error::custom(READ_SCOPE_MESSAGE))
  }
}

/// Reads within the scope that the strict deserializer gave.
struct WithinSeed<Read> {
  read: Read,
  holder: Holder,
}

impl<'input, Read: ScopedRead<'input>> DeserializeSeed<'input> for WithinSeed<Read> {
  type Value = Read::Value;

  fn deserialize<Input: Deserializer<'input>>(self, deserializer: Input) -> Result<Read::Value, Input::Error> {
    self.read.within(deserializer, self.holder)
  }
}

#[cfg(test)]
mod tests {
  use serde::Deserialize;
  use serde::de::IntoDeserializer;
  use serde::de::value::{BorrowedBytesDeserializer, BytesDeserializer, Error, UnitDeserializer};

  use crate::model::Confidence;

  /// A confidence read alone from a value that one of Serde's value deserializers gives, as a number or as a value
  /// that is no number.
  fn confidence<'input, Input: serde::Deserializer<'input, Error = Error>>(input: Input) -> bool {
    Confidence::deserialize(input).is_ok()
  }

  #[test]
  fn reads_a_value_that_a_value_deserializer_gives_alone() {
    assert!(confidence(60_u64.into_deserializer()));
    assert!(confidence(60_i64.into_deserializer()));
    assert!(confidence(60_u128.into_deserializer()));
    assert!(confidence(60_i128.into_deserializer()));
    assert!(!confidence(true.into_deserializer()));
    assert!(!confidence(60.0_f64.into_deserializer()));
    assert!(!confidence('6'.into_deserializer()));
    assert!(!confidence(BytesDeserializer::new(b"60")));
    assert!(!confidence(BorrowedBytesDeserializer::new(b"60")));
    assert!(!confidence(UnitDeserializer::new()));
    let none: Option<u8> = None;
    assert!(Confidence::deserialize(serde_json::to_value(none).unwrap_or_default()).is_err());
    let some = serde_json::json!(60);
    assert!(Confidence::deserialize(some).is_ok());
  }
}
