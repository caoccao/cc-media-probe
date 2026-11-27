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

//! A document of results written as it goes (`api.md` section 12): one entry at a time, to any Serde serializer, as
//! the maintainer chose on 2026-11-22.

use std::cell::Cell;

use serde::ser::{Error, SerializeSeq};
use serde::{Serialize, Serializer};

use super::constants::{ENTRIES_TAKEN_MESSAGE, POSITION_RANGE_MESSAGE};
use super::document_form::write_document;
use super::entry_form::PositionedEntry;
use super::{Document, DocumentEntry, Generator, SchemaVersion};
use crate::constants::LARGEST_EXACT;

impl Document {
  /// Writes the document of these results as it goes, which is what [`Document::from_results`] with the same
  /// entries writes, also for no entries. Each entry is written with the document's version and its position as its
  /// index, counted in a `u64` on every platform, and released before the next one is taken, so a scan of any length
  /// holds one entry at a time. The writer writes nothing after the document's closing: a caller who ends the
  /// document with a line break, or whose output is buffered, writes the break and flushes through the same output.
  ///
  /// ```
  /// use std::path::Path;
  /// use cc_media_probe::document::{DeriveOutcomeError, Document, DocumentEntry, Generator};
  /// use cc_media_probe::source::PortablePath;
  ///
  /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
  /// // One entry for each input, whichever way its probe ends, so each entry's index is its input's place. An entry
  /// // that cannot be built ends the scan, and its error is returned once the document is closed.
  /// let inputs = [("empty.bin", &b""[..]), ("notes.txt", &b"not a media file"[..])];
  /// let mut refused: Option<DeriveOutcomeError> = None;
  /// let entries = inputs.into_iter().map_while(|(name, bytes)| {
  ///     let input = PortablePath::from_path(Path::new(name));
  ///     let entry = match cc_media_probe::probe_bytes(bytes) {
  ///         Ok(report) => DocumentEntry::from_report(input, report),
  ///         Err(error) => Ok(DocumentEntry::from_error(input, error.to_record())),
  ///     };
  ///     entry.map_err(|error| refused = Some(error)).ok()
  /// });
  /// let mut output = Vec::new();
  /// let generator = Generator::new("media-catalog", "1.2.3");
  /// Document::serialize_results(&generator, entries, &mut serde_json::Serializer::new(&mut output))?;
  /// if let Some(error) = refused {
  ///     return Err(error.into());
  /// }
  /// let read: Document = serde_json::from_slice(&output)?;
  /// let names: Vec<&str> = read.results().iter().map(|entry| entry.input().display()).collect();
  /// assert_eq!(names, ["empty.bin", "notes.txt"]);
  /// # Ok(())
  /// # }
  /// ```
  ///
  /// # Errors
  ///
  /// The first error of the serializer ends the document, and no further entry is taken, so a caller who probes in
  /// the iterator stops probing. The entries are turned into their iterator only once the results have opened, so
  /// one whose `into_iter` opens a first input or prepares a first probe does nothing for an output that fails
  /// before them, as the review of the third slice's second part asked. The writer stops with an error before a
  /// position beyond 2^53 - 1, the limit of every number of the form: beyond it, JSON implementations need not agree
  /// on an integer's value, since a double holds 2^53 but cannot tell 2^53 + 1 from it (RFC 8259 section 6; RFC 7493
  /// section 2.2). The results are a sequence of unknown length, so a serializer that needs the length of a sequence
  /// up front fails at their start; one that holds such a sequence in memory writes the document only once it ends.
  pub fn serialize_results<Entries, Output>(
    generator: &Generator,
    entries: Entries,
    serializer: Output,
  ) -> Result<Output::Ok, Output::Error>
  where
    Entries: IntoIterator<Item = DocumentEntry>,
    Output: Serializer,
  {
    let results = StreamedResults {
      entries: Cell::new(Some(entries)),
      first_position: 0,
    };
    write_document(serializer, SchemaVersion::CURRENT, generator, None, &results)
  }
}

/// The entries, taken one at a time while they are written. Serializing takes them, so they are written once, and
/// turns them into their iterator only once the sequence of results has opened.
struct StreamedResults<Entries> {
  entries: Cell<Option<Entries>>,
  /// The position of the first entry: 0 for a document, and near a boundary in the tests below.
  first_position: u64,
}

impl<Entries: IntoIterator<Item = DocumentEntry>> Serialize for StreamedResults<Entries> {
  fn serialize<Output: Serializer>(&self, serializer: Output) -> Result<Output::Ok, Output::Error> {
    let entries = self
      .entries
      .take()
      .ok_or_else(|| Output::Error::custom(ENTRIES_TAKEN_MESSAGE))?;
    let mut sequence = serializer.serialize_seq(None)?;
    // `zip` turns the entries into their iterator here, after the sequence has opened. A position is a `u64` on every
    // platform, never a `usize` from `enumerate`, which a 32-bit target would let overflow at 2^32 entries. The
    // writer stops before a position beyond 2^53 - 1, so the counter never nears its own end.
    for (index, mut entry) in (self.first_position..).zip(entries) {
      if index > LARGEST_EXACT {
        return Err(Output::Error::custom(POSITION_RANGE_MESSAGE));
      }
      // The entry goes out of scope before the next one is taken, and `?` stops at the first output error.
      entry.set_schema_version(SchemaVersion::CURRENT);
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

  use super::StreamedResults;
  use crate::constants::LARGEST_EXACT;
  use crate::document::DocumentEntry;
  use crate::source::PortablePath;

  /// The indexes written for `count` error entries whose first position is `first_position`.
  fn indexes(first_position: u64, count: usize) -> Result<Vec<Value>, serde_json::Error> {
    let entries = (0..count).map(|_| {
      let error = crate::probe_bytes(b"").err().unwrap();
      DocumentEntry::from_error(PortablePath::from_path(Path::new("clip.mp4")), error.to_record())
    });
    let results = StreamedResults {
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
    let error = indexes(LARGEST_EXACT, 2).unwrap_err();
    assert_eq!(error.to_string(), "a document's last index is 2^53 - 1");
  }

  #[test]
  fn the_entries_are_written_once() {
    let results = StreamedResults {
      entries: Cell::new(Some(std::iter::empty::<DocumentEntry>())),
      first_position: 0,
    };
    assert_eq!(serde_json::to_value(&results).unwrap(), json!([]));
    let error = serde_json::to_value(&results).unwrap_err();
    assert_eq!(
      error.to_string(),
      "the entries of a document written as it goes were already written"
    );
  }
}
