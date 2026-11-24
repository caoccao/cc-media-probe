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

//! The identifier of an enum given in answer to a request for the scope, as the nineteenth review asked: the strict
//! deserializer's answer when it is the private static `READ_SCOPE_TOKEN` itself, compared by address, which no input
//! and no other deserializer can give. Any other enum, such as one that Serde's `EnumAccessDeserializer` gives to a
//! newtype request, is a value that JSON does not carry, as `Strict` refuses an enum that a format gives to
//! `deserialize_any` (`super::strict`).

use std::fmt;

use serde::de::{DeserializeSeed, Deserializer, Error, IgnoredAny, Visitor};

use crate::constants::{READ_SCOPE_IDENTIFIER_EXPECTING, READ_SCOPE_TOKEN};

/// Whether an enum's identifier is the strict deserializer's.
pub(super) struct IdentifierSeed;

impl<'input> DeserializeSeed<'input> for IdentifierSeed {
  type Value = bool;

  fn deserialize<Input: Deserializer<'input>>(self, deserializer: Input) -> Result<bool, Input::Error> {
    deserializer.deserialize_identifier(IdentifierVisitor)
  }
}

struct IdentifierVisitor;

impl<'input> Visitor<'input> for IdentifierVisitor {
  type Value = bool;

  fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(READ_SCOPE_IDENTIFIER_EXPECTING)
  }

  /// The private static itself, and no other bytes of the same content.
  fn visit_bytes<Failure: Error>(self, value: &[u8]) -> Result<bool, Failure> {
    Ok(std::ptr::eq(value, &READ_SCOPE_TOKEN[..]))
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
  fn visit_map<Members: serde::de::MapAccess<'input>>(self, mut members: Members) -> Result<bool, Members::Error> {
    while members.next_entry::<IgnoredAny, IgnoredAny>()?.is_some() {}
    Ok(false)
  }
}
