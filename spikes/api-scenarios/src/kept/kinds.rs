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

//! The map of an enum that carries data, `api.md` section 12, read as its members come, as the nineteenth review
//! asked: a member before `kind` is kept as written, since only the kind says how to read it, and once the kind is
//! known, a kind that this version knows reads its members from the input as they come and passes by every other
//! member, while a kind of a newer version keeps them all, unread, as its payload. A known kind leaves no member of
//! another known kind. Until then the map was kept whole before its kind was looked at, so the review's
//! `{"kind": "integer", "integer": "oops", "extension": null}` failed on the later null, and without it the error of
//! the integer had no place in the input.

use std::fmt;
use std::marker::PhantomData;

use serde::de::{DeserializeOwned, Error, IgnoredAny, IntoDeserializer, MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserializer, Serializer};

use super::members::KeptMembers;
use super::value::KeptValue;
use crate::constants::{KEPT_VALUE_EXPECTING, KIND_MEMBER};
use crate::neutral::read_kept;

/// An enum that carries data, read from the map of its kind.
pub(crate) trait KindRead: Sized {
  /// What a kind that this version knows gathers from its members.
  type Gathered;

  /// The members that the kinds that this version knows read, beside `kind`.
  const MEMBERS: &'static [&'static str];

  /// The message of a map without its kind, without a member that its kind needs, or with one of another kind.
  const MESSAGE: &'static str;

  /// What a kind that this version knows starts from, or `None` for a kind of a newer version.
  fn start(kind: &str) -> Option<Self::Gathered>;

  /// Whether the kind reads the member of that name.
  fn reads(gathered: &Self::Gathered, name: &str) -> bool;

  /// Reads the value of a member that the kind reads.
  fn gather<'input, Members: MapAccess<'input>>(
    gathered: &mut Self::Gathered,
    name: &str,
    value: MemberValue<'_, Members>,
  ) -> Result<(), Members::Error>;

  /// The value, once the map has ended.
  fn finish<Failure: Error>(gathered: Self::Gathered) -> Result<Self, Failure>;

  /// A kind of a newer version, with its other members as written.
  fn unrecognized(kind: String, members: KeptMembers) -> Self;
}

/// A member's value: the next one of the input, or one kept before the kind.
pub(crate) enum MemberValue<'access, Members> {
  /// The value that the map holds next.
  Live(&'access mut Members),
  /// A value kept as written, read strictly into its type (`crate::neutral::read_kept`).
  Kept(KeptValue),
}

impl<'input, Members: MapAccess<'input>> MemberValue<'_, Members> {
  /// The value, read into its type.
  pub(crate) fn read<Value: DeserializeOwned>(self) -> Result<Value, Members::Error> {
    match self {
      MemberValue::Live(members) => members.next_value(),
      MemberValue::Kept(value) => read_kept(IntoDeserializer::<Members::Error>::into_deserializer(value)),
    }
  }
}

/// Reads the map of an enum that carries data.
pub(crate) fn read_by_kind<'input, Value: KindRead, Input: Deserializer<'input>>(
  deserializer: Input,
) -> Result<Value, Input::Error> {
  deserializer.deserialize_map(KindVisitor(PhantomData))
}

/// What the kind has decided so far.
enum Decision<Gathered> {
  /// No kind read yet: every member is kept.
  Undecided,
  /// A kind that this version knows, with what its members gave so far.
  Known(Gathered),
  /// A kind of a newer version: every member is kept as its payload.
  Newer(String),
}

struct KindVisitor<Value>(PhantomData<Value>);

impl<'input, Value: KindRead> Visitor<'input> for KindVisitor<Value> {
  type Value = Value;

  fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(KEPT_VALUE_EXPECTING)
  }

  fn visit_map<Members: MapAccess<'input>>(self, mut members: Members) -> Result<Value, Members::Error> {
    let mut kept: Vec<(String, KeptValue)> = Vec::new();
    let mut decision = Decision::Undecided;
    while let Some(name) = members.next_key::<String>()? {
      if name == KIND_MEMBER {
        let kind: String = members.next_value()?;
        decision = match Value::start(&kind) {
          Some(mut gathered) => {
            for (name, value) in std::mem::take(&mut kept) {
              take_member::<Value, Members>(&mut gathered, &name, MemberValue::Kept(value))?;
            }
            Decision::Known(gathered)
          }
          None => Decision::Newer(kind),
        };
        continue;
      }
      match &mut decision {
        Decision::Known(gathered) => take_member::<Value, Members>(gathered, &name, MemberValue::Live(&mut members))?,
        Decision::Undecided | Decision::Newer(_) => kept.push((name, members.next_value()?)),
      }
    }
    match decision {
      Decision::Undecided => Err(Members::Error::custom(Value::MESSAGE)),
      Decision::Known(gathered) => Value::finish(gathered),
      Decision::Newer(kind) => Ok(Value::unrecognized(
        kind,
        KeptMembers::from_checked(kept).into_payload()?,
      )),
    }
  }
}

/// A member of a known kind: read when the kind reads it, refused when another known kind would, and passed by
/// otherwise, as any unknown member is.
fn take_member<'input, Value: KindRead, Members: MapAccess<'input>>(
  gathered: &mut Value::Gathered,
  name: &str,
  value: MemberValue<'_, Members>,
) -> Result<(), Members::Error> {
  if Value::reads(gathered, name) {
    return Value::gather(gathered, name, value);
  }
  if Value::MEMBERS.contains(&name) {
    return Err(Members::Error::custom(Value::MESSAGE));
  }
  match value {
    MemberValue::Live(members) => members.next_value::<IgnoredAny>().map(drop),
    MemberValue::Kept(_) => Ok(()),
  }
}

/// Writes the kind and the members that a catch-all kept, in the order read.
pub(crate) fn write_kept<Output: Serializer>(
  serializer: Output,
  kind: &str,
  members: &KeptMembers,
) -> Result<Output::Ok, Output::Error> {
  let mut map = serializer.serialize_map(None)?;
  map.serialize_entry(KIND_MEMBER, kind)?;
  for (name, value) in members.iter() {
    map.serialize_entry(name, value)?;
  }
  map.end()
}
