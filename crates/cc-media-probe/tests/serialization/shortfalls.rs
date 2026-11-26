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

//! A shortfall that a report's own records state is recorded in its completeness, as the maintainer chose at the
//! second review of the third slice's first part (`api.md` sections 5.3, 6.9 and 13; `json-output.md` section 10),
//! which read a tag set of an incomplete scope and a companion refused by a limit beside a complete report: a tag set
//! of an incomplete scope leaves the tags incomplete; a companion's entry needs the companions complete or cut
//! short, and cut short when it was not opened; an entry's limit is exhausted; and an expected companion rules out
//! "not applicable". As the maintainer chose at its third review, which read a nameless expected companion beside
//! complete companions, an expected companion that no entry requested rules out complete companions; and a
//! shortfall that a record states beside a status of a newer version needs the part's first reason.

use cc_media_probe::MediaReport;
use serde_json::{Value, json};

use crate::fixtures::{primary_source, report};

fn reads(report: &Value) -> bool {
  serde_json::from_value::<MediaReport>(report.clone()).is_ok()
}

fn tag_set(scope: &str) -> Value {
  json!({
    "stream_targets": [],
    "edition_targets": [],
    "chapter_targets": [],
    "attachment_targets": [],
    "target_scope": scope,
    "superseded": false,
    "tags": [],
  })
}

/// The report with a part that fell short for the reason given, its status set to the status given.
fn short(mut report: Value, part: &str, status: &str, reason: &Value) -> Value {
  let part_reason = json!({ "part": part, "reason": reason });
  report["completeness"][part] = json!(status);
  report["completeness"]["reasons"] = json!([part_reason]);
  report["completeness"]["record"]["first_reasons"] = json!([part_reason]);
  report
}

#[test]
fn a_tag_set_of_an_incomplete_scope_leaves_the_tags_incomplete() {
  let truncated = json!({ "kind": "truncated_input" });
  let with_sets = |report: Value, scope: &str| {
    let mut report = report;
    report["tag_sets"] = json!([tag_set(scope)]);
    report
  };
  assert!(reads(&with_sets(report("1.0"), "whole_file")));
  assert!(reads(&with_sets(
    short(report("1.0"), "tags", "incomplete", &truncated),
    "incomplete"
  )));
  assert!(reads(&with_sets(
    short(report("1.0"), "tags", "future_status", &truncated),
    "incomplete"
  )));
  // The third review's case: an incomplete scope beside tags of a newer status and no first reason of the tags.
  assert!(!reads(&with_sets(
    with_tags(report("1.0"), "future_status"),
    "incomplete"
  )));
  // The review's case: an incomplete scope beside complete tags.
  assert!(!reads(&with_sets(report("1.0"), "incomplete")));
  assert!(!reads(&with_sets(
    with_tags(report("1.0"), "not_requested"),
    "incomplete"
  )));
}

fn with_tags(mut report: Value, status: &str) -> Value {
  report["completeness"]["tags"] = json!(status);
  report
}

fn companion(outcome: &str, reason: Option<Value>) -> Value {
  let mut entry = json!({ "identifier": "1", "role": "subtitle_index", "outcome": outcome });
  if let Some(reason) = reason {
    entry["reason"] = reason;
  }
  entry
}

/// A report whose source table holds the primary input and that companion, with its companions in that status.
fn requested(entry: Value, status: &str) -> Value {
  let mut report = report("1.0");
  report["sources"] = Value::Array(vec![primary_source(), entry]);
  report["completeness"]["companions"] = json!(status);
  report
}

#[test]
fn a_companions_entry_needs_the_companions_requested_and_one_not_opened_needs_them_cut_short() {
  let not_found = json!({ "kind": "companion_not_found" });
  let declined = json!({ "kind": "companion_declined" });
  assert!(reads(&requested(companion("opened", None), "complete")));
  assert!(reads(&requested(companion("opened", None), "future_status")));
  for (entry, reason) in [
    (companion("not_found", Some(not_found.clone())), not_found.clone()),
    (companion("declined", Some(declined.clone())), declined.clone()),
  ] {
    let mut cut = short(
      requested(entry.clone(), "cut_short"),
      "companions",
      "cut_short",
      &reason,
    );
    cut["completeness"]["reasons"][0]["source_identifier"] = json!("1");
    cut["completeness"]["record"]["first_reasons"][0]["source_identifier"] = json!("1");
    assert!(reads(&cut), "{entry}");
    assert!(!reads(&requested(entry.clone(), "complete")), "{entry}");
  }
  for status in ["not_applicable", "not_enabled"] {
    assert!(!reads(&requested(companion("opened", None), status)), "{status}");
  }
}

