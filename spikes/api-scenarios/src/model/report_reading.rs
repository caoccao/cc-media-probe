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

//! How a report is read, `api.md` section 5.3: its members as written, then the check that the headers status agrees
//! with the header states of the streams. A status of `complete` beside a known state that is not final, or
//! `incomplete` while every state is known and final, is malformed. Beside a state that a newer version wrote, the
//! status is the verdict of the version that wrote it. The completeness checks that its reasons, its record's first
//! reasons and its statuses agree part by part, wherever it is read (`crate::completeness`; fifteenth review); the
//! report adds the one check of that agreement that needs its streams.

use super::attachments::Attachment;
use super::chapters::ChapterEdition;
use super::format::Format;
use super::report::MediaReport;
use super::stream::Stream;
use super::tags::TagSet;
use super::usage::Usage;
use super::values::{Estimate, MediaType, Provenance, Rational, Seconds, paired};
use crate::completeness::{Completeness, HeaderState, HeadersStatus, Part};
use crate::constants::{
  HEADERS_DISAGREE_MESSAGE, LARGEST_EXACT, QUANTITY_MESSAGE, REASONS_DISAGREE_MESSAGE, STREAM_INDEX_MESSAGE,
};
use crate::details::FormatDetails;
use crate::diagnostics::Diagnostic;
use crate::document::InvalidDocument;
use crate::neutral::Rooted;
use crate::source::SourceEntry;

/// A report's members as read, other than its version, before the headers status is checked.
#[derive(serde::Deserialize)]
pub(super) struct ReportData {
  format: Format,
  media_type: Option<MediaType>,
  size: Option<u64>,
  duration: Option<Seconds>,
  duration_provenance: Option<Provenance>,
  estimated_duration: Option<Estimate<Seconds>>,
  bit_rate: Option<Rational>,
  bit_rate_provenance: Option<Provenance>,
  streams: Vec<Stream>,
  chapter_editions: Vec<ChapterEdition>,
  tag_sets: Vec<TagSet>,
  attachments: Vec<Attachment>,
  diagnostics: Vec<Diagnostic>,
  completeness: Completeness,
  usage: Usage,
  sources: Vec<SourceEntry>,
  format_details: Option<FormatDetails>,
}

impl TryFrom<Rooted<ReportData>> for MediaReport {
  type Error = InvalidDocument;

  /// The report of a version read first, whose headers status and reasons agree with the rest.
  fn try_from(rooted: Rooted<ReportData>) -> Result<MediaReport, InvalidDocument> {
    let data = rooted.body;
    if !headers_agree(data.completeness.headers(), &data.streams) {
      return Err(InvalidDocument::new(HEADERS_DISAGREE_MESSAGE));
    }
    if !newer_headers_agree(&data.completeness, &data.streams) {
      return Err(InvalidDocument::new(REASONS_DISAGREE_MESSAGE));
    }
    let positions = (0_usize..).zip(&data.streams);
    if positions
      .into_iter()
      .any(|(position, stream)| stream.index() != position)
    {
      return Err(InvalidDocument::new(STREAM_INDEX_MESSAGE));
    }
    if data.size.is_some_and(|size| size > LARGEST_EXACT) {
      return Err(InvalidDocument::new(QUANTITY_MESSAGE));
    }
    paired(&data.duration, &data.duration_provenance).map_err(InvalidDocument::new)?;
    paired(&data.bit_rate, &data.bit_rate_provenance).map_err(InvalidDocument::new)?;
    Ok(MediaReport {
      schema_version: rooted.version,
      format: data.format,
      media_type: data.media_type,
      size: data.size,
      duration: data.duration,
      duration_provenance: data.duration_provenance,
      estimated_duration: data.estimated_duration,
      bit_rate: data.bit_rate,
      bit_rate_provenance: data.bit_rate_provenance,
      streams: data.streams,
      chapter_editions: data.chapter_editions,
      tag_sets: data.tag_sets,
      attachments: data.attachments,
      diagnostics: data.diagnostics,
      completeness: data.completeness,
      usage: data.usage,
      sources: data.sources,
      format_details: data.format_details,
    })
  }
}

/// Whether a stream has a header state that this version knows and that is not final: a shortfall of the headers
/// that this version sees, whatever the headers status says (fourteenth review).
fn known_header_shortfall(streams: &[Stream]) -> bool {
  streams
    .iter()
    .map(Stream::header_status)
    .any(|status| !matches!(status.state(), HeaderState::Unrecognized(_)) && !status.is_final())
}

/// Whether the headers status agrees with the states that this version knows. A state of a newer version leaves
/// either status possible, since only the version that wrote it knows whether it is final.
fn headers_agree(status: &HeadersStatus, streams: &[Stream]) -> bool {
  let known_short = known_header_shortfall(streams);
  let every_final = streams.iter().all(|stream| stream.header_status().is_final());
  match status {
    HeadersStatus::Complete => !known_short,
    HeadersStatus::Incomplete => !every_final,
    HeadersStatus::Unrecognized(_) => true,
  }
}

/// Whether a headers status of a newer version agrees with the streams: a stream's known header state that is not
/// final still records a shortfall of the headers, so the record holds the first reason of the headers (fourteenth
/// review). Every other agreement of the reasons, the record and the statuses needs no stream, and the completeness
/// checks it itself.
fn newer_headers_agree(completeness: &Completeness, streams: &[Stream]) -> bool {
  !matches!(completeness.headers(), HeadersStatus::Unrecognized(_))
    || !known_header_shortfall(streams)
    || completeness.record().first_reason(&Part::Headers).is_some()
}
