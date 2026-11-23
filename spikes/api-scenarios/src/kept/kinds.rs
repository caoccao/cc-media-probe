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

//! The map of an enum that carries data, `api.md` section 12: its members kept as written until the kind is known,
//! and the rule that a known kind leaves no member of another known kind behind. A variant without a value reads
//! `kind` alone; a variant with one reads the member named after its kind.

use serde::de::Error as _;
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serializer};

use super::members::KeptMembers;
use crate::constants::KIND_MEMBER;

/// The kind of a map and its other members, as written. A map without a kind fails with `message`.
pub(crate) fn read_kind<'input, Input: Deserializer<'input>>(
  deserializer: Input,
  message: &'static str,
) -> Result<(String, KeptMembers), Input::Error> {
  let mut members = KeptMembers::deserialize(deserializer)?;
  let kind: String = members
    .take::<_, Input::Error>(KIND_MEMBER)?
    .ok_or_else(|| Input::Error::custom(message))?;
  Ok((kind, members))
}

/// Fails with `message` when a member that a known kind reads is still there, once the kind took its own.
pub(crate) fn refuse_other_kinds<Failure: serde::de::Error>(
  members: &KeptMembers,
  known_members: &[&str],
  message: &'static str,
) -> Result<(), Failure> {
  if known_members.iter().any(|name| members.contains(name)) {
    return Err(Failure::custom(message));
  }
  Ok(())
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
