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

//! An enum that the type asked for, written as an object of one member: the variant's name, with what the variant
//! holds as its value, as Serde's derive writes it by default. `Strict` reads every value with `deserialize_any`
//! (seventeenth review), so the format gives such an enum as a map, which is read here as the format itself would
//! read it, the member's value strictly, and a record that the variant holds as a record. A second member is
//! malformed input. A variant without a value is written as its name alone, which `super::visitor` reads.

use serde::de::{DeserializeSeed, Deserializer, EnumAccess, Error, IgnoredAny, MapAccess, VariantAccess, Visitor};

use super::access::StrictMap;
use crate::constants::{ENUM_MEMBER_MESSAGE, VARIANT_RECORD_NAME};

/// Reads the enum from the map's one member, and checks that no other member follows.
pub(super) fn read_variant<'input, Read: Visitor<'input>, Members: MapAccess<'input>>(
  read: Read,
  mut map: StrictMap<Members>,
) -> Result<Read::Value, Members::Error> {
  let value = read.visit_enum(VariantMember { map: &mut map })?;
  match map.next_key::<IgnoredAny>()? {
    Some(_) => Err(Members::Error::custom(ENUM_MEMBER_MESSAGE)),
    None => Ok(value),
  }
}

/// The map's one member, as the enum's variant.
struct VariantMember<'map, Members> {
  map: &'map mut StrictMap<Members>,
}

impl<'input, Members: MapAccess<'input>> EnumAccess<'input> for VariantMember<'_, Members> {
  type Error = Members::Error;
  type Variant = Self;

  fn variant_seed<Seed: DeserializeSeed<'input>>(self, seed: Seed) -> Result<(Seed::Value, Self), Members::Error> {
    match self.map.next_key_seed(seed)? {
      Some(variant) => Ok((variant, self)),
      None => Err(Members::Error::custom(ENUM_MEMBER_MESSAGE)),
    }
  }
}

impl<'input, Members: MapAccess<'input>> VariantAccess<'input> for VariantMember<'_, Members> {
  type Error = Members::Error;

  /// A variant without a value is written as its name, never as an object; the value read here is null at best,
  /// which the form refuses.
  fn unit_variant(self) -> Result<(), Members::Error> {
    self.map.next_value()
  }

  fn newtype_variant_seed<Seed: DeserializeSeed<'input>>(self, seed: Seed) -> Result<Seed::Value, Members::Error> {
    self.map.next_value_seed(seed)
  }

  fn tuple_variant<Read: Visitor<'input>>(self, length: usize, visitor: Read) -> Result<Read::Value, Members::Error> {
    self.map.next_value_seed(TupleSeed { length, visitor })
  }

  fn struct_variant<Read: Visitor<'input>>(
    self,
    fields: &'static [&'static str],
    visitor: Read,
  ) -> Result<Read::Value, Members::Error> {
    self.map.next_value_seed(RecordSeed { fields, visitor })
  }
}

/// The values of a variant that holds several, as an array.
struct TupleSeed<Read> {
  length: usize,
  visitor: Read,
}

impl<'input, Read: Visitor<'input>> DeserializeSeed<'input> for TupleSeed<Read> {
  type Value = Read::Value;

  fn deserialize<Input: Deserializer<'input>>(self, deserializer: Input) -> Result<Read::Value, Input::Error> {
    deserializer.deserialize_tuple(self.length, self.visitor)
  }
}

/// The record that a variant holds, as an object of named members.
struct RecordSeed<Read> {
  fields: &'static [&'static str],
  visitor: Read,
}

impl<'input, Read: Visitor<'input>> DeserializeSeed<'input> for RecordSeed<Read> {
  type Value = Read::Value;

  fn deserialize<Input: Deserializer<'input>>(self, deserializer: Input) -> Result<Read::Value, Input::Error> {
    deserializer.deserialize_struct(VARIANT_RECORD_NAME, self.fields, self.visitor)
  }
}
