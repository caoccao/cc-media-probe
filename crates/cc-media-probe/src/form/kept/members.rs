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

//! The members of a map kept as written, in their order. A member is present when its name is, whatever it holds,
//! and a map that names a member twice fails, as a struct of Serde's derive does. A kept member read into its type,
//! as one that stood before the kind of an enum that carries data, is read strictly, as from the input
//! (`super::reading::read_kept`).

use std::fmt;

use serde::de::{MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::value::KeptValue;
use crate::form::constants::{DUPLICATE_MEMBER_MESSAGE, KEPT_VALUE_EXPECTING};

/// The members of a map, in the order written.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub(crate) struct KeptMembers(Vec<(String, KeptValue)>);

impl KeptMembers {
  /// Reads the members of a map, refusing a name given twice.
  pub(crate) fn from_map<'input, Members: MapAccess<'input>>(
    mut members: Members,
  ) -> Result<KeptMembers, Members::Error> {
    let mut kept: Vec<(String, KeptValue)> = Vec::new();
    while let Some(member) = members.next_entry::<String, KeptValue>()? {
      kept.push(member);
    }
    KeptMembers::from_members(kept)
  }

  /// The members of a map that were read in that order, refusing a name given twice. The names are checked in a
  /// set apart from the order, so a large map of a newer version costs time linear in its members, as the review of
  /// the third slice's first part asked.
  pub(crate) fn from_members<Failure: serde::de::Error>(
    members: Vec<(String, KeptValue)>,
  ) -> Result<KeptMembers, Failure> {
    if crate::form::each_once(&members, |(name, _)| name.as_str()) {
      Ok(KeptMembers(members))
    } else {
      Err(Failure::custom(DUPLICATE_MEMBER_MESSAGE))
    }
  }

  /// Members whose names were already checked, each once.
  pub(super) fn from_checked(members: Vec<(String, KeptValue)>) -> KeptMembers {
    KeptMembers(members)
  }

  /// The members as a slice, in the order written.
  pub(crate) fn as_slice(&self) -> &[(String, KeptValue)] {
    &self.0
  }

  /// The members, in the order written.
  pub(crate) fn iter(&self) -> impl Iterator<Item = (&String, &KeptValue)> {
    self.0.iter().map(|(name, value)| (name, value))
  }
}

impl IntoIterator for KeptMembers {
  type Item = (String, KeptValue);
  type IntoIter = std::vec::IntoIter<(String, KeptValue)>;

  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}

impl Serialize for KeptMembers {
  fn serialize<Output: Serializer>(&self, serializer: Output) -> Result<Output::Ok, Output::Error> {
    serializer.collect_map(self.iter())
  }
}

impl<'input> Deserialize<'input> for KeptMembers {
  fn deserialize<Input: Deserializer<'input>>(deserializer: Input) -> Result<KeptMembers, Input::Error> {
    deserializer.deserialize_map(KeptMembersVisitor)
  }
}

/// Reads a map's members.
struct KeptMembersVisitor;

impl<'input> Visitor<'input> for KeptMembersVisitor {
  type Value = KeptMembers;

  fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(KEPT_VALUE_EXPECTING)
  }

  fn visit_map<Members: MapAccess<'input>>(self, members: Members) -> Result<KeptMembers, Members::Error> {
    KeptMembers::from_map(members)
  }
}
