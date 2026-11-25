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

//! `ReferenceOwner`, the entity that holds an external reference (`api.md` section 6.12).

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
