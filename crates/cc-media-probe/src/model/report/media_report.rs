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

//! `MediaReport`, what a probe found (`api.md` section 6.2; field guide section 3.1).

use super::{FileTags, InventoryClass, StreamTags, TagsWithKey};
use crate::Format;
use crate::completeness::Completeness;
use crate::details::FormatDetails;
use crate::diagnostics::Diagnostic;
use crate::model::{
  Attachment, ChapterEdition, Estimate, ExternalReference, MediaType, Program, Provenance, Rational, Seconds, Stream,
  Tag, TagKey, TagSet, Usage, UtcTime,
};
use crate::source::SourceEntry;

/// What a probe found: the format, the streams, the chapters, tags and attachments, the diagnostics, and the
/// completeness that says what was established and why the rest was not (R23). A report never borrows its input.
///
/// ```
/// use cc_media_probe::MediaReport;
/// use cc_media_probe::model::TagKey;
///
/// fn summary(report: &MediaReport) {
///     println!(
///         "{} ({:?}), {:?} bytes, {} streams, {}",
///         report.format(),
///         report.media_type().map(|media_type| media_type.as_str()),
///         report.size(),
///         report.streams().len(),
///         report.completeness(),
///     );
///     if let Some(duration) = report.duration() {
///         println!("duration {duration} ({:?})", report.duration_provenance());
///     } else if let Some(estimate) = report.estimated_duration() {
///         println!("about {} ({})", estimate.value(), estimate.method().as_str());
///     }
///     if let Some(rate) = report.bit_rate() {
///         println!("{rate} bit/s ({:?})", report.bit_rate_provenance());
///     } else if let Some(estimate) = report.estimated_bit_rate() {
///         println!("about {} bit/s", estimate.value());
///     }
///     println!("created {:?}", report.creation_time().and_then(|time| time.to_system_time()));
///     println!("first stream {:?}", report.stream(0).map(|stream| stream.kind().as_str()));
///     println!(
///         "{} programs, {} editions, {} tag sets, {} attachments, {} references, {} diagnostics, {} sources",
///         report.programs().len(),
///         report.chapter_editions().len(),
///         report.tag_sets().len(),
///         report.attachments().len(),
///         report.external_references().len(),
///         report.diagnostics().len(),
///         report.sources().len(),
///     );
///     println!(
///         "inventory {}, {} bytes read, details {}",
///         report.inventory_class().as_str(),
///         report.usage().bytes_read(),
///         report.format_details().is_some(),
///     );
///     println!("{} file tags, {} tags of stream 0", report.file_tags().count(), report.stream_tags(0).count());
///     println!("{} titles anywhere", report.tags_with_key(&TagKey::Title).count());
///     println!("title {:?}", report.tag(&TagKey::Title).and_then(|tag| tag.value()).map(|value| value.as_str()));
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MediaReport {
  format: Format,
  media_type: Option<MediaType>,
  size: Option<u64>,
  duration: Option<Seconds>,
  duration_provenance: Option<Provenance>,
  bit_rate: Option<Rational>,
  bit_rate_provenance: Option<Provenance>,
  estimated_duration: Option<Estimate<Seconds>>,
  estimated_bit_rate: Option<Estimate<Rational>>,
  creation_time: Option<UtcTime>,
  streams: Vec<Stream>,
  programs: Vec<Program>,
  chapter_editions: Vec<ChapterEdition>,
  tag_sets: Vec<TagSet>,
  attachments: Vec<Attachment>,
  external_references: Vec<ExternalReference>,
  diagnostics: Vec<Diagnostic>,
  completeness: Completeness,
  inventory_class: InventoryClass,
  usage: Usage,
  sources: Vec<SourceEntry>,
  format_details: Option<FormatDetails>,
}

impl MediaReport {
  /// The format, or the family for a detection certain of the family only.
  #[must_use]
  pub fn format(&self) -> &Format {
    &self.format
  }

  /// The media type (F3).
  #[must_use]
  pub fn media_type(&self) -> Option<&MediaType> {
    self.media_type.as_ref()
  }

  /// The size of the input in bytes, from the source, where it lies within the exact range.
  #[must_use]
  pub fn size(&self) -> Option<u64> {
    self.size
  }

