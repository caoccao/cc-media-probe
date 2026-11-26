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
use crate::form_record::form_record;
use crate::source::SourceEntry;

form_record! {
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
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    format_details: Option<FormatDetails>,
    diagnostics: Vec<Diagnostic>,
    sources: Vec<SourceEntry>,
  }
  check = Observations::check_form;
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

  /// Whether the observations hold a source table of their own, which names each source once and holds the primary
  /// input, and diagnostics that name its sources and no stream, since no stream is known before an inventory, as
  /// the maintainer chose at the review of the third slice's first part (`json-output.md` section 10).
  #[cfg(feature = "serde")]
  fn check_form(&self) -> Result<(), &'static str> {
    let sources = crate::source::table_identifiers(&self.sources)?;
    if self.diagnostics.iter().any(|diagnostic| diagnostic.stream().is_some()) {
      return Err(super::constants::OBSERVED_STREAM_MESSAGE);
    }
    let named = self
      .diagnostics
      .iter()
      .all(|diagnostic| sources.contains(&diagnostic.source_identifier()));
    if named {
      Ok(())
    } else {
      Err(crate::source::UNKNOWN_SOURCE_MESSAGE)
    }
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
        diagnostics: vec![Diagnostic::sample(None)],
        sources: vec![SourceEntry::sample()],
      }
    }

    /// What a probe saw before its requested companion, source 1, failed: the source table alone.
    pub(crate) fn of_a_failed_companion() -> Observations {
      Observations {
        format_details: None,
        diagnostics: Vec::new(),
        sources: vec![SourceEntry::sample(), SourceEntry::failed_companion()],
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