#[test]
fn a_companion_refused_by_a_limit_names_a_limit_that_the_record_holds_as_exhausted() {
  let limited = json!({ "kind": "companion_limited", "limit": "sources_opened" });
  let entry = companion("limit_reached", Some(limited.clone()));
  let mut cut = short(
    requested(entry.clone(), "cut_short"),
    "companions",
    "cut_short",
    &limited,
  );
  cut["completeness"]["reasons"][0]["source_identifier"] = json!("1");
  cut["completeness"]["record"]["first_reasons"][0]["source_identifier"] = json!("1");
  cut["completeness"]["record"]["exhausted"] = json!(["sources_opened"]);
  cut["completeness"]["record"]["first_stopped"] = json!([{ "limit": "sources_opened", "part": "companions" }]);
  assert!(reads(&cut));
  // The review's case: the entry beside companions that are not applicable and no exhausted limit.
  assert!(!reads(&requested(entry.clone(), "not_applicable")));
  // The entry's reason counted among the omitted reasons, with the input's end first: the limit that refused the
  // companion stopped the companions, so the record pairs it (fifth review of the third slice's first part).
  let truncated = json!({ "kind": "truncated_input" });
  let mut counted = short(
    requested(entry.clone(), "cut_short"),
    "companions",
    "cut_short",
    &truncated,
  );
  let record = &mut counted["completeness"]["record"];
  record["exhausted"] = json!(["retained_data", "sources_opened"]);
  record["first_stopped"] = json!([{ "limit": "sources_opened", "part": "companions" }]);
  record["omitted_reasons"] = json!("1");
  assert!(reads(&counted));
  counted["completeness"]["record"]["first_stopped"] = json!([]);
  assert!(!reads(&counted));
  // A cut-short report whose entry names a limit that the record does not hold as exhausted.
  let mut unexhausted = requested(entry, "cut_short");
  let declined = json!({ "part": "companions", "source_identifier": "1", "reason": { "kind": "companion_declined" } });
  unexhausted["completeness"]["reasons"] = json!([declined]);
  unexhausted["completeness"]["record"]["first_reasons"] = json!([declined]);
  assert!(!reads(&unexhausted));
}

#[test]
fn an_expected_companion_rules_out_companions_that_are_not_applicable() {
  let expected = |status: &str| {
    let mut report = report("1.0");
    report["external_references"] = json!([{ "kind": "expected_companion" }]);
    report["completeness"]["companions"] = json!(status);
    report
  };
  let lookup_off = short(
    expected("not_enabled"),
    "companions",
    "not_enabled",
    &json!({ "kind": "companions_not_enabled" }),
  );
  assert!(reads(&lookup_off), "{lookup_off}");
  assert!(!reads(&expected("not_applicable")));
}

/// An expected companion of that name, or a nameless one.
fn expected_companion(name: Option<&str>) -> Value {
  let mut reference = json!({ "kind": "expected_companion" });
  if let Some(name) = name {
    reference["identifier"] = json!("movie");
    reference["location"] = json!(name);
  }
  reference
}

#[test]
fn an_expected_companion_that_no_entry_requested_rules_out_complete_companions() {
  let expecting = |report: Value, name: Option<&str>| {
    let mut report = report;
    report["external_references"] = json!([expected_companion(name)]);
    report
  };
  let found = with_identity(companion("opened", None), "movie.sub");
  let unnamed = json!({ "kind": "companion_name_unavailable" });
  assert!(reads(&expecting(
    requested(found.clone(), "complete"),
    Some("movie.sub")
  )));
  assert!(reads(&expecting(
    short(report("1.0"), "companions", "cut_short", &unnamed),
    None
  )));
  // The review's case: a nameless expected companion, only the primary input, and complete companions.
  let mut nameless = expecting(report("1.0"), None);
  nameless["completeness"]["companions"] = json!("complete");
  assert!(!reads(&nameless), "{nameless}");
  // A companion that no entry requested, whatever entry the table holds.
  assert!(!reads(&expecting(requested(found, "complete"), Some("other.sub"))));
  // Beside companions of a newer version, the companions' first reason.
  assert!(reads(&expecting(
    short(report("1.0"), "companions", "future_status", &unnamed),
    None
  )));
  let mut newer = expecting(report("1.0"), None);
  newer["completeness"]["companions"] = json!("future_status");
  assert!(!reads(&newer));
}

/// The entry with that requested name.
fn with_identity(entry: Value, name: &str) -> Value {
  let mut entry = entry;
  entry["attempted_identity"] = json!({ "identifier": "movie", "requested_name": name });
  entry
}

#[test]
fn a_companion_not_opened_beside_companions_of_a_newer_version_needs_their_first_reason() {
  let not_found = json!({ "kind": "companion_not_found" });
  let entry = companion("not_found", Some(not_found.clone()));
  let mut kept = short(
    requested(entry.clone(), "future_status"),
    "companions",
    "future_status",
    &not_found,
  );
  kept["completeness"]["reasons"][0]["source_identifier"] = json!("1");
  kept["completeness"]["record"]["first_reasons"][0]["source_identifier"] = json!("1");
  assert!(reads(&kept), "{kept}");
  // The review's case: the entry beside a newer status and no first reason.
  assert!(!reads(&requested(entry, "future_status")));
}
