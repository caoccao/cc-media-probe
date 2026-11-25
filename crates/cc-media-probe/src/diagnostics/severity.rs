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

//! `Severity`, how much a diagnostic matters (`api.md` section 6.13).

use super::constants::{ERROR_NAME, INFORMATION_NAME, WARNING_NAME};
use crate::string_enumeration::string_enumeration;

string_enumeration! {
  /// How much a diagnostic matters.
  ///
  /// ```
  /// use cc_media_probe::MediaReport;
  /// use cc_media_probe::diagnostics::Severity;
  ///
  /// fn damaged_structures(report: &MediaReport) -> usize {
  ///     report.diagnostics().iter().filter(|diagnostic| matches!(diagnostic.severity(), Severity::Error)).count()
  /// }
  ///
  /// assert_eq!(Severity::Warning.as_str(), "warning");
  /// ```
  Severity {
    /// A structure that was damaged and skipped. The probe still returned a report.
    Error => ERROR_NAME,
    /// Something that may make a value less certain, such as a conflict between sources.
    Warning => WARNING_NAME,
    /// Something worth knowing that changes no value.
    Information => INFORMATION_NAME,
  }
}

#[cfg(test)]
mod tests {
  use super::Severity;
  use crate::model::UnrecognizedValue;

  #[test]
  fn names_each_severity() {
    assert_eq!(Severity::Error.as_str(), "error");
    assert_eq!(Severity::Warning.as_str(), "warning");
    assert_eq!(Severity::Information.as_str(), "information");
    assert_eq!(
      Severity::Unrecognized(UnrecognizedValue::sample("fatal")).as_str(),
      "fatal"
    );
  }
}
