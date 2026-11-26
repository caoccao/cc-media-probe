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

//! What a report's references identify, which the second review of the third slice's first part found unchecked
//! (`json-output.md` section 10; `api.md` sections 6.2, 6.9, 6.12 and 6.15): a resolved Matroska target names the
//! entity whose unique identifier it holds; a reference owned by a sample description stands in a format with
//! sample descriptions and within the declared count; an alias names an earlier source that is no alias; and a
//! chain of carriers ends. The third review of that part read the alias of a source that was not found, and an owner
//! of a newer kind taken for a stream: an alias was opened, for a source that was opened or failed later, and an
//! owner of a newer kind is not checked.

use cc_media_probe::MediaReport;
use cc_media_probe::source::SourceEntry;
use serde_json::{Value, json};

use crate::fixtures::{primary_source, report, stream};

fn reads(report: &Value) -> bool {
  serde_json::from_value::<MediaReport>(report.clone()).is_ok()
}

/// A Matroska report of one stream whose `TrackUID` is 17, and a tag set whose one stream target resolves to it by
/// that identifier.
fn targeted(identifier: &str, track_identifier: Option<&str>) -> Value {
  let mut report = report("1.0");
  let mut track = stream(0, "video");
  if let Some(track_identifier) = track_identifier {
    track["unique_identifier"] = json!(track_identifier);
  }
  report["streams"] = json!([track]);
  report["tag_sets"] = json!([{
    "stream_targets": [{
      "kind": "resolved",
      "identifier": identifier,
      "namespace": "matroska_unique_identifier",
      "location": { "kind": "stream", "index": 0, "chapter_path": [] },
    }],
    "edition_targets": [],
    "chapter_targets": [],
    "attachment_targets": [],
    "target_scope": "entities",
    "superseded": false,
    "tags": [],
  }]);
  report
}

#[test]
fn a_resolved_matroska_target_names_the_entity_whose_unique_identifier_it_holds() {
  assert!(reads(&targeted("17", Some("17"))));
  // The review's case: the UID 42 resolved to the stream whose UID is 17.
  assert!(!reads(&targeted("42", Some("17"))));
  // A stream without a unique identifier is no entity that a UID resolves to.
  assert!(!reads(&targeted("17", None)));
  let mut edition = report("1.0");
  edition["chapter_editions"] = json!([{ "unique_identifier": "7", "origin": "matroska_edition", "chapters": [] }]);
  let resolve_edition = |identifier: &str| {
    let mut report = edition.clone();
    report["tag_sets"] = json!([{
      "stream_targets": [],
      "edition_targets": [{
        "kind": "resolved",
        "identifier": identifier,
        "namespace": "matroska_unique_identifier",
        "location": { "kind": "edition", "index": 0, "chapter_path": [] },
      }],
      "chapter_targets": [],
      "attachment_targets": [],
      "target_scope": "entities",
      "superseded": false,
      "tags": [],
    }]);
    report
  };
  assert!(reads(&resolve_edition("7")), "{}", resolve_edition("7"));
  assert!(!reads(&resolve_edition("8")));
}

/// A report of that format with one stream, whose protection declares that count of sample descriptions when one is
/// given, and a reference owned by its sample description of that index.
fn owned(format: &str, count: Option<u64>, index: u64) -> Value {
  let mut report = report("1.0");
  report["format"] = json!(format);
  let mut track = stream(0, "video");
  if let Some(count) = count {
    track["protection"] = json!({
      "declarations": [],
      "fully_examined": false,
      "has_sample_descriptions": true,
      "sample_description_count": count,
      "sample_descriptions": [],
    });
  }
  report["streams"] = json!([track]);
  report["external_references"] = json!([{
    "kind": "data_reference",
    "owner": { "kind": "stream", "index": 0, "chapter_path": [], "sample_description_index": index },
  }]);
  report
}

#[test]
fn a_reference_of_a_sample_description_names_one_that_its_stream_can_have() {
  assert!(reads(&owned("mp4", None, 2)));
  assert!(reads(&owned("mp4", Some(2), 2)));
  assert!(reads(&owned("iso_base_media", Some(1), 1)));
  assert!(reads(&owned("future_format", None, 2)));
  // The review's cases: a Matroska track has no sample descriptions, and an index beyond the declared count.
  assert!(!reads(&owned("matroska", None, 2)));
  assert!(!reads(&owned("mp4", Some(1), 2)));
}

fn companion(identifier: &str, alias_of: Option<&str>) -> Value {
  let mut entry = json!({ "identifier": identifier, "role": "subtitle_index", "outcome": "opened" });
  if let Some(alias_of) = alias_of {
    entry["alias_of"] = json!(alias_of);
  }
  entry
}