  /// The duration of the presentation, never an edition's playback duration.
  #[must_use]
  pub fn duration(&self) -> Option<Seconds> {
    self.duration
  }

  /// Where the duration came from.
  #[must_use]
  pub fn duration_provenance(&self) -> Option<&Provenance> {
    self.duration_provenance.as_ref()
  }

  /// The overall bit rate, in bits per second.
  #[must_use]
  pub fn bit_rate(&self) -> Option<Rational> {
    self.bit_rate
  }

  /// Where the bit rate came from.
  #[must_use]
  pub fn bit_rate_provenance(&self) -> Option<&Provenance> {
    self.bit_rate_provenance.as_ref()
  }

  /// An estimate of the duration, apart from `duration()` (R25).
  #[must_use]
  pub fn estimated_duration(&self) -> Option<&Estimate<Seconds>> {
    self.estimated_duration.as_ref()
  }

  /// An estimate of the bit rate, apart from `bit_rate()` (R25).
  #[must_use]
  pub fn estimated_bit_rate(&self) -> Option<&Estimate<Rational>> {
    self.estimated_bit_rate.as_ref()
  }

  /// When the file was created, only from a source that states UTC (F4).
  #[must_use]
  pub fn creation_time(&self) -> Option<UtcTime> {
    self.creation_time
  }

  /// The streams, in index order.
  #[must_use]
  pub fn streams(&self) -> &[Stream] {
    &self.streams
  }

  /// The stream at `index`.
  #[must_use]
  pub fn stream(&self, index: usize) -> Option<&Stream> {
    self.streams.get(index)
  }

  /// The programs.
  #[must_use]
  pub fn programs(&self) -> &[Program] {
    &self.programs
  }

  /// The editions of chapters.
  #[must_use]
  pub fn chapter_editions(&self) -> &[ChapterEdition] {
    &self.chapter_editions
  }

  /// Every tag set that the probe read, superseded ones included, in report order.
  #[must_use]
  pub fn tag_sets(&self) -> &[TagSet] {
    &self.tag_sets
  }

  /// The attachments of the requested roles.
  #[must_use]
  pub fn attachments(&self) -> &[Attachment] {
    &self.attachments
  }

  /// The references to media outside the input, within the retained-data limit.
  #[must_use]
  pub fn external_references(&self) -> &[ExternalReference] {
    &self.external_references
  }

  /// The diagnostics, within the diagnostics limit.
  #[must_use]
  pub fn diagnostics(&self) -> &[Diagnostic] {
    &self.diagnostics
  }

  /// What was established, what was not, and why.
  #[must_use]
  pub fn completeness(&self) -> &Completeness {
    &self.completeness
  }

  /// How the format reached its stream inventory.
  #[must_use]
  pub fn inventory_class(&self) -> &InventoryClass {
    &self.inventory_class
  }

  /// What the budget accounted for.
  #[must_use]
  pub fn usage(&self) -> &Usage {
    &self.usage
  }

  /// Every source that the probe requested; until companion lookup ships, the primary input alone.
  #[must_use]
  pub fn sources(&self) -> &[SourceEntry] {
    &self.sources
  }

  /// The format's typed details.
  #[must_use]
  pub fn format_details(&self) -> Option<&FormatDetails> {
    self.format_details.as_ref()
  }

  /// The top-level tags of every tag set whose scope is `WholeFile`. A set of the scope `Incomplete` is never
  /// among them, so a track's title cut off from its target never becomes the file's title.
  #[must_use]
  pub fn file_tags(&self) -> FileTags<'_> {
    FileTags::new(&self.tag_sets)
  }

  /// The top-level tags of every tag set that targets the stream at `index`, directly or by a wildcard. A wildcard
  /// applies only to the streams that the report has, so an index without a stream gives no tags.
  #[must_use]
  pub fn stream_tags(&self, index: usize) -> StreamTags<'_> {
    let sets: &[TagSet] = if index < self.streams.len() {
      &self.tag_sets
    } else {
      &[]
    };
    StreamTags::new(sets, index)
  }

  /// Every top-level tag with the key `key`, at any target and of any scope, with a value or without one.
  #[must_use]
  pub fn tags_with_key(&self, key: &TagKey) -> TagsWithKey<'_> {
    TagsWithKey::new(&self.tag_sets, key)
  }

  /// The first tag of `file_tags()` with the key `key` and a value. A whole-file set whose only such tag is nested
  /// has none.
  #[must_use]
  pub fn tag(&self, key: &TagKey) -> Option<&Tag> {
    self
      .file_tags()
      .find(|tag| tag.key() == Some(key) && tag.value().is_some())
  }
}

