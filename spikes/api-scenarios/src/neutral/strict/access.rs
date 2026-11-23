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

//! The sequences and maps of `Strict`: every element and every member's value is read strictly, and a map notes each
//! member's name, refusing one given twice.

use std::collections::HashSet;

use serde::de::{DeserializeSeed, Deserializer, Error, MapAccess, SeqAccess};

use super::deserializer::Strict;
use crate::constants::DUPLICATE_MEMBER_MESSAGE;

/// A seed whose value is read strictly.
pub(super) struct StrictSeed<'name, Seed> {
  seed: Seed,
  name: Option<&'name mut Option<String>>,
}

impl<'name, Seed> StrictSeed<'name, Seed> {
  /// The seed of a value.
  pub(super) fn value(seed: Seed) -> StrictSeed<'name, Seed> {
    StrictSeed { seed, name: None }
  }
}

impl<'input, Seed: DeserializeSeed<'input>> DeserializeSeed<'input> for StrictSeed<'_, Seed> {
  type Value = Seed::Value;

  fn deserialize<Inner: Deserializer<'input>>(self, deserializer: Inner) -> Result<Seed::Value, Inner::Error> {
    match self.name {
      Some(slot) => self.seed.deserialize(Strict::name(deserializer, slot)),
      None => self.seed.deserialize(Strict::value(deserializer)),
    }
  }
}

/// A sequence whose elements are read strictly.
pub(super) struct StrictSequence<Items> {
  items: Items,
}

impl<Items> StrictSequence<Items> {
  pub(super) fn new(items: Items) -> StrictSequence<Items> {
    StrictSequence { items }
  }
}

impl<'input, Items: SeqAccess<'input>> SeqAccess<'input> for StrictSequence<Items> {
  type Error = Items::Error;

  fn next_element_seed<Seed: DeserializeSeed<'input>>(
    &mut self,
    seed: Seed,
  ) -> Result<Option<Seed::Value>, Items::Error> {
    self.items.next_element_seed(StrictSeed::value(seed))
  }

  fn size_hint(&self) -> Option<usize> {
    self.items.size_hint()
  }
}

/// A map whose members are read strictly, each name once.
pub(crate) struct StrictMap<Members> {
  members: Members,
  names: HashSet<String>,
}

impl<Members> StrictMap<Members> {
  pub(super) fn new(members: Members) -> StrictMap<Members> {
    StrictMap::with_names(members, HashSet::new())
  }

  /// A map of which some members were already read, under these names.
  pub(crate) fn with_names(members: Members, names: HashSet<String>) -> StrictMap<Members> {
    StrictMap { members, names }
  }
}

impl<'input, Members: MapAccess<'input>> MapAccess<'input> for StrictMap<Members> {
  type Error = Members::Error;

  fn next_key_seed<Seed: DeserializeSeed<'input>>(
    &mut self,
    seed: Seed,
  ) -> Result<Option<Seed::Value>, Members::Error> {
    let mut name = None;
    let key = self.members.next_key_seed(StrictSeed {
      seed,
      name: Some(&mut name),
    })?;
    if let Some(name) = name
      && !self.names.insert(name)
    {
      return Err(Members::Error::custom(DUPLICATE_MEMBER_MESSAGE));
    }
    Ok(key)
  }

  fn next_value_seed<Seed: DeserializeSeed<'input>>(&mut self, seed: Seed) -> Result<Seed::Value, Members::Error> {
    self.members.next_value_seed(StrictSeed::value(seed))
  }

  fn size_hint(&self) -> Option<usize> {
    self.members.size_hint()
  }
}
