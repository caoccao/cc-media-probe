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

/// A diagnostic as written: each offset as a number or beyond 2^53 - 1 its twin (`json-output.md` sections 5.5 and
/// 7.12).
#[cfg(feature = "serde")]
#[derive(serde::Serialize, serde::Deserialize)]
struct DiagnosticForm {
  severity: Severity,
  code: DiagnosticCode,
  message: String,
  source_identifier: SourceIdentifier,
  #[serde(skip_serializing_if = "Option::is_none")]
  offset: Option<u64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  offset_exact: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  logical_offset: Option<u64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  logical_offset_exact: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  stream: Option<usize>,
}

#[cfg(feature = "serde")]
impl serde::Serialize for Diagnostic {
  fn serialize<Output: serde::Serializer>(&self, serializer: Output) -> Result<Output::Ok, Output::Error> {
    let (offset, offset_exact) = crate::form::unsigned_members(self.offset);
    let (logical_offset, logical_offset_exact) = crate::form::unsigned_members(self.logical_offset);
    let form = DiagnosticForm {
      severity: self.severity.clone(),
      code: self.code.clone(),
      message: self.message.clone(),
      source_identifier: self.source_identifier,
      offset,
      offset_exact,
      logical_offset,
      logical_offset_exact,
      stream: self.stream,
    };
    serde::Serialize::serialize(&form, serializer)
  }
}

#[cfg(feature = "serde")]
impl TryFrom<DiagnosticForm> for Diagnostic {
  type Error = &'static str;

  fn try_from(form: DiagnosticForm) -> Result<Diagnostic, &'static str> {
    Ok(Diagnostic {
      severity: form.severity,
      code: form.code,
      message: form.message,
      source_identifier: form.source_identifier,
      offset: crate::form::unsigned_value(form.offset, form.offset_exact)?,
      logical_offset: crate::form::unsigned_value(form.logical_offset, form.logical_offset_exact)?,
      stream: form.stream,
    })
  }
}

#[cfg(feature = "serde")]
crate::form::checked_form!(Diagnostic from DiagnosticForm);

#[cfg(test)]
mod tests {
  use super::Diagnostic;
  use crate::diagnostics::{DiagnosticCode, Severity};
  use crate::source::SourceIdentifier;

  impl Diagnostic {
    /// A warning about the stream at `stream`, or about the file without one.
    pub(crate) fn sample(stream: Option<usize>) -> Diagnostic {
      Diagnostic {
        severity: Severity::Warning,
        code: DiagnosticCode::sample("matroska.seek_head_disagrees"),
        message: "the seek head disagrees with the segment".to_owned(),
        source_identifier: SourceIdentifier::sample(0),
        offset: Some(4_096),
        logical_offset: None,
        stream,
      }
    }
  }

  #[test]
  fn gives_each_field() {
    let diagnostic = Diagnostic::sample(Some(1));
    assert_eq!(diagnostic.severity(), &Severity::Warning);
    assert_eq!(diagnostic.code().as_str(), "matroska.seek_head_disagrees");
    assert_eq!(diagnostic.message(), "the seek head disagrees with the segment");
    assert_eq!(diagnostic.source_identifier(), SourceIdentifier::sample(0));
    assert_eq!(diagnostic.offset(), Some(4_096));
    assert_eq!(diagnostic.logical_offset(), None);
    assert_eq!(diagnostic.stream(), Some(1));
  }
}
