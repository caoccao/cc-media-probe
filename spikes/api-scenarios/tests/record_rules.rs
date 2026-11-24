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

//! The sixteenth review's cases of `api.md` sections 5.3 and 10.2 and `json-output.md` sections 4.4 and 7.2. An
//! error record holds the members of a source failure exactly when its kind is one: the review read a
//! `source_failed` record without `source_failure_kind` and an `unknown_format` record with one. Every reason that
//! `reasons()` lacks is counted: the review read a completeness whose statuses were all complete, with no first
//! reason and one reason omitted, for which `is_complete()` was true, and two parts whose first kept reasons both
//! differed from their first reasons with one reason omitted. The review read incomplete attachments without any
//! reason, which the stub's check of the statuses skipped. And a first reason compares with the first kept reason in
//! its feature too, which the stub did not carry. The seventeenth review read a limit record with one reason omitted
//! and no first reason, alone and in an error record without a partial report, where the rule did not run.

use cc_media_probe::completeness::{Completeness, LimitRecord};
use cc_media_probe::details::h264::H264Details;
use cc_media_probe::details::iso_base_media::IsoBaseMediaDetails;
use cc_media_probe::error::ErrorRecord;
use cc_media_probe::model::TagSet;
use cc_media_probe::probe_bytes;
use serde_json::{Value, json};

/// An error record of that kind, with the members given and without a source failure's members otherwise.
fn error_record(kind: &str, members: &[(&str, Value)]) -> Value {
  let mut error = serde_json::to_value(probe_bytes(b"").unwrap_err().to_record()).unwrap();
  error["kind"] = json!(kind);
  for (name, value) in members {
    error[*name] = value.clone();
  }
  error
}

fn reads_error(kind: &str, members: &[(&str, Value)]) -> bool {
  serde_json::from_value::<ErrorRecord>(error_record(kind, members)).is_ok()
}

#[test]
fn a_source_failure_holds_its_kind_of_failure_and_no_other_kind_does() {
  let failure = ("source_failure_kind", json!("not_found"));
  let code = ("operating_system_error_code", json!(2));
  // The review's first case, and a source failure as the library writes one.
  assert!(!reads_error("source_failed", &[]));
  assert!(!reads_error("source_failed", std::slice::from_ref(&code)));
  assert!(reads_error("source_failed", std::slice::from_ref(&failure)));
  assert!(reads_error("source_failed", &[failure.clone(), code.clone()]));
  // The review's second case, and every other kind that this version knows.
  for kind in [
    "unknown_format",
    "format_not_determined",
    "unsupported_format",
    "feature_disabled",
    "no_stream_inventory",
  ] {
    assert!(reads_error(kind, &[]), "{kind}");
    assert!(!reads_error(kind, std::slice::from_ref(&failure)), "{kind}");
    assert!(!reads_error(kind, std::slice::from_ref(&code)), "{kind}");
  }
  // A kind of a newer version holds whatever members it needs.
  assert!(reads_error("quota_exceeded", &[failure, code]));
  assert!(reads_error("quota_exceeded", &[]));
}

/// A completeness whose parts are complete but those given, with those kept reasons and that record.
fn completeness(incomplete: &[&str], reasons: Value, record: Value) -> Value {
  let mut completeness = json!({
    "detection": "Complete",
    "discovery": "Complete",
    "headers": "Complete",
    "tags": "Complete",
    "chapters": "Complete",
    "attachments": "Complete",
    "reasons": reasons,
    "record": record,
  });
  for part in incomplete {
    completeness[*part] = json!("Incomplete");
  }
  completeness
}

fn reads(completeness: Value) -> bool {
  serde_json::from_value::<Completeness>(completeness).is_ok()
}

fn reason(part: &str, kind: &str) -> Value {
  json!({ "part": part, "reason": { "kind": kind } })
}

fn record(first_reasons: Value, omitted_reasons: &str) -> Value {
  json!({ "exhausted": [], "first_stopped": [], "first_reasons": first_reasons, "omitted_reasons": omitted_reasons, "omitted_references": "0" })
}

