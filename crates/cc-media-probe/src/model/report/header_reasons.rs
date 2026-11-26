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

//! The reasons of the streams' header states (`api.md` section 5.4), as the maintainer chose at the fourth review of
//! the third slice's first part, which read a stream whose header stopped on `bytes_read` beside no exhausted limit
//! and only `truncated_input` among the reasons of the headers, with no reason omitted. A header state that this
//! version knows and that holds a reason records a shortfall of the headers, which the report records against
//! `Part::Headers` with the stream, so:
//!
//! - **Its counter.** A limit that the reason names is among the record's exhausted limits.
//! - **Its record.** The reason is kept among the reasons of the headers with the stream, or is the record's first
//!   reason of the headers, or was counted among the omitted reasons, as a source entry's reason is
//!   (`crate::source::request_reasons_not_kept`).
//!
//! A state of a newer version is not checked, since only its version knows what its reason records.

use std::collections::HashSet;

use super::MediaReport;
use crate::Limit;
use crate::completeness::{HeaderState, Part, PartReason, Reason, stated_reasons_not_kept};
use crate::model::Stream;

/// The reason of each stream's header state of this version, with the stream's index.
fn header_reasons(streams: &[Stream]) -> impl Iterator<Item = (usize, &Reason)> {
  streams.iter().filter_map(|stream| {
    let status = stream.header_status();
    let known = !matches!(status.state(), HeaderState::Unrecognized(_));
    status.reason().filter(|_| known).map(|reason| (stream.index(), reason))
  })
}

/// Whether each limit that a header state's reason names is among the record's exhausted limits.
pub(super) fn header_reasons_find_counters(report: &MediaReport) -> bool {
  let exhausted: HashSet<&Limit> = report.completeness.record().exhausted_limits().iter().collect();
  header_reasons(&report.streams).all(|(_, reason)| reason.finds_its_counter(|limit| exhausted.contains(limit)))
}

/// The reasons of the header states that the completeness neither keeps nor gives as the first reason of the
/// headers, which it counts among the first reasons that it lacks.
pub(super) fn header_reasons_not_kept(report: &MediaReport) -> u64 {
  stated_reasons_not_kept(
    &report.completeness,
    &Part::Headers,
    PartReason::stream,
    header_reasons(&report.streams),
  )
}
