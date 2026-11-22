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
  for (index, bytes) in [&b""[..], &b"not media"[..]].into_iter().enumerate() {
    let record = probe_bytes(bytes).unwrap_err().to_record();
    let input = PortablePath::from_path(Path::new("input.bin"));
    entries.push(DocumentEntry::from_error(index, input, record));
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
    assert_eq!(entry.index(), index);
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
  assert_eq!(
    read_error(text),
    "a nested record's schema version differs from the document's"
  );
}

#[test]
fn reading_refuses_a_document_of_another_major_version() {
  let mut text = results_document();
  text["schema_version"] = json!("2.0");
  assert_eq!(
    read_error(text),
    "the document has a schema major version that this version does not read"
  );
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
  // The entries of plan section 8.9 that hold an error, with the message that the record needs. A member that this
  // version does not know, `detected`, is ignored.
  let error = |kind: &str| json!({ "schema_version": "1.0", "kind": kind, "message": "not probed" });
  let mut written = json!({
    "schema_version": "1.0",
    "generator": { "name": "cc-media-probe-cli", "version": "0.1.0" },
    "results": [
      { "index": 2, "input": "notes.txt", "outcome": "error", "error": error("unknown_format") },
      { "index": 3, "input": "tape.mxf", "outcome": "error", "error": error("unsupported_format") },
    ],
  });
  written["results"][1]["error"]["detected"] = json!({});
  let read: Document = serde_json::from_value(written.clone()).unwrap();
  let entry = &read.results()[0];
  assert_eq!((entry.index(), entry.input().display()), (2, "notes.txt"));
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
  let entry = DocumentEntry::from_error(0, PortablePath::from_path(path), record);
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
  let entry = DocumentEntry::from_error(0, PortablePath::from_path(&path), record);
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
}

#[test]
fn a_document_rebuilt_from_newer_entries_gives_them_its_version() {
  let mut newer = results_document();
  newer["schema_version"] = json!("1.1");
  for index in 0..2 {
    newer["results"][index]["error"]["schema_version"] = json!("1.1");
  }
  let read: Document = serde_json::from_value(newer).unwrap();
  assert_eq!(read.schema_version().minor(), 1);
  let rebuilt = Document::from_results(Generator::new("catalog", "1"), read.results().to_vec());
  let text = serde_json::to_value(&rebuilt).unwrap();
  assert_eq!(text["schema_version"], json!("1.0"));
  for index in 0..2 {
    assert_eq!(text["results"][index]["error"]["schema_version"], json!("1.0"));
  }
  assert!(serde_json::from_value::<Document>(text).is_ok());
}

#[test]
fn reading_checks_every_nested_record_of_an_entry() {
  // An entry of a newer outcome may hold several records; each must have the document's version.
  let mut text = results_document();
  text["results"][0]["outcome"] = json!("skipped");
  text["results"][0]["detection"] = json!({ "schema_version": "1.0", "outcome": "identified" });
  assert!(serde_json::from_value::<Document>(text.clone()).is_ok());
  text["results"][0]["error"]["schema_version"] = json!("2.0");
  assert_eq!(
    read_error(text),
    "a nested record's schema version differs from the document's"
  );
}

#[test]
fn an_entry_whose_outcome_is_underivable_is_refused_and_takes_the_outcome_given() {
  let detection = |outcome: &str| -> Detection {
    serde_json::from_value(json!({ "schema_version": "1.0", "outcome": outcome })).unwrap()
  };
  let input = || PortablePath::from_path(Path::new("input.bin"));
  let known = DocumentEntry::from_detection(0, input(), detection("family_only")).unwrap();
  assert_eq!(known.outcome(), &EntryOutcome::Partial);
  // A detection outcome of a newer version decides an outcome that this version cannot derive.
  let refused = DocumentEntry::from_detection(0, input(), detection("identified_by_content")).unwrap_err();
  assert_eq!(
    refused.to_string(),
    "the record holds a value of a newer version that decides the entry's outcome, so the outcome must be given"
  );
  let entry = refused.with_outcome(EntryOutcome::Partial);
  assert_eq!(entry.outcome(), &EntryOutcome::Partial);
  let text = serde_json::to_value(Document::from_results(Generator::new("catalog", "1"), vec![entry])).unwrap();
  assert_eq!(text["results"][0]["outcome"], json!("partial"));
  let read: Document = serde_json::from_value(text).unwrap();
  assert_eq!(
    read.results()[0].detection().unwrap().outcome().as_str(),
    "identified_by_content"
  );
}
