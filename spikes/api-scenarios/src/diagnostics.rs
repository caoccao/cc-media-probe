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

//! Diagnostics, `api.md` section 6.13.

use std::fmt;

use crate::model::UnrecognizedValue;
use crate::source::SourceIdentifier;

/// A problem found in the input that did not stop the probe. Its offset is a quantity that the record cannot do
/// without, so beyond 2^53 - 1 it is its twin `offset_exact` (`json-output.md` section 5.5; fourteenth review).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(into = "DiagnosticData"))]
pub struct Diagnostic {
  severity: Severity,
  code: DiagnosticCode,
  message: String,
  source_identifier: SourceIdentifier,
  offset: Option<u64>,
  stream: Option<usize>,
}

#[cfg(feature = "serde")]
crate::neutral::checked_record_traits!(Diagnostic from DiagnosticData);

/// A diagnostic's members as written, its offset as its number or its twin.
#[cfg(feature = "serde")]
#[derive(serde::Serialize, serde::Deserialize)]
struct DiagnosticData {
  severity: Severity,
  code: DiagnosticCode,
  message: String,
  source_identifier: SourceIdentifier,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  offset: Option<u64>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  offset_exact: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  stream: Option<usize>,
}

#[cfg(feature = "serde")]
impl TryFrom<DiagnosticData> for Diagnostic {
  type Error = crate::document::InvalidDocument;

  /// The offset from at most one of its number and its twin, each in its own range.
  fn try_from(read: DiagnosticData) -> Result<Diagnostic, crate::document::InvalidDocument> {
    Ok(Diagnostic {
      severity: read.severity,
      code: read.code,
      message: read.message,
      source_identifier: read.source_identifier,
      offset: crate::twin::unsigned_value(read.offset, read.offset_exact)?,
      stream: read.stream,
    })
  }
}

#[cfg(feature = "serde")]
impl From<Diagnostic> for DiagnosticData {
  fn from(diagnostic: Diagnostic) -> DiagnosticData {
    let (offset, offset_exact) = crate::twin::unsigned_members(diagnostic.offset);
    DiagnosticData {
      severity: diagnostic.severity,
      code: diagnostic.code,
      message: diagnostic.message,
      source_identifier: diagnostic.source_identifier,
      offset,
      offset_exact,
      stream: diagnostic.stream,
    }
  }
}

impl Diagnostic {
  /// How serious it is.
  pub fn severity(&self) -> &Severity {
    &self.severity
  }

  /// The stable code.
  pub fn code(&self) -> &DiagnosticCode {
    &self.code
  }

  /// The message for people; it may change at any time.
  pub fn message(&self) -> &str {
    &self.message
  }

  /// The source the problem is in.
  pub fn source_identifier(&self) -> SourceIdentifier {
    self.source_identifier
  }

  /// The offset in that source.
  pub fn offset(&self) -> Option<u64> {
    self.offset
  }

  /// The stream concerned.
  pub fn stream(&self) -> Option<usize> {
    self.stream
  }
}

/// How serious a diagnostic is.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum Severity {
  /// A damaged structure was skipped.
  Error,
  /// Something unexpected.
  Warning,
  /// For information.
  Information,
  /// A value that a newer version wrote.
  #[cfg_attr(feature = "serde", serde(with = "crate::model::unrecognized_text"))]
  Unrecognized(UnrecognizedValue<Severity>),
}

/// A stable code, such as `matroska.seek_head_disagrees`.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DiagnosticCode(String);

impl DiagnosticCode {
  /// The code.
  pub fn as_str(&self) -> &str {
    &self.0
  }
}

impl fmt::Display for DiagnosticCode {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.0)
  }
}
