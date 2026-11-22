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

//! The document of `api.md` section 12, which the CLI writes and a library user can write too.

use std::fmt;

use crate::constants::{
  ENTRY_WITHOUT_ONE_RECORD_MESSAGE, ERROR_NAME, OK_NAME, PARTIAL_NAME, SCHEMA_MAJOR, SCHEMA_MINOR,
  USAGE_ERROR_WITH_RESULTS_MESSAGE,
};
use crate::error::ErrorRecord;
use crate::model::{Detection, DetectionOutcome, MediaReport, UnrecognizedValue};
use crate::source::PortablePath;

/// A schema version, `major.minor`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct SchemaVersion {
  major: u32,
  minor: u32,
}

impl SchemaVersion {
  /// The version this release writes.
  pub const CURRENT: SchemaVersion = SchemaVersion {
    major: SCHEMA_MAJOR,
    minor: SCHEMA_MINOR,
  };

  /// The major version.
  pub fn major(&self) -> u32 {
    self.major
  }

  /// The minor version.
  pub fn minor(&self) -> u32 {
    self.minor
  }
}

/// The program that wrote a document.
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Generator {
  name: String,
  version: String,
}

impl Generator {
  /// A generator of that name and version.
  pub fn new(name: impl Into<String>, version: impl Into<String>) -> Generator {
    Generator {
      name: name.into(),
      version: version.into(),
    }
  }

  /// The program's name.
  pub fn name(&self) -> &str {
    &self.name
  }

  /// The program's version.
  pub fn version(&self) -> &str {
    &self.version
  }
}

/// A command line that could not be run, as the `usage_error` object of plan section 8.9.
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct UsageError {
  message: String,
}

impl UsageError {
  /// A usage error with that message.
  pub fn new(message: impl Into<String>) -> UsageError {
    UsageError {
      message: message.into(),
    }
  }

  /// The message.
  pub fn message(&self) -> &str {
    &self.message
  }
}

/// One result per input, or a usage error and no results.
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(try_from = "DocumentData")]
pub struct Document {
  schema_version: SchemaVersion,
  generator: Generator,
  usage_error: Option<UsageError>,
  results: Vec<DocumentEntry>,
}

/// A document as read, before its invariant is checked.
#[derive(serde::Deserialize)]
struct DocumentData {
  schema_version: SchemaVersion,
  generator: Generator,
  usage_error: Option<UsageError>,
  results: Vec<DocumentEntry>,
}

impl TryFrom<DocumentData> for Document {
  type Error = InvalidDocument;

  /// A usage error never stands beside results.
  fn try_from(data: DocumentData) -> Result<Document, InvalidDocument> {
    if data.usage_error.is_some() && !data.results.is_empty() {
      return Err(InvalidDocument(USAGE_ERROR_WITH_RESULTS_MESSAGE));
    }
    Ok(Document {
      schema_version: data.schema_version,
      generator: data.generator,
      usage_error: data.usage_error,
      results: data.results,
    })
  }
}

impl Document {
  /// A document of the current schema version with these results; with none, the document of `--json --version`.
  pub fn from_results(generator: Generator, results: Vec<DocumentEntry>) -> Document {
    Document {
      schema_version: SchemaVersion::CURRENT,
      generator,
      usage_error: None,
      results,
    }
  }

  /// A document of the current schema version that holds a usage error and no results.
  pub fn from_usage_error(generator: Generator, error: UsageError) -> Document {
    Document {
      schema_version: SchemaVersion::CURRENT,
      generator,
      usage_error: Some(error),
      results: Vec::new(),
    }
  }

  /// The schema version, which every nested root record carries too.
  pub fn schema_version(&self) -> SchemaVersion {
    self.schema_version
  }

  /// The program that wrote the document.
  pub fn generator(&self) -> &Generator {
    &self.generator
  }

  /// The usage error, where one occurred.
  pub fn usage_error(&self) -> Option<&UsageError> {
    self.usage_error.as_ref()
  }

  /// The entries, in order.
  pub fn results(&self) -> &[DocumentEntry] {
    &self.results
  }
}

/// What an entry holds.
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum EntryOutcome {
  /// A complete report, or an identified format.
  Ok,
  /// An incomplete report, or a detection of a family only, an ambiguous result or a hint only.
  Partial,
  /// An error record, or a detection of insufficient data or an unknown format.
  Error,
  /// A value that a newer version wrote.
  Unrecognized(UnrecognizedValue),
}

