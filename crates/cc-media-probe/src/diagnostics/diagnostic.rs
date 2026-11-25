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

//! `Diagnostic`, one thing that a probe noticed (`api.md` section 6.13).

use super::{DiagnosticCode, Severity};
use crate::source::SourceIdentifier;

/// One thing that a probe noticed on the way, from `MediaReport::diagnostics()`, kept within the diagnostics
/// limit; the record counts the others.
///
/// ```
/// use cc_media_probe::MediaReport;
///
/// fn warnings(report: &MediaReport) {
///     for diagnostic in report.diagnostics() {
///         println!(
///             "{} {}: {} (source {}, byte {:?}, logical byte {:?}, stream {:?})",
///             diagnostic.severity().as_str(),
///             diagnostic.code(),
///             diagnostic.message(),
///             diagnostic.source_identifier(),
///             diagnostic.offset(),
///             diagnostic.logical_offset(),
///             diagnostic.stream(),
///         );
///     }
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Diagnostic {
  severity: Severity,
  code: DiagnosticCode,
  message: String,
  source_identifier: SourceIdentifier,
  offset: Option<u64>,
  logical_offset: Option<u64>,
  stream: Option<usize>,
}

impl Diagnostic {
  /// How much the diagnostic matters.
  #[must_use]
  pub fn severity(&self) -> &Severity {
    &self.severity
  }

  /// The stable name of the diagnostic.
  #[must_use]
  pub fn code(&self) -> &DiagnosticCode {
    &self.code
  }

  /// The message for people, which may change in any release. It begins in lowercase unless it begins with a
  /// proper name, and ends without punctuation.
  #[must_use]
  pub fn message(&self) -> &str {
    &self.message
  }

  /// The source that the diagnostic concerns.
  #[must_use]
  pub fn source_identifier(&self) -> SourceIdentifier {
    self.source_identifier
  }

  /// The byte offset, relative to that source's own start.
  #[must_use]
  pub fn offset(&self) -> Option<u64> {
    self.offset
  }

  /// The offset in the stream that several files form, where they form one.
  #[must_use]
  pub fn logical_offset(&self) -> Option<u64> {
    self.logical_offset
  }

  /// The stream that the diagnostic concerns.
  #[must_use]
  pub fn stream(&self) -> Option<usize> {
    self.stream
  }
}

#[cfg(test)]
mod tests {
  use super::Diagnostic;
  use crate::diagnostics::{DiagnosticCode, Severity};
  use crate::source::SourceIdentifier;

  impl Diagnostic {
    /// A warning about the stream at `stream`.
    pub(crate) fn sample(stream: usize) -> Diagnostic {
      Diagnostic {
        severity: Severity::Warning,
        code: DiagnosticCode::sample("matroska.seek_head_disagrees"),
        message: "the seek head disagrees with the segment".to_owned(),
        source_identifier: SourceIdentifier::sample(0),
        offset: Some(4_096),
        logical_offset: None,
        stream: Some(stream),
      }
    }
  }

  #[test]
  fn gives_each_field() {
    let diagnostic = Diagnostic::sample(1);
    assert_eq!(diagnostic.severity(), &Severity::Warning);
    assert_eq!(diagnostic.code().as_str(), "matroska.seek_head_disagrees");
    assert_eq!(diagnostic.message(), "the seek head disagrees with the segment");
    assert_eq!(diagnostic.source_identifier(), SourceIdentifier::sample(0));
    assert_eq!(diagnostic.offset(), Some(4_096));
    assert_eq!(diagnostic.logical_offset(), None);
    assert_eq!(diagnostic.stream(), Some(1));
  }
}
