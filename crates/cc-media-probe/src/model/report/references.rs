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

//! What a report names exists in it (`json-output.md` section 10), as the maintainer chose at the review of the third
//! slice's first part, which read a report without sources and a stream embedded in a stream that it lacked. The
//! source table names each source once and holds the primary input. Every stream index names a stream of the
//! report: a stream's carrier, which is another stream, a program's streams, a diagnostic's stream and the stream of
//! a part's reason, kept or first. Every source identifier names an entry of the table: a diagnostic's, a reason's
//! and an offset's of a declaration range. Every resolved location names an entity of the report: a tag target's,
//! a reference owner's and the owner of an entry of the typed details. A location of a newer kind is not checked.
//!
//! The second review of that part found three references that named something else than they said, which are
//! checked too:
//!
//! - **Carriers.** A chain of carriers ends: no stream is embedded, through its carriers, in itself.
//! - **The identity of a target.** A resolved target of the namespace `matroska_unique_identifier` names the entity
//!   whose unique identifier it holds, a `TrackUID`, `EditionUID`, `ChapterUID` or `FileUID` (RFC 9559 sections
//!   5.1.8.1.1.3 to 5.1.8.1.1.6), since its identifier is how the probe resolved it.
//! - **A sample description of an owner.** A reference owned by a stream's sample description stands in a report of
//!   a format with sample descriptions, and names a description within the count that the stream's protection
//!   declares, when that count is known (`api.md` section 6.12). A format of a newer version is not checked, and
//!   neither is an owner of a kind of a newer version, which the third review of that part read as a stream.
//! - **A sample description of a codec candidate,** as the fourth review of that part asked, which read a candidate
//!   of description 2 in a Matroska stream and in an MP4 stream that declares one description: a candidate stands in
//!   a report of a format with sample descriptions and names one within the stream's count, and a stream of a format
//!   without them misses no candidate, since it has none (`api.md` section 6.4).

use std::collections::HashSet;

use super::MediaReport;
use crate::completeness::PartReason;
use crate::details::{Conflict, ExactValue, ObservedBound, SharedDetails};
use crate::model::{
  Attachment, Chapter, ChapterEdition, EntityKind, EntityLocation, Identifier, IdentifierNamespace, Protection, Stream,
  TargetReference, TargetReferenceKind,
};
use crate::source::{SourceIdentifier, UNKNOWN_SOURCE_MESSAGE, table_identifiers};

/// The checks of the report's references.
pub(super) fn check_references(report: &MediaReport) -> Result<(), &'static str> {
  let sources = table_identifiers(&report.sources)?;
  if !sources_named(report, &sources) {
    return Err(UNKNOWN_SOURCE_MESSAGE);
  }
  if !streams_named(report) || !carriers_end(&report.streams) {
    return Err(super::constants::UNKNOWN_STREAM_MESSAGE);
  }
  if !locations_resolve(report) || !targets_identify(report) || !owners_describe(report) {
    return Err(super::constants::UNKNOWN_ENTITY_MESSAGE);
  }
  Ok(())
}

/// The reasons of the completeness, kept and first.
fn part_reasons(report: &MediaReport) -> impl Iterator<Item = &PartReason> {
  let completeness = &report.completeness;
  completeness
    .reasons()
    .iter()
    .chain(completeness.record().first_reasons())
}

/// Whether every source identifier names an entry of the table.
fn sources_named(report: &MediaReport, sources: &HashSet<SourceIdentifier>) -> bool {
  let ranges = report
    .streams
    .iter()
    .filter_map(Stream::declaration_range)
    .flat_map(|range| range.first_offset().into_iter().chain(range.last_offset()))
    .map(crate::source::SourceOffset::source_identifier);
  report
    .diagnostics
    .iter()
    .map(crate::diagnostics::Diagnostic::source_identifier)
    .chain(part_reasons(report).filter_map(PartReason::source_identifier))
    .chain(ranges)
    .all(|identifier| sources.contains(&identifier))
}

/// Whether every stream index names a stream of the report, and a carrier is another stream.
fn streams_named(report: &MediaReport) -> bool {
  let count = report.streams.len();
  let carriers = report.streams.iter().all(|stream| {
    stream
      .embedded_in()
      .is_none_or(|carrier| carrier < count && carrier != stream.index())
  });
  let programs = report
    .programs
    .iter()
    .flat_map(crate::model::Program::streams)
    .all(|index| *index < count);
  let others = report
    .diagnostics
    .iter()
    .filter_map(crate::diagnostics::Diagnostic::stream)
    .chain(part_reasons(report).filter_map(PartReason::stream))
    .all(|index| index < count);
  carriers && programs && others
}

/// How far the walk of `carriers_end` has followed a stream.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Walk {
  /// Not reached yet.
  Unvisited,
  /// On the chain that the walk follows now.
  OnChain,
  /// On a chain that ends.
  Ends,
}

