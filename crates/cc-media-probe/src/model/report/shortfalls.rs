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

//! The shortfalls that a report's own records state (`api.md` sections 5.3, 5.4, 6.3, 6.9 and 13; `json-output.md`
//! section 10), held against its completeness, as the maintainer chose at the second review of the third slice's
//! first part, which read a tag set of an incomplete scope and a companion refused by a limit beside a complete
//! report:
//!
//! - **The tags.** A tag set whose target scope is incomplete leaves the tags incomplete (section 6.9).
//! - **The companions.** A companion's entry in the source table says that lookup was on and a request was made, so
//!   the companions are complete or cut short, and cut short when the entry was not opened: not found, declined or
//!   refused by a limit (section 13). An expected companion that the report lists says that the input expects one,
//!   so the companions are not "not applicable". The reason of an entry that names a limit finds its counter among
//!   the record's exhausted limits.
//!
//! The third review of that part read a nameless expected companion beside complete companions, so, as the
//! maintainer chose, an expected companion that no entry requested, whose name no entry's requested name is, rules
//! out complete companions: lookup was off, no name could be built, or a limit refused it before its request. A
//! nameless one matches no entry. That review also read a tag set of an incomplete scope beside tags of a newer
//! status and no first reason, so a shortfall that a record states beside a status of a newer version needs that
//! part's first reason, which the newer status cannot erase, as a header state does (section 5.3).
//!
//! The fourth review of that part read a protection declaration of the scope `unread` beside complete discovery, so
//! such a declaration, which records an interruption of the structure that held it, leaves discovery incomplete
//! (section 6.3). A protection that was not examined in full states no shortfall of its own, since an encoding that
//! this version cannot undo leaves it so by design.
//!
//! The reasons that the records state agree with the completeness: a reason that names a request is its entry's
//! (`crate::source::requests_agree`); and the reason of each entry not opened and of each header state is kept, the
//! first reason of its part, or counted among the omitted reasons, with a limit that a header state's reason names
//! among the exhausted ones (`super::header_reasons`, fourth review).

use std::collections::HashSet;

use super::MediaReport;
use super::constants::{SHORTFALL_MESSAGE, UNCOUNTED_REASON_MESSAGE};
use super::header_reasons::{header_reasons_find_counters, header_reasons_not_kept};
use crate::completeness::{CompanionStatus, Part, Shortfall, omitted_at_least};
use crate::model::{Protection, ProtectionDeclaration, ProtectionScope, ReferenceKind, Stream, TagSet, TargetScope};
use crate::source::{
  REQUEST_REASON_MESSAGE, SourceRole, reasons_find_counters, request_reasons_not_kept, requests_agree,
};

/// The checks of the shortfalls that the report's records state.
pub(super) fn check_shortfalls(report: &MediaReport) -> Result<(), &'static str> {
  let completeness = &report.completeness;
  let record = completeness.record();
  let stated = tags_agree(report)
    && discovery_agrees(report)
    && companions_agree(report)
    && reasons_find_counters(&report.sources, record.exhausted_limits())
    && header_reasons_find_counters(report);
  if !stated {
    return Err(SHORTFALL_MESSAGE);
  }
  let reasons = completeness.reasons().iter().chain(record.first_reasons());
  if !requests_agree(&report.sources, reasons) {
    return Err(REQUEST_REASON_MESSAGE);
  }
  let counted = omitted_at_least(completeness)
    .saturating_add(request_reasons_not_kept(&report.sources, completeness))
    .saturating_add(header_reasons_not_kept(report));
  if counted <= record.omitted_reasons() {
    Ok(())
  } else {
    Err(UNCOUNTED_REASON_MESSAGE)
  }
}

/// Whether the part has its first reason in the record, which a shortfall beside a status of a newer version needs.
fn has_first_reason(report: &MediaReport, part: &Part) -> bool {
  report.completeness.record().first_reason(part).is_some()
}

/// Whether a shortfall that a record states, when `cut`, stands beside a status that records one, or beside a status
/// of a newer version, `None`, and the part's first reason.
fn stated_beside(report: &MediaReport, part: &Part, shortfall: Option<bool>, cut: bool) -> bool {
  match shortfall {
    Some(true) => true,
    Some(false) => !cut,
    None => !cut || has_first_reason(report, part),
  }
}

/// Whether a tag set of an incomplete scope stands beside incomplete tags.
fn tags_agree(report: &MediaReport) -> bool {
  let cut = report
    .tag_sets
    .iter()
    .map(TagSet::target_scope)
    .any(|scope| matches!(scope, TargetScope::Incomplete));
  stated_beside(report, &Part::Tags, report.completeness.tags().shortfall(), cut)
}

/// Whether a protection declaration whose scope was not read stands beside incomplete discovery.
fn discovery_agrees(report: &MediaReport) -> bool {
  let cut = report
    .streams
    .iter()
    .filter_map(Stream::protection)
    .flat_map(Protection::declarations)
    .map(ProtectionDeclaration::scope)
    .any(|scope| matches!(scope, ProtectionScope::Unread));
  stated_beside(
    report,
    &Part::Discovery,
    report.completeness.discovery().shortfall(),
    cut,
  )
}

/// Whether the companions status agrees with the source table and the expected companions.
fn companions_agree(report: &MediaReport) -> bool {
  let companions: Vec<_> = report
    .sources
    .iter()
    .filter(|entry| !matches!(entry.role(), SourceRole::Primary))
    .collect();
  let refused = companions.iter().any(|entry| entry.was_refused());
  let requested_names: HashSet<&str> = companions
    .iter()
    .filter_map(|entry| entry.attempted_identity())
    .filter_map(|identity| identity.requested_name())
    .collect();
  let mut expected = report
    .external_references
    .iter()
    .filter(|reference| matches!(reference.kind(), ReferenceKind::ExpectedCompanion))
    .peekable();
  let any_expected = expected.peek().is_some();
  let unrequested = expected.any(|reference| reference.location().is_none_or(|name| !requested_names.contains(name)));
  let requested = !companions.is_empty();
  match report.completeness.companions() {
    CompanionStatus::NotApplicable => !requested && !any_expected,
    CompanionStatus::NotEnabled => !requested,
    CompanionStatus::Complete => !refused && !unrequested,
    CompanionStatus::CutShort => true,
    CompanionStatus::Unrecognized(_) => !(refused || unrequested) || has_first_reason(report, &Part::Companions),
  }
}
