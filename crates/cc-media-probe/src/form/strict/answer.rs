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

//! The answer of `Strict` to a request for the scope, as the nineteenth review of `api.md` asked: an enum's variant
//! whose identifier is the bytes of a private static, given as that static itself, and whose two elements are the
//! version of the root record whose body it reads, as text that is empty without one, and the strict deserializer
//! itself, from which the value then reads as it would have from the request. A sequence of two, which the spike
//! answered first, could be imitated by a sequence of the input, since Serde's `SeqDeserializer` gives its elements
//! to a newtype request. No input holds the static, so the answer is recognized by the address of its identifier,
//! never by its content (`crate::form::scope`).

use std::marker::PhantomData;

use serde::de::{DeserializeSeed, EnumAccess, Error, IntoDeserializer, SeqAccess, VariantAccess, Visitor};
use serde::{Deserializer, forward_to_deserialize_any};

use super::deserializer::{Holder, Strict};
use crate::document::SchemaVersion;
use crate::form::constants::{READ_SCOPE_MESSAGE, READ_SCOPE_TOKEN};

/// The answer: the variant that only `Strict` gives, and what it carries.
pub(super) struct ScopeAnswer<'slot, Inner> {
  holder: Holder,
  deserializer: Strict<'slot, Inner>,
}

impl<'slot, Inner> ScopeAnswer<'slot, Inner> {
  pub(super) fn new(holder: Holder, deserializer: Strict<'slot, Inner>) -> ScopeAnswer<'slot, Inner> {
    ScopeAnswer { holder, deserializer }
  }
}

impl<'input, 'slot, Inner: Deserializer<'input>> EnumAccess<'input> for ScopeAnswer<'slot, Inner> {
  type Error = Inner::Error;
  type Variant = ScopeAccess<'slot, Inner>;

  fn variant_seed<Seed: DeserializeSeed<'input>>(
    self,
    seed: Seed,
  ) -> Result<(Seed::Value, ScopeAccess<'slot, Inner>), Inner::Error> {
    let identifier = seed.deserialize(TokenDeserializer(PhantomData))?;
    Ok((
      identifier,
      ScopeAccess {
        holder: Some(self.holder),
        deserializer: Some(self.deserializer),
      },
    ))
  }
}

/// The identifier of the answer: the private static itself, as borrowed bytes.
struct TokenDeserializer<Failure>(PhantomData<Failure>);

impl<'input, Failure: Error> Deserializer<'input> for TokenDeserializer<Failure> {
  type Error = Failure;

  fn deserialize_any<Read: Visitor<'input>>(self, visitor: Read) -> Result<Read::Value, Failure> {
    visitor.visit_borrowed_bytes(&READ_SCOPE_TOKEN)
  }

  forward_to_deserialize_any! {
    <Read: Visitor<'input>>
    bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string bytes byte_buf option unit unit_struct
    newtype_struct seq tuple tuple_struct map struct enum identifier ignored_any
  }
}

/// The answer's two elements, each taken once, as the variant's tuple.
pub(super) struct ScopeAccess<'slot, Inner> {
  holder: Option<Holder>,
  deserializer: Option<Strict<'slot, Inner>>,
}

impl<'input, Inner: Deserializer<'input>> VariantAccess<'input> for ScopeAccess<'_, Inner> {
  type Error = Inner::Error;

  fn unit_variant(self) -> Result<(), Inner::Error> {
    Err(Inner::Error::custom(READ_SCOPE_MESSAGE))
  }

  fn newtype_variant_seed<Seed: DeserializeSeed<'input>>(self, _seed: Seed) -> Result<Seed::Value, Inner::Error> {
    Err(Inner::Error::custom(READ_SCOPE_MESSAGE))
  }

  fn tuple_variant<Read: Visitor<'input>>(self, _length: usize, visitor: Read) -> Result<Read::Value, Inner::Error> {
    visitor.visit_seq(self)
  }

  fn struct_variant<Read: Visitor<'input>>(
    self,
    _fields: &'static [&'static str],
    _visitor: Read,
  ) -> Result<Read::Value, Inner::Error> {
    Err(Inner::Error::custom(READ_SCOPE_MESSAGE))
  }
}

impl<'input, Inner: Deserializer<'input>> SeqAccess<'input> for ScopeAccess<'_, Inner> {
  type Error = Inner::Error;

  fn next_element_seed<Seed: DeserializeSeed<'input>>(
    &mut self,
    seed: Seed,
  ) -> Result<Option<Seed::Value>, Inner::Error> {
    if let Some(holder) = self.holder.take() {
      let text = holder.map_or_else(String::new, SchemaVersion::to_text);
      return seed
        .deserialize(IntoDeserializer::<Inner::Error>::into_deserializer(text))
        .map(Some);
    }
    match self.deserializer.take() {
      Some(deserializer) => seed.deserialize(deserializer).map(Some),
      None => Ok(None),
    }
  }
}

#[cfg(test)]
mod tests {
  use serde::de::value::U64Deserializer;
  use serde::de::{IgnoredAny, VariantAccess};

  use super::{ScopeAccess, Strict};

  fn access() -> ScopeAccess<'static, U64Deserializer<serde::de::value::Error>> {
    ScopeAccess {
      holder: Some(None),
      deserializer: Some(Strict::value(U64Deserializer::new(1), None)),
    }
  }

  #[test]
  fn answers_only_as_the_tuple_of_its_two_elements() {
    assert!(access().unit_variant().is_err());
    assert!(access().newtype_variant::<IgnoredAny>().is_err());
    assert!(access().struct_variant(&[], IgnoredAny).is_err());
  }
}