/// Whether every chain of carriers ends, so that no stream is embedded, through its carriers, in itself. Each stream
/// is followed once, so the walk is linear in the streams.
fn carriers_end(streams: &[Stream]) -> bool {
  let mut walks = vec![Walk::Unvisited; streams.len()];
  let mut chain = Vec::new();
  for start in 0..streams.len() {
    let mut current = Some(start);
    while let Some(index) = current {
      let Some(walk) = walks.get_mut(index) else {
        break;
      };
      match walk {
        Walk::Ends => break,
        Walk::OnChain => return false,
        Walk::Unvisited => {
          *walk = Walk::OnChain;
          chain.push(index);
          current = streams.get(index).and_then(Stream::embedded_in);
        }
      }
    }
    for index in chain.drain(..) {
      if let Some(walk) = walks.get_mut(index) {
        *walk = Walk::Ends;
      }
    }
  }
  true
}

/// Whether the location names an entity of the report.
fn resolves(report: &MediaReport, location: &EntityLocation) -> bool {
  let index = location.index();
  match location.kind() {
    EntityKind::Stream => index < report.streams.len(),
    EntityKind::Edition => index < report.chapter_editions.len(),
    EntityKind::Chapter => chapter(report, location).is_some(),
    EntityKind::Attachment => index < report.attachments.len(),
    EntityKind::Program => index < report.programs.len(),
    EntityKind::Unrecognized(_) => true,
  }
}

/// The chapter that a location of a chapter names, as `ChapterEdition::chapter_at` resolves its path.
fn chapter<'report>(report: &'report MediaReport, location: &EntityLocation) -> Option<&'report Chapter> {
  report
    .chapter_editions
    .get(location.index())
    .and_then(|edition| edition.chapter_at(location.chapter_path()))
}

/// The owners of the entries of a detail record: its conflicts, its observed bounds and its exact values.
fn detail_owners(details: &dyn SharedDetails) -> impl Iterator<Item = &EntityLocation> {
  details
    .conflicts()
    .iter()
    .filter_map(Conflict::owner)
    .chain(details.observed_lower_bounds().iter().filter_map(ObservedBound::owner))
    .chain(details.exact_values().iter().filter_map(ExactValue::owner))
}

/// The resolved targets of the report's tag sets.
fn resolved_targets(report: &MediaReport) -> impl Iterator<Item = &TargetReference> {
  report
    .tag_sets
    .iter()
    .flat_map(|set| {
      set
        .stream_targets()
        .iter()
        .chain(set.edition_targets())
        .chain(set.chapter_targets())
        .chain(set.attachment_targets())
    })
    .filter(|target| matches!(target.kind(), TargetReferenceKind::Resolved))
}

/// Whether every resolved location names an entity of the report.
fn locations_resolve(report: &MediaReport) -> bool {
  let resolved = resolved_targets(report).filter_map(TargetReference::location);
  let owners = report
    .external_references
    .iter()
    .filter_map(crate::model::ExternalReference::owner)
    .map(crate::model::ReferenceOwner::location);
  let format_owners = report
    .format_details
    .iter()
    .flat_map(|details| detail_owners(details.shared()));
  let codec_owners = report
    .streams
    .iter()
    .filter_map(Stream::codec_details)
    .flat_map(|details| detail_owners(details.shared()));
  resolved
    .chain(owners)
    .chain(format_owners)
    .chain(codec_owners)
    .all(|location| resolves(report, location))
}

/// The unique identifier of the entity that a location names, where the report holds the entity and it has one.
fn unique_identifier<'report>(report: &'report MediaReport, location: &EntityLocation) -> Option<&'report Identifier> {
  let index = location.index();
  match location.kind() {
    EntityKind::Stream => report.streams.get(index).and_then(Stream::unique_identifier),
    EntityKind::Edition => report
      .chapter_editions
      .get(index)
      .and_then(ChapterEdition::unique_identifier),
    EntityKind::Chapter => chapter(report, location).and_then(Chapter::unique_identifier),
    EntityKind::Attachment => report.attachments.get(index).and_then(Attachment::unique_identifier),
    EntityKind::Program | EntityKind::Unrecognized(_) => None,
  }
}

/// Whether each resolved target of the Matroska namespace names the entity whose unique identifier it holds.
fn targets_identify(report: &MediaReport) -> bool {
  resolved_targets(report).all(|target| match (target.namespace(), target.location()) {
    (Some(IdentifierNamespace::MatroskaUniqueIdentifier), Some(location))
      if !matches!(location.kind(), EntityKind::Unrecognized(_)) =>
    {
      unique_identifier(report, location) == target.identifier()
    }
    _ => true,
  })
}

/// Whether each reference owned by a stream's sample description, and each codec candidate, names one that the
/// stream can have.
fn owners_describe(report: &MediaReport) -> bool {
  let Some(described) = report.format.has_sample_descriptions() else {
    return true;
  };
  let within_stream = |stream: Option<&Stream>, index: u64| {
    described
      && stream
        .and_then(Stream::protection)
        .and_then(Protection::sample_description_count)
        .is_none_or(|count| index <= count)
  };
  let owners = report
    .external_references
    .iter()
    .filter_map(crate::model::ExternalReference::owner)
    .filter(|owner| matches!(owner.location().kind(), EntityKind::Stream))
    .all(|owner| {
      owner
        .sample_description_index()
        .is_none_or(|index| within_stream(report.streams.get(owner.location().index()), index))
    });
  let candidates = report.streams.iter().all(|stream| {
    let codec = stream.codec();
    (described || codec.has_every_candidate())
      && codec
        .candidates()
        .iter()
        .all(|candidate| within_stream(Some(stream), candidate.sample_description_index()))
  });
  owners && candidates
}
