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

//! How a report is read (`json-output.md` sections 4.2 and 10): its version first, through `Rooted`, then its
//! members as written, and then the checks that need more than one member. A stream's index is its position, so
//! `stream(index)` finds it (fourteenth review of `api.md`). The headers status agrees with the header states of the
//! streams: `complete` beside no known state that is not final, and `incomplete` beside a state that is not final.
//! Beside a state of a newer version the status is the verdict of the version that wrote it, and a headers status of
//! a newer version beside a known state that is not final needs the headers' first reason. A protection says that
//! its format has sample descriptions exactly when the report's format is of the ISO base media family (second review
//! of the skeleton's first slice). The completeness checks itself, alone as here. The bytes that the source entries
//! read add up to at most the usage's bytes read, which every source charges (third review of the third slice's
//! first part). What the report names exists in it (`references`), a section that was not requested holds nothing
//! (`skipped`), and a shortfall that its own records state is recorded in its completeness (`shortfalls`).

use super::{InventoryClass, MediaReport};
use crate::Format;
use crate::completeness::{Completeness, HeaderState, HeadersStatus, Part};
use crate::details::FormatDetails;
use crate::diagnostics::Diagnostic;
use crate::form::{PROVENANCE_MESSAGE, Rooted};
use crate::model::{
  Attachment, ChapterEdition, Estimate, ExternalReference, MediaType, Program, Provenance, Rational, Seconds, Stream,
  TagSet, Usage, UtcTime,
};
use crate::source::SourceEntry;

/// The members of a report other than its version, as read.
#[derive(serde::Deserialize)]
pub(super) struct ReportBody {
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

impl TryFrom<Rooted<ReportBody>> for MediaReport {
  type Error = &'static str;

  fn try_from(rooted: Rooted<ReportBody>) -> Result<MediaReport, &'static str> {
    let body = rooted.body;
    let report = MediaReport {
      schema_version: rooted.version,
      format: body.format,
      media_type: body.media_type,
      size: body.size,
      duration: body.duration,
      duration_provenance: body.duration_provenance,
      bit_rate: body.bit_rate,
      bit_rate_provenance: body.bit_rate_provenance,
      estimated_duration: body.estimated_duration,
      estimated_bit_rate: body.estimated_bit_rate,
      creation_time: body.creation_time,
      streams: body.streams,
      programs: body.programs,
      chapter_editions: body.chapter_editions,
      tag_sets: body.tag_sets,
      attachments: body.attachments,
      external_references: body.external_references,
      diagnostics: body.diagnostics,
      completeness: body.completeness,
      inventory_class: body.inventory_class,
      usage: body.usage,
      sources: body.sources,
      format_details: body.format_details,
    };
    check_report(&report)?;
    Ok(report)
  }
}

/// The checks of a report that need more than one of its members.
fn check_report(report: &MediaReport) -> Result<(), &'static str> {
  use super::constants::{HEADERS_DISAGREE_MESSAGE, SAMPLE_DESCRIPTIONS_MESSAGE, STREAM_INDEX_MESSAGE};
  if report.duration.is_some() != report.duration_provenance.is_some()
    || report.bit_rate.is_some() != report.bit_rate_provenance.is_some()
  {
    return Err(PROVENANCE_MESSAGE);
  }
  if (0_usize..)
    .zip(&report.streams)
    .any(|(position, stream)| stream.index() != position)
  {
    return Err(STREAM_INDEX_MESSAGE);
  }
  if !headers_agree(&report.completeness, &report.streams) {
    return Err(HEADERS_DISAGREE_MESSAGE);
  }
  if !sample_descriptions_agree(&report.format, &report.streams) {
    return Err(SAMPLE_DESCRIPTIONS_MESSAGE);
  }
  if !crate::source::bytes_read_within(&report.sources, report.usage.bytes_read()) {
    return Err(crate::source::BYTES_READ_MESSAGE);
  }
  super::references::check_references(report)?;
  super::skipped::check_skipped(report)?;
  super::shortfalls::check_shortfalls(report)
}

/// Whether a stream has a header state that this version knows and that is not final: a shortfall of the headers
/// that this version sees, whatever the headers status says (fourteenth review of `api.md`).
fn known_header_shortfall(streams: &[Stream]) -> bool {
  streams
    .iter()
    .map(Stream::header_status)
    .any(|status| !matches!(status.state(), HeaderState::Unrecognized(_)) && !status.is_final())
}

/// Whether the headers status agrees with the header states of the streams.
fn headers_agree(completeness: &Completeness, streams: &[Stream]) -> bool {
  let known_shortfall = known_header_shortfall(streams);
  match completeness.headers() {
    HeadersStatus::Complete => !known_shortfall,
    HeadersStatus::Incomplete => !streams.iter().all(|stream| stream.header_status().is_final()),
    _ => !known_shortfall || completeness.record().first_reason(&Part::Headers).is_some(),
  }
}

/// Whether each protection says that its format has sample descriptions exactly when the report's format is of the
/// ISO base media family; a format of a newer version is not checked.
fn sample_descriptions_agree(format: &Format, streams: &[Stream]) -> bool {
  format.has_sample_descriptions().is_none_or(|expected| {
    streams
      .iter()
      .filter_map(Stream::protection)
      .all(|protection| protection.has_sample_descriptions() == expected)
  })
}
