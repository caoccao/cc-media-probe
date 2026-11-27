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

//! The document written as it goes (`api.md` section 12; `json-output.md` section 3): `Document::serialize_results`
//! writes what `Document::from_results` with the same entries writes, takes the next entry only after the previous
//! one was written, takes no entry after an output error, and prepares the entries only once the results open.

use std::cell::Cell;
use std::io::{self, Write};
use std::path::Path;
use std::rc::Rc;

use cc_media_probe::document::{Document, DocumentEntry, Generator};
use cc_media_probe::source::PortablePath;
use serde_json::{Value, json};

use crate::fixtures::{detection, document, entry, error_record, partial_report, report, with_partial};

fn error_entry() -> DocumentEntry {
  let record = cc_media_probe::probe_bytes(b"").unwrap_err().to_record();
  DocumentEntry::from_error(PortablePath::from_path(Path::new("input.bin")), record)
}

/// Two error entries, an entry of an identified detection, and the entries of a document of version 1.1: a report,
/// and an error record that carries a partial report.
fn mixed_entries() -> Vec<DocumentEntry> {
  let detection = serde_json::from_value(detection("1.0")).unwrap();
  let detection = DocumentEntry::from_detection(PortablePath::from_path(Path::new("clip.mp4")), detection).unwrap();
  let failed = with_partial(error_record("1.1"), partial_report("1.1"));
  let newer = document(
    "1.1",
    json!([
      entry(0, "ok", "report", report("1.1")),
      entry(1, "error", "error", failed)
    ]),
  );
  let newer: Document = serde_json::from_value(newer).unwrap();
  let mut entries = vec![error_entry(), error_entry(), detection];
  entries.extend(newer.results().iter().cloned());
  entries
}

fn written(entries: Vec<DocumentEntry>, pretty: bool) -> String {
  let generator = Generator::new("catalog", "1.2.3");
  let mut output = Vec::new();
  if pretty {
    Document::serialize_results(&generator, entries, &mut serde_json::Serializer::pretty(&mut output)).unwrap();
  } else {
    Document::serialize_results(&generator, entries, &mut serde_json::Serializer::new(&mut output)).unwrap();
  }
  String::from_utf8(output).unwrap()
}

#[test]
fn the_writer_writes_what_from_results_writes() {
  for entries in [Vec::new(), mixed_entries()] {
    let document = Document::from_results(Generator::new("catalog", "1.2.3"), entries.clone());
    assert_eq!(
      written(entries.clone(), false),
      serde_json::to_string(&document).unwrap()
    );
    assert_eq!(written(entries, true), serde_json::to_string_pretty(&document).unwrap());
  }
  // The entries of version 1.1 are written with the document's version, down to the partial report of the error
  // record, so the document reads back.
  let text = written(mixed_entries(), false);
  let value: Value = serde_json::from_str(&text).unwrap();
  assert_eq!(value["results"][3]["report"]["schema_version"], json!("1.0"));
  assert_eq!(
    value["results"][4]["error"]["partial_report"]["schema_version"],
    json!("1.0")
  );
  let indexes: Vec<Value> = (0..5).map(|index| value["results"][index]["index"].clone()).collect();
  assert_eq!(indexes, [json!(0), json!(1), json!(2), json!(3), json!(4)]);
  let read: Document = serde_json::from_str(&text).unwrap();
  assert_eq!(read.results().len(), 5);
}

/// An output that accepts a number of bytes and then fails, as a closed pipe does, and counts what it accepted.
struct FailingOutput {
  allowed: usize,
  accepted: Rc<Cell<usize>>,
}

impl Write for FailingOutput {
  fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
    let room = self.allowed.saturating_sub(self.accepted.get());
    if room == 0 {
      return Err(io::Error::from(io::ErrorKind::BrokenPipe));
    }
    let count = buffer.len().min(room);
    self.accepted.set(self.accepted.get().saturating_add(count));
    Ok(count)
  }

  fn flush(&mut self) -> io::Result<()> {
    Ok(())
  }
}

/// Writes three entries to an output that fails after `allowed` bytes, and gives the output's byte count at the
/// moment each entry was taken.
fn take_points(allowed: usize) -> Vec<usize> {
  let accepted = Rc::new(Cell::new(0));
  let taken = Rc::new(Cell::new(Vec::new()));
  let entries = (0..3).map(|_| {
    let mut points = taken.take();
    points.push(accepted.get());
    taken.set(points);
    error_entry()
  });
  let output = FailingOutput {
    allowed,
    accepted: Rc::clone(&accepted),
  };
  let generator = Generator::new("catalog", "1");
  let error = Document::serialize_results(&generator, entries, &mut serde_json::Serializer::new(output)).unwrap_err();
  assert!(error.is_io());
  taken.take()
}

#[test]
fn an_output_failure_stops_the_entries() {
  let empty = serde_json::to_string(&Document::from_results(Generator::new("catalog", "1"), Vec::new())).unwrap();
  // Everything up to the opening of the results.
  let prefix = empty.len() - "]}".len();
  // A document writes an entry with its position as its index, so the entry's length is what it adds.
  let one = Document::from_results(Generator::new("catalog", "1"), vec![error_entry()]);
  let entry = serde_json::to_string(&one).unwrap().len() - empty.len();
  // The output fails inside the first entry: the second is never taken.
  assert_eq!(take_points(prefix + 1), [prefix]);
  // It fails inside the second entry, which was taken only after the first was written.
  assert_eq!(take_points(prefix + entry + 2), [prefix, prefix + entry]);
  // It fails at the document's last byte, after every entry was taken.
  let all = serde_json::to_string(&Document::from_results(
    Generator::new("catalog", "1"),
    vec![error_entry(); 3],
  ));
  let all = all.unwrap().len();
  assert_eq!(take_points(all - 1).len(), 3);
}

/// Entries that note when they are turned into their iterator, as a source that opens its first input then would.
struct Prepared<'flag> {
  prepared: &'flag Cell<bool>,
}

impl IntoIterator for Prepared<'_> {
  type Item = DocumentEntry;
  type IntoIter = std::vec::IntoIter<DocumentEntry>;

  fn into_iter(self) -> Self::IntoIter {
    self.prepared.set(true);
    vec![error_entry()].into_iter()
  }
}

/// Whether the writer prepared the entries for an output that fails after `allowed` bytes.
fn prepared_for(allowed: usize) -> bool {
  let prepared = Cell::new(false);
  let output = FailingOutput {
    allowed,
    accepted: Rc::new(Cell::new(0)),
  };
  let entries = Prepared { prepared: &prepared };
  let generator = Generator::new("catalog", "1");
  let error = Document::serialize_results(&generator, entries, &mut serde_json::Serializer::new(output)).unwrap_err();
  assert!(error.is_io());
  prepared.get()
}

#[test]
fn the_entries_are_prepared_only_once_the_results_open() {
  // The review of the third slice's second part saw a source of entries prepared for an output whose first write
  // failed. The writer now turns the entries into their iterator only once the results have opened.
  let empty = serde_json::to_string(&Document::from_results(Generator::new("catalog", "1"), Vec::new())).unwrap();
  // Everything up to and including the results' opening bracket.
  let opened = empty.len() - "]}".len();
  for allowed in [0, 1, opened - 1] {
    assert!(!prepared_for(allowed), "{allowed}");
  }
  assert!(prepared_for(opened));
}
