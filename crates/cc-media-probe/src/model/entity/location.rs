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

//! `EntityLocation` and `EntityKind` (`api.md` sections 6.9 and 11.3).

use super::constants::{ATTACHMENT_NAME, CHAPTER_NAME, EDITION_NAME, PROGRAM_NAME, STREAM_NAME};
use crate::string_enumeration::string_enumeration;

/// The location of an entity of the report: a stream, an edition, a chapter, an attachment or a program. A tag
/// target that resolves names its entity this way, and so do the entries of the typed details that name an owner.
///
/// ```
/// use cc_media_probe::MediaReport;
/// use cc_media_probe::model::{EntityKind, EntityLocation};
///
/// fn describe(report: &MediaReport, location: &EntityLocation) -> String {
///     match location.kind() {
///         EntityKind::Stream => format!("stream {}", location.index()),
///         EntityKind::Chapter => {
///             let title = report
///                 .chapter_editions()
///                 .get(location.index())
///                 .and_then(|edition| edition.chapter_at(location.chapter_path()))
///                 .and_then(|chapter| chapter.titles().first())
///                 .map_or("untitled", |title| title.text());
///             format!("chapter {title} of edition {}", location.index())
///         }
///         kind => format!("{} {}", kind.as_str(), location.index()),
///     }
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct EntityLocation {
  kind: EntityKind,
  index: usize,
  chapter_path: Vec<usize>,
}

impl EntityLocation {
  /// The kind of entity.
  #[must_use]
  pub fn kind(&self) -> &EntityKind {
    &self.kind
  }

  /// The index of the stream, edition, attachment or program, or for a chapter, the index of its edition.
  #[must_use]
  pub fn index(&self) -> usize {
    self.index
  }

  /// For a chapter, the path of child indexes that `ChapterEdition::chapter_at` takes; empty otherwise.
  #[must_use]
  pub fn chapter_path(&self) -> &[usize] {
    &self.chapter_path
  }
}

string_enumeration! {
  /// The kind of an entity of the report. `EntityLocation` shows an example.
  ///
  /// ```
  /// use cc_media_probe::model::EntityKind;
  ///
  /// assert_eq!(EntityKind::Attachment.as_str(), "attachment");
  /// ```
  EntityKind {
    /// A stream, by its index.
    Stream => STREAM_NAME,
    /// A chapter edition, by its index.
    Edition => EDITION_NAME,
    /// A chapter, by its edition's index and its path.
    Chapter => CHAPTER_NAME,
    /// An attachment, by its index.
    Attachment => ATTACHMENT_NAME,
    /// A program, by its index.
    Program => PROGRAM_NAME,
  }
}

#[cfg(test)]
mod tests {
  use super::{EntityKind, EntityLocation};
  use crate::model::UnrecognizedValue;

  impl EntityLocation {
    /// The entity of this kind at this index, with this chapter path.
    pub(crate) fn sample(kind: EntityKind, index: usize, chapter_path: &[usize]) -> EntityLocation {
      EntityLocation {
        kind,
        index,
        chapter_path: chapter_path.to_vec(),
      }
    }
  }

  #[test]
  fn locates_its_entity() {
    let chapter = EntityLocation::sample(EntityKind::Chapter, 0, &[2, 1]);
    assert_eq!(chapter.kind(), &EntityKind::Chapter);
    assert_eq!(chapter.index(), 0);
    assert_eq!(chapter.chapter_path(), &[2, 1]);
    for (kind, name) in [
      (EntityKind::Stream, "stream"),
      (EntityKind::Edition, "edition"),
      (EntityKind::Chapter, "chapter"),
      (EntityKind::Attachment, "attachment"),
      (EntityKind::Program, "program"),
      (
        EntityKind::Unrecognized(UnrecognizedValue::sample("segment")),
        "segment",
      ),
    ] {
      assert_eq!(kind.as_str(), name);
    }
  }
}
