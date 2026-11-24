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

//! The document written as it goes, `api.md` section 12: it writes what `Document::from_results` writes, takes the
//! next entry only after the previous one was written, and takes no entry after an output error.

use std::cell::Cell;
use std::io::{self, Write};
use std::path::Path;
use std::rc::Rc;

use cc_media_probe::document::{Document, DocumentEntry, Generator};
use cc_media_probe::probe_bytes;
use cc_media_probe::source::PortablePath;
use serde_json::json;

fn error_entry() -> DocumentEntry {
  let record = probe_bytes(b"").unwrap_err().to_record();
  DocumentEntry::from_error(PortablePath::from_path(Path::new("input.bin")), record)
}

/// Two error entries, a detection entry, and an entry read from a document of version 1.1, whose error record
/// carries a partial report.
fn mixed_entries() -> Vec<DocumentEntry> {
  let detection = serde_json::from_value(json!({ "schema_version": "1.0", "outcome": "identified", "format": "Matroska", "confidence": 100, "candidates": [] })).unwrap();
  let detection = DocumentEntry::from_detection(PortablePath::from_path(Path::new("clip.mp4")), detection);
  let mut newer = serde_json::to_value(Document::from_results(
    Generator::new("catalog", "1"),
    vec![error_entry()],
  ))
  .unwrap();
  newer["schema_version"] = json!("1.1");
  newer["results"][0]["error"]["schema_version"] = json!("1.1");
  newer["results"][0]["error"]["partial_report"] = json!({
    "schema_version": "1.1",
    "format": "Matroska",
    "streams": [],
    "chapter_editions": [],
    "tag_sets": [],
    "attachments": [],
    "diagnostics": [],
    "completeness": {
      "detection": "Complete",
      "discovery": "Complete",
      "headers": "Complete",
      "tags": "Complete",
      "chapters": "Complete",
      "attachments": "Complete",
      "reasons": [],
      "record": { "exhausted": [], "first_stopped": [] },
    },
    "usage": { "bytes_read": 0, "peak_working_memory": 0 },
    "sources": [],
  });
  let newer: Document = serde_json::from_value(newer).unwrap();
  vec![
    error_entry(),
    error_entry(),
    detection.unwrap(),
    newer.results()[0].clone(),
  ]
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
  // The entry of version 1.1 is written with the document's version, down to the partial report of its error
  // record, so the document reads back.
  let text = written(mixed_entries(), false);
  let value: serde_json::Value = serde_json::from_str(&text).unwrap();
  assert_eq!(value["results"][3]["error"]["schema_version"], json!("1.0"));
  assert_eq!(
    value["results"][3]["error"]["partial_report"]["schema_version"],
    json!("1.0")
  );
  let read: Document = serde_json::from_str(&text).unwrap();
  assert_eq!(read.results().len(), 4);
}

/// An output that accepts a number of bytes and then fails, as a closed pipe does, and counts what it accepted.
struct FailingOutput {
  allowed: usize,
  accepted: Rc<Cell<usize>>,
}

impl Write for FailingOutput {
  fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
    let room = self.allowed - self.accepted.get();
    if room == 0 {
      return Err(io::Error::from(io::ErrorKind::BrokenPipe));
    }
    let count = buffer.len().min(room);
    self.accepted.set(self.accepted.get() + count);
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
  let error = Document::serialize_results(
    &Generator::new("catalog", "1"),
    entries,
    &mut serde_json::Serializer::new(output),
  )
  .unwrap_err();
  assert!(error.is_io());
  taken.take()
}

#[test]
fn an_output_failure_stops_the_entries() {
  let empty = serde_json::to_string(&Document::from_results(Generator::new("catalog", "1"), Vec::new())).unwrap();
  // Everything up to the opening of the results, and one entry.
  let prefix = empty.len() - "]}".len();
  // A document writes an entry with its position as its index, so the entry's length is what it adds.
  let one = Document::from_results(Generator::new("catalog", "1"), vec![error_entry()]);
  let entry = serde_json::to_string(&one).unwrap().len() - empty.len();
  // The output fails inside the first entry: the second is never taken.
  assert_eq!(take_points(prefix + 1), [prefix]);
  // It fails inside the second entry, which was taken only after the first was written.
  assert_eq!(take_points(prefix + entry + 2), [prefix, prefix + entry]);
}
