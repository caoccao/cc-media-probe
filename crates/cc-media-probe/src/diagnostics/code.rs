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

//! `DiagnosticCode`, the stable name of a diagnostic (`api.md` section 6.13).

use std::fmt;

/// The stable name of a diagnostic, an open set of dotted names such as `matroska.seek_head_disagrees`. A code
/// keeps its meaning across releases; a message may change at any time. `Display` writes the code.
///
/// ```
/// use cc_media_probe::MediaReport;
///
/// fn has_diagnostic(report: &MediaReport, code: &str) -> bool {
///     report.diagnostics().iter().any(|diagnostic| diagnostic.code().as_str() == code)
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct DiagnosticCode(String);

#[cfg(feature = "serde")]
crate::form::text_form!(DiagnosticCode);

impl DiagnosticCode {
  /// The code as text.
  #[must_use]
  pub fn as_str(&self) -> &str {
    &self.0
  }
}

impl fmt::Display for DiagnosticCode {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.0)
  }
}

#[cfg(test)]
mod tests {
  use super::DiagnosticCode;

  impl DiagnosticCode {
    /// The code `code`.
    pub(crate) fn sample(code: &str) -> DiagnosticCode {
      DiagnosticCode(code.to_owned())
    }
  }

  #[test]
  fn keeps_its_text() {
    let code = DiagnosticCode::sample("matroska.seek_head_disagrees");
    assert_eq!(code.as_str(), "matroska.seek_head_disagrees");
    assert_eq!(code.to_string(), "matroska.seek_head_disagrees");
  }
}
