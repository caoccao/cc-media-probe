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

//! `ChapterEdition` and `ChapterOrigin`, one edition of chapters (`api.md` section 6.8; F18).

use super::Chapter;
use super::constants::{MATROSKA_EDITION_NAME, QUICKTIME_CHAPTER_TRACK_NAME};
use crate::Seconds;
use crate::form_record::form_record;
use crate::model::{Identifier, Provenance};
use crate::string_enumeration::string_enumeration;

form_record! {
  /// One edition of chapters, from `MediaReport::chapter_editions()`. An ordered edition has a playback timeline of
  /// its own, with a `playback_duration()`; the report's `duration()` is the segment's and is never replaced by it.
  ///
  /// ```
  /// use cc_media_probe::MediaReport;
  ///
  /// fn editions(report: &MediaReport) {
  ///     for edition in report.chapter_editions() {
  ///         println!(
  ///             "{} edition {:?}: default {:?}, ordered {:?}, hidden {:?}, linked {:?}, {} chapters",
  ///             edition.origin().as_str(),
  ///             edition.unique_identifier().map(|identifier| identifier.as_str()),
  ///             edition.is_default(),
  ///             edition.is_ordered(),
  ///             edition.is_hidden(),
  ///             edition.has_linked_chapters(),
  ///             edition.chapters().len(),
  ///         );
  ///         if let Some(duration) = edition.playback_duration() {
  ///             println!("  plays for {duration} ({:?})", edition.playback_duration_provenance());
  ///         }
  ///         if let Some(second) = edition.chapter_at(&[0, 1]) {
  ///             println!("  the second subchapter of the first chapter starts at {:?}", second.start());
  ///         }
  ///     }
  /// }
  /// ```
  #[derive(Clone, Debug, PartialEq, Eq, Hash)]
  pub struct ChapterEdition {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    unique_identifier: Option<Identifier>,
    origin: ChapterOrigin,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    default: Option<bool>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    ordered: Option<bool>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    hidden: Option<bool>,
    #[cfg_attr(
      feature = "serde",
      serde(rename = "has_linked_chapters", skip_serializing_if = "Option::is_none")
    )]
    linked_chapters: Option<bool>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    playback_duration: Option<Seconds>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    playback_duration_provenance: Option<Provenance>,
    #[cfg_attr(
      feature = "serde",
      serde(serialize_with = "crate::model::tree::write_flat", deserialize_with = "crate::model::tree::read_flat")
    )]
    chapters: Vec<Chapter>,
  }
  pairs = [
    playback_duration with playback_duration_provenance,
  ];
  check = ChapterEdition::check_form;
}

impl ChapterEdition {
  /// The edition's unique identifier; absent for structures without one (F18).
  #[must_use]
  pub fn unique_identifier(&self) -> Option<&Identifier> {
    self.unique_identifier.as_ref()
  }

  /// The structure that the edition came from (F18).
  #[must_use]
  pub fn origin(&self) -> &ChapterOrigin {
    &self.origin
  }

  /// Whether the edition is the default one.
  #[must_use]
  pub fn is_default(&self) -> Option<bool> {
    self.default
  }

  /// Whether the edition is ordered, so that its chapters define a playback timeline of their own.
  #[must_use]
  pub fn is_ordered(&self) -> Option<bool> {
    self.ordered
  }

  /// Whether the edition is hidden.
  #[must_use]
  pub fn is_hidden(&self) -> Option<bool> {
    self.hidden
  }

  /// Whether a chapter links the media of another segment; `None` is unknown in the field guide's sense.
  #[must_use]
  pub fn has_linked_chapters(&self) -> Option<bool> {
    self.linked_chapters
  }

  /// The length of an ordered edition's playback timeline.
  #[must_use]
  pub fn playback_duration(&self) -> Option<Seconds> {
    self.playback_duration
  }

  /// Where the playback duration came from.
  #[must_use]
  pub fn playback_duration_provenance(&self) -> Option<&Provenance> {
    self.playback_duration_provenance.as_ref()
  }

  /// The top-level chapters, in stored order.
  #[must_use]
  pub fn chapters(&self) -> &[Chapter] {
    &self.chapters
  }

  /// The chapter that a path of child indexes names, from the top level down, such as the path of a tag target's
  /// location; `None` for an empty path and for a path that names no chapter.
  #[must_use]
  pub fn chapter_at(&self, path: &[usize]) -> Option<&Chapter> {
    let (first, rest) = path.split_first()?;
    rest.iter().try_fold(self.chapters.get(*first)?, |chapter, index| {
      chapter.children().get(*index)
    })
  }

