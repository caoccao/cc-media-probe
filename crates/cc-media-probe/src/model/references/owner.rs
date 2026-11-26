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

//! `ReferenceOwner`, the entity that holds an external reference (`api.md` section 6.12). Serialized, it is the
//! members of its entity location with `sample_description_index` beside them (`json-output.md` section 7.11), as
//! the maintainer chose at the review of the third slice's first part, where the reader had refused that flat record
//! and the writer had nested the location: a stream, a chapter or an attachment, or an entity of a newer version, and
//! the index only for a stream, from 1.

#[cfg(feature = "serde")]
use crate::model::EntityKind;
use crate::model::EntityLocation;

/// The entity that holds an external reference: an attachment by index, a chapter by its edition index and child
/// path, or a stream by index with its sample description.
///
/// ```
/// use cc_media_probe::model::ExternalReference;
///
/// fn owner(reference: &ExternalReference) -> String {
///     reference.owner().map_or_else(
///         || "the whole file".to_owned(),
///         |owner| {
///             format!(
///                 "{} {} (description {:?})",
///                 owner.location().kind().as_str(),
///                 owner.location().index(),
///                 owner.sample_description_index(),
///             )
///         },
///     )
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ReferenceOwner {
  location: EntityLocation,
  sample_description_index: Option<u64>,
}

impl ReferenceOwner {
  /// The entity that holds the reference.
  #[must_use]
  pub fn location(&self) -> &EntityLocation {
    &self.location
  }

  /// For a stream's sample description, its index as the file numbers it.
  #[must_use]
  pub fn sample_description_index(&self) -> Option<u64> {
    self.sample_description_index
  }
}

/// An owner as written: the members of its location, then its sample description's index.
#[cfg(feature = "serde")]
#[derive(serde::Serialize)]
struct OwnerMembers<'owner> {
  kind: &'owner EntityKind,
  index: usize,
  chapter_path: &'owner [usize],
  #[serde(skip_serializing_if = "Option::is_none")]
  sample_description_index: Option<u64>,
}

/// An owner as read.
#[cfg(feature = "serde")]
#[derive(serde::Deserialize)]
struct OwnerForm {
  kind: EntityKind,
  index: usize,
  chapter_path: Vec<usize>,
  sample_description_index: Option<u64>,
}

#[cfg(feature = "serde")]
impl serde::Serialize for ReferenceOwner {
  fn serialize<Output: serde::Serializer>(&self, serializer: Output) -> Result<Output::Ok, Output::Error> {
    let members = OwnerMembers {
      kind: self.location.kind(),
      index: self.location.index(),
      chapter_path: self.location.chapter_path(),
      sample_description_index: self.sample_description_index,
    };
    serde::Serialize::serialize(&members, serializer)
  }
}

#[cfg(feature = "serde")]
impl TryFrom<OwnerForm> for ReferenceOwner {
  type Error = &'static str;

  /// The location, which names a chapter by its path, and the index of a stream's sample description, from 1. An
  /// owner holds its reference, so an edition and a program own none (`api.md` section 6.12); an entity of a newer
  /// version is not checked.
  fn try_from(form: OwnerForm) -> Result<ReferenceOwner, &'static str> {
    let fits = match form.kind {
      EntityKind::Stream => form.sample_description_index.is_none_or(|index| index >= 1),
      EntityKind::Chapter | EntityKind::Attachment => form.sample_description_index.is_none(),
      EntityKind::Unrecognized(_) => true,
      _ => false,
    };
    if !fits {
      return Err(super::constants::OWNER_MESSAGE);
    }
    Ok(ReferenceOwner {
      location: EntityLocation::from_members(form.kind, form.index, form.chapter_path)?,
      sample_description_index: form.sample_description_index,
    })
  }
}

#[cfg(feature = "serde")]
crate::form::checked_form!(ReferenceOwner from OwnerForm);

#[cfg(test)]
mod tests {
  use super::ReferenceOwner;
  use crate::model::{EntityKind, EntityLocation};

  impl ReferenceOwner {
    /// The sample description `index` of the stream at `stream`.
    pub(crate) fn sample(stream: usize, index: u64) -> ReferenceOwner {
      ReferenceOwner {
        location: EntityLocation::sample(EntityKind::Stream, stream, &[]),
        sample_description_index: Some(index),
      }
    }
  }

  #[test]
  fn locates_the_owner() {
    let owner = ReferenceOwner::sample(1, 2);
    assert_eq!(owner.location().index(), 1);
    assert_eq!(owner.sample_description_index(), Some(2));
  }
}