#[cfg(test)]
mod tests {
  use super::MediaReport;
  use crate::Format;
  use crate::completeness::Completeness;
  use crate::details::{FormatDetails, UnrecognizedDetails};
  use crate::diagnostics::Diagnostic;
  use crate::model::{
    Attachment, ChapterEdition, Estimate, EstimateMethod, ExternalReference, InventoryClass, MediaType, Protection,
    Provenance, Rational, Seconds, Stream, Tag, TagKey, TagSet, TagValue, TargetReference, TargetScope, Usage, UtcTime,
  };
  use crate::source::SourceEntry;

  impl MediaReport {
    /// A complete Matroska report of a video, an audio and a subtitle stream, with every list filled.
    pub(crate) fn sample() -> MediaReport {
      MediaReport {
        format: Format::Matroska,
        media_type: Some(MediaType::sample("video/matroska")),
        size: Some(1_048_576),
        duration: Some(Seconds::sample(5_400, 1)),
        duration_provenance: Some(Provenance::Declared),
        bit_rate: Rational::new(1_553, 1),
        bit_rate_provenance: Some(Provenance::Derived),
        estimated_duration: Some(Estimate::sample(
          Seconds::sample(5_400, 1),
          EstimateMethod::EndpointTimestamps,
        )),
        estimated_bit_rate: None,
        creation_time: Some(UtcTime::sample(1_795_422_600, 0)),
        streams: vec![
          Stream::video_sample(0),
          Stream::audio_sample(1),
          Stream::subtitle_sample(2),
        ],
        programs: Vec::new(),
        chapter_editions: vec![ChapterEdition::sample()],
        tag_sets: Vec::new(),
        attachments: vec![Attachment::sample()],
        external_references: vec![ExternalReference::sample()],
        diagnostics: vec![Diagnostic::sample(0)],
        completeness: Completeness::complete(),
        inventory_class: InventoryClass::Declared,
        usage: Usage::sample(),
        sources: vec![SourceEntry::sample()],
        format_details: Some(FormatDetails::Unrecognized(UnrecognizedDetails::sample("matroska"))),
      }
    }

    /// The report established before a failure of the primary input cut its tags short.
    pub(crate) fn cut_by_a_source_failure() -> MediaReport {
      MediaReport {
        completeness: Completeness::cut_by_a_source_failure(),
        ..MediaReport::sample()
      }
    }

    /// The same report with these tag sets.
    pub(crate) fn with_tag_sets(self, tag_sets: Vec<TagSet>) -> MediaReport {
      MediaReport { tag_sets, ..self }
    }
  }

  fn text(tag: &Tag) -> Option<&str> {
    match tag.value()? {
      TagValue::Text(text) => Some(text),
      _ => None,
    }
  }

  #[test]
  fn gives_each_field() {
    let report = MediaReport::sample();
    assert_eq!(report.format(), &Format::Matroska);
    assert_eq!(report.media_type().map(MediaType::as_str), Some("video/matroska"));
    assert_eq!(report.size(), Some(1_048_576));
    assert_eq!(report.duration(), Some(Seconds::sample(5_400, 1)));
    assert_eq!(report.duration_provenance(), Some(&Provenance::Declared));
    assert_eq!(report.bit_rate(), Rational::new(1_553, 1));
    assert_eq!(report.bit_rate_provenance(), Some(&Provenance::Derived));
    assert!(report.estimated_duration().is_some());
    assert_eq!(report.estimated_bit_rate(), None);
    assert_eq!(report.creation_time(), Some(UtcTime::sample(1_795_422_600, 0)));
    assert_eq!(report.streams().len(), 3);
    assert_eq!(report.stream(1).map(Stream::index), Some(1));
    assert_eq!(report.stream(3), None);
    assert_eq!(report.programs(), []);
    assert_eq!(report.chapter_editions().len(), 1);
    assert_eq!(report.tag_sets(), []);
    assert_eq!(report.attachments().len(), 1);
    assert_eq!(report.external_references().len(), 1);
    assert_eq!(report.diagnostics().len(), 1);
    assert!(report.completeness().is_complete());
    assert_eq!(report.inventory_class(), &InventoryClass::Declared);
    assert_eq!(report.usage(), &Usage::sample());
    assert_eq!(report.sources().len(), 1);
    assert!(report.format_details().is_some());
  }

