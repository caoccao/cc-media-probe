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

//! The identifier of an enum given in answer to a request for the scope, as the nineteenth review of `api.md` asked:
//! the strict deserializer's answer when it is the private static `READ_SCOPE_TOKEN` itself, compared by address,
//! which no input and no other deserializer can give. Any other enum, such as one that Serde's
//! `EnumAccessDeserializer` gives to a newtype request, is a value that JSON does not carry, as `Strict` refuses an
//! enum that a format gives to `deserialize_any` (`crate::form::strict`).

use std::fmt;

use serde::de::{DeserializeSeed, Deserializer, Error, IgnoredAny, MapAccess, Visitor};

use crate::form::constants::{READ_SCOPE_IDENTIFIER_EXPECTING, READ_SCOPE_TOKEN};

/// Whether an enum's identifier is the strict deserializer's.
pub(super) struct IdentifierSeed;

impl<'input> DeserializeSeed<'input> for IdentifierSeed {
  type Value = bool;

  fn deserialize<Input: Deserializer<'input>>(self, deserializer: Input) -> Result<bool, Input::Error> {
    deserializer.deserialize_identifier(IdentifierVisitor)
  }
}

/// Tells the static from every other identifier.
struct IdentifierVisitor;

impl<'input> Visitor<'input> for IdentifierVisitor {
  type Value = bool;

  fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(READ_SCOPE_IDENTIFIER_EXPECTING)
  }

  /// The private static itself, and no other bytes of the same content.
  fn visit_bytes<Failure: Error>(self, value: &[u8]) -> Result<bool, Failure> {
    Ok(std::ptr::eq(value.as_ptr(), READ_SCOPE_TOKEN.as_ptr()) && value.len() == READ_SCOPE_TOKEN.len())
  }

  fn visit_borrowed_bytes<Failure: Error>(self, value: &'input [u8]) -> Result<bool, Failure> {
    self.visit_bytes(value)
  }

  fn visit_byte_buf<Failure: Error>(self, _value: Vec<u8>) -> Result<bool, Failure> {
    Ok(false)
  }

  fn visit_str<Failure: Error>(self, _value: &str) -> Result<bool, Failure> {
    Ok(false)
  }

  fn visit_u64<Failure: Error>(self, _value: u64) -> Result<bool, Failure> {
    Ok(false)
  }

  /// Any other identifier, passed by.
  fn visit_map<Members: MapAccess<'input>>(self, mut members: Members) -> Result<bool, Members::Error> {
    while members.next_entry::<IgnoredAny, IgnoredAny>()?.is_some() {}
    Ok(false)
  }
}

#[cfg(test)]
mod tests {
  use serde::de::value::{Error, MapDeserializer, StrDeserializer, U64Deserializer};
  use serde::de::{DeserializeSeed, IntoDeserializer};

  use super::IdentifierSeed;

  #[test]
  fn tells_the_static_from_every_other_identifier() {
    assert_eq!(
      IdentifierSeed.deserialize(StrDeserializer::<Error>::new("scope")).ok(),
      Some(false)
    );
    assert_eq!(
      IdentifierSeed.deserialize(U64Deserializer::<Error>::new(1)).ok(),
      Some(false)
    );
    let bytes = serde::de::value::BytesDeserializer::<Error>::new(b"scope");
    assert_eq!(IdentifierSeed.deserialize(bytes).ok(), Some(false));
    let members = MapDeserializer::<_, Error>::new(vec![("a", 1_u64)].into_iter());
    assert_eq!(IdentifierSeed.deserialize(members).ok(), Some(false));
    assert!(
      IdentifierSeed
        .deserialize(true.into_deserializer() as serde::de::value::BoolDeserializer<Error>)
        .is_err()
    );
  }
}
