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

//! How a document is read and written (`api.md` section 12; `json-output.md` section 3). A document is a root record:
//! its version is read before anything else, wherever it stands, and every record that an entry holds compares its
//! version with the document's before it decodes its body (`crate::form::Rooted`).
//!
//! Each entry is checked as soon as it is read, as the maintainer chose for the second part of the third slice: the
//! read stops at the first entry that breaks a rule of its own, before it decodes the next, and a reader that keeps
//! a place in the input, such as `serde_json`, gives the error the place where the read stopped, just after that
//! entry. Entries that stand before the document's version are held with the other members before it and checked
//! once the version has been read, so their error has the place where the read stopped after the version instead.
//! A usage error beside results is checked once the document has been read, since the two members can stand in
//! either order.

use serde::de::{Error, SeqAccess, Visitor};
use serde::ser::SerializeStruct;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::constants::{
  DOCUMENT_MEMBER_COUNT, DOCUMENT_NAME, GENERATOR_MEMBER, RESULTS_DOCUMENT_MEMBER_COUNT, RESULTS_EXPECTING,
  RESULTS_MEMBER, USAGE_ERROR_MEMBER, USAGE_ERROR_WITH_RESULTS_MESSAGE,
};
use super::entry_form::{EntryBody, PositionedEntry};
use super::{Document, DocumentEntry, Generator, SchemaVersion, UsageError};
use crate::form::{Rooted, SCHEMA_VERSION_MEMBER, WithKept};

impl Serialize for Document {
  fn serialize<Output: Serializer>(&self, serializer: Output) -> Result<Output::Ok, Output::Error> {
    write_document(
      serializer,
      self.schema_version,
      &self.generator,
      self.usage_error.as_ref(),
      &HeldResults(&self.results),
    )
  }
}

/// Writes a document's members in their order: its version, its generator, its usage error where it has one, and its
/// results, which a document held in memory and the writer of `Document::serialize_results` each give. The count of
/// members is exact, since a format that writes the length of a map up front, such as CBOR, writes the one given.
pub(super) fn write_document<Output: Serializer, Results: Serialize>(
  serializer: Output,
  version: SchemaVersion,
  generator: &Generator,
  usage_error: Option<&UsageError>,
  results: &Results,
) -> Result<Output::Ok, Output::Error> {
  let count = if usage_error.is_some() {
    DOCUMENT_MEMBER_COUNT
  } else {
    RESULTS_DOCUMENT_MEMBER_COUNT
  };
  let mut document = serializer.serialize_struct(DOCUMENT_NAME, count)?;
  document.serialize_field(SCHEMA_VERSION_MEMBER, &version)?;
  document.serialize_field(GENERATOR_MEMBER, generator)?;
  match usage_error {
    Some(error) => document.serialize_field(USAGE_ERROR_MEMBER, error)?,
    None => document.skip_field(USAGE_ERROR_MEMBER)?,
  }
  document.serialize_field(RESULTS_MEMBER, results)?;
  document.end()
}

/// The results of a document held in memory, each entry with its position as its index. Their length is given up
/// front (`json-output.md` section 2).
struct HeldResults<'document>(&'document [DocumentEntry]);

impl Serialize for HeldResults<'_> {
  fn serialize<Output: Serializer>(&self, serializer: Output) -> Result<Output::Ok, Output::Error> {
    serializer.collect_seq(
      (0_u64..)
        .zip(self.0)
        .map(|(index, entry)| PositionedEntry { index, entry }),
    )
  }
}

/// A document's members other than its version, as read.
#[derive(Deserialize)]
pub(super) struct DocumentBody {
  generator: Generator,
  usage_error: Option<UsageError>,
  results: ReadResults,
}

impl TryFrom<Rooted<DocumentBody>> for Document {
  type Error = &'static str;

  /// A document whose usage error never stands beside results. Its entries were checked as they were read.
  fn try_from(rooted: Rooted<DocumentBody>) -> Result<Document, &'static str> {
    let (version, body) = (rooted.version, rooted.body);
    if body.usage_error.is_some() && !body.results.0.is_empty() {
      return Err(USAGE_ERROR_WITH_RESULTS_MESSAGE);
    }
    Ok(Document {
      schema_version: version,
      generator: body.generator,
      usage_error: body.usage_error,
      results: body.results.0,
    })
  }
}

/// A document's entries, each checked at its position as soon as it is read.
struct ReadResults(Vec<DocumentEntry>);

impl<'input> Deserialize<'input> for ReadResults {
  fn deserialize<Input: Deserializer<'input>>(deserializer: Input) -> Result<ReadResults, Input::Error> {
    deserializer.deserialize_seq(ResultsVisitor)
  }
}

/// Reads the entries in their order, counting their positions in a `u64` on every platform.
struct ResultsVisitor;

impl<'input> Visitor<'input> for ResultsVisitor {
  type Value = ReadResults;

  fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    formatter.write_str(RESULTS_EXPECTING)
  }

  fn visit_seq<Items: SeqAccess<'input>>(self, mut items: Items) -> Result<ReadResults, Items::Error> {
    let mut entries = Vec::new();
    let mut position = 0_u64;
    while let Some(entry) = items.next_element::<WithKept<EntryBody>>()? {
      entries.push(entry.into_entry(position).map_err(Items::Error::custom)?);
      position = position.saturating_add(1);
    }
    Ok(ReadResults(entries))
  }
}
