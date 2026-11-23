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

//! The iterator of the flat tag lookups, `api.md` section 6.9. The specification names one type per method
//! (C-ITER-TY); the stub shares one. Every lookup sees the top level only: a nested tag is an attribute of its parent
//! (Matroska tags draft, section 3), so a key below the top level does not mean what it means there (tenth review).

use super::tags::{Tag, TagKey, TagSet};

/// The top-level tags of some tag sets, the sets in report order and each set's tags in stored order.
#[derive(Clone, Debug)]
pub struct TagsOf<'report> {
  tags: std::vec::IntoIter<&'report Tag>,
}

impl<'report> TagsOf<'report> {
  /// The top-level tags of `tag_sets`.
  pub(super) fn new(tag_sets: impl Iterator<Item = &'report TagSet>) -> TagsOf<'report> {
    let tags: Vec<&Tag> = tag_sets.flat_map(TagSet::tags).collect();
    TagsOf { tags: tags.into_iter() }
  }

  /// The top-level tags of `tag_sets` that have `key`, with a value or without one.
  pub(super) fn with_key(tag_sets: impl Iterator<Item = &'report TagSet>, key: &TagKey) -> TagsOf<'report> {
    let tags: Vec<&Tag> = tag_sets
      .flat_map(TagSet::tags)
      .filter(|tag| tag.key() == Some(key))
      .collect();
    TagsOf { tags: tags.into_iter() }
  }
}

impl<'report> Iterator for TagsOf<'report> {
  type Item = &'report Tag;

  fn next(&mut self) -> Option<&'report Tag> {
    self.tags.next()
  }
}
