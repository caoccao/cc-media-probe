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

//! `Observations`, what a probe saw before its stream inventory (`api.md` section 10.1).

use crate::details::FormatDetails;
use crate::diagnostics::Diagnostic;
use crate::source::SourceEntry;

/// What a probe saw before it stopped short of its stream inventory, such as the track identifiers and raw ticks
/// of an ISO media segment, which are in its format details (R20).
///
/// ```
/// use cc_media_probe::Error;
///
/// fn observed(error: &Error) -> Option<String> {
///     let observations = error.observations()?;
///     Some(format!(
///         "details {}, {} diagnostics, {} sources",
///         observations.format_details().is_some(),
///         observations.diagnostics().len(),
///         observations.sources().len(),
///     ))
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Observations {
  format_details: Option<FormatDetails>,
  diagnostics: Vec<Diagnostic>,
  sources: Vec<SourceEntry>,
}

impl Observations {
  /// The format's details as far as they were read.
  #[must_use]
  pub fn format_details(&self) -> Option<&FormatDetails> {
    self.format_details.as_ref()
  }

  /// The diagnostics so far.
  #[must_use]
  pub fn diagnostics(&self) -> &[Diagnostic] {
    &self.diagnostics
  }

  /// The sources so far.
  #[must_use]
  pub fn sources(&self) -> &[SourceEntry] {
    &self.sources
  }
}

#[cfg(test)]
mod tests {
  use super::Observations;
  use crate::details::{FormatDetails, UnrecognizedDetails};
  use crate::diagnostics::Diagnostic;
  use crate::source::SourceEntry;

  impl Observations {
    /// What a probe saw of an ISO media segment.
    pub(crate) fn sample() -> Observations {
      Observations {
        format_details: Some(FormatDetails::Unrecognized(UnrecognizedDetails::sample(
          "iso_base_media",
        ))),
        diagnostics: vec![Diagnostic::sample(0)],
        sources: vec![SourceEntry::sample()],
      }
    }
  }

  #[test]
  fn gives_what_was_seen() {
    let observations = Observations::sample();
    assert!(observations.format_details().is_some());
    assert_eq!(observations.diagnostics().len(), 1);
    assert_eq!(observations.sources().len(), 1);
  }
}
