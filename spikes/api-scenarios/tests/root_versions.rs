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

//! The twelfth review's case, `api.md` section 12: each of the four root records gives the version that it was read
//! with through `schema_version()`, so a caller who holds a bare record need not write it again to learn it.

use std::path::Path;

use cc_media_probe::document::{Document, DocumentEntry, Generator, SchemaVersion};
use cc_media_probe::error::ErrorRecord;
use cc_media_probe::model::Detection;
use cc_media_probe::source::PortablePath;
use cc_media_probe::{MediaReport, probe_bytes};
use serde_json::json;

fn report_text(version: &str) -> serde_json::Value {
  json!({
    "schema_version": version,
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
      "record": { "exhausted": [], "first_stopped": [], "first_reasons": [], "omitted_reasons": "0", "omitted_references": "0" },
    },
    "usage": { "bytes_read": 0, "peak_working_memory": 0 },
    "sources": [],
  })
}

#[test]
fn a_bare_root_record_gives_the_version_that_it_was_read_with() {
  let report: MediaReport = serde_json::from_value(report_text("1.1")).unwrap();
  assert_eq!(
    (report.schema_version().major(), report.schema_version().minor()),
    (1, 1)
  );
  let detection: Detection = serde_json::from_value(json!({
    "schema_version": "1.1",
    "outcome": "identified_by_content",
    "candidates": [],
    "usage": { "bytes_read": 0, "peak_working_memory": 0 },
  }))
  .unwrap();
  assert_eq!(detection.schema_version().minor(), 1);
  let mut record = serde_json::to_value(probe_bytes(b"").unwrap_err().to_record()).unwrap();
  record["schema_version"] = json!("1.1");
  let error: ErrorRecord = serde_json::from_value(record).unwrap();
  assert_eq!(error.schema_version().minor(), 1);
}

#[test]
fn a_record_that_this_version_made_or_a_document_holds_has_the_current_version() {
  let made = probe_bytes(b"").unwrap_err().to_record();
  assert_eq!(made.schema_version(), SchemaVersion::CURRENT);
  // A document gives each record that it holds its own version.
  let report: MediaReport = serde_json::from_value(report_text("1.1")).unwrap();
  let entry = DocumentEntry::from_report(PortablePath::from_path(Path::new("movie.mkv")), report).unwrap();
  let document = Document::from_results(Generator::new("catalog", "1"), vec![entry]);
  let held = document.results()[0].report().unwrap();
  assert_eq!(held.schema_version(), SchemaVersion::CURRENT);
  assert_eq!(held.schema_version(), document.schema_version());
}

#[test]
fn a_bare_root_record_written_again_keeps_the_version_that_it_was_read_with() {
  // The thirteenth review asked to tell the two apart: a record that this version built writes 1.0, and one that it
  // read and writes again directly writes the version that it was read with; a document gives every record its own.
  let report: MediaReport = serde_json::from_value(report_text("1.1")).unwrap();
  assert_eq!(serde_json::to_value(&report).unwrap()["schema_version"], json!("1.1"));
  let built = probe_bytes(b"").unwrap_err().to_record();
  assert_eq!(serde_json::to_value(&built).unwrap()["schema_version"], json!("1.0"));
  let entry = DocumentEntry::from_report(PortablePath::from_path(Path::new("movie.mkv")), report).unwrap();
  let document = serde_json::to_value(Document::from_results(Generator::new("catalog", "1"), vec![entry])).unwrap();
  assert_eq!(document["results"][0]["report"]["schema_version"], json!("1.0"));
}