#[test]
fn a_reason_omitted_needs_a_first_reason() {
  // The review's first case: nothing fell short, yet one reason was omitted, and `is_complete()` was true.
  assert!(!reads(completeness(&[], json!([]), record(json!([]), "1"))));
  assert!(reads(completeness(&[], json!([]), record(json!([]), "0"))));
}

#[test]
fn each_first_reason_that_the_kept_reasons_lack_or_change_is_omitted_on_its_own() {
  let first = json!([reason("Tags", "TruncatedInput"), reason("Chapters", "TruncatedInput")]);
  let parts = ["tags", "chapters"];
  // The review's second case: both parts' first kept reasons differ, and one reason was omitted.
  let differing = json!([
    reason("Tags", "RangeUnavailable"),
    reason("Chapters", "RangeUnavailable")
  ]);
  assert!(!reads(completeness(
    &parts,
    differing.clone(),
    record(first.clone(), "1")
  )));
  assert!(reads(completeness(&parts, differing, record(first.clone(), "2"))));
  // One part without a kept reason and the other with a different one.
  let one = json!([reason("Chapters", "RangeUnavailable")]);
  assert!(!reads(completeness(&parts, one.clone(), record(first.clone(), "1"))));
  assert!(reads(completeness(&parts, one, record(first.clone(), "2"))));
  // A later reason of a part omitted beside its kept first reason counts once.
  let kept = json!([reason("Tags", "TruncatedInput"), reason("Chapters", "TruncatedInput")]);
  assert!(reads(completeness(&parts, kept.clone(), record(first.clone(), "0"))));
  assert!(reads(completeness(&parts, kept, record(first, "1"))));
}

#[test]
fn incomplete_attachments_have_their_first_reason() {
  // The review's case: incomplete attachments with no reason at all.
  assert!(!reads(completeness(
    &["attachments"],
    json!([]),
    record(json!([]), "0")
  )));
  let first = json!([reason("Attachments", "TruncatedInput")]);
  assert!(reads(completeness(
    &["attachments"],
    json!([]),
    record(first.clone(), "1")
  )));
  assert!(!reads(completeness(&[], json!([]), record(first, "1"))));
}

#[test]
fn the_first_kept_reason_names_the_first_reasons_feature() {
  let disabled = |feature: &str| json!({ "part": "Tags", "reason": { "kind": "FeatureDisabled", "feature": feature } });
  // The stub writes a value of a newer version under the name of its catch-all.
  let disabled_newer = |feature: &str| json!({ "part": "Tags", "reason": { "kind": "FeatureDisabled", "feature": { "Unrecognized": feature } } });
  let first = json!([disabled("Zlib")]);
  assert!(reads(completeness(
    &["tags"],
    json!([disabled("Zlib")]),
    record(first.clone(), "0")
  )));
  // The review's case: a feature of a newer version in the first reason, another in the first kept one.
  let newer = json!([disabled_newer("brotli")]);
  assert!(!reads(completeness(
    &["tags"],
    newer.clone(),
    record(first.clone(), "0")
  )));
  assert!(reads(completeness(&["tags"], newer, record(first, "1"))));
  // A disabled feature is named, and only by that kind.
  let bare = json!([reason("Tags", "FeatureDisabled")]);
  assert!(!reads(completeness(&["tags"], bare.clone(), record(bare, "0"))));
  let truncated = json!([{ "part": "Tags", "reason": { "kind": "TruncatedInput", "feature": "Zlib" } }]);
  assert!(!reads(completeness(
    &["tags"],
    truncated.clone(),
    record(truncated, "0")
  )));
}

