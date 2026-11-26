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

//! The members that an error's known kind decides (`api.md` sections 5.1 and 10; `json-output.md` section 4.4), as
//! the review of the third slice's first part found an unsupported format without its format and an unknown format
//! with one: the format exactly for an unsupported format, a disabled feature and a probe stopped before its
//! inventory, which name it, and never for an unknown or undetermined format; a hint only for an unknown format; no
//! candidate outranked beside a kind at which no candidate validated; and at most 32 causes, as the chain is
//! followed. A source failure may name a format or not, and a kind of a newer version is not checked.
//!
//! The second review of that part read endings that section 5.1 rules out, which are checked too:
//!
//! - **The candidates.** An unknown format's candidates were all rejected, and an undetermined format's include no
//!   validated one. An undetermined format has a reason, for too few bytes, or a candidate that could not be
//!   checked, which carries its own cause.
//! - **The reason.** A disabled feature's reason is `feature_disabled`, which names the feature, and a probe stopped
//!   before its inventory has the reason that stopped it.
//! - **The source.** A source failure names the source that failed.
//!
//! The third review of that part read an undetermined format whose reason was a companion's decline, and one whose
//! candidates that could not be checked stood beside a reason of its own, so the two rows of its ending are told
//! apart, as a detection tells them (section 8): too few bytes have a reason, the input's end or a limit, and no
//! candidate; an ambiguous result has no reason, and a candidate that could not be checked, which carries its own
//! cause, or one of a state of a newer version, which only its version can tell.

use super::constants::{ERROR_KIND_MESSAGE, MAXIMUM_CAUSES};
use super::parts::ErrorParts;
use super::record::Description;
use super::{CheckState, ErrorKind};
use crate::completeness::ReasonKind;

/// The checks of the members that a known kind decides.
pub(super) fn check_kind(parts: &ErrorParts, description: &Description) -> Result<(), &'static str> {
  let format = match parts.kind {
    ErrorKind::UnsupportedFormat | ErrorKind::FeatureDisabled | ErrorKind::NoStreamInventory => parts.format.is_some(),
    ErrorKind::UnknownFormat | ErrorKind::FormatNotDetermined => parts.format.is_none(),
    ErrorKind::SourceFailed | ErrorKind::Unrecognized(_) => true,
  };
  let hint = parts.hint.is_none() || matches!(parts.kind, ErrorKind::UnknownFormat | ErrorKind::Unrecognized(_));
  let reason_kind = parts.reason.as_ref().map(crate::completeness::Reason::kind);
  let ending = match parts.kind {
    ErrorKind::UnknownFormat => states_all(parts, |state| matches!(state, CheckState::Rejected)),
    ErrorKind::FormatNotDetermined => {
      let row = match &parts.reason {
        Some(reason) => parts.candidates.is_empty() && reason.tells_too_few_bytes(),
        None => states_any(parts, |state| {
          matches!(state, CheckState::NotCheckable | CheckState::Unrecognized(_))
        }),
      };
      row
        && states_all(parts, |state| {
          !matches!(state, CheckState::Validated | CheckState::Outranked)
        })
    }
    ErrorKind::FeatureDisabled => matches!(reason_kind, Some(ReasonKind::FeatureDisabled)),
    ErrorKind::NoStreamInventory => reason_kind.is_some(),
    ErrorKind::SourceFailed => parts.source_identifier.is_some(),
    ErrorKind::UnsupportedFormat | ErrorKind::Unrecognized(_) => true,
  };
  if format && hint && ending && description.cause_messages.len() <= MAXIMUM_CAUSES {
    Ok(())
  } else {
    Err(ERROR_KIND_MESSAGE)
  }
}

/// Whether every candidate of a known state is in a state that the test takes; one of a newer version is not checked.
fn states_all(parts: &ErrorParts, test: impl Fn(&CheckState) -> bool) -> bool {
  parts
    .candidates
    .iter()
    .map(super::CandidateCheck::state)
    .all(|state| matches!(state, CheckState::Unrecognized(_)) || test(state))
}

/// Whether some candidate is in a state that the test takes.
fn states_any(parts: &ErrorParts, test: impl Fn(&CheckState) -> bool) -> bool {
  parts.candidates.iter().map(super::CandidateCheck::state).any(test)
}
