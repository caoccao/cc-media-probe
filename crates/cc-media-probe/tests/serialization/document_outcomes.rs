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

//! An entry's outcome follows from the record that it holds, by the table of plan section 8.9 (`api.md` section 12;
//! `json-output.md` section 3): an entry's constructors derive it, and reading a document checks it. A report beside
//! a status of a newer version allows `ok` and `partial` while no status that this version knows records a shortfall
//! and the record holds no first reason, and `partial` alone otherwise; a report never allows `error`. A detection
//! outcome of a newer version allows any outcome. An outcome that this version cannot derive is refused with
//! `DeriveOutcomeError`, which gives its parts back and takes an outcome that the record allows.

use std::path::Path;

use cc_media_probe::document::{DeriveOutcomeError, Document, DocumentEntry, EntryOutcome, Generator};
use cc_media_probe::source::PortablePath;
use cc_media_probe::{Detection, MediaReport};
use serde_json::{Value, json};

use crate::fixtures::{document_holding, error_record, reads_document, record, report, stream, usage, with};

fn input() -> PortablePath {
  PortablePath::from_path(Path::new("input.bin"))
}

/// The outcome that the constructor derives for the report, or `None` where it refuses the report.
fn outcome_of(report: &Value) -> Option<EntryOutcome> {
  let read: MediaReport = serde_json::from_value(report.clone()).unwrap_or_else(|error| panic!("{report}: {error}"));
  DocumentEntry::from_report(input(), read)
    .ok()
    .map(|entry| entry.outcome().clone())
}

/// The outcomes of the three that a document reads beside the report.
fn allowed_for_report(report: &Value) -> Vec<&'static str> {
  ["ok", "partial", "error"]
    .into_iter()
    .filter(|outcome| reads_document(&document_holding(outcome, "report", report.clone())))
    .collect()
}

/// A reason of discovery that the read limit gave, which stopped it first.
fn discovery_stopped(mut report: Value) -> Value {
  let reason = json!({ "part": "discovery", "reason": { "kind": "limit_exhausted", "limit": "bytes_read" } });
  report["completeness"]["discovery"] = json!("incomplete");
  report["completeness"]["reasons"] = json!([reason]);
  let mut stopped = with(record(), "first_reasons", json!([reason]));
  stopped["exhausted"] = json!(["bytes_read"]);
  stopped["first_stopped"] = json!([{ "limit": "bytes_read", "part": "discovery" }]);
  report["completeness"]["record"] = stopped;
  report
}

/// The report with the status of its tags written by a newer version.
fn beside_a_newer_status(mut report: Value) -> Value {
  report["completeness"]["tags"] = json!("partly_skipped");
  report
}

#[test]
fn a_report_gives_ok_when_complete_and_partial_when_it_fell_short() {
  let complete = report("1.0");
  assert_eq!(outcome_of(&complete), Some(EntryOutcome::Ok));
  assert_eq!(allowed_for_report(&complete), ["ok"]);
  let short = discovery_stopped(report("1.0"));
  assert_eq!(outcome_of(&short), Some(EntryOutcome::Partial));
  assert_eq!(allowed_for_report(&short), ["partial"]);
}

#[test]
fn a_limit_that_stopped_no_part_leaves_the_entry_ok() {
  // The fifth review of the third slice's first part: every part complete, the diagnostics limit exhausted and one
  // diagnostic omitted, which reads back complete, so the entry is `ok`, which `--strict` accepts; beside it, one
  // whose limit stopped a part is `partial`.
  let mut complete = report("1.0");
  complete["completeness"]["record"]["exhausted"] = json!(["diagnostics"]);
  complete["completeness"]["record"]["omitted_diagnostics"] = json!("1");
  assert_eq!(outcome_of(&complete), Some(EntryOutcome::Ok));
  assert_eq!(allowed_for_report(&complete), ["ok"]);
  let mut short = discovery_stopped(report("1.0"));
  short["completeness"]["record"]["exhausted"] = json!(["bytes_read", "diagnostics"]);
  short["completeness"]["record"]["omitted_diagnostics"] = json!("1");
  assert_eq!(outcome_of(&short), Some(EntryOutcome::Partial));
  assert_eq!(allowed_for_report(&short), ["partial"]);
}

#[test]
fn a_known_shortfall_rules_out_ok_beside_a_newer_status() {
  // No newer status makes a report complete whose discovery this version sees cut short.
  let short = beside_a_newer_status(discovery_stopped(report("1.0")));
  assert_eq!(outcome_of(&short), Some(EntryOutcome::Partial));
  assert_eq!(allowed_for_report(&short), ["partial"]);
  // A first reason of a part that only a newer version knows says so too.
  let mut newer_part = beside_a_newer_status(report("1.0"));
  let reason = json!({ "part": "programs", "reason": { "kind": "truncated_input" } });
  newer_part["completeness"]["reasons"] = json!([reason]);
  newer_part["completeness"]["record"]["first_reasons"] = json!([reason]);
  assert_eq!(outcome_of(&newer_part), Some(EntryOutcome::Partial));
  assert_eq!(allowed_for_report(&newer_part), ["partial"]);
}

