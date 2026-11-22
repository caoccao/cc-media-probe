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

//! A document of results written as it goes, `api.md` section 12: one entry at a time, to any Serde serializer.

use std::cell::Cell;

use serde::ser::{Error as _, SerializeSeq, SerializeStruct};
use serde::{Serialize, Serializer};

use super::contents::{Document, Generator};
use super::entry::DocumentEntry;
use super::version::SchemaVersion;
use crate::constants::{
  DOCUMENT_NAME, ENTRIES_TAKEN_MESSAGE, GENERATOR_MEMBER, RESULTS_DOCUMENT_MEMBER_COUNT, RESULTS_MEMBER,
  SCHEMA_VERSION_MEMBER,
};

impl Document {
  /// Writes the document of these results as it goes: what `from_results` with the same entries writes. Each entry
  /// is written with the document's version and released before the next one is taken, and the first error of the
  /// serializer ends the document without taking another entry. The results are a sequence of unknown length, so a
  /// serializer that needs the length up front cannot write them.
  pub fn serialize_results<Entries, Output>(
    generator: &Generator,
    entries: Entries,
    serializer: Output,
  ) -> Result<Output::Ok, Output::Error>
  where
    Entries: IntoIterator<Item = DocumentEntry>,
    Output: Serializer,
  {
    let mut document = serializer.serialize_struct(DOCUMENT_NAME, RESULTS_DOCUMENT_MEMBER_COUNT)?;
    document.serialize_field(SCHEMA_VERSION_MEMBER, &SchemaVersion::CURRENT)?;
    document.serialize_field(GENERATOR_MEMBER, generator)?;
    document.serialize_field(RESULTS_MEMBER, &Results(Cell::new(Some(entries.into_iter()))))?;
    document.end()
  }
}

/// The entries, taken one at a time while they are written. Serializing takes them, so they are written once.
struct Results<Entries>(Cell<Option<Entries>>);

impl<Entries: Iterator<Item = DocumentEntry>> Serialize for Results<Entries> {
  fn serialize<Output: Serializer>(&self, serializer: Output) -> Result<Output::Ok, Output::Error> {
    let entries = self
      .0
      .take()
      .ok_or_else(|| Output::Error::custom(ENTRIES_TAKEN_MESSAGE))?;
    let mut sequence = serializer.serialize_seq(None)?;
    for entry in entries {
      // The entry goes out of scope before the next one is taken, and `?` stops at the first output error.
      sequence.serialize_element(&entry.with_schema_version(SchemaVersion::CURRENT))?;
    }
    sequence.end()
  }
}
