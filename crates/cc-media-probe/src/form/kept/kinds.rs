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

//! The map of an enum that carries data and of a typed detail, `api.md` section 12, read as its members come, as the
//! nineteenth review asked: a member before `kind` is kept as written, since only the kind says how to read it, and
//! once the kind is known, a kind that this version knows reads its members from the input as they come and passes
//! by every other member, while a kind of a newer version keeps them all, unread, as its payload. A known kind leaves
//! no member of another known kind. The review's `{"kind": "integer", "integer": "oops", "extension": null}` failed on
//! the later null while the whole map was kept, and without it the error of the integer had no place in the input.

use std::fmt;
use std::marker::PhantomData;

use serde::de::{DeserializeOwned, Error, IgnoredAny, IntoDeserializer, MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserializer, Serializer};

use super::members::KeptMembers;
use super::reading::read_kept;
use super::value::KeptValue;
use crate::form::constants::{KEPT_VALUE_EXPECTING, KIND_MEMBER, KIND_MESSAGE};

/// An enum that carries data, read from the map of its kind. The enum macros implement it; a typed detail has no
/// kind that this version knows, so it takes the defaults, which read none.
pub(crate) trait KindRead: Sized {
  /// The serialized names of the kinds that this version knows.
  const KINDS: &'static [&'static str] = &[];

  /// Whether the known kind reads the member of that name.
  fn reads(_kind: &str, _name: &str) -> bool {
    false
  }

  /// Reads the value of a member that the known kind reads into the value gathered so far.
  fn gather<'input, Members: MapAccess<'input>>(
    _kind: &'static str,
    _gathered: &mut Option<Self>,
    _name: &str,
    _value: MemberValue<'_, Members>,
  ) -> Result<(), Members::Error> {
    Ok(())
  }

  /// The value of a known kind once the map has ended: a kind without a value, or the value gathered, or `None`
  /// when the member that the kind names is missing.
  fn finish(_kind: &'static str, _gathered: Option<Self>) -> Option<Self> {
    None
  }

  /// A kind of a newer version, with its other members as written.
  fn unrecognized(kind: String, members: KeptMembers) -> Self;
}

/// A member's value: the next one of the input, or one kept before the kind.
pub(crate) enum MemberValue<'access, Members> {
  /// The value that the map holds next.
  Live(&'access mut Members),
  /// A value kept as written, read strictly into its type (`super::reading`).
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
enum Decision<Value> {
  /// No kind read yet: every member is kept.
  Undecided,
  /// A kind that this version knows, with what its members gave so far.
  Known(&'static str, Option<Value>),
  /// A kind of a newer version: every member is kept as its payload.
  Newer(String),
}

/// Reads the map of a kind.
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
        decision = match Value::KINDS.iter().copied().find(|known| *known == kind) {
          Some(known) => {
            let mut gathered = None;
            for (name, value) in std::mem::take(&mut kept) {
              take_member::<Value, Members>(known, &mut gathered, &name, MemberValue::Kept(value))?;
            }
            Decision::Known(known, gathered)
          }
          None => Decision::Newer(kind),
        };
        continue;
      }
      match &mut decision {
        Decision::Known(kind, gathered) => {
          take_member::<Value, Members>(kind, gathered, &name, MemberValue::Live(&mut members))?;
        }
        Decision::Undecided | Decision::Newer(_) => kept.push((name, members.next_value()?)),
      }
    }
    match decision {
      Decision::Undecided => Err(Members::Error::custom(KIND_MESSAGE)),
      Decision::Known(kind, gathered) => {
        Value::finish(kind, gathered).ok_or_else(|| Members::Error::custom(KIND_MESSAGE))
      }
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
  kind: &'static str,
  gathered: &mut Option<Value>,
  name: &str,
  value: MemberValue<'_, Members>,
) -> Result<(), Members::Error> {
  if Value::reads(kind, name) {
    if gathered.is_some() {
      return Err(Members::Error::custom(KIND_MESSAGE));
    }
    return Value::gather(kind, gathered, name, value);
  }
  if Value::KINDS.iter().any(|other| Value::reads(other, name)) {
    return Err(Members::Error::custom(KIND_MESSAGE));
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

#[cfg(test)]
mod tests {
  use serde::de::value::{Error, MapDeserializer};

  use super::{KindRead, MemberValue};
  use crate::details::FormatDetails;
  use crate::form::kept::value::KeptValue;

  #[test]
  fn a_typed_detail_knows_no_kind_of_its_own_yet() {
    assert_eq!(FormatDetails::KINDS, &[] as &[&str]);
    assert!(!FormatDetails::reads("matroska", "segment_uid"));
    let mut gathered = None;
    let value: MemberValue<'_, MapDeserializer<'_, std::vec::IntoIter<(KeptValue, KeptValue)>, Error>> =
      MemberValue::Kept(KeptValue::Boolean(true));
    assert!(FormatDetails::gather("matroska", &mut gathered, "flag", value).is_ok());
    assert!(FormatDetails::finish("matroska", gathered).is_none());
  }
}
