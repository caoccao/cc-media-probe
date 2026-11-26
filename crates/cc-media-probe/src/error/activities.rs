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

//! The activities that an error's probe ran and the counters that they charge (`api.md` sections 5.1 and 5.3; plan
//! section 7.5), as the maintainer chose at the sixth review of the third slice's first part, which read an unknown
//! format beside the companion requests exhausted, and at its seventh review, which read the stream limit exhausted
//! beside a primary file that could not be opened, and companion requests counted beside an unknown format:
//!
//! - **A probe that never ran its prober** ended at detection, an unknown, undetermined or unsupported format, or
//!   failed before detection found a format: a source failure without a format, a partial report or observations,
//!   at the source's opening or while detection read. A disabled feature and a probe stopped before its inventory ran
//!   it (section 5.1), whatever they hold, since such a probe omits observations when it observed nothing.
//! - **Such a probe charged no counter of a later activity,** so each limit that its record exhausts can stop
//!   detection: none that only companion lookup charges, nor any that only the prober charges for a stream's header,
//!   an attachment's payload and the streams retained (`crate::Limit::can_stop`). Its usage counts no companion
//!   request, no companion source opened and no payload retained, and the observations that an error which ended at
//!   detection may hold name no companion. Its record names detection alone (`super::detection_stops`).
//!
//! A partial report's completeness checks its companions itself, and the report its usage. A kind of a newer version
//! is not checked.

use super::ErrorKind;
use super::constants::ACTIVITY_COUNTER_MESSAGE;
use super::parts::ErrorParts;
use crate::completeness::Part;
use crate::source::{SourceEntry, SourceRole};

/// Whether the error's probe ran its prober, so that a later activity can have charged its counters: a source failure
/// that holds a partial report, observations or the format that detection found, and a disabled feature or a probe
/// stopped before its inventory. A kind of a newer version is taken to have run it, so it is not checked.
pub(super) fn prober_ran(parts: &ErrorParts) -> bool {
  match parts.kind {
    ErrorKind::SourceFailed => parts.format.is_some() || parts.partial_report.is_some() || parts.observations.is_some(),
    _ => !parts.kind.ended_at_detection(),
  }
}

/// The checks of the counters that the error's probe can have charged.
pub(super) fn check_activities(parts: &ErrorParts) -> Result<(), &'static str> {
  if prober_ran(parts) {
    return Ok(());
  }
  let observed_companion = parts.observations.as_ref().is_some_and(|observations| {
    observations
      .sources()
      .iter()
      .map(SourceEntry::role)
      .any(|role| !matches!(role, SourceRole::Primary))
  });
  let fits =
    parts.record.exhausted_limits_can_stop(&Part::Detection) && parts.usage.charges_no_prober() && !observed_companion;
  if fits { Ok(()) } else { Err(ACTIVITY_COUNTER_MESSAGE) }
}
