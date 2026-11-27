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

//! The outcome that a record warrants, by the table of plan section 8.9 (`api.md` section 12; `json-output.md`
//! section 3), which an entry's constructors derive and reading checks.
//!
//! - **A report** warrants `ok` when it is complete, by the test of `Completeness::is_complete()`, and `partial`
//!   when a status that this version knows records a shortfall or its record holds a first reason, whatever a status
//!   of a newer version says, since no newer status makes such a report complete. A report whose known statuses are
//!   all complete and whose record holds no first reason, beside a status of a newer version, warrants no outcome
//!   that this version can derive: it allows `ok` and `partial`, and never `error`. A known header state that is
//!   not final, which reading holds to a first reason of the headers beside a headers status of a newer version, is
//!   seen through that first reason.
//! - **A detection** warrants `ok` when the format was identified, `partial` for a family only, an ambiguous result
//!   or a hint only, and `error` for insufficient data or an unknown format. A detection outcome of a newer version
//!   warrants none, and allows every outcome that this version knows.
//! - **An error record** allows `error` alone.
//!
//! An outcome of a newer version is allowed by no record here: reading keeps such an entry without these checks, and
//! `DeriveOutcomeError::with_outcome` refuses it.

use super::EntryOutcome;
use crate::completeness::shortfalls;
use crate::model::{Detection, DetectionOutcome, MediaReport};

impl EntryOutcome {
  /// The outcome that a report warrants, or `None` for one that only a status of a newer version decides.
  pub(super) fn of_report(report: &MediaReport) -> Option<EntryOutcome> {
    let completeness = report.completeness();
    if completeness.is_complete() {
      return Some(EntryOutcome::Ok);
    }
    let known_shortfall = completeness.record().has_first_reason()
      || shortfalls(completeness)
        .iter()
        .any(|(_, shortfall)| *shortfall == Some(true));
    known_shortfall.then_some(EntryOutcome::Partial)
  }

  /// The outcome that a detection warrants, or `None` for a detection outcome of a newer version.
  pub(super) fn of_detection(detection: &Detection) -> Option<EntryOutcome> {
    match detection.outcome() {
      DetectionOutcome::Identified => Some(EntryOutcome::Ok),
      DetectionOutcome::FamilyOnly | DetectionOutcome::Ambiguous | DetectionOutcome::HintOnly => {
        Some(EntryOutcome::Partial)
      }
      DetectionOutcome::InsufficientData | DetectionOutcome::Unknown => Some(EntryOutcome::Error),
      DetectionOutcome::Unrecognized(_) => None,
    }
  }

  /// Whether a report allows this outcome: the one that it warrants, or `ok` and `partial` for one that only a
  /// status of a newer version decides.
  pub(super) fn allows_report(&self, report: &MediaReport) -> bool {
    self.allows(EntryOutcome::of_report(report), |outcome| {
      matches!(outcome, EntryOutcome::Ok | EntryOutcome::Partial)
    })
  }

  /// Whether a detection allows this outcome: the one that it warrants, or any that this version knows beside a
  /// detection outcome of a newer version.
  pub(super) fn allows_detection(&self, detection: &Detection) -> bool {
    self.allows(EntryOutcome::of_detection(detection), |_| true)
  }

  /// Whether a record that warrants that outcome allows this one: an outcome that this version knows, equal to the
  /// warranted one, or one that `open` takes where the record warrants none that this version can derive.
  fn allows(&self, warranted: Option<EntryOutcome>, open: fn(&EntryOutcome) -> bool) -> bool {
    match (self, warranted) {
      (EntryOutcome::Unrecognized(_), _) => false,
      (outcome, Some(warranted)) => *outcome == warranted,
      (outcome, None) => open(outcome),
    }
  }
}