#[test]
fn a_known_header_state_that_is_not_final_is_a_shortfall_beside_a_newer_headers_status() {
  // The fourteenth review's case: a stream cut short, a headers status of a newer version, and the first reason of
  // the headers, which reading requires beside such a state.
  let mut cut = report("1.0");
  let mut short_stream = stream(0, "video");
  short_stream["header_status"] =
    json!({ "state": "not_found_within_budget", "reason": { "kind": "truncated_input" } });
  cut["streams"] = json!([short_stream]);
  cut["completeness"]["headers"] = json!("partially_final");
  let reason = json!({ "part": "headers", "stream": 0, "reason": { "kind": "truncated_input" } });
  cut["completeness"]["reasons"] = json!([reason]);
  cut["completeness"]["record"]["first_reasons"] = json!([reason]);
  assert_eq!(outcome_of(&cut), Some(EntryOutcome::Partial));
  assert_eq!(allowed_for_report(&cut), ["partial"]);
}

#[test]
fn a_newer_status_alone_leaves_ok_and_partial_open() {
  let newer = beside_a_newer_status(report("1.0"));
  assert_eq!(outcome_of(&newer), None);
  assert_eq!(allowed_for_report(&newer), ["ok", "partial"]);
  let read = || serde_json::from_value::<MediaReport>(newer.clone()).unwrap();
  let refused = DocumentEntry::from_report(input(), read()).unwrap_err();
  // An outcome that the report does not allow, and one of a newer version, give the refusal back unchanged.
  let skipped: Document = serde_json::from_value(document_holding("skipped", "report", newer.clone())).unwrap();
  let newer_outcome = skipped.results()[0].outcome().clone();
  assert!(matches!(newer_outcome, EntryOutcome::Unrecognized(_)));
  for outcome in [EntryOutcome::Error, newer_outcome] {
    assert_eq!(refused.clone().with_outcome(outcome).unwrap_err(), refused);
  }
  // An outcome that the report allows builds an entry, and its document reads back.
  for outcome in [EntryOutcome::Ok, EntryOutcome::Partial] {
    let entry = refused.clone().with_outcome(outcome.clone()).unwrap();
    assert_eq!(entry.outcome(), &outcome);
    let document = Document::from_results(Generator::new("catalog", "1"), vec![entry]);
    let read_back: Document = serde_json::from_value(serde_json::to_value(&document).unwrap()).unwrap();
    assert_eq!(read_back, document);
  }
}

/// A detection of that outcome with the members that it needs.
fn detection_of(outcome: &str) -> Value {
  let mut detection = json!({ "schema_version": "1.0", "outcome": outcome, "candidates": [], "usage": usage() });
  match outcome {
    "identified" => {
      detection["format"] = json!("matroska");
      detection["confidence"] = json!(100);
    }
    "family_only" => detection["format"] = json!("iso_base_media"),
    "ambiguous" => {
      detection["candidates"] =
        json!([{ "format": "mp4", "confidence": 60 }, { "format": "quicktime", "confidence": 60 }]);
    }
    "hint_only" => detection["hint"] = json!({ "format": "matroska", "origin": "file_name" }),
    _ => {}
  }
  detection
}

#[test]
fn a_detection_allows_the_outcome_that_it_warrants() {
  for (outcome, warranted) in [
    ("identified", "ok"),
    ("family_only", "partial"),
    ("ambiguous", "partial"),
    ("hint_only", "partial"),
    ("insufficient_data", "error"),
    ("unknown", "error"),
  ] {
    let detection: Detection = serde_json::from_value(detection_of(outcome)).unwrap();
    let entry = DocumentEntry::from_detection(input(), detection).unwrap();
    assert_eq!(entry.outcome().as_str(), warranted, "{outcome}");
    for label in ["ok", "partial", "error"] {
      let document = document_holding(label, "detection", detection_of(outcome));
      assert_eq!(reads_document(&document), label == warranted, "{outcome} as {label}");
    }
  }
  // A detection outcome of a newer version allows every outcome, and keeps the one that was written.
  for label in ["ok", "partial", "error"] {
    let document = document_holding(label, "detection", detection_of("verified"));
    let read: Document = serde_json::from_value(document).unwrap();
    assert_eq!(read.results()[0].outcome().as_str(), label);
    assert_eq!(read.results()[0].detection().unwrap().outcome().as_str(), "verified");
  }
  // An identification without its format is no detection of that outcome, so no entry holds it.
  let without_format = crate::fixtures::without(detection_of("identified"), "format");
  assert!(!reads_document(&document_holding("ok", "detection", without_format)));
}

