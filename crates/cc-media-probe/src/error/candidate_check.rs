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

//! `CandidateCheck` and `CheckState`, how the check of one candidate went (`api.md` sections 5.1 and 10.1).

use super::constants::{NOT_CHECKABLE_NAME, OUTRANKED_NAME, REJECTED_NAME, VALIDATED_NAME};
use crate::Format;
use crate::completeness::Reason;
use crate::form_record::form_record;
use crate::model::Confidence;
use crate::string_enumeration::string_enumeration;

form_record! {
  /// How the check of one candidate format went, in the rank of `api.md` section 5.1; each candidate is a different
  /// format.
  ///
  /// ```
  /// use cc_media_probe::Error;
  /// use cc_media_probe::error::CheckState;
  ///
  /// fn checks(error: &Error) -> Vec<String> {
  ///     error
  ///         .candidates()
  ///         .iter()
  ///         .map(|check| {
  ///             let cause = check.reason().map_or("", |reason| reason.kind().as_str());
  ///             let state = match check.state() {
  ///                 CheckState::NotCheckable => "could not be checked",
  ///                 state => state.as_str(),
  ///             };
  ///             format!("{} ({}): {state} {cause}", check.format(), check.confidence())
  ///         })
  ///         .collect()
  /// }
  /// ```
  #[derive(Clone, Debug, PartialEq, Eq, Hash)]
  pub struct CandidateCheck {
    format: Format,
    confidence: Confidence,
    state: CheckState,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    reason: Option<Reason>,
  }
  check = CandidateCheck::check_form;
}

impl CandidateCheck {
  /// The candidate's format.
  #[must_use]
  pub fn format(&self) -> &Format {
    &self.format
  }

  /// How sure detection was of the candidate.
  #[must_use]
  pub fn confidence(&self) -> Confidence {
    self.confidence
  }

  /// How the check went.
  #[must_use]
  pub fn state(&self) -> &CheckState {
    &self.state
  }

  /// The cause that kept the candidate from validating, when it has one: for `NotCheckable`, an unavailable
  /// range, a truncated input or a limit by name. A candidate without a check of its own has none.
  #[must_use]
  pub fn reason(&self) -> Option<&Reason> {
    self.reason.as_ref()
  }

  /// Whether the reason fits the state (`api.md` section 5.1), as the second review of the third slice's first part
  /// asked, which read a validated candidate with the reason `truncated_input`: only a candidate that could not be
  /// checked has a cause, an unavailable range, a truncated input or a limit by name, and a state or a reason of a
  /// newer version is not checked.
  #[cfg(feature = "serde")]
  fn check_form(&self) -> Result<(), &'static str> {
    use crate::completeness::ReasonKind;
    let fits = match (&self.state, self.reason.as_ref().map(Reason::kind)) {
      (_, None) | (CheckState::Unrecognized(_), _) => true,
      (CheckState::NotCheckable, Some(kind)) => matches!(
        kind,
        ReasonKind::RangeUnavailable
          | ReasonKind::TruncatedInput
          | ReasonKind::LimitExhausted
          | ReasonKind::Unrecognized(_)
      ),
      _ => false,
    };
    if fits {
      Ok(())
    } else {
      Err(super::constants::CANDIDATE_CHECK_MESSAGE)
    }
  }
}

string_enumeration! {
  /// How the check of a candidate went. `CandidateCheck` shows an example.
  ///
  /// ```
  /// use cc_media_probe::error::CheckState;
  ///
  /// assert_eq!(CheckState::NotCheckable.as_str(), "not_checkable");
  /// ```
  CheckState {
    /// The check found the format.
    Validated => VALIDATED_NAME,
    /// The check found that it is not the format.
    Rejected => REJECTED_NAME,
    /// The check could not run in full, or the format has no check.
    NotCheckable => NOT_CHECKABLE_NAME,
    /// The check never ran, because a candidate ranked ahead validated.
    Outranked => OUTRANKED_NAME,
  }
}

#[cfg(test)]
mod tests {
  use super::{CandidateCheck, CheckState};
  use crate::Format;
  use crate::completeness::{Reason, ReasonKind};
  use crate::model::{Confidence, UnrecognizedValue};

  impl CandidateCheck {
    /// The check of this format in this state, with this reason.
    pub(crate) fn sample(format: Format, state: CheckState, reason: Option<Reason>) -> CandidateCheck {
      CandidateCheck {
        format,
        confidence: Confidence::new(50).unwrap(),
        state,
        reason,
      }
    }
  }

  #[test]
  fn keeps_how_the_check_went() {
    let check = CandidateCheck::sample(
      Format::Mp4,
      CheckState::NotCheckable,
      Some(Reason::sample(ReasonKind::RangeUnavailable)),
    );
    assert_eq!(check.format(), &Format::Mp4);
    assert_eq!(check.confidence().value(), 50);
    assert_eq!(check.state(), &CheckState::NotCheckable);
    assert_eq!(check.reason().map(Reason::kind), Some(&ReasonKind::RangeUnavailable));
    for (state, name) in [
      (CheckState::Validated, "validated"),
      (CheckState::Rejected, "rejected"),
      (CheckState::NotCheckable, "not_checkable"),
      (CheckState::Outranked, "outranked"),
      (
        CheckState::Unrecognized(UnrecognizedValue::sample("deferred")),
        "deferred",
      ),
    ] {
      assert_eq!(state.as_str(), name);
    }
  }
}
