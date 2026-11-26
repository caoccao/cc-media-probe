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

//! The members that an entry, a detection or an error record does not read, as the maintainer chose at the
//! eighteenth review: held until the record's outcome is known, then kept for an outcome of a newer version, whose
//! payload they are, and passed by as they come for an outcome that this version knows. An error record's kind is its
//! outcome here. The member that decides is read the way a root record's version is: what comes before it is held,
//! and what comes after it is never held for an outcome that this version knows. Every writer of the form writes the
//! outcome before any member that a newer version adds, so a reader holds nothing for a record of a known outcome.
//! Serde's `flatten`, which the stub used until then, held every member that the record does not read until the
//! record's end, whatever the outcome, so the review's ignored array of 200,000 numbers in a detection that this
//! version knows cost about 21 MB.
//!
//! The record's own members are read by its derive, from a map that reads each name and hands it over, and that sees
//! which members the derive passes by: the derive asks for a member that it does not know as Serde's `IgnoredAny`.

use std::marker::PhantomData;
use std::{fmt, mem};

use serde::de::value::{MapAccessDeserializer, StringDeserializer};
use serde::de::{DeserializeSeed, MapAccess, Visitor};
use serde::{Deserialize, Deserializer};

use super::members::KeptMembers;
use super::passing::Passing;
use super::value::KeptValue;
use crate::form::constants::{OUTCOME_RECORD_EXPECTING, OUTCOME_RECORD_NAME};

/// A record whose outcome decides whether the members that it does not read are kept.
pub(crate) trait Outcome {
  /// The name of the member that holds the outcome, such as `outcome`, or an error record's `kind`.
  const MEMBER: &'static str;

  /// Whether the outcome's text names an outcome of a newer version.
  fn is_newer(text: &str) -> bool;
}

/// A record's members as read, and the members that it does not read: those of an outcome of a newer version as its
/// payload, and none for an outcome that this version knows.
pub(crate) struct WithKept<Data> {
  pub(crate) data: Data,
  pub(crate) kept: KeptMembers,
}

impl<'input, Data: Deserialize<'input> + Outcome> Deserialize<'input> for WithKept<Data> {
  fn deserialize<Input: Deserializer<'input>>(deserializer: Input) -> Result<WithKept<Data>, Input::Error> {
    deserializer.deserialize_struct(OUTCOME_RECORD_NAME, &[], WithKeptVisitor(PhantomData))
  }
}

/// Reads the record's map.
struct WithKeptVisitor<Data>(PhantomData<Data>);

impl<'input, Data: Deserialize<'input> + Outcome> Visitor<'input> for WithKeptVisitor<Data> {
  type Value = WithKept<Data>;

  fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(OUTCOME_RECORD_EXPECTING)
  }

  fn visit_map<Members: MapAccess<'input>>(self, members: Members) -> Result<WithKept<Data>, Members::Error> {
    let mut keeping = Keeping {
      members,
      member: Data::MEMBER,
      is_newer: Data::is_newer,
      decision: Decision::Undecided,
      name: None,
      held: Vec::new(),
    };
    let data = Data::deserialize(MapAccessDeserializer::new(&mut keeping))?;
    let kept = match keeping.decision {
      Decision::Newer => KeptMembers::from_checked(keeping.held).into_payload()?,
      Decision::Undecided | Decision::Known => KeptMembers::default(),
    };
    Ok(WithKept { data, kept })
  }
}

/// What the outcome has decided so far.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Decision {
  /// No outcome read yet: a member that the record does not read is held.
  Undecided,
  /// An outcome of a newer version: such a member is kept.
  Newer,
  /// An outcome that this version knows: such a member is passed by, and none is held.
  Known,
}

/// The record's map, which reads each name, hands the outcome over once it has decided, and holds or passes by each
/// member that the record does not read. The map below it reads strictly, so a name is a string, given once.
struct Keeping<Members> {
  members: Members,
  member: &'static str,
  is_newer: fn(&str) -> bool,
  decision: Decision,
  name: Option<String>,
  held: Vec<(String, KeptValue)>,
}

impl<'input, Members: MapAccess<'input>> MapAccess<'input> for Keeping<Members> {
  type Error = Members::Error;

  fn next_key_seed<Seed: DeserializeSeed<'input>>(
    &mut self,
    seed: Seed,
  ) -> Result<Option<Seed::Value>, Members::Error> {
    let Some(name) = self.members.next_key::<String>()? else {
      return Ok(None);
    };
    let key = seed.deserialize(StringDeserializer::<Members::Error>::new(name.clone()))?;
    self.name = Some(name);
    Ok(Some(key))
  }

  fn next_value_seed<Seed: DeserializeSeed<'input>>(&mut self, seed: Seed) -> Result<Seed::Value, Members::Error> {
    let name = self.name.take().unwrap_or_default();
    if name == self.member {
      let text: String = self.members.next_value()?;
      self.decision = if (self.is_newer)(&text) {
        Decision::Newer
      } else {
        Decision::Known
      };
      if self.decision == Decision::Known {
        drop(mem::take(&mut self.held));
      }
      return seed.deserialize(StringDeserializer::<Members::Error>::new(text));
    }
    self.members.next_value_seed(Passing {
      seed,
      name,
      decision: self.decision,
      held: &mut self.held,
    })
  }

  fn size_hint(&self) -> Option<usize> {
    self.members.size_hint()
  }
}