#[test]
fn an_error_record_allows_error_alone() {
  for (label, reads) in [("error", true), ("ok", false), ("partial", false)] {
    let document = document_holding(label, "error", error_record("1.0"));
    assert_eq!(reads_document(&document), reads, "{label}");
  }
  let text = document_holding("ok", "error", error_record("1.0"));
  let message = serde_json::from_value::<Document>(text).unwrap_err().to_string();
  assert!(
    message.starts_with("an entry's outcome disagrees with the record it holds"),
    "{message}"
  );
}

#[test]
fn a_refusal_gives_its_parts_back() {
  // C-INTERMEDIATE: the refusal gives back what it was given, the record without a copy. C-GOOD-ERR: it is an error
  // that crosses threads.
  fn assert_error<Value: std::error::Error + Send + Sync + 'static>() {}
  assert_error::<DeriveOutcomeError>();
  let detection: Detection = serde_json::from_value(detection_of("verified")).unwrap();
  let refused = DocumentEntry::from_detection(input(), detection.clone()).unwrap_err();
  assert_eq!(
    refused.to_string(),
    "the record holds a value of a newer version that decides the entry's outcome, so the outcome must be given"
  );
  assert_eq!(refused.input(), &input());
  assert!(refused.report().is_none() && refused.clone().into_report().is_none());
  assert_eq!(refused.detection(), Some(&detection));
  assert_eq!(refused.clone().into_detection(), Some(detection));
  let entry = refused.with_outcome(EntryOutcome::Error).unwrap();
  assert_eq!(entry.outcome(), &EntryOutcome::Error);
  let report: MediaReport = serde_json::from_value(beside_a_newer_status(report("1.0"))).unwrap();
  let refused = DocumentEntry::from_report(input(), report.clone()).unwrap_err();
  assert_eq!(refused.report(), Some(&report));
  assert!(refused.detection().is_none() && refused.clone().into_detection().is_none());
  assert_eq!(refused.into_report(), Some(report));
}

#[test]
fn a_report_whose_reasons_contradict_its_statuses_fails_with_its_document() {
  // The twelfth review's case: a complete report that lists a reason of the tags reads under no outcome.
  let mut contradicting = report("1.0");
  contradicting["completeness"]["reasons"] = json!([{ "part": "tags", "reason": { "kind": "truncated_input" } }]);
  assert!(reads_document(&document_holding("ok", "report", report("1.0"))));
  assert_eq!(allowed_for_report(&contradicting), Vec::<&str>::new());
}

#[test]
fn a_part_that_fell_short_under_a_full_budget_gives_partial() {
  // The retained data could keep no reason, so `reasons()` is empty and only the record says that discovery fell
  // short; the entry reads the record, as `is_complete()` does.
  let mut short = report("1.0");
  short["completeness"]["discovery"] = json!("incomplete");
  let first = json!([{ "part": "discovery", "reason": { "kind": "truncated_input" } }]);
  short["completeness"]["record"] = crate::fixtures::record_omitting(first, "1");
  assert_eq!(outcome_of(&short), Some(EntryOutcome::Partial));
  assert_eq!(allowed_for_report(&short), ["partial"]);
}

#[test]
fn beside_a_header_state_of_a_newer_version_the_headers_status_is_the_writers_verdict() {
  let with_state = |state: Value, headers: &str| {
    let mut newer = report("1.0");
    newer["streams"] = json!([with(stream(0, "video"), "header_status", state)]);
    newer["completeness"]["headers"] = json!(headers);
    newer
  };
  // Only the version that wrote the state knows whether it is final, and it wrote the headers status from it.
  let finished = with_state(json!({ "state": "decoded" }), "complete");
  assert_eq!(outcome_of(&finished), Some(EntryOutcome::Ok));
  let mut short = with_state(json!({ "state": "decoded" }), "incomplete");
  let reason = json!({ "part": "headers", "stream": 0, "reason": { "kind": "truncated_input" } });
  short["completeness"]["reasons"] = json!([reason]);
  short["completeness"]["record"]["first_reasons"] = json!([reason]);
  assert_eq!(outcome_of(&short), Some(EntryOutcome::Partial));
  // A headers status of a newer version beside final states leaves the outcome open, as any newer status does.
  let open = with_state(json!({ "state": "parsed" }), "partly_read");
  assert_eq!(outcome_of(&open), None);
  assert_eq!(allowed_for_report(&open), ["ok", "partial"]);
}
