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

//! `Chapter`, one chapter of an edition (`api.md` section 6.8; field guide section 3.7).

use std::fmt;
use std::hash::{Hash, Hasher};

use super::SegmentReference;
use crate::Seconds;
use crate::model::tree::{Branching, copy, equal, format, hash, release};
use crate::model::{Identifier, LocalizedText, Provenance};

/// One chapter of an edition, with its subchapters. A chapter keeps two kinds of time apart: its `start()` and
/// `end()` on the timeline of the segment that holds its media, and, in an ordered edition, its
/// `playback_position()` on the edition's own timeline.
///
/// A probe bounds the nesting by the nesting-depth limit, and a document can hold a tree of any depth. No operation
/// on the tree recurses on its depth: dropping, cloning, comparing, hashing and `Debug` walk it with an explicit
/// stack, and a caller who walks it does the same.
///
/// ```
/// use cc_media_probe::model::{Chapter, ChapterEdition};
///
/// fn list_chapters(edition: &ChapterEdition) {
///     let mut pending: Vec<(usize, &Chapter)> = edition.chapters().iter().rev().map(|chapter| (0, chapter)).collect();
///     while let Some((depth, chapter)) = pending.pop() {
///         let title = chapter.titles().first().map_or("untitled", |title| title.text());
///         let start = chapter.start().map(|start| start.to_string()).unwrap_or_default();
///         println!(
///             "{:indent$}{title} at {start} ({:?}) to {:?} ({:?}), hidden {:?}, enabled {:?}, playback {:?}",
///             "",
///             chapter.start_provenance(),
///             chapter.end(),
///             chapter.end_provenance(),
///             chapter.is_hidden(),
///             chapter.is_enabled(),
///             chapter.playback_position(),
///             indent = depth * 2,
///         );
///         println!(
///             "{:indent$}identifiers {:?} {:?}, linked {}",
///             "",
///             chapter.unique_identifier().map(|identifier| identifier.as_str()),
///             chapter.string_identifier(),
///             chapter.segment_reference().is_some(),
///             indent = depth * 2,
///         );
///         pending.extend(chapter.children().iter().rev().map(|child| (depth + 1, child)));
///     }
/// }
/// ```
pub struct Chapter {
  node: ChapterNode,
  branches: Vec<Chapter>,
}

/// A chapter's own members, without the chapters below it.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct ChapterNode {
  unique_identifier: Option<Identifier>,
  string_identifier: Option<String>,
  start: Option<Seconds>,
  start_provenance: Option<Provenance>,
  end: Option<Seconds>,
  end_provenance: Option<Provenance>,
  segment_reference: Option<SegmentReference>,
  titles: Vec<LocalizedText>,
  hidden: Option<bool>,
  enabled: Option<bool>,
  playback_position: Option<Seconds>,
}

impl Chapter {
  /// The chapter's unique identifier, such as a Matroska `ChapterUID`.
  #[must_use]
  pub fn unique_identifier(&self) -> Option<&Identifier> {
    self.node.unique_identifier.as_ref()
  }

  /// A textual identifier as written, such as a Matroska `ChapterStringUID`.
  #[must_use]
  pub fn string_identifier(&self) -> Option<&str> {
    self.node.string_identifier.as_deref()
  }

  /// The start, on the timeline of the segment that holds the chapter's media.
  #[must_use]
  pub fn start(&self) -> Option<Seconds> {
    self.node.start
  }

  /// Where the start came from.
  #[must_use]
  pub fn start_provenance(&self) -> Option<&Provenance> {
    self.node.start_provenance.as_ref()
  }

  /// The end, on the same timeline; absent for a point without an end.
  #[must_use]
  pub fn end(&self) -> Option<Seconds> {
    self.node.end
  }

  /// Where the end came from.
  #[must_use]
  pub fn end_provenance(&self) -> Option<&Provenance> {
    self.node.end_provenance.as_ref()
  }

  /// The segment whose media the chapter plays, which the probe never opens.
  #[must_use]
  pub fn segment_reference(&self) -> Option<&SegmentReference> {
    self.node.segment_reference.as_ref()
  }

  /// The chapter's titles, with the languages and countries they apply to.
  #[must_use]
  pub fn titles(&self) -> &[LocalizedText] {
    &self.node.titles
  }

  /// Whether the chapter is hidden.
  #[must_use]
  pub fn is_hidden(&self) -> Option<bool> {
    self.node.hidden
  }

  /// Whether the chapter is enabled.
  #[must_use]
  pub fn is_enabled(&self) -> Option<bool> {
    self.node.enabled
  }

