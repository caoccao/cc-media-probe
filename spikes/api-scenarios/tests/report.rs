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

//! Cases of a report that `api.md` sections 5.3 and 6.4 define, read from JSON since the stub builds no report: a
//! limit that stopped one candidate's validation check while another validated; the established description of a
//! track's first sample that the probe could not keep; and a full budget, under which `reasons()` is empty while
//! the record says why each part fell short, and `is_complete()`, `Display` and the entry's outcome all agree.

use std::path::Path;

use cc_media_probe::completeness::{DetectionStatus, LimitRecord, Part};
use cc_media_probe::details::FormatDetails;
use cc_media_probe::document::{DocumentEntry, EntryOutcome};
use cc_media_probe::model::CodecInformation;
use cc_media_probe::source::PortablePath;
use cc_media_probe::{Limit, MediaReport};
use serde_json::{Value, json};

/// A report without streams, whose completeness has that detection status, those reasons and that record. The stub
/// keeps the default form of Serde's derive for the statuses, so a catch-all is written as a tagged value.
fn report(detection: Value, reasons: Value, record: Value) -> MediaReport {
  report_with(json!({
    "detection": detection,
    "discovery": "Complete",
    "headers": "Complete",
    "tags": "Complete",
    "chapters": "Complete",
    "attachments": "Complete",
    "reasons": reasons,
    "record": record,
  }))
}

/// A report without streams, with that completeness.
fn report_with(completeness: Value) -> MediaReport {
  serde_json::from_value(json!({
    "schema_version": "1.0",
    "format": "Mp4",
    "streams": [],
    "chapter_editions": [],
    "tag_sets": [],
    "attachments": [],
    "diagnostics": [],
    "completeness": completeness,
    "usage": { "bytes_read": 0, "peak_working_memory": 0 },
    "sources": [],
  }))
  .unwrap()
}

#[test]
fn a_report_whose_detection_was_cut_short_is_incomplete() {
  // The working-memory limit stopped the check of one candidate, and the other candidate validated. The record keeps
  // the first reason of detection beside the kept one, as reading requires since the twelfth review.
  let reason = json!({
    "part": "Detection",
    "reason": { "kind": "LimitExhausted", "limit": "WorkingMemory" },
  });
  let stopped = report(
    json!("Incomplete"),
    json!([reason]),
    json!({
      "exhausted": ["WorkingMemory"],
      "first_stopped": [{ "limit": "WorkingMemory", "part": "Detection" }],
      "first_reasons": [reason],
      "omitted_reasons": "0",
      "omitted_references": "0",
    }),
  );
  let completeness = stopped.completeness();
  assert_eq!(completeness.detection(), &DetectionStatus::Incomplete);
  assert!(!completeness.is_complete());
  assert_eq!(completeness.reasons()[0].part(), &Part::Detection);
  assert!(completeness.record().exhausted(&Limit::WorkingMemory));
  assert_eq!(
    completeness.record().first_stopped(&Limit::WorkingMemory),
    Some(&Part::Detection)
  );
  let input = || PortablePath::from_path(Path::new("clip.mp4"));
  let entry = DocumentEntry::from_report(input(), stopped).unwrap();
  assert_eq!(entry.outcome(), &EntryOutcome::Partial);
  let empty_record = json!({ "exhausted": [], "first_stopped": [], "first_reasons": [], "omitted_reasons": "0", "omitted_references": "0" });
  let checked = report(json!("Complete"), json!([]), empty_record.clone());
  assert!(checked.completeness().is_complete());
  assert_eq!(
    DocumentEntry::from_report(input(), checked).unwrap().outcome(),
    &EntryOutcome::Ok
  );
  // A detection status that a newer version wrote decides an outcome that this version cannot derive.
  let newer = report(json!({ "Unrecognized": "checked_by_content" }), json!([]), empty_record);
  assert!(DocumentEntry::from_report(input(), newer).is_err());
}

fn codec(candidates: Value, has_every_candidate: bool) -> CodecInformation {
  serde_json::from_value(json!({
    "candidates": candidates,
    "has_every_candidate": has_every_candidate,
    "raw_identifiers": [],
  }))
  .unwrap()
}

