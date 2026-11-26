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

//! The rules that a record checks on its own members, read alone, after the review of the third slice's first part
//! (`api.md` sections 5.4, 6.3, 6.4, 6.9, 6.10, 6.15 and 10.2; `json-output.md` section 10): a source entry's
//! identifier and reason agree with its role and outcome; a sample description's entry counts from 1 and is never
//! unknown, and a protection examined in full declares something; a header state holds the identification and the
//! reason that its state allows; an error of a known kind has a format exactly when its kind does, and at most 32
//! causes; codec candidates rise from 1 and decide the codec; a tag target's identifier, kind and location agree, and
//! so do a tag set's scope and its targets; an attachment's data have its size, and a linked picture has neither;
//! and a raw path of a known encoding is no valid Unicode.

use cc_media_probe::completeness::HeaderStatus;
use cc_media_probe::error::ErrorRecord;
use cc_media_probe::model::{
  Attachment, CodecCandidate, CodecInformation, Protection, SampleDescriptionProtection, TagSet, TargetReference,
};
use cc_media_probe::source::{RawPath, SourceEntry};
use serde::de::DeserializeOwned;
use serde_json::{Value, json};

use crate::fixtures::{error_record, with, without};

fn reads<Type: DeserializeOwned>(value: &Value) -> bool {
  serde_json::from_value::<Type>(value.clone()).is_ok()
}

fn source(identifier: &str, role: &str, outcome: &str, reason: Option<&str>) -> Value {
  let mut entry = json!({ "identifier": identifier, "role": role, "outcome": outcome });
  if let Some(kind) = reason {
    entry["reason"] = json!({ "kind": kind });
    if kind == "companion_limited" {
      entry["reason"]["limit"] = json!("companion_requests");
    }
  }
  entry
}

#[test]
fn a_source_entrys_identifier_and_reason_agree_with_its_role_and_outcome() {
  for entry in [
    source("0", "primary", "opened", None),
    source("0", "primary", "failed", None),
    source("1", "subtitle_index", "opened", None),
    source("1", "subtitle_index", "not_found", Some("companion_not_found")),
    source("2", "subtitle_index", "declined", Some("companion_declined")),
    source("3", "subtitle_index", "limit_reached", Some("companion_limited")),
    source("3", "subtitle_index", "not_found", Some("future_reason")),
    source("4", "subtitle_index", "future_outcome", Some("companion_declined")),
  ] {
    assert!(reads::<SourceEntry>(&entry), "{entry}");
  }
  for entry in [
    source("4", "primary", "opened", None),
    source("0", "subtitle_index", "opened", None),
    source("0", "primary", "opened", Some("companion_declined")),
    source("0", "primary", "not_found", Some("companion_not_found")),
    source("1", "subtitle_index", "opened", Some("companion_declined")),
    source("1", "subtitle_index", "failed", Some("truncated_input")),
    source("1", "subtitle_index", "not_found", None),
    source("1", "subtitle_index", "not_found", Some("companion_declined")),
    source("2", "subtitle_index", "declined", Some("companion_limited")),
    source("3", "subtitle_index", "limit_reached", Some("companion_not_found")),
    with(source("2", "subtitle_index", "opened", None), "alias_of", json!("2")),
  ] {
    assert!(!reads::<SourceEntry>(&entry), "{entry}");
  }
}

#[test]
fn a_sample_description_counts_from_1_and_a_protection_examined_in_full_declares_something() {
  assert!(reads::<SampleDescriptionProtection>(
    &json!({ "index": 1, "state": "clear" })
  ));
  assert!(reads::<SampleDescriptionProtection>(
    &json!({ "index": 2, "state": "future_state" })
  ));
  for entry in [
    json!({ "index": 0, "state": "clear" }),
    json!({ "index": 1, "state": "unknown" }),
  ] {
    assert!(!reads::<SampleDescriptionProtection>(&entry), "{entry}");
  }
  let protection = |fully_examined: bool, declarations: Value| {
    json!({
      "declarations": declarations,
      "fully_examined": fully_examined,
      "has_sample_descriptions": false,
      "sample_descriptions": [],
    })
  };
  let packets = json!([{ "scope": "packets" }]);
  assert!(reads::<Protection>(&protection(false, json!([]))));
  assert!(reads::<Protection>(&protection(true, packets.clone())));
  assert!(reads::<Protection>(&protection(false, packets)));
  assert!(!reads::<Protection>(&protection(true, json!([]))));
}

