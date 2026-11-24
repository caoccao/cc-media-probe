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
//!
//! A map also follows the rule of serde_json's private map, which its `arbitrary_precision` feature gives for a
//! number that is not an integer within 64 bits: one member, `$serde_json::private::Number`, holding the number's
//! text. The Serde data model cannot tell it from an object of the same shape in the input, so the form gives the
//! shape one rule wherever it stands (`json-output.md` section 2). A map of a known type, a record, an enum's object
//! or a map that a type reads, never has a member of that name, as the maintainer chose at the seventeenth review:
//! the review read a disposition of `1.25` with the feature on, refused with the version first and read as an empty
//! disposition with the version last. A map of no known type is the number it spells, as the maintainer chose at the
//! sixteenth review, so the private map of a number that is not finite fails as a value that JSON does not carry,
//! wherever such a map is read strictly: in a catch-all, a kept member and a member that no type reads, and before
//! the version of a nested root record, whose holder reads strictly (seventeenth review). The map's one value
//! streams to its type while the visitor notes whether it is such a text, so a map of that name in a member that no
//! type reads is walked as it comes, never held, whatever it holds (eighteenth review).

use std::collections::HashSet;

use serde::de::{DeserializeSeed, Deserializer, Error, MapAccess, SeqAccess};

use super::deserializer::{Holder, Strict};
use super::visitor::Note;
use crate::constants::{
  ARBITRARY_PRECISION_NAME, DUPLICATE_MEMBER_MESSAGE, NOT_JSON_VALUE_MESSAGE, RESERVED_NAME_MESSAGE,
};

/// A seed whose value is read strictly.
pub(super) struct StrictSeed<'slot, Seed> {
  seed: Seed,
  note: Note<'slot>,
  holder: Holder,
}

impl<'slot, Seed> StrictSeed<'slot, Seed> {
  /// The seed of a value, inside the body of a root record of that version, or of none.
  pub(super) fn value(seed: Seed, holder: Holder) -> StrictSeed<'slot, Seed> {
    StrictSeed {
      seed,
      note: Note::Nothing,
      holder,
    }
  }
}

impl<'input, Seed: DeserializeSeed<'input>> DeserializeSeed<'input> for StrictSeed<'_, Seed> {
  type Value = Seed::Value;

  fn deserialize<Inner: Deserializer<'input>>(self, deserializer: Inner) -> Result<Seed::Value, Inner::Error> {
    self
      .seed
      .deserialize(Strict::noting(deserializer, self.note, self.holder))
  }
}

/// A sequence whose elements are read strictly.
pub(super) struct StrictSequence<Items> {
  items: Items,
  holder: Holder,
}

impl<Items> StrictSequence<Items> {
  pub(super) fn new(items: Items, holder: Holder) -> StrictSequence<Items> {
    StrictSequence { items, holder }
  }
}

impl<'input, Items: SeqAccess<'input>> SeqAccess<'input> for StrictSequence<Items> {
  type Error = Items::Error;

  fn next_element_seed<Seed: DeserializeSeed<'input>>(
    &mut self,
    seed: Seed,
  ) -> Result<Option<Seed::Value>, Items::Error> {
    self.items.next_element_seed(StrictSeed::value(seed, self.holder))
  }

  fn size_hint(&self) -> Option<usize> {
    self.items.size_hint()
  }
}

/// How far a map of no known type is the private map of a number that is not finite.
#[derive(Clone, Copy, PartialEq, Eq)]
enum PrivateMap {
  /// No member read yet.
  Empty,
  /// The first member has the private map's name, and its value comes next.
  Named,
  /// The one member read so far holds the text of a number that is not finite.
  NotFinite,
  /// Any other map.
  Other,
}

/// A map whose members are read strictly, each name once.
pub(crate) struct StrictMap<Members> {
  members: Members,
  names: HashSet<String>,
  holder: Holder,
  typed: bool,
  private: PrivateMap,
}

impl<Members> StrictMap<Members> {
  /// A map of a known type when `typed` is set, of none otherwise.
  pub(super) fn new(members: Members, holder: Holder, typed: bool) -> StrictMap<Members> {
    StrictMap {
      members,
      names: HashSet::new(),
      holder,
      typed,
      private: PrivateMap::Empty,
    }
  }

  /// The body of a root record, of which some members were already read, under these names, inside the body of a
  /// root record of that version, or of none.
  pub(crate) fn with_names(members: Members, names: HashSet<String>, holder: Holder) -> StrictMap<Members> {
    StrictMap {
      names,
      ..StrictMap::new(members, holder, true)
    }
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
      note: Note::Name(&mut name),
      holder: None,
    })?;
    if key.is_none() && self.private == PrivateMap::NotFinite {
      return Err(Members::Error::custom(NOT_JSON_VALUE_MESSAGE));
    }
    if let Some(name) = name {
      let private = name == ARBITRARY_PRECISION_NAME;
      if self.typed && private {
        return Err(Members::Error::custom(RESERVED_NAME_MESSAGE));
      }
      self.private = match self.private {
        PrivateMap::Empty if private => PrivateMap::Named,
        _ => PrivateMap::Other,
      };
      if !self.names.insert(name) {
        return Err(Members::Error::custom(DUPLICATE_MEMBER_MESSAGE));
      }
    }
    Ok(key)
  }

  /// The value of the private map's one member streams to the seed, which the eighteenth review found held whole,
  /// while its visitor notes whether it is the text of a number that is not finite.
  fn next_value_seed<Seed: DeserializeSeed<'input>>(&mut self, seed: Seed) -> Result<Seed::Value, Members::Error> {
    if self.private != PrivateMap::Named {
      return self.members.next_value_seed(StrictSeed::value(seed, self.holder));
    }
    let mut not_finite = false;
    let value = self.members.next_value_seed(StrictSeed {
      seed,
      note: Note::NotFinite(&mut not_finite),
      holder: self.holder,
    })?;
    self.private = match not_finite {
      true => PrivateMap::NotFinite,
      false => PrivateMap::Other,
    };
    Ok(value)
  }

  fn size_hint(&self) -> Option<usize> {
    self.members.size_hint()
  }
}
