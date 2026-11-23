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

//! The read side of `api.md` section 12: a document has the wire form of that section, a document read back gives
//! its entries through their accessors, and reading refuses a document that breaks its rules. A usage error never
//! stands beside results, an entry of a known outcome holds exactly one record that warrants that outcome, and every
//! nested record carries the document's version. An input is a display string, with its raw form beside it when it
//! is lossy, and an identifier is a string at every width. An entry whose outcome this version cannot derive is
//! refused, and takes the outcome given.

use std::path::Path;

use cc_media_probe::details::FormatDetails;
use cc_media_probe::document::{Document, DocumentEntry, EntryOutcome, Generator, SchemaVersion, UsageError};
use cc_media_probe::model::Detection;
use cc_media_probe::probe_bytes;
use cc_media_probe::source::{PortablePath, SourceIdentifier};
use serde_json::{Value, json};

fn error_entries() -> Vec<DocumentEntry> {
  let mut entries = Vec::new();
  for bytes in [&b""[..], &b"not media"[..]] {
    let record = probe_bytes(bytes).unwrap_err().to_record();
    let input = PortablePath::from_path(Path::new("input.bin"));
    entries.push(DocumentEntry::from_error(input, record));
  }
  entries
}

#[test]
fn a_document_read_back_gives_its_entries() {
  let document = Document::from_results(Generator::new("catalog", "1.2.3"), error_entries());
  let text = serde_json::to_string(&document).unwrap();
  let read: Document = serde_json::from_str(&text).unwrap();
  assert_eq!(read, document);
  assert_eq!(read.schema_version(), SchemaVersion::CURRENT);
  assert_eq!(read.generator().name(), "catalog");
  assert_eq!(read.generator().version(), "1.2.3");
  assert_eq!(read.usage_error(), None);
  let mut kinds = Vec::new();
  for (index, entry) in read.results().iter().enumerate() {
    let written: Value = serde_json::from_str(&text).unwrap();
    assert_eq!(
      written["results"][index]["index"],
      json!(index),
      "an entry's index is its position"
    );
    assert_eq!(entry.input().display(), "input.bin");
    assert_eq!(entry.outcome(), &EntryOutcome::Error);
    assert!(entry.report().is_none());
    assert!(entry.detection().is_none());
    kinds.push(entry.error().unwrap().kind().as_str().to_owned());
  }
  assert_eq!(kinds, ["format_not_determined", "unsupported_format"]);
}

#[test]
fn a_usage_error_document_reads_back_without_results() {
  let error = UsageError::new("standard input (-) was named twice");
  let document = Document::from_usage_error(Generator::new("catalog", "1.2.3"), error);
  let text = serde_json::to_string(&document).unwrap();
  let read: Document = serde_json::from_str(&text).unwrap();
  assert_eq!(read, document);
  assert_eq!(
    read.usage_error().unwrap().message(),
    "standard input (-) was named twice"
  );
  assert!(read.results().is_empty());
}

#[test]
fn reading_refuses_a_usage_error_beside_results() {
  let with_results = serde_json::to_value(Document::from_results(Generator::new("catalog", "1"), error_entries()));
  let mut text = serde_json::to_value(Document::from_usage_error(
    Generator::new("catalog", "1"),
    UsageError::new("x"),
  ))
  .unwrap();
  text["results"] = with_results.unwrap()["results"].clone();
  let error = serde_json::from_value::<Document>(text).unwrap_err();
  assert_eq!(error.to_string(), "a document with a usage error holds no results");
}

#[test]
fn reading_refuses_an_entry_of_a_known_outcome_without_its_record() {
  let mut text = serde_json::to_value(Document::from_results(Generator::new("catalog", "1"), error_entries())).unwrap();
  text["results"][0]["error"] = Value::Null;
  assert!(serde_json::from_value::<Document>(text).is_err());
}

fn results_document() -> Value {
  serde_json::to_value(Document::from_results(Generator::new("catalog", "1"), error_entries())).unwrap()
}

fn read_error(text: Value) -> String {
  serde_json::from_value::<Document>(text).unwrap_err().to_string()
}

/// A report without streams, of that version, such as the partial report that an error record carries.
fn report_text(version: &str) -> Value {
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
      "record": { "exhausted": [], "first_stopped": [] },
    },
    "usage": { "bytes_read": 0, "peak_working_memory": 0 },
    "sources": [],
  })
}

#[test]
fn the_document_has_the_wire_form_of_section_12() {
  let text = results_document();
  assert_eq!(text["schema_version"], json!("1.0"));
  assert!(text.get("usage_error").is_none(), "an absent member is omitted");
  let entry = &text["results"][0];
  assert_eq!(entry["input"], json!("input.bin"), "an input is its display string");
  assert!(entry.get("input_lossy").is_none() && entry.get("input_raw").is_none());
  assert_eq!(entry["outcome"], json!("error"));
  assert!(entry.get("report").is_none() && entry.get("detection").is_none());
  assert_eq!(entry["error"]["schema_version"], json!("1.0"));
  assert_eq!(entry["error"]["kind"], json!("format_not_determined"));
  assert!(entry["error"].get("partial_report").is_none());
}