#[test]
fn a_known_description_that_was_not_kept_leaves_the_codec_absent() {
  // The first fragment selects description 2, which the retained-data limit kept the probe from keeping.
  let lost = codec(json!([]), false);
  assert_eq!(lost.codec(), None);
  assert_eq!(lost.name(), None);
  assert!(lost.candidates().is_empty());
  assert!(!lost.has_every_candidate());
  assert_eq!(lost.to_string(), "a description that was not read");
  let details: FormatDetails = serde_json::from_value(json!({
    "IsoBaseMedia": {
      "fragment_track_identifiers": ["1"],
      "tracks": [{ "stream_index": 0, "first_sample_description_index": 2 }],
    },
  }))
  .unwrap();
  let FormatDetails::IsoBaseMedia(iso) = details else {
    panic!("the ISO details");
  };
  assert_eq!(iso.track(0).unwrap().first_sample_description_index(), Some(2));
  // Before the selection is established, the kept description 1 is a candidate, and the details hold no index.
  let unresolved = codec(
    json!([{ "sample_description_index": 1, "codec": "H264", "raw_identifiers": [] }]),
    false,
  );
  assert_eq!(unresolved.to_string(), "H.264 or a description that was not read");
}

#[test]
fn a_full_budget_keeps_the_first_reason_of_each_part_in_the_record() {
  // The retained-data limit was full when discovery stopped, so `reasons()` holds nothing; the record still says why.
  let reason = json!({ "part": "Discovery", "reason": { "kind": "TruncatedInput" } });
  // The stub's `Limit` has three counters, and the retained data is not among them, so the record leaves the
  // exhausted counter out.
  let record = json!({
    "exhausted": [],
    "first_stopped": [],
    "first_reasons": [reason],
    "omitted_reasons": "3",
    "omitted_references": "0",
  });
  let full = report_with(json!({
    "detection": "Complete",
    "discovery": "Incomplete",
    "headers": "Complete",
    "tags": "Complete",
    "chapters": "Complete",
    "attachments": "Complete",
    "reasons": [],
    "record": record,
  }));
  let completeness = full.completeness();
  assert!(completeness.reasons().is_empty());
  let first = completeness.record().first_reason(&Part::Discovery).unwrap();
  assert_eq!(first.reason().kind().as_str(), "truncated_input");
  assert_eq!(first.reason().detail(), None, "the record keeps no detail text");
  assert_eq!(completeness.record().first_reason(&Part::Detection), None);
  assert_eq!(completeness.record().omitted_reasons(), 3);
  // A first reason is a known shortfall, so the entry is `partial`, never `ok`.
  let entry = DocumentEntry::from_report(PortablePath::from_path(Path::new("clip.mp4")), full).unwrap();
  assert_eq!(entry.outcome(), &EntryOutcome::Partial);
}

#[test]
fn a_part_that_fell_short_under_a_full_budget_is_incomplete_through_every_accessor() {
  // The seventh review's case: the tags fell short while the retained-data limit was full, so `reasons()` is empty
  // and only the record keeps the reason.
  let short = report_with(json!({
    "detection": "Complete",
    "discovery": "Complete",
    "headers": "Complete",
    "tags": "Incomplete",
    "chapters": "Complete",
    "attachments": "Complete",
    "reasons": [],
    "record": {
      "exhausted": [],
      "first_stopped": [],
      "first_reasons": [{ "part": "Tags", "reason": { "kind": "TruncatedInput" } }],
      "omitted_reasons": "1",
      "omitted_references": "0",
    },
  }));
  let completeness = short.completeness();
  assert!(completeness.reasons().is_empty());
  assert!(
    !completeness.is_complete(),
    "the boolean that `--strict` and the quick start read"
  );
  assert_eq!(completeness.to_string(), "incomplete: truncated_input");
  let entry = DocumentEntry::from_report(PortablePath::from_path(Path::new("clip.mp4")), short).unwrap();
  assert_eq!(entry.outcome(), &EntryOutcome::Partial);
}

#[test]
fn a_reason_member_that_holds_null_is_malformed() {
  // The form writes no null, so null is never read as an absent member (the maintainer's choice at the seventh
  // review).
  let read = |reason: Value| {
    serde_json::from_value::<LimitRecord>(json!({
      "exhausted": [],
      "first_stopped": [],
      "first_reasons": [reason],
      "omitted_reasons": "0",
      "omitted_references": "0",
    }))
  };
  assert!(read(json!({ "part": "Tags", "reason": { "kind": "TruncatedInput" } })).is_ok());
  for malformed in [
    json!({ "part": "Tags", "stream": null, "reason": { "kind": "TruncatedInput" } }),
    json!({ "part": "Tags", "reason": { "kind": "TruncatedInput", "detail": null } }),
  ] {
    assert!(read(malformed.clone()).is_err(), "{malformed}");
  }
}