#[test]
fn a_limit_record_that_counts_an_omitted_reason_has_a_first_reason_wherever_it_is_read() {
  // The seventeenth review's case: the rule ran only through `Completeness`, so the record read alone and inside an
  // error record without a partial report, although both of its members belong to the record.
  let record = json!({ "exhausted": [], "first_stopped": [], "first_reasons": [], "omitted_reasons": "1", "omitted_references": "0" });
  assert!(serde_json::from_value::<LimitRecord>(record.clone()).is_err());
  assert!(!reads_error("format_not_determined", &[("record", record)]));
  let counted = json!({
    "exhausted": [],
    "first_stopped": [],
    "first_reasons": [{ "part": "Tags", "reason": { "kind": "TruncatedInput" } }],
    "omitted_reasons": "1",
    "omitted_references": "0",
  });
  assert!(serde_json::from_value::<LimitRecord>(counted).is_ok());
  let none = json!({ "exhausted": [], "first_stopped": [], "first_reasons": [], "omitted_reasons": "0", "omitted_references": "0" });
  assert!(serde_json::from_value::<LimitRecord>(none).is_ok());
}

/// A record that counts one omitted reference, whose retained data first stopped `stopped`, with those first reasons.
fn omitted_reference(stopped: &str, first_reasons: Value) -> Value {
  json!({
    "exhausted": ["RetainedData"],
    "first_stopped": [{ "limit": "RetainedData", "part": stopped }],
    "first_reasons": first_reasons,
    "omitted_references": "1",
    "omitted_reasons": "0",
  })
}

fn retained(part: &str) -> Value {
  json!({ "part": part, "reason": { "kind": "LimitExhausted", "limit": "RetainedData" } })
}

#[test]
fn a_limit_record_that_counts_an_omitted_reference_names_the_retained_data_and_the_part_that_found_it() {
  // The eighteenth review's case: one reference omitted beside an empty record, read inside an `ok` document, where
  // `api.md` section 6.12 exhausts the retained data and leaves the part that found the reference incomplete.
  let bare = json!({ "exhausted": [], "first_stopped": [], "first_reasons": [], "omitted_references": "1", "omitted_reasons": "0" });
  assert!(serde_json::from_value::<LimitRecord>(bare.clone()).is_err());
  assert!(!reads_error("format_not_determined", &[("record", bare.clone())]));
  assert!(!reads(completeness(&[], json!([]), bare)));
  // The parts that find references: discovery, the chapters and the attachments; the tags find none.
  for (part, status) in [
    ("Discovery", "discovery"),
    ("Chapters", "chapters"),
    ("Attachments", "attachments"),
  ] {
    let record = omitted_reference(part, json!([retained(part)]));
    assert!(serde_json::from_value::<LimitRecord>(record.clone()).is_ok(), "{part}");
    assert!(
      reads(completeness(&[status], json!([retained(part)]), record)),
      "{part}"
    );
  }
  let tags = omitted_reference("Tags", json!([retained("Tags")]));
  assert!(serde_json::from_value::<LimitRecord>(tags).is_err());
  // The retained data may first have stopped another part: the part that found the reference has its first reason.
  let later = omitted_reference("Tags", json!([retained("Discovery"), retained("Tags")]));
  assert!(serde_json::from_value::<LimitRecord>(later).is_ok());
  // A first reason of such a part without the retained data exhausted is not enough.
  let unexhausted = json!({
    "exhausted": [],
    "first_stopped": [],
    "first_reasons": [reason("Discovery", "TruncatedInput")],
    "omitted_references": "1",
    "omitted_reasons": "0",
  });
  assert!(serde_json::from_value::<LimitRecord>(unexhausted).is_err());
}

#[test]
fn every_member_of_a_limit_record_is_required() {
  // The nineteenth review's case: a record of `exhausted` and `first_stopped` alone, which read and was written back
  // with an empty `first_reasons` and both counts at "0".
  let full = json!({
    "exhausted": [],
    "first_stopped": [],
    "first_reasons": [],
    "omitted_reasons": "0",
    "omitted_references": "0",
  });
  assert!(serde_json::from_value::<LimitRecord>(full.clone()).is_ok());
  for member in [
    "exhausted",
    "first_stopped",
    "first_reasons",
    "omitted_reasons",
    "omitted_references",
  ] {
    let mut short = full.clone();
    short.as_object_mut().unwrap().remove(member);
    let error = serde_json::from_value::<LimitRecord>(short).unwrap_err().to_string();
    assert!(
      error.contains(&format!("missing field `{member}`")),
      "{member}: {error}"
    );
  }
}

