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
//! status is the verdict of the version that wrote it.

use super::attachments::Attachment;
use super::chapters::ChapterEdition;
use super::format::Format;
use super::report::MediaReport;
use super::stream::Stream;
use super::tags::TagSet;
use super::usage::Usage;
use super::values::{Estimate, MediaType, Provenance, Rational, Seconds};
use crate::completeness::{Completeness, HeaderState, HeadersStatus};
use crate::constants::HEADERS_DISAGREE_MESSAGE;
use crate::details::FormatDetails;
use crate::diagnostics::Diagnostic;
use crate::document::{InvalidDocument, SchemaVersion};
use crate::source::SourceEntry;

/// A report's members as read, before the headers status is checked.
#[derive(serde::Deserialize)]
pub(super) struct ReportData {
  #[serde(deserialize_with = "crate::document::read_current_major")]
  schema_version: SchemaVersion,
  format: Format,
  media_type: Option<MediaType>,
  size: Option<u64>,
  duration: Option<Seconds>,
  duration_provenance: Option<Provenance>,
  estimated_duration: Option<Estimate<Seconds>>,
  bit_rate: Option<Rational>,
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

impl TryFrom<ReportData> for MediaReport {
  type Error = InvalidDocument;

  fn try_from(data: ReportData) -> Result<MediaReport, InvalidDocument> {
    if !headers_agree(data.completeness.headers(), &data.streams) {
      return Err(InvalidDocument::new(HEADERS_DISAGREE_MESSAGE));
    }
    Ok(MediaReport {
      schema_version: data.schema_version,
      format: data.format,
      media_type: data.media_type,
      size: data.size,
      duration: data.duration,
      duration_provenance: data.duration_provenance,
      estimated_duration: data.estimated_duration,
      bit_rate: data.bit_rate,
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

/// Whether the headers status agrees with the states that this version knows. A state of a newer version leaves
/// either status possible, since only the version that wrote it knows whether it is final.
fn headers_agree(status: &HeadersStatus, streams: &[Stream]) -> bool {
  let statuses = || streams.iter().map(Stream::header_status);
  let known_short =
    statuses().any(|status| !matches!(status.state(), HeaderState::Unrecognized(_)) && !status.is_final());
  let every_final = statuses().all(|status| status.is_final());
  match status {
    HeadersStatus::Complete => !known_short,
    HeadersStatus::Incomplete => !every_final,
    HeadersStatus::Unrecognized(_) => true,
  }
}