#[test]
fn the_number_of_reasons_not_kept_is_a_decimal_string() {
  // Like the other two omitted counts, so a reader that holds numbers as binary64 loses no count above 2^53 - 1. A
  // count above 0 has a first reason to belong to (seventeenth review).
  let record = |omitted: Value| {
    serde_json::from_value::<LimitRecord>(json!({
      "exhausted": [],
      "first_stopped": [],
      "first_reasons": [{ "part": "Tags", "reason": { "kind": "TruncatedInput" } }],
      "omitted_reasons": omitted,
      "omitted_references": "0",
    }))
  };
  let large = record(json!("9007199254740993")).unwrap();
  assert_eq!(large.omitted_reasons(), 9_007_199_254_740_993);
  assert_eq!(
    serde_json::to_value(&large).unwrap()["omitted_reasons"],
    json!("9007199254740993")
  );
  assert!(record(json!(3)).is_err(), "a number is not the form");
}

/// A report whose one stream has that header status, with that headers status and complete parts otherwise. A
/// headers status of `Incomplete` has the first reason of the headers in the record, as reading requires since the
/// twelfth review, which a full budget kept out of the reasons and the record counts (fifteenth review); no other
/// part has a reason.
fn report_with_header_state(header_status: Value, headers: Value) -> Result<MediaReport, serde_json::Error> {
  let empty_record = json!({ "exhausted": [], "first_stopped": [], "first_reasons": [], "omitted_reasons": "0", "omitted_references": "0" });
  let mut text = serde_json::to_value(report(json!("Complete"), json!([]), empty_record)).unwrap();
  if headers == json!("Incomplete") {
    text["completeness"]["record"]["first_reasons"] = json!([{
      "part": "Headers",
      "stream": 0,
      "reason": { "kind": "TruncatedInput" },
    }]);
    text["completeness"]["record"]["omitted_reasons"] = json!("1");
  }
  text["completeness"]["headers"] = headers;
  text["streams"] = json!([{
    "index": 0,
    "kind": "Video",
    "codec": { "candidates": [], "has_every_candidate": true, "raw_identifiers": [] },
    "header_status": header_status,
    "titles": [],
    "disposition": {},
  }]);
  serde_json::from_value(text)
}

fn outcome(report: MediaReport) -> Option<EntryOutcome> {
  DocumentEntry::from_report(PortablePath::from_path(Path::new("clip.mp4")), report)
    .ok()
    .map(|entry| entry.outcome().clone())
}

#[test]
fn the_headers_status_carries_the_header_states_into_completeness() {
  // The eighth review's case: a header state that is not final is a shortfall, which `is_complete()` sees through
  // the headers status of the record, since `Completeness` holds no stream.
  let cut = json!({ "state": "NotFoundWithinBudget", "reason": { "kind": "TruncatedInput" } });
  let short = report_with_header_state(cut.clone(), json!("Incomplete")).unwrap();
  assert!(!short.completeness().is_complete());
  assert_eq!(short.completeness().to_string(), "incomplete: truncated_input");
  assert_eq!(outcome(short), Some(EntryOutcome::Partial));
  let parsed = json!({ "state": "Parsed" });
  assert_eq!(
    outcome(report_with_header_state(parsed.clone(), json!("Complete")).unwrap()),
    Some(EntryOutcome::Ok)
  );
  // A status that contradicts a state that this version knows is malformed.
  assert!(report_with_header_state(cut, json!("Complete")).is_err());
  assert!(report_with_header_state(parsed, json!("Incomplete")).is_err());
}

#[test]
fn beside_a_header_state_of_a_newer_version_the_headers_status_is_the_writers_verdict() {
  // Only the version that wrote the state knows whether it is final, and it wrote the headers status from it.
  let newer = json!({ "state": { "Unrecognized": "deferred" } });
  let short = report_with_header_state(newer.clone(), json!("Incomplete")).unwrap();
  assert!(!short.completeness().is_complete());
  assert_eq!(outcome(short), Some(EntryOutcome::Partial));
  let finished = report_with_header_state(newer, json!("Complete")).unwrap();
  assert!(finished.completeness().is_complete());
  assert_eq!(outcome(finished), Some(EntryOutcome::Ok));
  // A headers status of a newer version leaves the outcome open, as any newer status does.
  let unknown =
    report_with_header_state(json!({ "state": "Parsed" }), json!({ "Unrecognized": "partly_read" })).unwrap();
  assert!(!unknown.completeness().is_complete());
  assert_eq!(outcome(unknown), None);
}

#[test]
fn a_codec_that_the_input_never_named_has_a_display_of_its_own() {
  // The tenth review's case: a Matroska track entry cut short before its `CodecID` has no codec, no raw identifier
  // and no sample descriptions, so it has every candidate it needs and none to show.
  let unnamed = codec(json!([]), true);
  assert_eq!(unnamed.codec(), None);
  assert!(unnamed.raw_identifiers().is_empty());
  assert_eq!(unnamed.to_string(), "a codec that was not read");
}