#[test]
fn the_lists_and_flags_of_version_1_0_are_required() {
  // The siblings found while revising the review's case: a tag set's flag and two lists of typed details that a
  // reader took as absent, and an empty list that the writer left out.
  let tag_set = json!({
    "stream_targets": [],
    "edition_targets": [],
    "chapter_targets": [],
    "attachment_targets": [],
    "target_scope": "WholeFile",
    "superseded": false,
    "tags": [],
  });
  let h264 = json!({ "sequence_parameter_sets": [], "picture_parameter_sets": [], "conflicts": [] });
  let iso = json!({ "fragment_track_identifiers": [], "tracks": [] });
  for (value, member) in [
    (&tag_set, "superseded"),
    (&h264, "picture_parameter_sets"),
    (&iso, "tracks"),
  ] {
    let mut short = value.clone();
    short.as_object_mut().unwrap().remove(member);
    let read = match member {
      "superseded" => serde_json::from_value::<TagSet>(short).map(drop),
      "picture_parameter_sets" => serde_json::from_value::<H264Details>(short).map(drop),
      _ => serde_json::from_value::<IsoBaseMediaDetails>(short).map(drop),
    };
    assert!(
      read
        .unwrap_err()
        .to_string()
        .contains(&format!("missing field `{member}`")),
      "{member}"
    );
  }
  let details = serde_json::from_value::<IsoBaseMediaDetails>(iso.clone()).unwrap();
  assert_eq!(serde_json::to_value(&details).unwrap(), iso);
  assert!(serde_json::from_value::<TagSet>(tag_set).is_ok());
  assert!(serde_json::from_value::<H264Details>(h264).is_ok());
}

/// A record whose retained data first stopped discovery and which omitted one reference, with that first reason and
/// that count of reasons omitted.
fn reference_omitted(first: Value, omitted_reasons: &str) -> Value {
  json!({
    "exhausted": ["RetainedData"],
    "first_stopped": [{ "limit": "RetainedData", "part": "Discovery" }],
    "first_reasons": [first],
    "omitted_reasons": omitted_reasons,
    "omitted_references": "1",
  })
}

#[test]
fn an_omitted_reference_has_the_retained_data_reason_of_the_part_that_found_it() {
  // The nineteenth review's case: one reference omitted, the retained data exhausted, no reason omitted, and a
  // truncation as discovery's only reason, which the record alone accepted.
  let truncated = reason("Discovery", "TruncatedInput");
  let discovery = ["discovery"];
  assert!(!reads(completeness(
    &discovery,
    json!([truncated]),
    reference_omitted(truncated.clone(), "0")
  )));
  // The reason kept after the truncation, or omitted and counted beyond the first reasons that the kept ones lack.
  let kept = json!([truncated, retained("Discovery")]);
  assert!(reads(completeness(
    &discovery,
    kept,
    reference_omitted(truncated.clone(), "0")
  )));
  assert!(reads(completeness(
    &discovery,
    json!([truncated]),
    reference_omitted(truncated.clone(), "1")
  )));
  // The reason as the part's first, kept or counted.
  let first = retained("Discovery");
  assert!(reads(completeness(
    &discovery,
    json!([first]),
    reference_omitted(first.clone(), "0")
  )));
  assert!(reads(completeness(
    &discovery,
    json!([]),
    reference_omitted(first, "1")
  )));
  // The reason of a part that finds no references does not count: the tags.
  let record = json!({
    "exhausted": ["RetainedData"],
    "first_stopped": [{ "limit": "RetainedData", "part": "Tags" }],
    "first_reasons": [truncated, retained("Tags")],
    "omitted_reasons": "0",
    "omitted_references": "1",
  });
  let reasons = json!([truncated, retained("Tags")]);
  assert!(!reads(completeness(&["discovery", "tags"], reasons, record)));
}
