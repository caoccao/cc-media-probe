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

use crate::constants::{SCHEMA_MAJOR, SCHEMA_MINOR};
use crate::error::ErrorRecord;
use crate::model::MediaReport;
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

  /// The entries, in order.
  pub fn results(&self) -> &[DocumentEntry] {
    &self.results
  }
}

/// The result for one input.
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct DocumentEntry {
  index: usize,
  input: PortablePath,
  report: Option<MediaReport>,
  error: Option<ErrorRecord>,
}

impl DocumentEntry {
  /// An entry that holds a report; its outcome is `ok` or `partial`.
  pub fn report(index: usize, input: PortablePath, report: MediaReport) -> DocumentEntry {
    DocumentEntry {
      index,
      input,
      report: Some(report),
      error: None,
    }
  }

  /// An entry that holds an error record; its outcome is `error`.
  pub fn error(index: usize, input: PortablePath, record: ErrorRecord) -> DocumentEntry {
    DocumentEntry {
      index,
      input,
      report: None,
      error: Some(record),
    }
  }
}
