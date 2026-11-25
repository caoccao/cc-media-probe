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

//! The walk that every tag lookup shares (`api.md` section 6.9): the top-level tags of the tag sets that are not
//! superseded, in report order and each set's stored order, of the sets and tags that a selection takes.

use crate::model::{EntityKind, Tag, TagKey, TagSet, TargetReferenceKind, TargetScope};

/// Which sets and which of their top-level tags a lookup takes.
pub(super) trait Selection {
  /// Whether the lookup takes the tags of `set`, which is not superseded.
  fn takes_set(&self, set: &TagSet) -> bool;

  /// Whether the lookup takes `tag`, a top-level tag of a set that it takes.
  fn takes_tag(&self, tag: &Tag) -> bool;
}

/// The top-level tags of every set whose scope is `WholeFile`.
#[derive(Clone, Debug)]
pub(super) struct WholeFile;

impl Selection for WholeFile {
  fn takes_set(&self, set: &TagSet) -> bool {
    matches!(set.target_scope(), TargetScope::WholeFile)
  }

  fn takes_tag(&self, _tag: &Tag) -> bool {
    true
  }
}

/// The top-level tags of every set that targets one stream, directly or by a wildcard.
#[derive(Clone, Debug)]
pub(super) struct OfStream(pub(super) usize);

impl Selection for OfStream {
  fn takes_set(&self, set: &TagSet) -> bool {
    set.stream_targets().iter().any(|target| match target.kind() {
      TargetReferenceKind::Wildcard => true,
      TargetReferenceKind::Resolved => target
        .location()
        .is_some_and(|location| matches!(location.kind(), EntityKind::Stream) && location.index() == self.0),
      _ => false,
    })
  }

  fn takes_tag(&self, _tag: &Tag) -> bool {
    true
  }
}

/// Every top-level tag with one key, at any target and of any scope.
#[derive(Clone, Debug)]
pub(super) struct WithKey(pub(super) TagKey);

impl Selection for WithKey {
  fn takes_set(&self, _set: &TagSet) -> bool {
    true
  }

  fn takes_tag(&self, tag: &Tag) -> bool {
    tag.key() == Some(&self.0)
  }
}

/// The walk of a lookup: the sets still to visit, the tags of the set being visited, and the selection.
#[derive(Clone, Debug)]
pub(super) struct TopLevelTags<'report, Chosen> {
  sets: std::slice::Iter<'report, TagSet>,
  tags: std::slice::Iter<'report, Tag>,
  selection: Chosen,
}

impl<'report, Chosen: Selection> TopLevelTags<'report, Chosen> {
  /// The walk over `sets` that `selection` chooses.
  pub(super) fn new(sets: &'report [TagSet], selection: Chosen) -> TopLevelTags<'report, Chosen> {
    TopLevelTags {
      sets: sets.iter(),
      tags: [].iter(),
      selection,
    }
  }

  /// The next tag that the selection takes, or `None` once every set was visited.
  pub(super) fn next_tag(&mut self) -> Option<&'report Tag> {
    loop {
      if let Some(tag) = self.tags.find(|tag| self.selection.takes_tag(tag)) {
        return Some(tag);
      }
      let set = self
        .sets
        .find(|set| !set.is_superseded() && self.selection.takes_set(set))?;
      self.tags = set.tags().iter();
    }
  }
}
