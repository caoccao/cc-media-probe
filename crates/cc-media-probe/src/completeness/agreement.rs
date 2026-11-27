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

//! The agreement of a completeness that was read (`json-output.md` section 7.2): its kept reasons, its record's first
//! reasons and its statuses agree part by part, wherever it is read, alone too, since the check needs nothing but the
//! completeness (fifteenth review of `api.md`). The one check of that agreement that needs the streams, of a headers
//! status of a newer version beside a known header state that is not final, stays with the report.
//!
//! - **The kept reasons.** Each reason in `reasons` has the first reason of its part in the record, and a limit
//!   reason among them names a limit that the record pairs with the part that it stopped first.
//! - **The statuses.** A known part whose status records a shortfall has a first reason, and one whose status
//!   records none has no first reason. A status of a newer version leaves its part open.
//! - **The omitted reasons.** Each first reason that `reasons` lacks or changes was counted in `omitted_reasons`, on
//!   its own (sixteenth review).
//! - **The omitted references.** An omitted reference has the reason of the part that found it, `limit_exhausted`
//!   naming the retained data: kept in `reasons`, the part's first reason, or counted among the reasons omitted beyond
//!   the first reasons that `reasons` lacks (nineteenth review). `omitted_at_least` gives the count that both
//!   rules need, to which a report adds the reasons of its source entries that `reasons` lacks (third review of the
//!   third slice's first part).
//! - **The companion counters.** Companions that are not applicable or not enabled never ran, so the record exhausts
//!   neither of the counters that only companion lookup charges, as the maintainer chose at the sixth review of the
//!   third slice's first part, which read the companion requests exhausted beside companions not applicable. A
//!   status of a newer version is not checked.
//!
//! Each check looks a part up in an index of the reasons by part, never in a list, so the checks stay linear
//! whatever parts of a newer version a completeness names (review of the third slice's first part).

use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use super::constants::{COMPANION_COUNTER_MESSAGE, PART_COUNT, REASONS_DISAGREE_MESSAGE};
use super::{CompanionStatus, Completeness, LimitRecord, Part, PartReason, Reason};
use crate::Limit;
use crate::constants::COMPANION_LIMITS;

/// Whether the statuses, the reasons and the record of a completeness agree.
pub(super) fn check_agreement(completeness: &Completeness) -> Result<(), &'static str> {
  let agree = kept_reasons_agree(completeness)
    && statuses_agree(completeness)
    && omitted_at_least(completeness) <= completeness.record().omitted_reasons();
  if !agree {
    return Err(REASONS_DISAGREE_MESSAGE);
  }
  let companions_never_ran = matches!(
    completeness.companions(),
    CompanionStatus::NotApplicable | CompanionStatus::NotEnabled
  );
  if companions_never_ran && !completeness.record().exhausts_none_of(&COMPANION_LIMITS) {
    Err(COMPANION_COUNTER_MESSAGE)
  } else {
    Ok(())
  }
}

/// The fewest reasons that the completeness says were omitted: each first reason that the kept reasons lack or
/// change, and the reason of an omitted reference or of an omitted declaration when it is neither kept nor first,
/// one for both, since one reason of discovery naming the retained data can give both. A report adds the reasons of
/// its source entries and of its streams' header states that the kept reasons lack (`api.md` sections 5.4 and 6.15).
pub(crate) fn omitted_at_least(completeness: &Completeness) -> u64 {
  let not_kept = first_reasons_not_kept(completeness);
  if reference_reason_counted(completeness) || declaration_reason_counted(completeness) {
    not_kept.saturating_add(1)
  } else {
    not_kept
  }
}

/// Whether each kept reason has its part's first reason in the record, and names only a limit that stopped a part
/// (fifth review of the third slice's first part), which for a reason of detection stopped detection first (fourth
/// review).
fn kept_reasons_agree(completeness: &Completeness) -> bool {
  let record = completeness.record();
  let first_parts: HashSet<&Part> = record.first_reasons().iter().map(PartReason::part).collect();
  record.stopped_detection_first(LimitRecord::of_detection(completeness.reasons().iter()))
    && record.limits_stopped(completeness.reasons().iter().map(PartReason::reason))
    && completeness
      .reasons()
      .iter()
      .all(|reason| first_parts.contains(reason.part()))
}

/// Whether a known status records a shortfall: `None` for a status of a newer version.
fn shortfall(satisfied: bool, unrecognized: bool) -> Option<bool> {
  (!unrecognized).then_some(!satisfied)
}

/// Each part with whether its status records a shortfall, or `None` for a status of a newer version, which the
/// agreement of a completeness and the outcome of a document's entry read.
pub(crate) fn shortfalls(completeness: &Completeness) -> [(Part, Option<bool>); PART_COUNT] {
  [
    (Part::Detection, completeness.detection().shortfall()),
    (Part::Discovery, completeness.discovery().shortfall()),
    (Part::Headers, completeness.headers().shortfall()),
    (Part::Tags, completeness.tags().shortfall()),
    (Part::Chapters, completeness.chapters().shortfall()),
    (Part::Attachments, completeness.attachments().shortfall()),
    (Part::Timing, completeness.timing().shortfall()),
    (Part::Companions, completeness.companions().shortfall()),
  ]
}