impl EntryOutcome {
  /// The serialized name, or the text of an unrecognized value.
  pub fn as_str(&self) -> &str {
    match self {
      EntryOutcome::Ok => OK_NAME,
      EntryOutcome::Partial => PARTIAL_NAME,
      EntryOutcome::Error => ERROR_NAME,
      EntryOutcome::Unrecognized(value) => value.as_str(),
    }
  }
}

/// The result for one input.
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(try_from = "EntryData")]
pub struct DocumentEntry {
  index: usize,
  input: PortablePath,
  outcome: EntryOutcome,
  report: Option<MediaReport>,
  detection: Option<Detection>,
  error: Option<ErrorRecord>,
}

/// An entry as read, before its invariant is checked.
#[derive(serde::Deserialize)]
struct EntryData {
  index: usize,
  input: PortablePath,
  outcome: EntryOutcome,
  report: Option<MediaReport>,
  detection: Option<Detection>,
  error: Option<ErrorRecord>,
}

impl TryFrom<EntryData> for DocumentEntry {
  type Error = InvalidDocument;

  /// An entry of an outcome this version knows holds exactly one record.
  fn try_from(data: EntryData) -> Result<DocumentEntry, InvalidDocument> {
    let records = [data.report.is_some(), data.detection.is_some(), data.error.is_some()];
    let known = !matches!(data.outcome, EntryOutcome::Unrecognized(_));
    if known && records.into_iter().filter(|present| *present).count() != 1 {
      return Err(InvalidDocument(ENTRY_WITHOUT_ONE_RECORD_MESSAGE));
    }
    Ok(DocumentEntry {
      index: data.index,
      input: data.input,
      outcome: data.outcome,
      report: data.report,
      detection: data.detection,
      error: data.error,
    })
  }
}

impl DocumentEntry {
  /// An entry that holds a report; its outcome is `ok` or `partial`.
  pub fn from_report(index: usize, input: PortablePath, report: MediaReport) -> DocumentEntry {
    let outcome = if report.completeness().is_complete() {
      EntryOutcome::Ok
    } else {
      EntryOutcome::Partial
    };
    DocumentEntry {
      index,
      input,
      outcome,
      report: Some(report),
      detection: None,
      error: None,
    }
  }

  /// An entry that holds a detection; its outcome follows the `--detect` rows of plan section 8.9.
  pub fn from_detection(index: usize, input: PortablePath, detection: Detection) -> DocumentEntry {
    let outcome = match detection.outcome() {
      DetectionOutcome::Identified => EntryOutcome::Ok,
      DetectionOutcome::FamilyOnly | DetectionOutcome::Ambiguous | DetectionOutcome::HintOnly => EntryOutcome::Partial,
      DetectionOutcome::InsufficientData | DetectionOutcome::Unknown | DetectionOutcome::Unrecognized(_) => {
        EntryOutcome::Error
      }
    };
    DocumentEntry {
      index,
      input,
      outcome,
      report: None,
      detection: Some(detection),
      error: None,
    }
  }

  /// An entry that holds an error record; its outcome is `error`.
  pub fn from_error(index: usize, input: PortablePath, record: ErrorRecord) -> DocumentEntry {
    DocumentEntry {
      index,
      input,
      outcome: EntryOutcome::Error,
      report: None,
      detection: None,
      error: Some(record),
    }
  }

  /// The input's position.
  pub fn index(&self) -> usize {
    self.index
  }

  /// The input.
  pub fn input(&self) -> &PortablePath {
    &self.input
  }

  /// What the entry holds.
  pub fn outcome(&self) -> &EntryOutcome {
    &self.outcome
  }

  /// The report, for a probe.
  pub fn report(&self) -> Option<&MediaReport> {
    self.report.as_ref()
  }

  /// The detection, for `--detect`.
  pub fn detection(&self) -> Option<&Detection> {
    self.detection.as_ref()
  }

  /// The error record.
  pub fn error(&self) -> Option<&ErrorRecord> {
    self.error.as_ref()
  }
}

/// Why a document that was read breaks the rules of `api.md` section 12.
#[derive(Debug)]
pub struct InvalidDocument(&'static str);

impl fmt::Display for InvalidDocument {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(self.0)
  }
}