fn header(state: &str, identification: Option<&str>, reason: Option<Value>) -> Value {
  let mut status = json!({ "state": state });
  if let Some(identification) = identification {
    status["identification"] = json!(identification);
  }
  if let Some(reason) = reason {
    status["reason"] = reason;
  }
  status
}

fn reason(kind: &str) -> Value {
  match kind {
    "limit_exhausted" => json!({ "kind": kind, "limit": "header_bytes_per_stream" }),
    "feature_disabled" => json!({ "kind": kind, "feature": "zlib" }),
    _ => json!({ "kind": kind }),
  }
}

#[test]
fn a_header_state_holds_the_identification_and_the_reason_that_it_allows() {
  for status in [
    header("parsed", None, None),
    header("partially_parsed_by_design", None, None),
    header("protected", None, None),
    header("identified", Some("unknown_codec"), None),
    header("identified", Some("future_identification"), None),
    header("not_found_within_budget", None, Some(reason("truncated_input"))),
    header("not_found_within_budget", None, Some(reason("limit_exhausted"))),
    header(
      "partially_parsed_for_lack_of_bytes",
      None,
      Some(reason("range_unavailable")),
    ),
    header(
      "partially_parsed_for_lack_of_bytes",
      None,
      Some(reason("source_failed")),
    ),
    header(
      "partially_parsed_for_lack_of_bytes",
      None,
      Some(reason("future_reason")),
    ),
    header("malformed", None, Some(reason("damaged_structure"))),
    header("compression_unavailable", None, None),
    header("compression_unavailable", None, Some(reason("feature_disabled"))),
    header("not_attempted", None, None),
    header("not_attempted", None, Some(reason("source_failed"))),
    header("future_state", Some("unknown_codec"), Some(reason("truncated_input"))),
  ] {
    assert!(reads::<HeaderStatus>(&status), "{status}");
  }
  for status in [
    header("parsed", Some("unknown_codec"), Some(reason("truncated_input"))),
    header("parsed", Some("unknown_codec"), None),
    header("parsed", None, Some(reason("truncated_input"))),
    header("protected", None, Some(reason("source_failed"))),
    header("identified", None, None),
    header("identified", Some("pending_parser"), Some(reason("truncated_input"))),
    header("not_found_within_budget", None, None),
    header("not_found_within_budget", None, Some(reason("damaged_structure"))),
    header(
      "not_found_within_budget",
      Some("unknown_codec"),
      Some(reason("truncated_input")),
    ),
    header(
      "partially_parsed_for_lack_of_bytes",
      None,
      Some(reason("companion_not_found")),
    ),
    header("malformed", None, None),
    header("malformed", None, Some(reason("truncated_input"))),
    header("compression_unavailable", None, Some(reason("truncated_input"))),
    header("not_attempted", None, Some(reason("limit_exhausted"))),
  ] {
    assert!(!reads::<HeaderStatus>(&status), "{status}");
  }
  let newer_feature = json!({ "kind": "feature_disabled", "feature": "future_feature" });
  assert!(reads::<HeaderStatus>(&header(
    "compression_unavailable",
    None,
    Some(newer_feature)
  )));
}

fn known_error(kind: &str, format: Option<&str>) -> Value {
  let mut record = without(without(error_record("1.0"), "source_failure_kind"), "source_identifier");
  record["kind"] = json!(kind);
  record["message"] = json!("message");
  if let Some(format) = format {
    record["format"] = json!(format);
  }
  for (name, value) in crate::fixtures::ending_members(kind) {
    record[name] = value;
  }
  record
}

#[test]
fn an_error_of_a_known_kind_has_a_format_exactly_when_its_kind_does() {
  for record in [
    known_error("unsupported_format", Some("matroska")),
    known_error("feature_disabled", Some("matroska")),
    known_error("no_stream_inventory", Some("matroska")),
    known_error("unknown_format", None),
    known_error("format_not_determined", None),
    error_record("1.0"),
    with(error_record("1.0"), "format", json!("matroska")),
    known_error("future_kind", Some("matroska")),
    known_error("future_kind", None),
    with(
      known_error("unknown_format", None),
      "hint",
      json!({ "format": "matroska", "origin": "format_option" }),
    ),
  ] {
    assert!(reads::<ErrorRecord>(&record), "{record}");
  }
  let outranked = json!([{ "format": "matroska", "confidence": 50, "state": "outranked" }]);
  for record in [
    known_error("unsupported_format", None),
    known_error("feature_disabled", None),
    known_error("no_stream_inventory", None),
    known_error("unknown_format", Some("matroska")),
    known_error("format_not_determined", Some("matroska")),
    with(
      known_error("format_not_determined", None),
      "hint",
      json!({ "format": "matroska", "origin": "format_option" }),
    ),
    with(known_error("unknown_format", None), "candidates", outranked.clone()),
    with(known_error("format_not_determined", None), "candidates", outranked),
  ] {
    assert!(!reads::<ErrorRecord>(&record), "{record}");
  }
}