/// Whether each known part's status and its first reason agree.
fn statuses_agree(completeness: &Completeness) -> bool {
  let record = completeness.record();
  shortfalls(completeness)
    .iter()
    .all(|(part, shortfall)| shortfall.is_none_or(|expected| expected == record.first_reason(part).is_some()))
}

/// The number of the record's first reasons that the kept reasons lack or change.
fn first_reasons_not_kept(completeness: &Completeness) -> u64 {
  let mut first_kept: HashMap<&Part, &PartReason> = HashMap::with_capacity(completeness.reasons().len());
  for kept in completeness.reasons() {
    first_kept.entry(kept.part()).or_insert(kept);
  }
  let not_kept = completeness
    .record()
    .first_reasons()
    .iter()
    .filter(|first| {
      !first_kept
        .get(first.part())
        .is_some_and(|kept| kept.same_apart_from_detail(first))
    })
    .count();
  u64::try_from(not_kept).unwrap_or(u64::MAX)
}

/// Whether an omitted reference's reason, the retained data exhausted for a part that finds references, is neither
/// kept nor first, so that it can only have been counted among the omitted reasons.
fn reference_reason_counted(completeness: &Completeness) -> bool {
  let omitted = completeness.record().omitted_references();
  counted(completeness, omitted, |reason| {
    reason.part().finds_references() && names(reason, &[Limit::RetainedData])
  })
}

/// Whether an omitted declaration's reason, the stream limit or the retained data exhausted for discovery, is
/// neither kept nor first (plan section 7.5), as the maintainer chose at the fourth review of the third slice's first
/// part, which read a declaration omitted beside complete discovery.
fn declaration_reason_counted(completeness: &Completeness) -> bool {
  let omitted = completeness.record().omitted_declarations();
  counted(completeness, omitted, |reason| {
    *reason.part() == Part::Discovery && names(reason, &[Limit::Streams, Limit::RetainedData])
  })
}

/// Whether something was omitted whose reason, which `stated` tells, is neither kept nor first.
fn counted(completeness: &Completeness, omitted: u64, stated: impl Fn(&PartReason) -> bool) -> bool {
  omitted > 0
    && !completeness.reasons().iter().any(&stated)
    && !completeness.record().first_reasons().iter().any(&stated)
}

/// Whether the reason is the exhaustion of one of the limits.
fn names(reason: &PartReason, limits: &[Limit]) -> bool {
  reason
    .reason()
    .exhausted_limit()
    .is_some_and(|limit| limits.contains(limit))
}

/// The number of the reasons that a report's records state, each with the key of the record that keeps it, that
/// are neither among the kept reasons of `part` with that key nor the record's first reason of `part`, so that each
/// can only have been counted among the omitted reasons: the reasons of the source entries by their source (third
/// review of the third slice's first part) and of the header states by their stream (fourth review). The first
/// reason that the kept reasons lack is counted among the first reasons, so it counts once.
pub(crate) fn stated_reasons_not_kept<'reason, Key: Eq + Hash>(
  completeness: &Completeness,
  part: &Part,
  key: impl Fn(&PartReason) -> Option<Key>,
  stated: impl Iterator<Item = (Key, &'reason Reason)>,
) -> u64 {
  let first = completeness.record().first_reason(part);
  let mut kept_by_key: HashMap<Key, Vec<&Reason>> = HashMap::new();
  for reason in completeness.reasons().iter().filter(|reason| reason.part() == part) {
    if let Some(found) = key(reason) {
      kept_by_key.entry(found).or_default().push(reason.reason());
    }
  }
  let not_kept = stated.filter(|(found, own)| {
    let given_first =
      first.is_some_and(|first| key(first).as_ref() == Some(found) && first.reason().same_apart_from_detail(own));
    let kept_too = kept_by_key
      .get(found)
      .is_some_and(|reasons| reasons.iter().any(|reason| reason.same_apart_from_detail(own)));
    !given_first && !kept_too
  });
  u64::try_from(not_kept.count()).unwrap_or(u64::MAX)
}

/// The shortfall of each kind of status, which `statuses_agree` reads and a report's records that state a shortfall
/// are held against.
pub(crate) trait Shortfall {
  /// Whether the status records a shortfall, or `None` for a status of a newer version.
  fn shortfall(&self) -> Option<bool>;
}

/// The shortfall of a status from its `is_satisfied()` and its catch-all.
macro_rules! shortfall_of {
  ($($status:ident),* $(,)?) => {
    $(
      impl Shortfall for super::$status {
        fn shortfall(&self) -> Option<bool> {
          shortfall(self.is_satisfied(), matches!(self, super::$status::Unrecognized(_)))
        }
      }
    )*
  };
}

shortfall_of!(
  DetectionStatus,
  DiscoveryStatus,
  HeadersStatus,
  SectionStatus,
  TimingStatus,
  CompanionStatus
);
