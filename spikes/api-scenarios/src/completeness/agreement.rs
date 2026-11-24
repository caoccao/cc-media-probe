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

//! How a completeness is read, `api.md` section 5.3: its members as written, then the agreement of its reasons, its
//! record's first reasons and its statuses, part by part (twelfth review). Since the fifteenth review the check runs
//! wherever a completeness is read, alone too, since it needs nothing but the completeness: the review read a
//! standalone completeness whose tags were incomplete without a first reason, which a report refused. The one check
//! that needs the streams, of a headers status of a newer version beside a known header state that is not final,
//! stays with the report (`crate::model`).
//!
//! The same review asked that the record's omitted count and its first reasons agree with the kept reasons:
//! `reasons()` lists the reasons in the order recorded, and each reason that it does not keep is counted in
//! `omitted_reasons()`. As the sixteenth review asked, each first reason that the kept reasons lack or change counts
//! on its own: one whose part has no kept reason, and one whose part's first kept reason differs from it apart from
//! its detail, since that part's first reason was then not kept. A reason omitted belongs to a part that fell short,
//! so a count above 0 needs a first reason. The review read a completeness whose statuses were all complete, with no
//! first reason and one reason omitted, for which `is_complete()` was true, and two parts whose first kept reasons
//! both differed with one reason omitted. The check of the statuses now covers the attachments too, which the same
//! review found incomplete with no reason at all.
//!
//! A reference omitted has the reason of the part that found it, as the nineteenth review asked: `LimitExhausted`
//! naming the retained data, for a part that finds references (`api.md` section 6.12). That reason is kept in
//! `reasons()`, is the part's first reason in the record, or is counted among the reasons omitted beyond the first
//! reasons that the kept ones lack. The review read one reference omitted, the retained data exhausted and no reason
//! omitted, with a truncation as discovery's only reason, which the record alone accepted.

use super::reason::ReasonKind;
use super::record::{LimitRecord, finds_references};
use super::statuses::{Completeness, DetectionStatus, DiscoveryStatus, HeadersStatus, Part, PartReason, SectionStatus};
use crate::constants::REASONS_DISAGREE_MESSAGE;
use crate::document::InvalidDocument;
use crate::limits::Limit;

crate::neutral::checked_strict_traits!(Completeness from CompletenessData);

/// A completeness's members as read, before they are checked against each other.
#[derive(serde::Deserialize)]
pub(super) struct CompletenessData {
  detection: DetectionStatus,
  discovery: DiscoveryStatus,
  headers: HeadersStatus,
  tags: SectionStatus,
  chapters: SectionStatus,
  attachments: SectionStatus,
  reasons: Vec<PartReason>,
  record: LimitRecord,
}

impl TryFrom<CompletenessData> for Completeness {
  type Error = InvalidDocument;

  fn try_from(data: CompletenessData) -> Result<Completeness, InvalidDocument> {
    let completeness = Completeness {
      detection: data.detection,
      discovery: data.discovery,
      headers: data.headers,
      tags: data.tags,
      chapters: data.chapters,
      attachments: data.attachments,
      reasons: data.reasons,
      record: data.record,
    };
    let agree = kept_reasons_agree(&completeness)
      && statuses_agree(&completeness)
      && omissions_agree(&completeness)
      && references_agree(&completeness);
    if !agree {
      return Err(InvalidDocument::new(REASONS_DISAGREE_MESSAGE));
    }
    Ok(completeness)
  }
}

/// Whether each kept reason has the first reason of its part, and a reason that names an exhausted limit finds it in
/// the record (fourteenth review).
fn kept_reasons_agree(completeness: &Completeness) -> bool {
  let record = completeness.record();
  completeness.reasons().iter().all(|reason| {
    record.first_reason(reason.part()).is_some() && reason.reason().finds_its_counter(record.exhausted_limits())
  })
}

/// Whether a part that this version knows has a first reason exactly when its status records a shortfall. A status
/// of a newer version agrees either way; the report checks the headers against its streams beside one.
fn statuses_agree(completeness: &Completeness) -> bool {
  let record = completeness.record();
  let statuses = [
    (Part::Detection, detection_shortfall(completeness.detection())),
    (Part::Discovery, discovery_shortfall(completeness.discovery())),
    (Part::Headers, headers_shortfall(completeness.headers())),
    (Part::Tags, section_shortfall(completeness.tags())),
    (Part::Chapters, section_shortfall(completeness.chapters())),
    (Part::Attachments, section_shortfall(completeness.attachments())),
  ];
  statuses
    .iter()
    .all(|(part, shortfall)| shortfall.is_none_or(|expected| expected == record.first_reason(part).is_some()))
}

/// Whether the omitted count covers, one by one, each first reason that the kept reasons lack or change, the first
/// kept reason of its part being that first reason apart from its detail when it was kept (fifteenth and sixteenth
/// reviews). That a count above 0 has a first reason to belong to, the record checks itself (seventeenth review).
fn omissions_agree(completeness: &Completeness) -> bool {
  first_reasons_not_kept(completeness).is_some_and(|not_kept| not_kept <= completeness.record().omitted_reasons())
}

/// The number of first reasons that the kept reasons lack or change.
fn first_reasons_not_kept(completeness: &Completeness) -> Option<u64> {
  let record: &LimitRecord = completeness.record();
  let first_kept = |first: &PartReason| completeness.reasons().iter().find(|kept| kept.part() == first.part());
  let not_kept = record
    .first_reasons()
    .iter()
    .filter(|first| !first_kept(first).is_some_and(|kept| kept.same_apart_from_detail(first)))
    .count();
  u64::try_from(not_kept).ok()
}

/// Whether a reference omitted has the reason of the part that found it, the retained data exhausted: kept, as that
/// part's first reason, which is then kept or counted, or counted among the reasons omitted beyond the first reasons
/// that the kept ones lack (nineteenth review).
fn references_agree(completeness: &Completeness) -> bool {
  let record = completeness.record();
  if record.omitted_references() == 0 {
    return true;
  }
  let retained = |reason: &PartReason| {
    finds_references(reason.part())
      && *reason.reason().kind() == ReasonKind::LimitExhausted
      && reason.reason().limit() == Some(&Limit::RetainedData)
  };
  let beyond_the_first =
    first_reasons_not_kept(completeness).is_some_and(|not_kept| record.omitted_reasons() > not_kept);
  completeness.reasons().iter().any(retained) || record.first_reasons().iter().any(retained) || beyond_the_first
}

/// Whether the status records a shortfall; `None` for a status of a newer version.
fn detection_shortfall(status: &DetectionStatus) -> Option<bool> {
  match status {
    DetectionStatus::Complete => Some(false),
    DetectionStatus::Incomplete => Some(true),
    DetectionStatus::Unrecognized(_) => None,
  }
}

fn discovery_shortfall(status: &DiscoveryStatus) -> Option<bool> {
  match status {
    DiscoveryStatus::Complete => Some(false),
    DiscoveryStatus::Incomplete => Some(true),
    DiscoveryStatus::Unrecognized(_) => None,
  }
}

fn headers_shortfall(status: &HeadersStatus) -> Option<bool> {
  match status {
    HeadersStatus::Complete => Some(false),
    HeadersStatus::Incomplete => Some(true),
    HeadersStatus::Unrecognized(_) => None,
  }
}

fn section_shortfall(status: &SectionStatus) -> Option<bool> {
  match status {
    SectionStatus::Complete | SectionStatus::NotRequested | SectionStatus::NotApplicable => Some(false),
    SectionStatus::Incomplete => Some(true),
    SectionStatus::Unrecognized(_) => None,
  }
}