#[test]
fn reading_refuses_an_outcome_that_the_record_does_not_warrant() {
  let mut text = results_document();
  text["results"][0]["outcome"] = json!("ok");
  assert_eq!(
    read_error(text),
    "an entry's outcome disagrees with the record it holds"
  );
}

#[test]
fn a_detection_entry_reads_back_only_with_the_outcome_it_warrants() {
  let detection = |outcome: &str, entry_outcome: &str| {
    let mut text = results_document();
    text["results"][0] = json!({
      "index": 0,
      "input": "input.bin",
      "outcome": entry_outcome,
      "detection": { "schema_version": "1.0", "outcome": outcome },
    });
    serde_json::from_value::<Document>(text)
  };
  let read = detection("family_only", "partial").unwrap();
  let entry = &read.results()[0];
  assert_eq!(entry.outcome(), &EntryOutcome::Partial);
  assert_eq!(entry.detection().unwrap().outcome().as_str(), "family_only");
  assert!(detection("identified", "partial").is_err());
  assert!(detection("insufficient_data", "ok").is_err());
  // A detection outcome of a newer version keeps the outcome that was written.
  let newer = detection("identified_by_content", "partial").unwrap();
  assert_eq!(
    newer.results()[0].detection().unwrap().outcome().as_str(),
    "identified_by_content"
  );
}

#[test]
fn reading_refuses_a_nested_record_of_another_version() {
  let mut text = results_document();
  text["results"][1]["error"]["schema_version"] = json!("1.1");
  assert_eq!(read_error(text), NESTED);
}

/// The version error of a record nested in another of the same major version.
const NESTED: &str = "a nested record's schema version differs from that of the document or record that holds it";
/// The version error of a root record of version 2.0, whether it is a document or nested in one. It names both
/// versions (plan section 8.8).
const OTHER_MAJOR: &str =
  "a root record has a schema major version that this version does not read: 2.0; this version reads 1.0";

#[test]
fn reading_refuses_a_document_of_another_major_version() {
  let mut text = results_document();
  text["schema_version"] = json!("2.0");
  assert_eq!(read_error(text), OTHER_MAJOR);
}

#[test]
fn an_outcome_of_a_newer_version_reads_back_as_written() {
  let mut text = results_document();
  text["results"][0]["outcome"] = json!("skipped");
  let read: Document = serde_json::from_value(text).unwrap();
  assert_eq!(read.results()[0].outcome().as_str(), "skipped");
}

#[test]
fn the_entry_form_of_the_plan_reads_back() {
  // The entries of plan section 8.9 that hold an error, with the message that the record needs, at their positions
  // 0 and 1 of this document. A member that this version does not know, `detected`, is ignored.
  let error = |kind: &str| json!({ "schema_version": "1.0", "kind": kind, "message": "not probed" });
  let mut written = json!({
    "schema_version": "1.0",
    "generator": { "name": "cc-media-probe-cli", "version": "0.1.0" },
    "results": [
      { "index": 0, "input": "notes.txt", "outcome": "error", "error": error("unknown_format") },
      { "index": 1, "input": "tape.mxf", "outcome": "error", "error": error("unsupported_format") },
    ],
  });
  written["results"][1]["error"]["detected"] = json!({});
  let read: Document = serde_json::from_value(written.clone()).unwrap();
  let entry = &read.results()[0];
  assert_eq!(entry.input().display(), "notes.txt");
  // The index is the entry's position, so an index that skips or repeats one is malformed.
  for indexes in [[2, 3], [0, 0]] {
    let mut skipped = written.clone();
    skipped["results"][0]["index"] = json!(indexes[0]);
    skipped["results"][1]["index"] = json!(indexes[1]);
    assert_eq!(read_error(skipped), "an entry's index is its position in the results");
  }
  assert!(!entry.input().is_lossy() && entry.input().raw().is_none());
  assert_eq!(entry.input().to_native().as_deref(), Some(Path::new("notes.txt")));
  assert_eq!(read.results()[1].error().unwrap().kind().as_str(), "unsupported_format");
  written["results"][1]["error"]
    .as_object_mut()
    .unwrap()
    .remove("detected");
  assert_eq!(serde_json::to_value(&read).unwrap(), written);
}

