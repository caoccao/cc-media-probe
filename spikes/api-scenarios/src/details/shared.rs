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

//! The detail types that every format and codec shares, `api.md` sections 11.1 to 11.3.

use crate::model::{EntityKind, EntityLocation};

/// An element that a specification gives a default: the effective value, and whether the file stored it (F1).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StoredValue<Value> {
  value: Value,
  stored: bool,
}

impl<Value> StoredValue<Value> {
  /// The effective value: the stored one, or the default.
  pub fn value(&self) -> &Value {
    &self.value
  }

  /// Whether the file stored the element.
  pub fn is_stored(&self) -> bool {
    self.stored
  }
}

/// Two or more sources that disagree about one field.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Conflict {
  field: String,
  owner: Option<EntityLocation>,
  values: Vec<ConflictingValue>,
}

impl Conflict {
  /// The field guide's name of the field.
  pub fn field(&self) -> &str {
    &self.field
  }

  /// The stream, edition, chapter, attachment or program whose field it is; absent for a field of the whole report.
  pub fn owner(&self) -> Option<&EntityLocation> {
    self.owner.as_ref()
  }

  /// The owner's index, when the owner is a stream.
  pub fn stream(&self) -> Option<usize> {
    self
      .owner
      .as_ref()
      .filter(|owner| matches!(owner.kind(), EntityKind::Stream))
      .map(EntityLocation::index)
  }

  /// Each source's value.
  pub fn values(&self) -> &[ConflictingValue] {
    &self.values
  }
}

/// One source's value in a conflict.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ConflictingValue {
  source: String,
  value: String,
}

impl ConflictingValue {
  /// The name of the source, such as an element's name.
  pub fn source(&self) -> &str {
    &self.source
  }

  /// The value as text.
  pub fn value(&self) -> &str {
    &self.value
  }
}

/// The details of a kind that a newer version wrote.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UnrecognizedDetails {
  kind: String,
}

impl UnrecognizedDetails {
  /// The kind name as written.
  pub fn kind(&self) -> &str {
    &self.kind
  }
}