#[test]
fn a_codec_of_a_newer_version_displays_the_name_that_it_wrote() {
  // The twelfth review's case, as the maintainer chose: a newer version identified the codec from its content, so it
  // has no raw identifier, and "a codec that was not read" stays for an absent codec.
  let read = |codec: Value, raw_identifiers: Value| -> CodecInformation {
    serde_json::from_value(json!({
      "codec": codec,
      "candidates": [],
      "has_every_candidate": true,
      "raw_identifiers": raw_identifiers,
    }))
    .unwrap()
  };
  let newer = json!({ "Unrecognized": "future_codec" });
  assert_eq!(read(newer.clone(), json!([])).to_string(), "future_codec");
  // The name that the newer version wrote comes before the identifier that the file wrote.
  // A four-character code is its bytes in hexadecimal, as the maintainer chose at the thirteenth review: `fut1`.
  let raw = json!([{ "namespace": "IsoSampleEntry", "value": "66757431" }]);
  assert_eq!(read(newer.clone(), raw.clone()).to_string(), "future_codec");
  assert_eq!(read(json!("Unknown"), raw).to_string(), "66757431");
  assert_eq!(read(json!("Unknown"), json!([])).to_string(), "an unknown codec");
  // A candidate is written the same way.
  let candidate = codec(
    json!([{ "sample_description_index": 1, "codec": newer, "raw_identifiers": [] }]),
    false,
  );
  assert_eq!(candidate.to_string(), "future_codec or a description that was not read");
}

#[test]
fn an_iso_description_cut_off_before_its_format_code_is_a_description_that_was_not_read() {
  // The eleventh review's case: the first sample selects description 1, cut off before its format code. A track with
  // sample descriptions is never "a codec that the input never named", so a candidate is missing.
  let cut = codec(json!([]), false);
  assert_eq!(cut.codec(), None);
  assert!(cut.raw_identifiers().is_empty());
  assert!(!cut.has_every_candidate());
  assert_eq!(cut.to_string(), "a description that was not read");
  let details: FormatDetails = serde_json::from_value(json!({
    "IsoBaseMedia": {
      "fragment_track_identifiers": [],
      "tracks": [{ "stream_index": 0, "first_sample_description_index": 1 }],
    },
  }))
  .unwrap();
  let FormatDetails::IsoBaseMedia(iso) = details else {
    panic!("the ISO details");
  };
  assert_eq!(iso.track(0).unwrap().first_sample_description_index(), Some(1));
}

#[test]
fn a_companion_diagnostic_names_its_source_in_decimal() {
  // The tenth review's case: the CLI prints `source 1`, which needs the identifier's number, and `Debug` is no such
  // text.
  let mut text = serde_json::to_value(report(
    json!("Complete"),
    json!([]),
    json!({ "exhausted": [], "first_stopped": [], "first_reasons": [], "omitted_reasons": "0", "omitted_references": "0" }),
  ))
  .unwrap();
  text["diagnostics"] = json!([{
    "severity": "Warning",
    "code": "vobsub.index_disagrees",
    "message": "the data file is shorter than its index says",
    "source_identifier": "1",
    "offset": 4096,
    "stream": 0,
  }]);
  let read: MediaReport = serde_json::from_value(text).unwrap();
  let diagnostic = &read.diagnostics()[0];
  let source = diagnostic.source_identifier();
  assert_eq!(source.value(), 1);
  assert_eq!(source.to_string(), "1");
  assert_eq!(format!("{source:?}"), "SourceIdentifier(1)");
  let location = format!(
    "(stream {}, source {source}, byte {})",
    diagnostic.stream().unwrap(),
    diagnostic.offset().unwrap()
  );
  assert_eq!(location, "(stream 0, source 1, byte 4096)");
}

#[test]
fn the_number_of_references_not_kept_is_a_decimal_string() {
  // The eighth review's case: an external reference found after the retained data was full is counted, not listed.
  // Since the eighteenth review the record says that the retained data ran out and which part fell short for it.
  let record: LimitRecord = serde_json::from_value(json!({
    "exhausted": ["RetainedData"],
    "first_stopped": [{ "limit": "RetainedData", "part": "Discovery" }],
    "first_reasons": [{ "part": "Discovery", "reason": { "kind": "LimitExhausted", "limit": "RetainedData" } }],
    "omitted_references": "9007199254740993",
    "omitted_reasons": "0",
  }))
  .unwrap();
  assert_eq!(record.omitted_references(), 9_007_199_254_740_993);
  assert_eq!(
    serde_json::to_value(&record).unwrap()["omitted_references"],
    json!("9007199254740993")
  );
}