#[cfg(unix)]
#[test]
fn a_lossy_input_has_its_flag_and_its_raw_form_beside_it() {
  use std::ffi::OsStr;
  use std::os::unix::ffi::OsStrExt;
  let path = Path::new(OsStr::from_bytes(b"caf\xe9.mkv"));
  let record = probe_bytes(b"").unwrap_err().to_record();
  let entry = DocumentEntry::from_error(PortablePath::from_path(path), record);
  let text = serde_json::to_value(Document::from_results(Generator::new("catalog", "1"), vec![entry])).unwrap();
  let written = &text["results"][0];
  assert_eq!(written["input"], json!("caf\u{FFFD}.mkv"));
  assert_eq!(written["input_lossy"], json!(true));
  assert_eq!(
    written["input_raw"],
    json!({ "encoding": "unix_bytes", "data": "636166e92e6d6b76" })
  );
  let read: Document = serde_json::from_value(text.clone()).unwrap();
  let input = read.results()[0].input();
  assert!(input.is_lossy());
  assert_eq!(input.raw().unwrap().encoding().as_str(), "unix_bytes");
  assert_eq!(input.to_native().as_deref(), Some(path));
  // The flag and the raw form stand together.
  for member in ["input_lossy", "input_raw"] {
    let mut without = text.clone();
    without["results"][0].as_object_mut().unwrap().remove(member);
    assert_eq!(
      read_error(without),
      "a lossy path has a raw form beside its display string, and only a lossy path"
    );
  }
}

#[cfg(windows)]
#[test]
fn a_lossy_input_has_its_flag_and_its_raw_form_beside_it() {
  use std::ffi::OsString;
  use std::os::windows::ffi::OsStringExt;
  let path = std::path::PathBuf::from(OsString::from_wide(&[0x0061, 0xd800]));
  let record = probe_bytes(b"").unwrap_err().to_record();
  let entry = DocumentEntry::from_error(PortablePath::from_path(&path), record);
  let text = serde_json::to_value(Document::from_results(Generator::new("catalog", "1"), vec![entry])).unwrap();
  assert_eq!(
    text["results"][0]["input_raw"],
    json!({ "encoding": "windows_wide", "data": "0061d800" })
  );
  let read: Document = serde_json::from_value(text).unwrap();
  assert_eq!(read.results()[0].input().to_native(), Some(path));
}

#[test]
fn an_identifier_is_a_string_at_every_width() {
  // The observations of a media segment keep the 32-bit `track_ID` of each fragment header.
  let mut text = results_document();
  text["results"][0]["error"]["kind"] = json!("no_stream_inventory");
  text["results"][0]["error"]["observations"] = json!({
    "format_details": { "IsoBaseMedia": { "fragment_track_identifiers": ["1", "4294967295"] } },
    "diagnostics": [],
  });
  let read: Document = serde_json::from_value(text.clone()).unwrap();
  let observations = read.results()[0].error().unwrap().observations().unwrap();
  let Some(FormatDetails::IsoBaseMedia(details)) = observations.format_details() else {
    panic!("the observations keep the ISO details");
  };
  assert_eq!(details.fragment_track_identifiers(), [1, u32::MAX]);
  assert_eq!(serde_json::to_value(&read).unwrap(), text);
  // A number is not the form of an identifier, whatever its width.
  let mut number = text;
  number["results"][0]["error"]["observations"]["format_details"]["IsoBaseMedia"]["fragment_track_identifiers"] =
    json!([1]);
  assert!(serde_json::from_value::<Document>(number).is_err());
  let source: SourceIdentifier = serde_json::from_value(json!("7")).unwrap();
  assert_eq!(serde_json::to_value(source).unwrap(), json!("7"));
  assert!(serde_json::from_value::<SourceIdentifier>(json!(7)).is_err());
  // Since the ninth review a source identifier is 64 bits wide, so every request that the limit allows has one, and
  // it is read in canonical decimal only.
  for wide in ["4294967296", "18446744073709551615"] {
    let source: SourceIdentifier = serde_json::from_value(json!(wide)).unwrap();
    assert_eq!(serde_json::to_value(source).unwrap(), json!(wide));
  }
  for malformed in ["+1", "01", "-1", "18446744073709551616", ""] {
    assert!(
      serde_json::from_value::<SourceIdentifier>(json!(malformed)).is_err(),
      "{malformed}"
    );
  }
}

#[test]
fn a_document_rebuilt_from_newer_entries_gives_them_its_version() {
  let mut newer = results_document();
  newer["schema_version"] = json!("1.1");
  for index in 0..2 {
    newer["results"][index]["error"]["schema_version"] = json!("1.1");
  }
  newer["results"][0]["error"]["partial_report"] = report_text("1.1");
  let read: Document = serde_json::from_value(newer).unwrap();
  assert_eq!(read.schema_version().minor(), 1);
  let rebuilt = Document::from_results(Generator::new("catalog", "1"), read.results().to_vec());
  let text = serde_json::to_value(&rebuilt).unwrap();
  assert_eq!(text["schema_version"], json!("1.0"));
  for index in 0..2 {
    assert_eq!(text["results"][index]["error"]["schema_version"], json!("1.0"));
  }
  // The report inside an error record is a root record nested in the document too.
  assert_eq!(
    text["results"][0]["error"]["partial_report"]["schema_version"],
    json!("1.0")
  );
  assert!(serde_json::from_value::<Document>(text).is_ok());
}

