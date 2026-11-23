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

/// A problem found in the input that did not stop the probe.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Diagnostic {
  severity: Severity,
  code: DiagnosticCode,
  message: String,
  source_identifier: SourceIdentifier,
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
  offset: Option<u64>,
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
  stream: Option<usize>,
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
