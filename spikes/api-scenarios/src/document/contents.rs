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

use super::entry::DocumentEntry;
use super::invalid::InvalidDocument;
use super::version::SchemaVersion;
use super::wire::{EntryData, serialize_results};
use crate::constants::{NESTED_VERSION_MESSAGE, OTHER_MAJOR_VERSION_MESSAGE, USAGE_ERROR_WITH_RESULTS_MESSAGE};

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
  #[serde(skip_serializing_if = "Option::is_none")]
  usage_error: Option<UsageError>,
  #[serde(serialize_with = "serialize_results")]
  results: Vec<DocumentEntry>,
}

/// A document as read, before its rules are checked.
#[derive(serde::Deserialize)]
struct DocumentData {
  #[serde(deserialize_with = "super::version::read_current_major")]
  schema_version: SchemaVersion,
  generator: Generator,
  usage_error: Option<UsageError>,
  results: Vec<EntryData>,
}

impl TryFrom<DocumentData> for Document {
  type Error = InvalidDocument;

  /// A document of this major version, every nested record of which carries its version, whose usage error never
  /// stands beside results, and whose entries each have their position as their index. The library checks the
  /// version before it decodes anything else; the stub checks it after.
  fn try_from(data: DocumentData) -> Result<Document, InvalidDocument> {
    if data.schema_version.major() != SchemaVersion::CURRENT.major() {
      return Err(InvalidDocument::new(OTHER_MAJOR_VERSION_MESSAGE));
    }
    if data.usage_error.is_some() && !data.results.is_empty() {
      return Err(InvalidDocument::new(USAGE_ERROR_WITH_RESULTS_MESSAGE));
    }
    let results = data
      .results
      .into_iter()
      .enumerate()
      .map(|(position, entry)| entry.into_entry(position))
      .collect::<Result<Vec<DocumentEntry>, InvalidDocument>>()?;
    let mut nested = results.iter().flat_map(DocumentEntry::nested_versions);
    if nested.any(|version| version != data.schema_version) {
      return Err(InvalidDocument::new(NESTED_VERSION_MESSAGE));
    }
    Ok(Document {
      schema_version: data.schema_version,
      generator: data.generator,
      usage_error: data.usage_error,
      results,
    })
  }
}

impl Document {
  /// A document of the current schema version with these results; with none, the document of `--json --version`.
  /// Each nested record takes the document's version, whatever version it was read with.
  pub fn from_results(generator: Generator, results: Vec<DocumentEntry>) -> Document {
    Document {
      schema_version: SchemaVersion::CURRENT,
      generator,
      usage_error: None,
      results: results
        .into_iter()
        .map(|entry| entry.with_schema_version(SchemaVersion::CURRENT))
        .collect(),
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

  /// The entries, in order; each entry's position is its index.
  pub fn results(&self) -> &[DocumentEntry] {
    &self.results
  }
}