#[test]
fn an_error_record_follows_at_most_32_causes() {
  let causes = |count: usize| json!((0..count).map(|cause| format!("cause {cause}")).collect::<Vec<_>>());
  assert!(reads::<ErrorRecord>(&with(
    error_record("1.0"),
    "cause_messages",
    causes(32)
  )));
  assert!(!reads::<ErrorRecord>(&with(
    error_record("1.0"),
    "cause_messages",
    causes(33)
  )));
}

fn candidate(index: u64, codec: &str, code: &str) -> Value {
  json!({
    "sample_description_index": index,
    "codec": codec,
    "raw_identifiers": [{ "namespace": "iso_sample_entry", "value": code }],
  })
}

/// The information of that codec and those candidates, whose raw identifiers are the candidates', in their order.
fn codec(codec: Option<&str>, candidates: Vec<Value>, has_every_candidate: bool) -> Value {
  let identifiers: Vec<Value> = candidates
    .iter()
    .flat_map(|candidate| candidate["raw_identifiers"].as_array().cloned().unwrap_or_default())
    .collect();
  let mut information = json!({ "has_every_candidate": has_every_candidate, "raw_identifiers": identifiers });
  information["candidates"] = Value::Array(candidates);
  if let Some(codec) = codec {
    information["codec"] = json!(codec);
  }
  information
}

#[test]
fn codec_candidates_rise_from_1_and_decide_the_codec() {
  assert!(reads::<CodecCandidate>(&candidate(1, "h264", "61766331")));
  assert!(!reads::<CodecCandidate>(&candidate(0, "h264", "61766331")));
  let avc = || vec![candidate(1, "h264", "61766331"), candidate(2, "h264", "61766333")];
  let unknown = |second: &str| vec![candidate(1, "unknown", "66757431"), candidate(2, "unknown", second)];
  let mixed = vec![candidate(1, "h264", "61766331"), candidate(2, "opus", "4f707573")];
  for information in [
    codec(Some("h264"), avc(), true),
    codec(None, avc(), false),
    codec(None, mixed.clone(), true),
    codec(Some("unknown"), unknown("66757431"), true),
    codec(None, unknown("66757432"), true),
    codec(Some("h264"), Vec::new(), true),
    codec(None, Vec::new(), true),
    codec(None, Vec::new(), false),
    with(
      codec(Some("h264"), Vec::new(), true),
      "codec_string",
      json!("avc1.64001f"),
    ),
  ] {
    assert!(reads::<CodecInformation>(&information), "{information}");
  }
  for information in [
    codec(Some("opus"), avc(), true),
    codec(None, avc(), true),
    codec(Some("h264"), avc(), false),
    codec(Some("h264"), mixed, true),
    codec(Some("unknown"), unknown("66757432"), true),
    codec(Some("h264"), avc().into_iter().rev().collect(), true),
    codec(
      Some("h264"),
      vec![candidate(1, "h264", "61766331"), candidate(1, "h264", "61766331")],
      true,
    ),
    codec(Some("h264"), Vec::new(), false),
    with(codec(None, Vec::new(), true), "codec_string", json!("avc1.64001f")),
    with(
      with(codec(None, Vec::new(), true), "profile", json!({ "name": "High" })),
      "profile_provenance",
      json!("bitstream"),
    ),
  ] {
    assert!(!reads::<CodecInformation>(&information), "{information}");
  }
}

fn target(kind: &str, identifier: Option<&str>, location: Option<Value>) -> Value {
  let mut reference = json!({ "kind": kind });
  if let Some(identifier) = identifier {
    reference["identifier"] = json!(identifier);
    reference["namespace"] = json!("matroska_unique_identifier");
  }
  if let Some(location) = location {
    reference["location"] = location;
  }
  reference
}

fn stream_location(index: usize) -> Value {
  json!({ "kind": "stream", "index": index, "chapter_path": [] })
}

fn tag_set(scope: &str, stream_targets: Vec<Value>, chapter_targets: Vec<Value>) -> Value {
  let mut set = json!({
    "edition_targets": [],
    "attachment_targets": [],
    "target_scope": scope,
    "superseded": false,
    "tags": [],
  });
  set["stream_targets"] = Value::Array(stream_targets);
  set["chapter_targets"] = Value::Array(chapter_targets);
  set
}

