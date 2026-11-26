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

//! `TagSet`, the tags of one target (`api.md` section 6.9).

use super::{NativeScope, Tag, TargetLevel, TargetReference, TargetScope};
use crate::form_record::form_record;

form_record! {
  /// The tags of one target: its target level, four lists of target references, its scope, and the top-level tags.
  /// All four lists empty means the whole file, but only once the probe read the target structure in full, which the
  /// scope says.
  ///
  /// ```
  /// use cc_media_probe::MediaReport;
  ///
  /// fn tag_sets(report: &MediaReport) {
  ///     for set in report.tag_sets() {
  ///         println!(
  ///             "{} at level {:?}: {} streams, {} editions, {} chapters, {} attachments, native {}, superseded {}",
  ///             set.target_scope().as_str(),
  ///             set.target_level().and_then(|level| level.value()),
  ///             set.stream_targets().len(),
  ///             set.edition_targets().len(),
  ///             set.chapter_targets().len(),
  ///             set.attachment_targets().len(),
  ///             set.native_scope().is_some(),
  ///             set.is_superseded(),
  ///         );
  ///         for tag in set.tags() {
  ///             println!("  {}", tag.name().as_written());
  ///         }
  ///     }
  /// }
  /// ```
  #[derive(Clone, Debug, PartialEq, Eq, Hash)]
  pub struct TagSet {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    target_level: Option<TargetLevel>,
    stream_targets: Vec<TargetReference>,
    edition_targets: Vec<TargetReference>,
    chapter_targets: Vec<TargetReference>,
    attachment_targets: Vec<TargetReference>,
    target_scope: TargetScope,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    native_scope: Option<NativeScope>,
    #[cfg_attr(
      feature = "serde",
      serde(serialize_with = "crate::model::tree::write_flat", deserialize_with = "crate::model::tree::read_flat")
    )]
    tags: Vec<Tag>,
    superseded: bool,
  }
  check = TagSet::check_form;
}

impl TagSet {
  /// The level of the entity that the set describes.
  #[must_use]
  pub fn target_level(&self) -> Option<&TargetLevel> {
    self.target_level.as_ref()
  }

  /// The streams that the set targets, as kept.
  #[must_use]
  pub fn stream_targets(&self) -> &[TargetReference] {
    &self.stream_targets
  }

  /// The editions that the set targets, as kept.
  #[must_use]
  pub fn edition_targets(&self) -> &[TargetReference] {
    &self.edition_targets
  }

  /// The chapters that the set targets, as kept.
  #[must_use]
  pub fn chapter_targets(&self) -> &[TargetReference] {
    &self.chapter_targets
  }

  /// The attachments that the set targets, as kept.
  #[must_use]
  pub fn attachment_targets(&self) -> &[TargetReference] {
    &self.attachment_targets
  }

  /// What the target lists mean.
  #[must_use]
  pub fn target_scope(&self) -> &TargetScope {
    &self.target_scope
  }

  /// The ISO family's scope of the set.
  #[must_use]
  pub fn native_scope(&self) -> Option<&NativeScope> {
    self.native_scope.as_ref()
  }

  /// The top-level tags, in stored order.
  #[must_use]
  pub fn tags(&self) -> &[Tag] {
    &self.tags
  }

  /// Whether a later element replaced the one that holds this set, as in a live Matroska segment. The set stays as
  /// history, and no lookup sees it.
  #[must_use]
  pub fn is_superseded(&self) -> bool {
    self.superseded
  }

  /// Whether the scope agrees with the targets, as the review of the third slice's first part found a whole-file set
  /// beside a resolved stream target, which then took part in both `file_tags()` and `stream_tags()` (`api.md`
  /// section 6.9): the whole file names no target, entities name at least one, and a resolved target of each list
  /// names an entity of that list's kind. An incomplete set or a scope of a newer version holds any targets. Only a
  /// target of the kind `resolved` is held to its list's kind, as the second review of that part asked, since only
  /// this version knows what the location of a target of a newer kind means.
  #[cfg(feature = "serde")]
  fn check_form(&self) -> Result<(), &'static str> {
    use crate::model::{EntityKind, TargetReferenceKind};
    let lists = [
      (&self.stream_targets, EntityKind::Stream),
      (&self.edition_targets, EntityKind::Edition),
      (&self.chapter_targets, EntityKind::Chapter),
      (&self.attachment_targets, EntityKind::Attachment),
    ];
    let named = lists.iter().any(|(targets, _)| !targets.is_empty());
    let scoped = match self.target_scope {
      TargetScope::WholeFile => !named,
      TargetScope::Entities => named,
      _ => true,
    };
    let kinds = lists.iter().all(|(targets, kind)| {
      targets
        .iter()
        .filter(|target| matches!(target.kind(), TargetReferenceKind::Resolved))
        .filter_map(TargetReference::location)
        .all(|location| location.kind() == kind)
    });
    if scoped && kinds {
      Ok(())
    } else {
      Err(super::constants::TAG_SET_MESSAGE)
    }
  }
}

#[cfg(test)]
mod tests {
  use super::TagSet;
  use crate::model::{NativeScope, Tag, TagKey, TargetLevel, TargetReference, TargetScope};

  impl TagSet {
    /// A set of this scope with these stream targets and tags.
    pub(crate) fn sample(target_scope: TargetScope, stream_targets: Vec<TargetReference>, tags: Vec<Tag>) -> TagSet {
      TagSet {
        target_level: Some(TargetLevel::sample()),
        stream_targets,
        edition_targets: Vec::new(),
        chapter_targets: Vec::new(),
        attachment_targets: Vec::new(),
        target_scope,
        native_scope: None,
        tags,
        superseded: false,
      }
    }

    /// The same set, superseded by a later element.
    pub(crate) fn superseded(self) -> TagSet {
      TagSet {
        superseded: true,
        ..self
      }
    }
  }

  #[test]
  fn gives_each_field() {
    let set = TagSet {
      native_scope: Some(NativeScope {}),
      ..TagSet::sample(
        TargetScope::Entities,
        vec![TargetReference::stream(0)],
        vec![Tag::sample("TITLE", Some(TagKey::Title), Some("Main"), Vec::new())],
      )
    };
    assert_eq!(set.target_level(), Some(&TargetLevel::sample()));
    assert_eq!(set.stream_targets().len(), 1);
    assert_eq!(set.edition_targets(), []);
    assert_eq!(set.chapter_targets(), []);
    assert_eq!(set.attachment_targets(), []);
    assert_eq!(set.target_scope(), &TargetScope::Entities);
    assert!(set.native_scope().is_some());
    assert_eq!(set.tags().len(), 1);
    assert!(!set.is_superseded());
    assert!(set.superseded().is_superseded());
  }
}
