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

//! The live error, `api.md` section 10.1, and its record, section 10.2.

use std::fmt;
use std::io;

use crate::constants::{
  FEATURE_DISABLED_MESSAGE, FEATURE_DISABLED_NAME, FORMAT_NOT_DETERMINED_MESSAGE, FORMAT_NOT_DETERMINED_NAME,
  NO_STREAM_INVENTORY_MESSAGE, NO_STREAM_INVENTORY_NAME, SOURCE_FAILED_MESSAGE, SOURCE_FAILED_NAME,
  UNKNOWN_FORMAT_MESSAGE, UNKNOWN_FORMAT_NAME, UNSUPPORTED_FORMAT_MESSAGE, UNSUPPORTED_FORMAT_NAME,
};
use crate::details::FormatDetails;
use crate::diagnostics::Diagnostic;
use crate::model::{Format, MediaReport, UnrecognizedValue, Usage};

/// Why a probe returned no report, `api.md` section 5.1.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum ErrorKind {
  /// A source failed.
  SourceFailed,
  /// Nothing matched, a hint only, or every candidate rejected.
  UnknownFormat,
  /// Too few bytes, or no candidate validated while at least one could not be checked.
  FormatNotDetermined,
  /// A detect-only format.
  UnsupportedFormat,
  /// All stream information lies behind a disabled feature.
  FeatureDisabled,
  /// The probe stopped before the stream inventory.
  NoStreamInventory,
  /// A value that a newer version wrote.
  Unrecognized(UnrecognizedValue),
}

impl ErrorKind {
  /// The serialized name, or the text of an unrecognized value.
  pub fn as_str(&self) -> &str {
    match self {
      ErrorKind::SourceFailed => SOURCE_FAILED_NAME,
      ErrorKind::UnknownFormat => UNKNOWN_FORMAT_NAME,
      ErrorKind::FormatNotDetermined => FORMAT_NOT_DETERMINED_NAME,
      ErrorKind::UnsupportedFormat => UNSUPPORTED_FORMAT_NAME,
      ErrorKind::FeatureDisabled => FEATURE_DISABLED_NAME,
      ErrorKind::NoStreamInventory => NO_STREAM_INVENTORY_NAME,
      ErrorKind::Unrecognized(value) => value.as_str(),
    }
  }

  /// The message for people.
  fn message(&self) -> &str {
    match self {
      ErrorKind::SourceFailed => SOURCE_FAILED_MESSAGE,
      ErrorKind::UnknownFormat => UNKNOWN_FORMAT_MESSAGE,
      ErrorKind::FormatNotDetermined => FORMAT_NOT_DETERMINED_MESSAGE,
      ErrorKind::UnsupportedFormat => UNSUPPORTED_FORMAT_MESSAGE,
      ErrorKind::FeatureDisabled => FEATURE_DISABLED_MESSAGE,
      ErrorKind::NoStreamInventory => NO_STREAM_INVENTORY_MESSAGE,
      ErrorKind::Unrecognized(value) => value.as_str(),
    }
  }
}

/// What a probe saw before it stopped short of a stream inventory.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Observations {
  format_details: Option<FormatDetails>,
  diagnostics: Vec<Diagnostic>,
}

impl Observations {
  /// The format's details, such as the track identifiers of an ISO media segment.
  pub fn format_details(&self) -> Option<&FormatDetails> {
    self.format_details.as_ref()
  }

  /// The diagnostics so far.
  pub fn diagnostics(&self) -> &[Diagnostic] {
    &self.diagnostics
  }
}

/// What `probe_*` returns instead of a report.
#[derive(Debug)]
pub struct Error {
  data: Box<ErrorData>,
}

/// The error's data, behind one pointer.
#[derive(Debug)]
struct ErrorData {
  kind: ErrorKind,
  format: Option<Format>,
  source_failure: Option<io::Error>,
  partial_report: Option<MediaReport>,
  observations: Option<Observations>,
  usage: Usage,
}

impl Error {
  /// An error of `kind` without a cause.
  pub(crate) fn new(kind: ErrorKind) -> Error {
    Error {
      data: Box::new(ErrorData {
        kind,
        format: None,
        source_failure: None,
        partial_report: None,
        observations: None,
        usage: Usage::default(),
      }),
    }
  }

  /// The error of kind `SourceFailed`, which keeps the I/O error as its cause.
  pub(crate) fn from_source_failure(source_failure: io::Error) -> Error {
    let mut error = Error::new(ErrorKind::SourceFailed);
    error.data.source_failure = Some(source_failure);
    error
  }

  /// The kind.
  pub fn kind(&self) -> &ErrorKind {
    &self.data.kind
  }

  /// The detected format or family.
  pub fn format(&self) -> Option<&Format> {
    self.data.format.as_ref()
  }

  /// The I/O error of a failed source.
  pub fn source_failure(&self) -> Option<&io::Error> {
    self.data.source_failure.as_ref()
  }

  /// The report so far, when the inventory was reached before a source failed.
  pub fn partial_report(&self) -> Option<&MediaReport> {
    self.data.partial_report.as_ref()
  }

  /// The report so far, by value.
  pub fn into_partial_report(self) -> Option<MediaReport> {
    self.data.partial_report
  }

  /// What was seen before an inventory.
  pub fn observations(&self) -> Option<&Observations> {
    self.data.observations.as_ref()
  }

  /// What the budget accounted for.
  pub fn usage(&self) -> &Usage {
    &self.data.usage
  }

  /// The serializable record.
  pub fn to_record(&self) -> ErrorRecord {
    ErrorRecord::from(self)
  }
}

impl fmt::Display for Error {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    match &self.data.source_failure {
      Some(cause) => write!(formatter, "{}: {cause}", self.data.kind.message()),
      None => formatter.write_str(self.data.kind.message()),
    }
  }
}

impl std::error::Error for Error {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    self
      .data
      .source_failure
      .as_ref()
      .map(|cause| cause as &(dyn std::error::Error + 'static))
  }
}

/// The serializable description of an error.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ErrorRecord {
  kind: ErrorKind,
  message: String,
  source_failure_kind: Option<String>,
  partial_report: Option<MediaReport>,
}

impl ErrorRecord {
  /// The kind.
  pub fn kind(&self) -> &ErrorKind {
    &self.kind
  }

  /// The message.
  pub fn message(&self) -> &str {
    &self.message
  }

  /// The I/O error kind as a stable string; the stub writes its debug form.
  pub fn source_failure_kind(&self) -> Option<&str> {
    self.source_failure_kind.as_deref()
  }

  /// The report so far.
  pub fn partial_report(&self) -> Option<&MediaReport> {
    self.partial_report.as_ref()
  }
}

impl From<&Error> for ErrorRecord {
  fn from(error: &Error) -> ErrorRecord {
    ErrorRecord {
      kind: error.data.kind.clone(),
      message: error.to_string(),
      source_failure_kind: error
        .data
        .source_failure
        .as_ref()
        .map(|cause| format!("{:?}", cause.kind())),
      partial_report: error.data.partial_report.clone(),
    }
  }
}

impl From<Error> for ErrorRecord {
  fn from(error: Error) -> ErrorRecord {
    ErrorRecord::from(&error)
  }
}