#[test]
fn a_tag_targets_identifier_kind_and_location_agree_and_so_do_a_sets_scope_and_targets() {
  for reference in [
    target("resolved", Some("1"), Some(stream_location(0))),
    target("unmatched", Some("18446744073709551615"), None),
    target("wildcard", Some("0"), None),
    target("wildcard", None, None),
    target("future_kind", Some("1"), Some(stream_location(0))),
    target("future_kind", Some("0"), None),
  ] {
    assert!(reads::<TargetReference>(&reference), "{reference}");
  }
  for reference in [
    // The review's case: the namespace's spelling holds beside a kind of a newer version too.
    target("future_kind", Some("01"), Some(stream_location(0))),
    target("resolved", Some("01"), Some(stream_location(0))),
    target("unmatched", Some("+1"), None),
    target("unmatched", Some("18446744073709551616"), None),
    target("resolved", Some("1"), None),
    target("unmatched", Some("1"), Some(stream_location(0))),
    target("wildcard", Some("0"), Some(stream_location(0))),
    target("wildcard", Some("1"), None),
    target("resolved", Some("0"), Some(stream_location(0))),
    target("unmatched", Some("0"), None),
    with(
      target("unmatched", None, None),
      "namespace",
      json!("matroska_unique_identifier"),
    ),
  ] {
    assert!(!reads::<TargetReference>(&reference), "{reference}");
  }
  let resolved = || target("resolved", Some("1"), Some(stream_location(0)));
  for set in [
    tag_set("whole_file", Vec::new(), Vec::new()),
    tag_set("entities", vec![resolved()], Vec::new()),
    tag_set("entities", vec![target("wildcard", Some("0"), None)], Vec::new()),
    tag_set("incomplete", Vec::new(), Vec::new()),
    tag_set("incomplete", vec![resolved()], Vec::new()),
    tag_set("future_scope", vec![resolved()], Vec::new()),
  ] {
    assert!(reads::<TagSet>(&set), "{set}");
  }
  let chapter = target(
    "resolved",
    Some("2"),
    Some(json!({ "kind": "chapter", "index": 0, "chapter_path": [0] })),
  );
  for set in [
    tag_set("whole_file", vec![resolved()], Vec::new()),
    tag_set("whole_file", Vec::new(), vec![target("unmatched", Some("1"), None)]),
    tag_set("entities", Vec::new(), Vec::new()),
    tag_set("entities", vec![chapter.clone()], Vec::new()),
    tag_set("entities", Vec::new(), vec![resolved()]),
  ] {
    assert!(!reads::<TagSet>(&set), "{set}");
  }
  assert!(reads::<TagSet>(&tag_set("entities", Vec::new(), vec![chapter])));
}

#[test]
fn an_attachments_data_have_its_size_and_a_linked_picture_has_neither() {
  for attachment in [
    json!({ "size": 2, "data": "abcd" }),
    json!({ "size": 0, "data": "" }),
    json!({ "size": 2 }),
    json!({ "link": "cover.jpg" }),
    json!({}),
  ] {
    assert!(reads::<Attachment>(&attachment), "{attachment}");
  }
  for attachment in [
    json!({ "size": 1, "data": "abcd" }),
    json!({ "size": 3, "data": "abcd" }),
    json!({ "link": "cover.jpg", "size": 2 }),
    json!({ "link": "cover.jpg", "data": "abcd" }),
    json!({ "link": "cover.jpg", "size": 2, "data": "abcd" }),
  ] {
    assert!(!reads::<Attachment>(&attachment), "{attachment}");
  }
}

#[test]
fn a_raw_path_of_a_known_encoding_is_no_valid_unicode() {
  assert!(reads::<RawPath>(
    &json!({ "encoding": "unix_bytes", "data": "ff2e62696e" })
  ));
  assert!(reads::<RawPath>(&json!({ "encoding": "windows_wide", "data": "d800" })));
  assert!(reads::<RawPath>(
    &json!({ "encoding": "future_encoding", "data": "61" })
  ));
  for path in [
    json!({ "encoding": "unix_bytes", "data": "61" }),
    json!({ "encoding": "unix_bytes", "data": "" }),
    json!({ "encoding": "windows_wide", "data": "0061" }),
  ] {
    assert!(!reads::<RawPath>(&path), "{path}");
  }
}