  /// The chapter's position on its ordered edition's playback timeline; absent in an edition that is not ordered.
  #[must_use]
  pub fn playback_position(&self) -> Option<Seconds> {
    self.node.playback_position
  }

  /// The chapters below this one, in stored order.
  #[must_use]
  pub fn children(&self) -> &[Chapter] {
    &self.branches
  }
}

impl Branching for Chapter {
  type Node = ChapterNode;

  fn node(&self) -> &ChapterNode {
    &self.node
  }

  fn branches(&self) -> &[Chapter] {
    &self.branches
  }

  fn branches_mut(&mut self) -> &mut Vec<Chapter> {
    &mut self.branches
  }

  fn from_parts(node: ChapterNode, branches: Vec<Chapter>) -> Chapter {
    Chapter { node, branches }
  }
}

impl Clone for Chapter {
  fn clone(&self) -> Chapter {
    copy(self)
  }
}

impl PartialEq for Chapter {
  fn eq(&self, other: &Chapter) -> bool {
    equal(self, other)
  }
}

impl Eq for Chapter {}

impl Hash for Chapter {
  fn hash<State: Hasher>(&self, state: &mut State) {
    hash(self, state);
  }
}

impl fmt::Debug for Chapter {
  /// Writes the nodes of the subtree in depth-first order, each with the position of its parent.
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    format(self, formatter)
  }
}

impl Drop for Chapter {
  fn drop(&mut self) {
    release(self);
  }
}

#[cfg(test)]
mod tests {
  use super::{Chapter, ChapterNode};
  use crate::Seconds;
  use crate::model::tree::Branching;
  use crate::model::{Identifier, Language, LanguageCodeSystem, LocalizedText, Provenance, SegmentReference};

  impl Chapter {
    /// A chapter titled `title` from `start` to `end` seconds, with these children.
    pub(crate) fn sample(title: &str, start: i64, end: i64, children: Vec<Chapter>) -> Chapter {
      let english = Language::sample("eng", Some("en"), LanguageCodeSystem::Iso639Alpha3);
      Chapter::from_parts(
        ChapterNode {
          unique_identifier: Some(Identifier::sample("1")),
          string_identifier: Some("intro".to_owned()),
          start: Some(Seconds::sample(start, 1)),
          start_provenance: Some(Provenance::Declared),
          end: Some(Seconds::sample(end, 1)),
          end_provenance: Some(Provenance::Declared),
          segment_reference: None,
          titles: vec![LocalizedText::sample(title, vec![english], &[])],
          hidden: Some(false),
          enabled: Some(true),
          playback_position: Some(Seconds::sample(start, 1)),
        },
        children,
      )
    }
  }

  #[test]
  fn gives_each_field_and_its_children() {
    let chapter = Chapter::sample("Prologue", 0, 90, vec![Chapter::sample("Opening", 0, 30, Vec::new())]);
    assert_eq!(chapter.unique_identifier().map(Identifier::as_str), Some("1"));
    assert_eq!(chapter.string_identifier(), Some("intro"));
    assert_eq!(chapter.start(), Some(Seconds::sample(0, 1)));
    assert_eq!(chapter.start_provenance(), Some(&Provenance::Declared));
    assert_eq!(chapter.end(), Some(Seconds::sample(90, 1)));
    assert_eq!(chapter.end_provenance(), Some(&Provenance::Declared));
    assert_eq!(chapter.segment_reference(), None);
    assert_eq!(chapter.titles()[0].text(), "Prologue");
    assert_eq!(chapter.is_hidden(), Some(false));
    assert_eq!(chapter.is_enabled(), Some(true));
    assert_eq!(chapter.playback_position(), Some(Seconds::sample(0, 1)));
    assert_eq!(chapter.children()[0].titles()[0].text(), "Opening");
    let linked = Chapter {
      node: ChapterNode {
        segment_reference: Some(SegmentReference::sample("00")),
        ..chapter.node.clone()
      },
      branches: Vec::new(),
    };
    assert!(linked.segment_reference().is_some());
  }

  #[test]
  fn clones_compares_and_shows_its_subtree() {
    let chapter = Chapter::sample("Prologue", 0, 90, vec![Chapter::sample("Opening", 0, 30, Vec::new())]);
    let copied = chapter.clone();
    assert_eq!(copied, chapter);
    assert_ne!(Chapter::sample("Prologue", 0, 90, Vec::new()), chapter);
    assert!(format!("{chapter:?}").starts_with("[(None, ChapterNode"));
    let set: std::collections::HashSet<Chapter> = [chapter, copied].into_iter().collect();
    assert_eq!(set.len(), 1);
  }
}
