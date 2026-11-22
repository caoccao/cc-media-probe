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

//! Chapter editions and chapters with their two timelines, `api.md` section 6.8.

use super::language::LocalizedText;
use super::values::{Identifier, Seconds};

/// One edition of chapters.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChapterEdition {
  unique_identifier: Option<Identifier>,
  default: Option<bool>,
  ordered: Option<bool>,
  hidden: Option<bool>,
  playback_duration: Option<Seconds>,
  chapters: Vec<Chapter>,
}

impl ChapterEdition {
  /// The edition's identifier.
  pub fn unique_identifier(&self) -> Option<&Identifier> {
    self.unique_identifier.as_ref()
  }

  /// Whether it is the default edition.
  pub fn is_default(&self) -> Option<bool> {
    self.default
  }

  /// Whether it is an ordered edition, which has playback time.
  pub fn is_ordered(&self) -> Option<bool> {
    self.ordered
  }

  /// Whether it is hidden.
  pub fn is_hidden(&self) -> Option<bool> {
    self.hidden
  }

  /// How long an ordered edition plays.
  pub fn playback_duration(&self) -> Option<Seconds> {
    self.playback_duration
  }

  /// The top-level chapters, in stored order.
  pub fn chapters(&self) -> &[Chapter] {
    &self.chapters
  }

  /// The chapter at `path`, the child indexes from the top level down; `None` for an empty or unknown path.
  pub fn chapter_at(&self, path: &[usize]) -> Option<&Chapter> {
    let (first, rest) = path.split_first()?;
    rest.iter().try_fold(self.chapters.get(*first)?, |chapter, index| {
      chapter.children.get(*index)
    })
  }
}

/// One chapter.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Chapter {
  unique_identifier: Option<Identifier>,
  start: Option<Seconds>,
  end: Option<Seconds>,
  titles: Vec<LocalizedText>,
  hidden: Option<bool>,
  enabled: Option<bool>,
  playback_position: Option<Seconds>,
  children: Vec<Chapter>,
}

impl Chapter {
  /// The chapter's identifier.
  pub fn unique_identifier(&self) -> Option<&Identifier> {
    self.unique_identifier.as_ref()
  }

  /// The start, in segment time.
  pub fn start(&self) -> Option<Seconds> {
    self.start
  }

  /// The end, in segment time; absent for a chapter that marks a point.
  pub fn end(&self) -> Option<Seconds> {
    self.end
  }

  /// The titles with their languages.
  pub fn titles(&self) -> &[LocalizedText] {
    &self.titles
  }

  /// Whether the chapter is hidden.
  pub fn is_hidden(&self) -> Option<bool> {
    self.hidden
  }

  /// Whether the chapter is enabled.
  pub fn is_enabled(&self) -> Option<bool> {
    self.enabled
  }

  /// The position on the playback timeline of an ordered edition.
  pub fn playback_position(&self) -> Option<Seconds> {
    self.playback_position
  }

  /// The nested chapters.
  pub fn children(&self) -> &[Chapter] {
    &self.children
  }
}
