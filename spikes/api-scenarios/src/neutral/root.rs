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

//! `Rooted`, a root record's version with its other members, read version first (plan section 8.8). The members
//! before `schema_version` are held; the version is read and its major checked, so a missing version or another
//! major is the version error before any other member can fail to decode; then the held members are decoded,
//! followed by the rest as they come, all of them strictly.

use std::collections::HashSet;
use std::fmt;
use std::marker::PhantomData;
use std::vec;

use serde::de::value::MapAccessDeserializer;
use serde::de::{DeserializeSeed, Error, IntoDeserializer, MapAccess, Visitor};
use serde::{Deserialize, Deserializer};

use super::held::HeldValue;
use super::strict::StrictMap;
use crate::constants::{
  DUPLICATE_MEMBER_MESSAGE, MISSING_VERSION_MESSAGE, OTHER_MAJOR_VERSION_MESSAGE, READER_VERSION_LABEL,
  ROOT_RECORD_EXPECTING, SCHEMA_VERSION_MEMBER,
};
use crate::document::SchemaVersion;
use crate::kept::KeptValue;

/// A root record as read: its version, of this major, and the members of its body, which hold no version.
pub(crate) struct Rooted<Body> {
  pub(crate) version: SchemaVersion,
  pub(crate) body: Body,
}

impl<'input, Body: Deserialize<'input>> Deserialize<'input> for Rooted<Body> {
  fn deserialize<Input: Deserializer<'input>>(deserializer: Input) -> Result<Rooted<Body>, Input::Error> {
    deserializer.deserialize_map(RootVisitor(PhantomData))
  }
}

/// Reads a root record's map, version first.
struct RootVisitor<Body>(PhantomData<Body>);

impl<'input, Body: Deserialize<'input>> Visitor<'input> for RootVisitor<Body> {
  type Value = Rooted<Body>;

  fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(ROOT_RECORD_EXPECTING)
  }

  fn visit_map<Members: MapAccess<'input>>(self, mut members: Members) -> Result<Rooted<Body>, Members::Error> {
    let mut held = Vec::new();
    let version = loop {
      let Some(name) = members.next_key::<String>()? else {
        return Err(Members::Error::custom(MISSING_VERSION_MESSAGE));
      };
      if name == SCHEMA_VERSION_MEMBER {
        break members.next_value::<SchemaVersion>()?;
      }
      held.push((name, members.next_value::<HeldValue>()?));
    };
    if version.major() != SchemaVersion::CURRENT.major() {
      return Err(Members::Error::custom(format!(
        "{OTHER_MAJOR_VERSION_MESSAGE}: {}; {READER_VERSION_LABEL} {}",
        String::from(version),
        String::from(SchemaVersion::CURRENT)
      )));
    }
    let mut names = HashSet::from([SCHEMA_VERSION_MEMBER.to_owned()]);
    let mut kept = Vec::with_capacity(held.len());
    for (name, value) in held {
      if !names.insert(name.clone()) {
        return Err(Members::Error::custom(DUPLICATE_MEMBER_MESSAGE));
      }
      kept.push((name, value.into_kept()?));
    }
    let body = Body::deserialize(MapAccessDeserializer::new(ChainedMembers {
      held: kept.into_iter(),
      pending: None,
      live: StrictMap::with_names(members, names),
    }))?;
    Ok(Rooted { version, body })
  }
}

/// The body's members: the held ones first, from the kept form, then those after the version, read strictly.
struct ChainedMembers<Members> {
  held: vec::IntoIter<(String, KeptValue)>,
  pending: Option<KeptValue>,
  live: StrictMap<Members>,
}

impl<'input, Members: MapAccess<'input>> MapAccess<'input> for ChainedMembers<Members> {
  type Error = Members::Error;

  fn next_key_seed<Seed: DeserializeSeed<'input>>(
    &mut self,
    seed: Seed,
  ) -> Result<Option<Seed::Value>, Members::Error> {
    match self.held.next() {
      Some((name, value)) => {
        self.pending = Some(value);
        seed
          .deserialize(IntoDeserializer::<Members::Error>::into_deserializer(name))
          .map(Some)
      }
      None => self.live.next_key_seed(seed),
    }
  }

  fn next_value_seed<Seed: DeserializeSeed<'input>>(&mut self, seed: Seed) -> Result<Seed::Value, Members::Error> {
    match self.pending.take() {
      Some(value) => seed.deserialize(IntoDeserializer::<Members::Error>::into_deserializer(value)),
      None => self.live.next_value_seed(seed),
    }
  }
}
