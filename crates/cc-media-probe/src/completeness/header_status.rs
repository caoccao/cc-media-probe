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
  identification: Option<Identification>,
  reason: Option<Reason>,
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
