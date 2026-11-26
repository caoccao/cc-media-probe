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

//! `HeaderStatus`, the outcome of header parsing for one stream (`api.md` section 5.4).

use std::fmt;

use super::{HeaderState, Identification, Reason};
use crate::form_record::form_record;

form_record! {
  /// The outcome of header parsing for one stream, from `Stream::header_status()`.
  ///
  /// `Display` writes the state with its identification or its reason, such as "not found within the budget
  /// (per-stream header limit)".
  ///
  /// ```
  /// use cc_media_probe::Stream;
  ///
  /// fn header(stream: &Stream) -> String {
  ///     let status = stream.header_status();
  ///     if status.is_parsed() {
  ///         "parsed".to_owned()
  ///     } else if status.is_final() {
  ///         format!("{status}, as far as this version goes")
  ///     } else {
  ///         let reason = status.reason().map_or("unknown", |reason| reason.kind().as_str());
  ///         format!("{} ({reason})", status.state().as_str())
  ///     }
  /// }
  /// ```
  #[derive(Clone, Debug, PartialEq, Eq, Hash)]
  pub struct HeaderStatus {
    state: HeaderState,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    identification: Option<Identification>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    reason: Option<Reason>,
  }
  check = HeaderStatus::check_form;
}

impl HeaderStatus {
  /// The state of header parsing.
  #[must_use]
  pub fn state(&self) -> &HeaderState {
    &self.state
  }

  /// Why the header was not parsed, for the state `Identified`.
  #[must_use]
  pub fn identification(&self) -> Option<&Identification> {
    self.identification.as_ref()
  }

  /// Why the header fell short, for the states that are not final: the input's end, a limit by name, an
  /// unavailable range, a damaged structure or, in the partial report of an error, the source that failed.
  #[must_use]
  pub fn reason(&self) -> Option<&Reason> {
    self.reason.as_ref()
  }

  /// Whether the state is final: the probe did everything that this version can do for the header. A state that a
  /// newer version wrote is not final.
  #[must_use]
  pub fn is_final(&self) -> bool {
    self.state.is_final()
  }

  /// Whether every header field was read.
  #[must_use]
  pub fn is_parsed(&self) -> bool {
    matches!(self.state, HeaderState::Parsed)
  }

  /// Whether the status holds the identification and the reason that its state allows (`api.md` section 5.4), as
  /// the maintainer chose at the review of the third slice's first part: an identification only for `identified`,
  /// which needs one; no reason for a final state; for `not_found_within_budget` and
  /// `partially_parsed_for_lack_of_bytes` the reason of a cut, `truncated_input`, `limit_exhausted`,
  /// `range_unavailable` or `source_failed`; for `malformed`, `damaged_structure`; for `compression_unavailable`
  /// none, or `feature_disabled` naming `zlib`; and for `not_attempted` none, or `source_failed`. A reason, a
  /// feature, an identification or a state of a newer version is not checked. As the maintainer chose at the seventh
  /// review of that part, the limit that the reason of a known state names can stop the headers (`Limit::can_stop`).
  #[cfg(feature = "serde")]
  fn check_form(&self) -> Result<(), &'static str> {
    use super::{Feature, ReasonKind};
    let kind = self.reason.as_ref().map(Reason::kind);
    let newer = matches!(kind, Some(ReasonKind::Unrecognized(_)));
    let fits = match self.state {
      HeaderState::Parsed | HeaderState::PartiallyParsedByDesign | HeaderState::Protected => kind.is_none(),
      HeaderState::Identified => kind.is_none() && self.identification.is_some(),
      HeaderState::NotFoundWithinBudget | HeaderState::PartiallyParsedForLackOfBytes => {
        newer
          || matches!(
            kind,
            Some(
              ReasonKind::TruncatedInput
                | ReasonKind::LimitExhausted
                | ReasonKind::RangeUnavailable
                | ReasonKind::SourceFailed
            )
          )
      }
      HeaderState::Malformed => newer || matches!(kind, Some(ReasonKind::DamagedStructure)),
      HeaderState::CompressionUnavailable => {
        let feature = self.reason.as_ref().and_then(Reason::feature);
        kind.is_none()
          || newer
          || matches!(kind, Some(ReasonKind::FeatureDisabled))
            && matches!(feature, Some(Feature::Zlib | Feature::Unrecognized(_)))
      }
      HeaderState::NotAttempted => kind.is_none() || newer || matches!(kind, Some(ReasonKind::SourceFailed)),
      HeaderState::Unrecognized(_) => return Ok(()),
    };
    let identified = matches!(self.state, HeaderState::Identified);
    if !fits || !(identified || self.identification.is_none()) {
      return Err(super::constants::HEADER_STATUS_MESSAGE);
    }
    if self
      .reason
      .as_ref()
      .is_none_or(|reason| reason.can_stop(&super::Part::Headers))
    {
      Ok(())
    } else {
      Err(crate::constants::SCOPED_LIMIT_MESSAGE)
    }
  }
}

impl fmt::Display for HeaderStatus {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(self.state.text())?;
    if let Some(identification) = &self.identification {
      write!(formatter, " ({})", identification.text())?;
    }
    match &self.reason {
      Some(reason) => write!(formatter, " ({})", reason.cause()),
      None => Ok(()),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::HeaderStatus;
  use crate::Limit;
  use crate::completeness::{HeaderState, Identification, Reason, ReasonKind};

  impl HeaderStatus {
    /// A header that was parsed in full.
    pub(crate) fn parsed() -> HeaderStatus {
      HeaderStatus {
        state: HeaderState::Parsed,
        identification: None,
        reason: None,
      }
    }

    /// A header in this state, with this reason.
    pub(crate) fn sample(state: HeaderState, reason: Option<Reason>) -> HeaderStatus {
      HeaderStatus {
        state,
        identification: None,
        reason,
      }
    }
  }

  #[test]
  fn tells_a_parsed_header_from_one_cut_short() {
    let parsed = HeaderStatus::parsed();
    assert!(parsed.is_parsed());
    assert!(parsed.is_final());
    assert_eq!(parsed.state(), &HeaderState::Parsed);
    assert_eq!(parsed.reason(), None);
    assert_eq!(parsed.to_string(), "parsed");
    let cut = HeaderStatus::sample(
      HeaderState::NotFoundWithinBudget,
      Some(Reason::exhausted(Limit::HeaderBytesPerStream)),
    );
    assert!(!cut.is_parsed());
    assert!(!cut.is_final());
    assert_eq!(cut.to_string(), "not found within the budget (per-stream header limit)");
    let truncated = HeaderStatus::sample(
      HeaderState::PartiallyParsedForLackOfBytes,
      Some(Reason::sample(ReasonKind::TruncatedInput)),
    );
    assert_eq!(
      truncated.to_string(),
      "partially parsed for lack of bytes (truncated input)"
    );
  }

  #[test]
  fn says_why_a_codec_was_only_identified() {
    let identified = HeaderStatus {
      state: HeaderState::Identified,
      identification: Some(Identification::PendingParser),
      reason: None,
    };
    assert!(identified.is_final());
    assert!(!identified.is_parsed());
    assert_eq!(identified.identification(), Some(&Identification::PendingParser));
    assert_eq!(identified.to_string(), "identified (parser pending)");
  }
}