#[test]
fn reading_checks_the_partial_report_inside_an_error_record() {
  // A source that fails after the inventory leaves an error that carries the report so far.
  let mut text = results_document();
  text["results"][0]["error"]["partial_report"] = report_text("1.0");
  let read: Document = serde_json::from_value(text.clone()).unwrap();
  assert!(read.results()[0].error().unwrap().partial_report().is_some());
  for (version, message) in [("1.1", NESTED), ("2.0", OTHER_MAJOR)] {
    text["results"][0]["error"]["partial_report"]["schema_version"] = json!(version);
    assert_eq!(read_error(text.clone()), message);
  }
}

#[test]
fn a_raw_path_of_a_known_encoding_is_read_only_as_whole_hexadecimal_units() {
  let lossy = |encoding: &str, data: &str| {
    let mut text = results_document();
    text["results"][0]["input"] = json!("caf\u{FFFD}.mkv");
    text["results"][0]["input_lossy"] = json!(true);
    text["results"][0]["input_raw"] = json!({ "encoding": encoding, "data": data });
    serde_json::from_value::<Document>(text)
  };
  // Two lowercase hexadecimal digits a byte, and four a 16-bit unit, as the writer writes them.
  let malformed = [
    ("unix_bytes", "gg"),
    ("unix_bytes", "636"),
    ("unix_bytes", "+f"),
    ("unix_bytes", "E9"),
    ("windows_wide", "0061d8"),
    ("windows_wide", "+061"),
    ("windows_wide", "D800"),
  ];
  for (encoding, data) in malformed {
    assert_eq!(
      lossy(encoding, data).unwrap_err().to_string(),
      "the data of a raw path is lowercase hexadecimal, two digits a byte for unix_bytes and four a unit for \
       windows_wide",
      "{encoding} {data}"
    );
  }
  // A foreign path is kept although this platform cannot rebuild it, and so is the data of an encoding that a
  // newer version wrote, as it was written.
  let native = |encoding: &str, data: &str| {
    let read = lossy(encoding, data).unwrap();
    let input = read.results()[0].input();
    assert_eq!(input.raw().unwrap().data(), data);
    input.to_native()
  };
  // Each data spells `caf`, an ill-formed byte or an unpaired surrogate, and `.mkv`, so it decodes to the display.
  assert_eq!(native("unix_bytes", "636166e92e6d6b76").is_some(), cfg!(unix));
  assert_eq!(
    native("windows_wide", "006300610066d800002e006d006b0076").is_some(),
    cfg!(windows)
  );
  assert_eq!(native("vms_records", "Any Text"), None);
}

#[test]
fn reading_checks_every_nested_record_of_an_entry() {
  // An entry of a newer outcome may hold several records; each must have the document's version.
  let mut text = results_document();
  text["results"][0]["outcome"] = json!("skipped");
  text["results"][0]["detection"] = json!({ "schema_version": "1.0", "outcome": "identified" });
  assert!(serde_json::from_value::<Document>(text.clone()).is_ok());
  text["results"][0]["error"]["schema_version"] = json!("1.1");
  assert_eq!(read_error(text), NESTED);
}

#[test]
fn an_entry_whose_outcome_is_underivable_is_refused_and_takes_the_outcome_given() {
  let detection = |outcome: &str| -> Detection {
    serde_json::from_value(json!({ "schema_version": "1.0", "outcome": outcome })).unwrap()
  };
  let input = || PortablePath::from_path(Path::new("input.bin"));
  let known = DocumentEntry::from_detection(input(), detection("family_only")).unwrap();
  assert_eq!(known.outcome(), &EntryOutcome::Partial);
  // A detection outcome of a newer version decides an outcome that this version cannot derive.
  let refused = DocumentEntry::from_detection(input(), detection("identified_by_content")).unwrap_err();
  assert_eq!(
    refused.to_string(),
    "the record holds a value of a newer version that decides the entry's outcome, so the outcome must be given"
  );
  let entry = refused.with_outcome(EntryOutcome::Partial).unwrap();
  assert_eq!(entry.outcome(), &EntryOutcome::Partial);
  let text = serde_json::to_value(Document::from_results(Generator::new("catalog", "1"), vec![entry])).unwrap();
  assert_eq!(text["results"][0]["outcome"], json!("partial"));
  let read: Document = serde_json::from_value(text).unwrap();
  assert_eq!(
    read.results()[0].detection().unwrap().outcome().as_str(),
    "identified_by_content"
  );
}
