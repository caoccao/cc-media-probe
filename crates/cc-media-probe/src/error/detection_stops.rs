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

//! The limits that stopped detection in an error record (`api.md` sections 5.1 and 5.3), as the maintainer chose at
//! the fourth review of the third slice's first part, which read an undetermined format of too few bytes whose read
//! limit the record said had stopped discovery first:
//!
//! - **The first part a limit stopped.** Detection comes first, so a limit that a candidate's cause names, or the
//!   reason of an undetermined format, stopped detection first. The record checks the same of its first reasons of
//!   detection, wherever it is read.
//! - **A probe that never ran its prober.** An unknown, undetermined or unsupported format ends the probe before
//!   discovery begins, so its record names no other part: every part that a limit stopped first, and the part of
//!   every first reason, is detection. As the maintainer chose at the seventh review of that part, which read a
//!   primary file that could not be opened, the same holds for a source failure before detection found a format
//!   (`super::activities::prober_ran`).
//!
//! A kind of a newer version is not checked.

use super::constants::DETECTION_STOPS_MESSAGE;
use super::parts::ErrorParts;
use super::{CandidateCheck, ErrorKind};
use crate::completeness::Part;

/// The checks of the limits and the parts that stopped detection.
pub(super) fn check_detection_stops(parts: &ErrorParts) -> Result<(), &'static str> {
  let record = &parts.record;
  if matches!(parts.kind, ErrorKind::Unrecognized(_)) {
    return Ok(());
  }
  let undetermined = parts
    .reason
    .as_ref()
    .filter(|_| matches!(parts.kind, ErrorKind::FormatNotDetermined));
  let causes = parts.candidates.iter().filter_map(CandidateCheck::reason);
  let stopped_first = record.stopped_detection_first(causes.chain(undetermined));
  let detection_only = super::activities::prober_ran(parts)
    || (record
      .first_reasons()
      .iter()
      .all(|first| *first.part() == Part::Detection)
      && record.stopped_parts().all(|part| *part == Part::Detection));
  if stopped_first && detection_only {
    Ok(())
  } else {
    Err(DETECTION_STOPS_MESSAGE)
  }
}
