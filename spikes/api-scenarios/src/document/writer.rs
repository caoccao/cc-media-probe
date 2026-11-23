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
use super::wire::PositionedEntry;
use crate::constants::{
  DOCUMENT_NAME, ENTRIES_TAKEN_MESSAGE, GENERATOR_MEMBER, LARGEST_EXACT, POSITION_RANGE_MESSAGE,
  RESULTS_DOCUMENT_MEMBER_COUNT, RESULTS_MEMBER, SCHEMA_VERSION_MEMBER,
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
    let results = Results {
      entries: Cell::new(Some(entries.into_iter())),
      first_position: 0,
    };
    document.serialize_field(RESULTS_MEMBER, &results)?;
    document.end()
  }
}

/// The entries, taken one at a time while they are written. Serializing takes them, so they are written once.
struct Results<Entries> {
  entries: Cell<Option<Entries>>,
  /// The position of the first entry: 0 for a document, and near a boundary in the tests below.
  first_position: u64,
}

impl<Entries: Iterator<Item = DocumentEntry>> Serialize for Results<Entries> {
  fn serialize<Output: Serializer>(&self, serializer: Output) -> Result<Output::Ok, Output::Error> {
    let entries = self
      .entries
      .take()
      .ok_or_else(|| Output::Error::custom(ENTRIES_TAKEN_MESSAGE))?;
    let mut sequence = serializer.serialize_seq(None)?;
    // A position is a `u64` on every platform, never a `usize` from `enumerate`, which a 32-bit target would let
    // overflow at 2^32 entries. The writer stops before a position beyond 2^53 - 1, so the counter never comes near
    // its own end.
    for (index, entry) in (self.first_position..).zip(entries) {
      if index > LARGEST_EXACT {
        return Err(Output::Error::custom(POSITION_RANGE_MESSAGE));
      }
      // The entry goes out of scope before the next one is taken, and `?` stops at the first output error.
      let entry = entry.with_schema_version(SchemaVersion::CURRENT);
      sequence.serialize_element(&PositionedEntry { index, entry: &entry })?;
    }
    sequence.end()
  }
}

#[cfg(test)]
mod tests {
  use std::cell::Cell;
  use std::path::Path;

  use serde_json::{Value, json};

  use super::Results;
  use crate::constants::LARGEST_EXACT;
  use crate::document::DocumentEntry;
  use crate::probe_bytes;
  use crate::source::PortablePath;

  /// The indexes written for `count` error entries whose first position is `first_position`.
  fn indexes(first_position: u64, count: usize) -> Result<Vec<Value>, serde_json::Error> {
    let entries = (0..count).map(|_| {
      DocumentEntry::from_error(
        PortablePath::from_path(Path::new("clip.mp4")),
        probe_bytes(b"").unwrap_err().to_record(),
      )
    });
    let results = Results {
      entries: Cell::new(Some(entries)),
      first_position,
    };
    let written = serde_json::to_value(&results)?;
    Ok(
      written
        .as_array()
        .unwrap()
        .iter()
        .map(|entry| entry["index"].clone())
        .collect(),
    )
  }

  #[test]
  fn a_position_counts_past_the_32_bit_range() {
    let first = u64::from(u32::MAX) - 1;
    assert_eq!(
      indexes(first, 3).unwrap(),
      vec![
        json!(4_294_967_294_u64),
        json!(4_294_967_295_u64),
        json!(4_294_967_296_u64)
      ]
    );
  }

  #[test]
  fn the_writer_stops_before_a_position_beyond_the_exact_range() {
    assert_eq!(indexes(LARGEST_EXACT, 1).unwrap(), vec![json!(LARGEST_EXACT)]);
    assert!(indexes(LARGEST_EXACT, 2).is_err());
  }
}