  /// Whether the edition's playback values and its link flag agree with its chapters, as the fourth review of the
  /// third slice's first part asked, which read an unordered edition with playback positions and a derived playback
  /// duration, and an edition said to link no chapter beside a chapter of another segment. An edition stated as
  /// unordered has no playback timeline, so neither a playback duration nor a chapter's playback position (`api.md`
  /// section 6.8), and one whose chapters link no other segment is not said to when a chapter does, since one link
  /// read is enough to say yes (field guide section 3.7). The chapters are walked with an explicit stack.
  #[cfg(feature = "serde")]
  fn check_form(&self) -> Result<(), &'static str> {
    let chapters = crate::model::tree::depth_first(&self.chapters);
    let unordered = self.ordered == Some(false);
    let playback = self.playback_duration.is_some()
      || chapters
        .iter()
        .any(|(_, chapter)| chapter.playback_position().is_some());
    let linked = chapters
      .iter()
      .any(|(_, chapter)| chapter.segment_reference().is_some());
    if (unordered && playback) || (self.linked_chapters == Some(false) && linked) {
      Err(super::constants::EDITION_MESSAGE)
    } else {
      Ok(())
    }
  }
}

string_enumeration! {
  /// The structure that an edition of chapters came from (F18). Later phases add their structures.
  ///
  /// ```
  /// use cc_media_probe::model::ChapterOrigin;
  ///
  /// assert_eq!(ChapterOrigin::MatroskaEdition.as_str(), "matroska_edition");
  /// ```
  ChapterOrigin {
    /// A Matroska `EditionEntry`.
    MatroskaEdition => MATROSKA_EDITION_NAME,
    /// A QuickTime chapter track.
    QuickTimeChapterTrack => QUICKTIME_CHAPTER_TRACK_NAME,
  }
}

#[cfg(test)]
mod tests {
  use super::{ChapterEdition, ChapterOrigin};
  use crate::Seconds;
  use crate::model::{Chapter, Identifier, Provenance, UnrecognizedValue};

  impl ChapterEdition {
    /// An ordered Matroska edition of two chapters, the first with two subchapters.
    pub(crate) fn sample() -> ChapterEdition {
      ChapterEdition {
        unique_identifier: Some(Identifier::sample("7")),
        origin: ChapterOrigin::MatroskaEdition,
        default: Some(true),
        ordered: Some(true),
        hidden: Some(false),
        linked_chapters: Some(false),
        playback_duration: Some(Seconds::sample(180, 1)),
        playback_duration_provenance: Some(Provenance::Derived),
        chapters: vec![
          Chapter::sample(
            "Prologue",
            0,
            90,
            vec![
              Chapter::sample("Opening", 0, 30, Vec::new()),
              Chapter::sample("Credits", 30, 90, Vec::new()),
            ],
          ),
          Chapter::sample("Story", 90, 180, Vec::new()),
        ],
      }
    }
  }

  #[test]
  fn gives_each_field() {
    let edition = ChapterEdition::sample();
    assert_eq!(edition.unique_identifier().map(Identifier::as_str), Some("7"));
    assert_eq!(edition.origin(), &ChapterOrigin::MatroskaEdition);
    assert_eq!(edition.is_default(), Some(true));
    assert_eq!(edition.is_ordered(), Some(true));
    assert_eq!(edition.is_hidden(), Some(false));
    assert_eq!(edition.has_linked_chapters(), Some(false));
    assert_eq!(edition.playback_duration(), Some(Seconds::sample(180, 1)));
    assert_eq!(edition.playback_duration_provenance(), Some(&Provenance::Derived));
    assert_eq!(edition.chapters().len(), 2);
  }

  #[test]
  fn finds_a_chapter_by_its_path() {
    let edition = ChapterEdition::sample();
    let title = |path: &[usize]| {
      edition
        .chapter_at(path)
        .map(|chapter| chapter.titles()[0].text().to_owned())
    };
    assert_eq!(title(&[0]), Some("Prologue".to_owned()));
    assert_eq!(title(&[0, 1]), Some("Credits".to_owned()));
    assert_eq!(title(&[1]), Some("Story".to_owned()));
    assert_eq!(title(&[2]), None);
    assert_eq!(title(&[1, 0]), None);
    assert_eq!(title(&[]), None);
  }

  #[test]
  fn names_each_origin() {
    assert_eq!(ChapterOrigin::MatroskaEdition.as_str(), "matroska_edition");
    assert_eq!(ChapterOrigin::QuickTimeChapterTrack.as_str(), "quicktime_chapter_track");
    assert_eq!(
      ChapterOrigin::Unrecognized(UnrecognizedValue::sample("flac_cue_sheet")).as_str(),
      "flac_cue_sheet"
    );
  }
}