fn sourced(sources: Vec<Value>) -> Value {
  let mut report = report("1.0");
  report["completeness"]["companions"] = json!("complete");
  report["sources"] = Value::Array(sources);
  report
}

#[test]
fn an_alias_names_an_earlier_source_that_is_no_alias() {
  assert!(reads(&sourced(vec![primary_source(), companion("1", Some("0"))])));
  assert!(reads(&sourced(vec![
    primary_source(),
    companion("1", None),
    companion("2", Some("1")),
    companion("3", Some("1")),
  ])));
  for sources in [
    // The review's case: each entry the alias of the other.
    vec![primary_source(), companion("1", Some("2")), companion("2", Some("1"))],
    vec![primary_source(), companion("1", Some("2")), companion("2", None)],
    vec![primary_source(), companion("1", Some("0")), companion("2", Some("1"))],
  ] {
    let report = sourced(sources);
    assert!(!reads(&report), "{}", report["sources"]);
  }
}

#[test]
fn every_chain_of_carriers_ends() {
  let carried = |carriers: &[Option<usize>]| {
    let mut report = report("1.0");
    let streams: Vec<Value> = (0..carriers.len())
      .zip(carriers)
      .map(|(index, carrier)| {
        let mut track = stream(index, "video");
        if let Some(carrier) = carrier {
          track["embedded_in"] = json!(carrier);
        }
        track
      })
      .collect();
    report["streams"] = Value::Array(streams);
    report
  };
  assert!(reads(&carried(&[None, Some(0), Some(1)])));
  assert!(reads(&carried(&[Some(2), Some(2), None])));
  // The review's case: two streams embedded in one another.
  assert!(!reads(&carried(&[Some(1), Some(0)])));
  assert!(!reads(&carried(&[None, Some(2), Some(3), Some(1)])));
}

#[test]
fn an_alias_was_opened_again_for_a_source_that_was_open() {
  let with_outcome = |entry: Value, outcome: &str| {
    let mut entry = entry;
    entry["outcome"] = json!(outcome);
    entry
  };
  let not_found = |entry: Value| {
    let mut entry = with_outcome(entry, "not_found");
    entry["reason"] = json!({ "kind": "companion_not_found" });
    entry
  };
  let cut = |sources: Vec<Value>| {
    let mut report = sourced(sources);
    let reason = json!({ "part": "companions", "source_identifier": "1", "reason": { "kind": "companion_not_found" } });
    report["completeness"]["companions"] = json!("cut_short");
    report["completeness"]["reasons"] = json!([reason]);
    report["completeness"]["record"]["first_reasons"] = json!([reason]);
    report
  };
  let primary = |outcome: &str| with_outcome(primary_source(), outcome);
  assert!(reads(&sourced(vec![primary("failed"), companion("1", Some("0"))])));
  assert!(reads(&sourced(vec![
    primary("future_outcome"),
    companion("1", Some("0"))
  ])));
  assert!(reads(&sourced(vec![
    primary_source(),
    with_outcome(companion("1", Some("0")), "future_outcome")
  ])));
  assert!(reads(&cut(vec![
    primary_source(),
    not_found(companion("1", None)),
    companion("2", None)
  ])));
  // The review's case: an opened source 2 that is the alias of source 1, which was not found.
  let unfound = cut(vec![
    primary_source(),
    not_found(companion("1", None)),
    companion("2", Some("1")),
  ]);
  assert!(!reads(&unfound), "{unfound}");
  // An alias that was never opened.
  assert!(!reads(&cut(vec![
    primary_source(),
    not_found(companion("1", Some("0")))
  ])));
}

#[test]
fn an_owner_of_a_newer_kind_is_no_stream() {
  let newer = |format: &str, count: Option<u64>| {
    let mut report = owned(format, count, 2);
    report["external_references"][0]["owner"]["kind"] = json!("future_owner");
    report
  };
  // The review's case: an owner of a newer kind with the sample description 2, in a Matroska report and beside a
  // stream that declares one description.
  assert!(reads(&newer("matroska", None)));
  assert!(reads(&newer("mp4", Some(1))));
}

#[test]
fn a_standalone_entry_knows_alone_that_an_alias_was_opened_and_names_an_earlier_source() {
  let read = |entry: Value| serde_json::from_value::<SourceEntry>(entry).is_ok();
  assert!(read(companion("8", Some("7"))));
  // The review's case: an entry alone that was not found and named source 7 as the source that it is an alias of.
  let mut not_found = companion("1", Some("7"));
  not_found["outcome"] = json!("not_found");
  not_found["reason"] = json!({ "kind": "companion_not_found" });
  assert!(!read(not_found));
  assert!(!read(companion("1", Some("7"))));
  assert!(!read(companion("1", Some("1"))));
  let mut newer = companion("8", Some("7"));
  newer["outcome"] = json!("future_outcome");
  assert!(read(newer));
}
