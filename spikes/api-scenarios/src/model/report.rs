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

//! `MediaReport`, `api.md` section 6.2, with the tag lookups of section 6.9.

use super::attachments::Attachment;
use super::chapters::ChapterEdition;
use super::format::Format;
use super::lookups::TagsOf;
use super::stream::Stream;
use super::tags::{Tag, TagKey, TagSet, TargetScope};
use super::usage::Usage;
use super::values::{Estimate, MediaType, Provenance, Rational, Seconds};
use crate::completeness::Completeness;
use crate::details::FormatDetails;
use crate::diagnostics::Diagnostic;
#[cfg(feature = "serde")]
use crate::document::SchemaVersion;
use crate::source::SourceEntry;

/// What a probe established about one input, a root record: serialized, it carries its schema version. Reading one
/// checks that its headers status agrees with the header states of its streams (`model/report_reading.rs`).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(
  feature = "serde",
  derive(serde::Serialize, serde::Deserialize),
  serde(try_from = "crate::neutral::Rooted<super::report_reading::ReportData>")
)]
pub struct MediaReport {
  #[cfg(feature = "serde")]
  pub(super) schema_version: SchemaVersion,
  pub(super) format: Format,
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
  pub(super) media_type: Option<MediaType>,
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
  pub(super) size: Option<u64>,
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
  pub(super) duration: Option<Seconds>,
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
  pub(super) duration_provenance: Option<Provenance>,
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
  pub(super) estimated_duration: Option<Estimate<Seconds>>,
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
  pub(super) bit_rate: Option<Rational>,
  pub(super) streams: Vec<Stream>,
  pub(super) chapter_editions: Vec<ChapterEdition>,
  pub(super) tag_sets: Vec<TagSet>,
  pub(super) attachments: Vec<Attachment>,
  pub(super) diagnostics: Vec<Diagnostic>,
  pub(super) completeness: Completeness,
  pub(super) usage: Usage,
  pub(super) sources: Vec<SourceEntry>,
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
  pub(super) format_details: Option<FormatDetails>,
}

impl MediaReport {
  /// The format, or the family for a family-only detection.
  pub fn format(&self) -> &Format {
    &self.format
  }

  /// The media type.
  pub fn media_type(&self) -> Option<&MediaType> {
    self.media_type.as_ref()
  }

  /// The size in bytes, from the source.
  pub fn size(&self) -> Option<u64> {
    self.size
  }

  /// The duration; never an estimate.
  pub fn duration(&self) -> Option<Seconds> {
    self.duration
  }

  /// Where the duration came from.
  pub fn duration_provenance(&self) -> Option<&Provenance> {
    self.duration_provenance.as_ref()
  }

  /// An estimate of the duration, with its method (R25).
  pub fn estimated_duration(&self) -> Option<&Estimate<Seconds>> {
    self.estimated_duration.as_ref()
  }

  /// The bit rate, in bits per second.
  pub fn bit_rate(&self) -> Option<Rational> {
    self.bit_rate
  }

  /// The streams, in index order.
  pub fn streams(&self) -> &[Stream] {
    &self.streams
  }

  /// The stream with that index.
  pub fn stream(&self, index: usize) -> Option<&Stream> {
    self.streams.get(index)
  }

  /// The chapter editions.
  pub fn chapter_editions(&self) -> &[ChapterEdition] {
    &self.chapter_editions
  }

  /// The tag sets, each with its targets.
  pub fn tag_sets(&self) -> &[TagSet] {
    &self.tag_sets
  }

  /// The top-level tags of every tag set whose scope was established as the whole file. A set cut short before its
  /// targets is never among them, and a nested tag, an attribute of its parent, never is either.
  pub fn file_tags(&self) -> TagsOf<'_> {
    TagsOf::new(
      self
        .effective_tag_sets()
        .filter(|tag_set| matches!(tag_set.target_scope(), TargetScope::WholeFile)),
    )
  }

  /// Every top-level tag with that key, at any target and of any scope, in the order of `file_tags()`.
  pub fn tags_with_key(&self, key: &TagKey) -> TagsOf<'_> {
    TagsOf::with_key(self.effective_tag_sets(), key)
  }

  /// The tag sets that no later element replaced, which every lookup visits (twelfth review). `tag_sets()` keeps the
  /// superseded ones as history.
  fn effective_tag_sets(&self) -> impl Iterator<Item = &TagSet> {
    self.tag_sets.iter().filter(|tag_set| !tag_set.is_superseded())
  }

  /// The attachments of the requested roles.
  pub fn attachments(&self) -> &[Attachment] {
    &self.attachments
  }

  /// The first top-level, file-level tag with that key and a value, from `file_tags()`.
  pub fn tag(&self, key: &TagKey) -> Option<&Tag> {
    self
      .file_tags()
      .find(|tag| tag.key() == Some(key) && tag.value().is_some())
  }

  /// The diagnostics.
  pub fn diagnostics(&self) -> &[Diagnostic] {
    &self.diagnostics
  }

  /// What was established, what was not, and why.
  pub fn completeness(&self) -> &Completeness {
    &self.completeness
  }

  /// What the budget accounted for.
  pub fn usage(&self) -> &Usage {
    &self.usage
  }

  /// Every source the probe requested.
  pub fn sources(&self) -> &[SourceEntry] {
    &self.sources
  }

  /// The format's details.
  pub fn format_details(&self) -> Option<&FormatDetails> {
    self.format_details.as_ref()
  }

  /// The schema version that the record was read with, or the current one for a record that this version made. A
  /// document gives each record it holds its own, and checks it against its own (twelfth review).
  #[cfg(feature = "serde")]
  pub fn schema_version(&self) -> SchemaVersion {
    self.schema_version
  }

  /// The record with another schema version: a document gives each record it holds its own.
  #[cfg(feature = "serde")]
  pub(crate) fn with_schema_version(mut self, version: SchemaVersion) -> MediaReport {
    self.schema_version = version;
    self
  }
}
