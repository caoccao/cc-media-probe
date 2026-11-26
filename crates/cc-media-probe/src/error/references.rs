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
//! What an error record names exists in it (`api.md` sections 10.1 and 10.2; `json-output.md` section 10). The
//! error's source names an entry of the source table that its record holds, its partial report's or its
//! observations', as the review of the third slice's first part asked; a record without either names only the
//! primary input, since only a probe that reached its prober requests a companion, and then holds one of the two.
//!
//! The second review of that part read first reasons that named the source 9 and the stream 9 beside observations
//! of the primary input alone, and a source failure of an AVI file with a Matroska partial report, so:
//!
//! - **The first reasons** of a record without a partial report name no stream, since no stream is known before an
//!   inventory, and name only sources of its table. A partial report checks its own, which are the record's.
//! - **The format** of a partial report is the error's, when both are of this version.
//! - **The counters.** A candidate's reason and the reason of an entry of the observations that name a limit find it
//!   among the record's exhausted limits, as the error's own reason does.
//!
//! Its third review read a Matroska partial report beside no format, a source failure that named a request that a
//! resolver declined, an entry not found beside the reason of a decline, and usage that counted fewer bytes than the
//! partial report beside it, so:
//!
//! - **The format.** A partial report says that detection found the format, so a known kind names it (section 5.1).
//! - **The failed source.** A source failure's entry has the outcome `failed`, or one of a newer version.
//! - **The requests.** A first reason that names a request is the reason of the observed entry that it names
//!   (`crate::source::requests_agree`); a partial report checks its own.
//! - **The bytes.** Every source charges the one budget (plan section 7.5), so the error's bytes read are at least
//!   its partial report's and at least what its observed entries read.
//!
//! Its fourth review read a source failure whose observations held a companion not found beside no first reason, so
//! an observed companion that was not opened, not found, declined or refused by a limit, needs the first reason of
//! the companions, which the record keeps whatever the limits (sections 5.3 and 6.15).

use std::collections::HashSet;

use super::constants::{
  ERROR_REFERENCES_MESSAGE, ERROR_USAGE_MESSAGE, FAILED_SOURCE_MESSAGE, OBSERVED_REFUSAL_MESSAGE,
  OBSERVED_STREAM_MESSAGE,
};
use super::parts::ErrorParts;
use super::{CandidateCheck, ErrorKind, Observations};
use crate::completeness::{Part, PartReason};
use crate::source::{
  REQUEST_REASON_MESSAGE, SourceEntry, SourceIdentifier, SourceOutcome, UNKNOWN_SOURCE_MESSAGE, bytes_read_within,
  reasons_find_counters, requests_agree,
};
use crate::{Format, Limit};

/// The checks of what the error record names.
pub(super) fn check_references(parts: &ErrorParts) -> Result<(), &'static str> {
  let table = parts
    .partial_report
    .as_ref()
    .map(|report| report.sources())
    .or_else(|| parts.observations.as_ref().map(Observations::sources));
  let identifiers: HashSet<SourceIdentifier> = table.map_or_else(
    || HashSet::from([SourceIdentifier::primary()]),
    |sources| sources.iter().map(SourceEntry::identifier).collect(),
  );
  // A partial report checks its own first reasons, which are the record's.
  let first_reasons = if parts.partial_report.is_some() {
    &[]
  } else {
    parts.record.first_reasons()
  };
  if first_reasons.iter().any(|first| first.stream().is_some()) {
    return Err(OBSERVED_STREAM_MESSAGE);
  }
  let named = first_reasons
    .iter()
    .filter_map(PartReason::source_identifier)
    .chain(parts.source_identifier)
    .all(|source| identifiers.contains(&source));
  if !named {
    return Err(UNKNOWN_SOURCE_MESSAGE);
  }
  if !failed_source_failed(parts, table.unwrap_or_default()) {
    return Err(FAILED_SOURCE_MESSAGE);
  }
  if !formats_agree(parts) || !counters_found(parts) {
    return Err(ERROR_REFERENCES_MESSAGE);
  }
  let observed = parts.observations.as_ref().map(Observations::sources);
  if !observed.is_none_or(|sources| requests_agree(sources, first_reasons)) {
    return Err(REQUEST_REASON_MESSAGE);
  }
  let refused = observed.is_some_and(|sources| sources.iter().any(SourceEntry::was_refused));
  if refused && parts.record.first_reason(&Part::Companions).is_none() {
    return Err(OBSERVED_REFUSAL_MESSAGE);
  }
  let bytes_read = parts.usage.bytes_read();
  let bytes_counted = parts
    .partial_report
    .as_ref()
    .is_none_or(|report| report.usage().bytes_read() <= bytes_read)
    && observed.is_none_or(|sources| bytes_read_within(sources, bytes_read));
  if bytes_counted {
    Ok(())
  } else {
    Err(ERROR_USAGE_MESSAGE)
  }
}

/// Whether the entry of a source failure's source has the outcome `failed`, or one of a newer version.
fn failed_source_failed(parts: &ErrorParts, table: &[SourceEntry]) -> bool {
  let failure = matches!(parts.kind, ErrorKind::SourceFailed);
  parts.source_identifier.filter(|_| failure).is_none_or(|source| {
    table
      .iter()
      .filter(|entry| entry.identifier() == source)
      .all(|entry| matches!(entry.outcome(), SourceOutcome::Failed | SourceOutcome::Unrecognized(_)))
  })
}

/// Whether a partial report stands beside the error's format, and is of it when both are of this version.
fn formats_agree(parts: &ErrorParts) -> bool {
  match (&parts.format, &parts.partial_report) {
    (Some(format), Some(report)) => {
      matches!(format, Format::Unrecognized(_))
        || matches!(report.format(), Format::Unrecognized(_))
        || format == report.format()
    }
    (None, Some(_)) => matches!(parts.kind, ErrorKind::Unrecognized(_)),
    _ => true,
  }
}

/// Whether the limits that the candidates' reasons and the observed entries' reasons name are exhausted.
fn counters_found(parts: &ErrorParts) -> bool {
  let exhausted = parts.record.exhausted_limits();
  let exhausted_set: HashSet<&Limit> = exhausted.iter().collect();
  let candidates_found = parts
    .candidates
    .iter()
    .filter_map(CandidateCheck::reason)
    .all(|reason| reason.finds_its_counter(|limit| exhausted_set.contains(limit)));
  candidates_found
    && parts
      .observations
      .as_ref()
      .is_none_or(|observations| reasons_find_counters(observations.sources(), exhausted))
}
