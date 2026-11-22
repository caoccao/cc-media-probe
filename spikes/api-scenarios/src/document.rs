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
use crate::constants::{ERROR_NAME, OK_NAME, PARTIAL_NAME, SCHEMA_MAJOR, SCHEMA_MINOR};
use crate::error::ErrorRecord;
use crate::model::{MediaReport, UnrecognizedValue};
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

/// One result per input, with a usage error where one occurred.
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Document {
  schema_version: SchemaVersion,
  generator: Generator,
  usage_error: Option<String>,
  results: Vec<DocumentEntry>,
}

impl Document {
  /// An empty document of the current schema version.
  pub fn new(generator: Generator) -> Document {
    Document {
      schema_version: SchemaVersion::CURRENT,
      generator,
      usage_error: None,
      results: Vec::new(),
    }
  }

  /// Adds a usage error.
  pub fn with_usage_error(mut self, message: impl Into<String>) -> Document {
    self.usage_error = Some(message.into());
    self
  }

  /// Adds an entry.
  pub fn push(&mut self, entry: DocumentEntry) {
    self.results.push(entry);
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
  pub fn usage_error(&self) -> Option<&str> {
    self.usage_error.as_deref()
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
  /// A complete report.
  Ok,
  /// An incomplete report.
  Partial,
  /// An error record.
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
pub struct DocumentEntry {
  index: usize,
  input: PortablePath,
  outcome: EntryOutcome,
  report: Option<MediaReport>,
  error: Option<ErrorRecord>,
}

impl DocumentEntry {
  /// An entry that holds a report; its outcome is `ok` or `partial`.
  pub fn from_report(index: usize, input: PortablePath, report: MediaReport) -> DocumentEntry {
    DocumentEntry {
      index,
      input,
      outcome: if report.completeness().is_complete() {
        EntryOutcome::Ok
      } else {
        EntryOutcome::Partial
      },
      report: Some(report),
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

  /// The report, for the outcomes `ok` and `partial`.
  pub fn report(&self) -> Option<&MediaReport> {
    self.report.as_ref()
  }

  /// The error record, for the outcome `error`.
  pub fn error(&self) -> Option<&ErrorRecord> {
    self.error.as_ref()
  }
}
