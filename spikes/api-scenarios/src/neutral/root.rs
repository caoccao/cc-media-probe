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
//! before `schema_version` are held; the version is read and checked, so a missing version or another version is the
//! version error before any other member can fail to decode; then the held members are decoded, followed by the rest
//! as they come, all of them strictly and in the order written.
//!
//! A root record read alone checks its major version. A root record nested in another, as an entry's report or an
//! error record's partial report, compares its version with that of the record that holds it, as soon as it has read
//! it and before it decodes its own body (thirteenth review). The record that holds it already has a version of this
//! major and reads strictly, so a null or a repeated name before the nested record's version is malformed input,
//! while every other failure comes after the version error, whatever the order of the parent's members.
//!
//! A member's name before the version is read as any value, as the fifteenth review asked: a name that is not a
//! string, such as an integer key of CBOR, is held with its value and fails once the version is checked, as a value
//! that JSON does not carry.

use std::collections::HashSet;
use std::fmt;
use std::marker::PhantomData;
use std::vec;

use serde::de::value::MapAccessDeserializer;
use serde::de::{DeserializeSeed, Error, IntoDeserializer, MapAccess, Visitor};
use serde::{Deserialize, Deserializer};

use super::held::HeldValue;
use super::scope::{ScopedRead, read_in_scope};
use super::strict::{Holder, StrictMap};
use crate::constants::{
  HOLDER_VERSION_LABEL, MISSING_VERSION_MESSAGE, NESTED_ROOT_VERSION_MESSAGE, NOT_JSON_VALUE_MESSAGE,
  OTHER_MAJOR_VERSION_MESSAGE, READER_VERSION_LABEL, ROOT_RECORD_EXPECTING, SCHEMA_VERSION_MEMBER,
  SCHEMA_VERSION_TEXT_MESSAGE,
};
use crate::document::SchemaVersion;

/// A root record as read: its version, of this major, and the members of its body, which hold no version.
pub(crate) struct Rooted<Body> {
  pub(crate) version: SchemaVersion,
  pub(crate) body: Body,
}

impl<'input, Body: Deserialize<'input>> Deserialize<'input> for Rooted<Body> {
  fn deserialize<Input: Deserializer<'input>>(deserializer: Input) -> Result<Rooted<Body>, Input::Error> {
    read_in_scope(deserializer, RootRead(PhantomData))
  }
}

/// The read of a root record in its scope: alone, or inside the body of another of a known version.
struct RootRead<Body>(PhantomData<Body>);

impl<'input, Body: Deserialize<'input>> ScopedRead<'input> for RootRead<Body> {
  type Value = Rooted<Body>;

  fn alone<Input: Deserializer<'input>>(self, deserializer: Input) -> Result<Rooted<Body>, Input::Error> {
    deserializer.deserialize_map(RootVisitor {
      holder: None,
      body: PhantomData,
    })
  }

  fn within<Input: Deserializer<'input>>(
    self,
    deserializer: Input,
    holder: Holder,
  ) -> Result<Rooted<Body>, Input::Error> {
    deserializer.deserialize_map(RootVisitor {
      holder,
      body: PhantomData,
    })
  }
}

/// Reads a root record's map, version first, beside the version of the record that holds it, if any.
struct RootVisitor<Body> {
  holder: Holder,
  body: PhantomData<Body>,
}

impl<'input, Body: Deserialize<'input>> Visitor<'input> for RootVisitor<Body> {
  type Value = Rooted<Body>;

  fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(ROOT_RECORD_EXPECTING)
  }

  fn visit_map<Members: MapAccess<'input>>(self, mut members: Members) -> Result<Rooted<Body>, Members::Error> {
    let mut held = Vec::new();
    let version = loop {
      let Some(name) = members.next_key::<HeldValue>()? else {
        return Err(Members::Error::custom(MISSING_VERSION_MESSAGE));
      };
      let name = match name {
        HeldValue::Text(name) if name == SCHEMA_VERSION_MEMBER => {
          break version_of(members.next_value::<HeldValue>()?)?;
        }
        HeldValue::Text(name) => Some(name),
        _ => None,
      };
      held.push((name, members.next_value::<HeldValue>()?));
    };
    check_version::<Members::Error>(version, self.holder)?;
    let body = Body::deserialize(MapAccessDeserializer::new(StrictMap::with_names(
      ChainedMembers {
        held: held.into_iter(),
        pending: None,
        live: members,
      },
      HashSet::from([SCHEMA_VERSION_MEMBER.to_owned()]),
      Some(version),
    )))?;
    Ok(Rooted { version, body })
  }
}

/// The version that a member holds, read as any value so that one of another type, a number or `null` alone, is the
/// version error too (fourteenth review). Inside a strict read, the parent has refused a `null` already.
fn version_of<Failure: Error>(value: HeldValue) -> Result<SchemaVersion, Failure> {
  match value {
    HeldValue::Text(text) => SchemaVersion::try_from(text).map_err(Failure::custom),
    _ => Err(Failure::custom(SCHEMA_VERSION_TEXT_MESSAGE)),
  }
}

/// The version error, or nothing: a nested record has the version of the record that holds it, and a record read
/// alone has this version's major.
fn check_version<Failure: Error>(version: SchemaVersion, holder: Holder) -> Result<(), Failure> {
  match holder {
    Some(holder) if version != holder => Err(Failure::custom(format!(
      "{NESTED_ROOT_VERSION_MESSAGE}: {}; {HOLDER_VERSION_LABEL} {}",
      String::from(version),
      String::from(holder)
    ))),
    Some(_) => Ok(()),
    None if version.major() != SchemaVersion::CURRENT.major() => Err(Failure::custom(format!(
      "{OTHER_MAJOR_VERSION_MESSAGE}: {}; {READER_VERSION_LABEL} {}",
      String::from(version),
      String::from(SchemaVersion::CURRENT)
    ))),
    None => Ok(()),
  }
}

/// The body's members: the held ones first, as they were read, then those after the version, from the input. The
/// map around them reads both strictly and notes every name, so a repeated name fails where it stands. A held member
/// whose name is not a string has no name, and fails where it stands.
struct ChainedMembers<Members> {
  held: vec::IntoIter<(Option<String>, HeldValue)>,
  pending: Option<HeldValue>,
  live: Members,
}

impl<'input, Members: MapAccess<'input>> MapAccess<'input> for ChainedMembers<Members> {
  type Error = Members::Error;

  fn next_key_seed<Seed: DeserializeSeed<'input>>(
    &mut self,
    seed: Seed,
  ) -> Result<Option<Seed::Value>, Members::Error> {
    match self.held.next() {
      Some((None, _)) => Err(Members::Error::custom(NOT_JSON_VALUE_MESSAGE)),
      Some((Some(name), value)) => {
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