  #[test]
  fn has_protections_that_agree_with_its_format() {
    let report = MediaReport::sample();
    let protections: Vec<&Protection> = report.streams().iter().filter_map(Stream::protection).collect();
    assert_eq!(protections.len(), 3);
    assert_eq!(report.format().family(), None);
    assert!(
      protections
        .iter()
        .all(|protection| !protection.has_sample_descriptions())
    );
  }

  #[test]
  fn looks_up_the_top_level_tags_of_the_sets_that_are_not_superseded() {
    let report = MediaReport::sample().with_tag_sets(vec![
      TagSet::sample(
        TargetScope::WholeFile,
        Vec::new(),
        vec![Tag::sample("TITLE", Some(TagKey::Title), Some("Old title"), Vec::new())],
      )
      .superseded(),
      TagSet::sample(
        TargetScope::WholeFile,
        Vec::new(),
        vec![
          Tag::sample(
            "ORIGINAL",
            None,
            None,
            vec![Tag::sample("TITLE", Some(TagKey::Title), Some("Original"), Vec::new())],
          ),
          Tag::sample("TITLE", Some(TagKey::Title), None, Vec::new()),
          Tag::sample("ARTIST", Some(TagKey::Artist), Some("Someone"), Vec::new()),
        ],
      ),
      TagSet::sample(
        TargetScope::Entities,
        vec![TargetReference::stream(1)],
        vec![Tag::sample(
          "TITLE",
          Some(TagKey::Title),
          Some("Commentary"),
          Vec::new(),
        )],
      ),
      TagSet::sample(
        TargetScope::Incomplete,
        vec![TargetReference::wildcard()],
        vec![Tag::sample(
          "COMMENT",
          Some(TagKey::Comment),
          Some("Every stream"),
          Vec::new(),
        )],
      ),
      TagSet::sample(
        TargetScope::Entities,
        vec![TargetReference::stream(2)],
        vec![Tag::sample("TITLE", Some(TagKey::Title), Some("Subtitles"), Vec::new())],
      ),
    ]);
    let file: Vec<&str> = report.file_tags().map(|tag| tag.name().as_written()).collect();
    assert_eq!(file, ["ORIGINAL", "TITLE", "ARTIST"]);
    assert_eq!(report.tag(&TagKey::Title), None);
    assert_eq!(report.tag(&TagKey::Artist).and_then(text), Some("Someone"));
    let stream: Vec<Option<&str>> = report.stream_tags(1).map(text).collect();
    assert_eq!(stream, [Some("Commentary"), Some("Every stream")]);
    assert_eq!(
      report.stream_tags(0).map(text).collect::<Vec<_>>(),
      [Some("Every stream")]
    );
    assert_eq!(report.stream(3), None);
    assert_eq!(report.stream_tags(3).count(), 0);
    assert_eq!(report.stream_tags(usize::MAX).count(), 0);
    let titles: Vec<Option<&str>> = report.tags_with_key(&TagKey::Title).map(text).collect();
    assert_eq!(titles, [None, Some("Commentary"), Some("Subtitles")]);
    let unmatched = MediaReport::sample().with_tag_sets(vec![TagSet::sample(
      TargetScope::Entities,
      vec![TargetReference::stream(9)],
      vec![Tag::sample("TITLE", Some(TagKey::Title), Some("Missing"), Vec::new())],
    )]);
    assert_eq!(unmatched.stream_tags(0).count(), 0);
    assert!(format!("{:?}", unmatched.file_tags().clone()).starts_with("FileTags"));
  }
}
