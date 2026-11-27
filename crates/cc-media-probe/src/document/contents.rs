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

//! `Document`, the output of `--json` that the command-line contract specifies, which a library user can write too
//! (`api.md` section 12; `json-output.md` section 3; plan section 8.9).

use super::{DocumentEntry, Generator, SchemaVersion, UsageError};

/// One result for each input, or a usage error and no results. Its two constructors keep it so, and it has no method
/// that adds an entry later. A document gives every record that it holds its own version, whatever version the
/// record was read with, down to the partial report of an error record, and reading checks every nested record
/// against the document's version before it decodes the record's body. Each entry's position in `results` is its
/// index. To write a document of results one entry at a time, see [`Document::serialize_results`].
///
/// ```
/// use std::path::Path;
/// use cc_media_probe::document::{Document, DocumentEntry, EntryOutcome, Generator, SchemaVersion};
/// use cc_media_probe::source::PortablePath;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// // One entry for each input, whichever way its probe ends, so each entry's index is its input's place.
/// let mut entries = Vec::new();
/// for (name, bytes) in [("empty.bin", &b""[..]), ("notes.txt", &b"not a media file"[..])] {
///     let input = PortablePath::from_path(Path::new(name));
///     entries.push(match cc_media_probe::probe_bytes(bytes) {
///         Ok(report) => DocumentEntry::from_report(input, report)?,
///         Err(error) => DocumentEntry::from_error(input, error.to_record()),
///     });
/// }
/// let document = Document::from_results(Generator::new("media-catalog", "1.2.3"), entries);
/// let text = serde_json::to_string_pretty(&document)?;
/// let read: Document = serde_json::from_str(&text)?;
/// assert_eq!(read, document);
/// assert_eq!(read.schema_version(), SchemaVersion::CURRENT);
/// assert_eq!(read.generator().name(), "media-catalog");
/// let names: Vec<&str> = read.results().iter().map(|entry| entry.input().display()).collect();
/// assert_eq!(names, ["empty.bin", "notes.txt"]);
/// // An empty input is never media.
/// assert_eq!(read.results()[0].outcome(), &EntryOutcome::Error);
/// # Ok(())
/// # }
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Deserialize)]
#[serde(try_from = "crate::form::Rooted<super::document_form::DocumentBody>")]
pub struct Document {
  pub(super) schema_version: SchemaVersion,
  pub(super) generator: Generator,
  pub(super) usage_error: Option<UsageError>,
  pub(super) results: Vec<DocumentEntry>,
}

impl Document {
  /// A document of the current version with these results, which gives each record that they hold its version in
  /// place. With no entries, it is the document that the command-line contract specifies for `--json --version`. The
  /// example of [`Document`] builds one.
  #[must_use]
  pub fn from_results(generator: Generator, mut results: Vec<DocumentEntry>) -> Document {
    for entry in &mut results {
      entry.set_schema_version(SchemaVersion::CURRENT);
    }
    Document {
      schema_version: SchemaVersion::CURRENT,
      generator,
      usage_error: None,
      results,
    }
  }

  /// A document of the current version that holds a usage error, whose `results` are empty.
  ///
  /// ```
  /// use cc_media_probe::document::{Document, Generator, UsageError};
  ///
  /// let error = UsageError::new("standard input (-) was named twice");
  /// let document = Document::from_usage_error(Generator::new("media-catalog", "1.2.3"), error);
  /// assert!(document.results().is_empty());
  /// assert_eq!(
  ///     document.usage_error().map(UsageError::message),
  ///     Some("standard input (-) was named twice")
  /// );
  /// ```
  #[must_use]
  pub fn from_usage_error(generator: Generator, error: UsageError) -> Document {
    Document {
      schema_version: SchemaVersion::CURRENT,
      generator,
      usage_error: Some(error),
      results: Vec::new(),
    }
  }

  /// The version that the document was read with, or `SchemaVersion::CURRENT` for one that this version built,
  /// which every record that it holds carries too. The example of [`Document`] reads it.
  #[must_use]
  pub fn schema_version(&self) -> SchemaVersion {
    self.schema_version
  }

  /// The program that wrote the document. The example of [`Document`] reads it.
  #[must_use]
  pub fn generator(&self) -> &Generator {
    &self.generator
  }

  /// The usage error, where the command line could not be run. The example of [`Document::from_usage_error`]
  /// reads it.
  #[must_use]
  pub fn usage_error(&self) -> Option<&UsageError> {
    self.usage_error.as_ref()
  }

  /// The entries, one for each input in order; an entry's position here is its index. The example of [`Document`]
  /// reads them.
  #[must_use]
  pub fn results(&self) -> &[DocumentEntry] {
    &self.results
  }
}
