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

//! `FileTags`, `StreamTags` and `TagsWithKey`, the iterators of the tag lookups (`api.md` section 6.9).

use super::selection::{OfStream, TopLevelTags, WholeFile, WithKey};
use crate::model::{Tag, TagKey, TagSet};

/// The top-level tags of every tag set whose scope is `WholeFile`, from `MediaReport::file_tags()`, skipping the
/// superseded sets, in report order and each set's stored order.
///
/// ```
/// use cc_media_probe::MediaReport;
///
/// fn file_tag_names(report: &MediaReport) -> Vec<&str> {
///     report.file_tags().map(|tag| tag.name().as_written()).collect()
/// }
/// ```
#[derive(Clone, Debug)]
pub struct FileTags<'report>(TopLevelTags<'report, WholeFile>);

/// The top-level tags of every tag set that targets one stream, directly or by a wildcard, from
/// `MediaReport::stream_tags()`, skipping the superseded sets. A set of the scope `Incomplete` takes part through
/// the targets it kept.
///
/// ```
/// use cc_media_probe::MediaReport;
///
/// fn stream_tag_count(report: &MediaReport, index: usize) -> usize {
///     report.stream_tags(index).count()
/// }
/// ```
#[derive(Clone, Debug)]
pub struct StreamTags<'report>(TopLevelTags<'report, OfStream>);

/// Every top-level tag with one key, at any target and of any scope, with a value or without one, from
/// `MediaReport::tags_with_key()`, skipping the superseded sets.
///
/// ```
/// use cc_media_probe::MediaReport;
/// use cc_media_probe::model::TagKey;
///
/// fn artists(report: &MediaReport) -> usize {
///     report.tags_with_key(&TagKey::Artist).filter(|tag| tag.value().is_some()).count()
/// }
/// ```
#[derive(Clone, Debug)]
pub struct TagsWithKey<'report>(TopLevelTags<'report, WithKey>);

impl<'report> FileTags<'report> {
  /// The lookup over `sets`.
  pub(super) fn new(sets: &'report [TagSet]) -> FileTags<'report> {
    FileTags(TopLevelTags::new(sets, WholeFile))
  }
}

impl<'report> StreamTags<'report> {
  /// The lookup over `sets` of the stream at `index`.
  pub(super) fn new(sets: &'report [TagSet], index: usize) -> StreamTags<'report> {
    StreamTags(TopLevelTags::new(sets, OfStream(index)))
  }
}

impl<'report> TagsWithKey<'report> {
  /// The lookup over `sets` of the key `key`.
  pub(super) fn new(sets: &'report [TagSet], key: &TagKey) -> TagsWithKey<'report> {
    TagsWithKey(TopLevelTags::new(sets, WithKey(key.clone())))
  }
}

impl<'report> Iterator for FileTags<'report> {
  type Item = &'report Tag;

  fn next(&mut self) -> Option<&'report Tag> {
    self.0.next_tag()
  }
}

impl<'report> Iterator for StreamTags<'report> {
  type Item = &'report Tag;

  fn next(&mut self) -> Option<&'report Tag> {
    self.0.next_tag()
  }
}

impl<'report> Iterator for TagsWithKey<'report> {
  type Item = &'report Tag;

  fn next(&mut self) -> Option<&'report Tag> {
    self.0.next_tag()
  }
}
