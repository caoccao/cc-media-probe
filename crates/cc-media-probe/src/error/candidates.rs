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

//! The candidates of an error record as a list (`api.md` sections 5.1 and 5.3; `json-output.md` section 4.4). They
//! name each format once (fifteenth review), and, as the third review of the third slice's first part asked, which
//! read rejected candidates of the confidences 10 and then 90 and an outranked candidate ahead of the validated one:
//!
//! - **Their rank.** Their confidence never rises, as a detection's does (section 8).
//! - **The order of their checks.** The probe checks the candidates in their rank and uses the first that validates,
//!   so one at most validated, those ahead of it were rejected or could not be checked, and those behind it were
//!   outranked. A candidate of a state of a newer version may have validated, so one outranked may follow it, as
//!   long as no candidate of a known check stands between them: the fourth review of that part read a newer state,
//!   an outranked candidate and then a validated one, and a newer state, a rejected candidate and then an outranked
//!   one, neither of which a probe that stops at its first winner gives.
//! - **The winner.** The error's format is the format of the candidate that validated, when both are of this
//!   version, as that review asked, which read an unsupported Matroska file whose one validated candidate was MP4.
//! - **The first cause.** The record's first reason of detection is the cause of the first candidate in their rank
//!   that has one, as the maintainer chose for every known kind (section 5.3): the probe checks them before anything
//!   else of detection can fall short. A format not determined for too few bytes has its reason as that first
//!   reason. Beside candidates without a cause, a known kind has no first reason of detection, as section 16
//!   decided and the fourth review read unchecked for an unknown format, except a source failure, whose source can
//!   fail while detection reads.
//!
//! A kind of a newer version is not held to the first cause.

use super::constants::{CANDIDATES_MESSAGE, DETECTION_REASON_MESSAGE, VALIDATED_FORMAT_MESSAGE};
use super::parts::ErrorParts;
use super::{CandidateCheck, CheckState, ErrorKind};
use crate::Format;
use crate::completeness::{Part, PartReason, Reason};
use crate::form::each_once;

/// The checks of the candidates as a list.
pub(super) fn check_candidates(parts: &ErrorParts) -> Result<(), &'static str> {
  if !ranked(&parts.candidates) {
    return Err(CANDIDATES_MESSAGE);
  }
  if !winner_named(parts) {
    return Err(VALIDATED_FORMAT_MESSAGE);
  }
  if first_cause_kept(parts) {
    Ok(())
  } else {
    Err(DETECTION_REASON_MESSAGE)
  }
}

/// How far the checks in the rank have come.
#[derive(Clone, Copy)]
enum Checks {
  /// No candidate won yet.
  Running,
  /// A candidate of a newer state came after the last known check, and may have won.
  Undecided,
  /// A candidate won, so every later one was outranked.
  Won,
}

/// Whether the candidates name each format once, fall in confidence and stand in the order of their checks.
fn ranked(candidates: &[CandidateCheck]) -> bool {
  let mut checks = Checks::Running;
  each_once(candidates, CandidateCheck::format)
    && candidates.is_sorted_by(|earlier, later| earlier.confidence() >= later.confidence())
    && candidates.iter().all(|candidate| {
      let (fits, next) = match (candidate.state(), checks) {
        (CheckState::Validated | CheckState::Rejected | CheckState::NotCheckable, Checks::Won)
        | (CheckState::Outranked, Checks::Running) => (false, checks),
        (CheckState::Validated | CheckState::Outranked, _) | (CheckState::Unrecognized(_), Checks::Won) => {
          (true, Checks::Won)
        }
        (CheckState::Rejected | CheckState::NotCheckable, _) => (true, Checks::Running),
        (CheckState::Unrecognized(_), _) => (true, Checks::Undecided),
      };
      checks = next;
      fits
    })
}

/// Whether the error's format is the validated candidate's, when both are of this version.
fn winner_named(parts: &ErrorParts) -> bool {
  let known = |format: &Format| !matches!(format, Format::Unrecognized(_));
  let winner = parts
    .candidates
    .iter()
    .find(|candidate| matches!(candidate.state(), CheckState::Validated))
    .map(CandidateCheck::format);
  match (winner, &parts.format) {
    (Some(winner), Some(format)) if known(winner) && known(format) => winner == format,
    _ => true,
  }
}

/// Whether the record's first reason of detection is the first cause that the candidates or the reason give.
fn first_cause_kept(parts: &ErrorParts) -> bool {
  let first = parts.record.first_reason(&Part::Detection).map(PartReason::reason);
  let same = |cause: &Reason| first.is_some_and(|first| first.same_apart_from_detail(cause));
  match (
    parts.candidates.iter().find_map(CandidateCheck::reason),
    &parts.kind,
    &parts.reason,
  ) {
    (_, ErrorKind::Unrecognized(_), _) | (None, ErrorKind::SourceFailed, _) => true,
    (Some(cause), _, _) => same(cause),
    (None, ErrorKind::FormatNotDetermined, Some(reason)) => same(reason),
    (None, _, _) => first.is_none(),
  }
}
