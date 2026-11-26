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

//! The rules of an error record, a completeness and a fixed-size record (`api.md` sections 5.3 and 10.2;
//! `json-output.md` sections 4.4 and 7.2). An error record holds the members of a source failure exactly when its
//! kind is one, with an operating system's code in canonical decimal of 64 bits. Every first reason that the kept
//! reasons lack or change is counted among the reasons omitted, a reason omitted needs a first reason, every known
//! status that records a shortfall has its first reason, and an omitted reference has the retained-data reason of the
//! part that found it. Every member of the record and every list and plain flag of version 1.0 is required.

use cc_media_probe::completeness::{Completeness, LimitRecord};
use cc_media_probe::error::ErrorRecord;
use cc_media_probe::model::TagSet;
use serde_json::{Value, json};

use crate::fixtures::{
  completeness as complete, error_record as source_failure, record as empty_record, record_omitting as record, with,
  without,
};

/// An error record of that kind, with the members given and without a source failure's members otherwise, and with
/// the format, the reason and the source that its kind names, when it names them.
fn error_record(kind: &str, members: &[(&str, Value)]) -> Value {
  let mut error = without(
    without(source_failure("1.0"), "source_failure_kind"),
    "source_identifier",
  );
  error["kind"] = json!(kind);
  if ["unsupported_format", "feature_disabled", "no_stream_inventory"].contains(&kind) {
    error["format"] = json!("matroska");
  }
  for (name, value) in crate::fixtures::ending_members(kind) {
    error[name] = value;
  }
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
  let code = ("operating_system_error_code", json!("2"));
  let partial = ("partial_report", crate::fixtures::partial_report("1.0"));
  let format = ("format", json!("matroska"));
  assert!(!reads_error("source_failed", &[]));
  assert!(!reads_error("source_failed", std::slice::from_ref(&code)));
  assert!(reads_error("source_failed", std::slice::from_ref(&failure)));
  assert!(reads_error(
    "source_failed",
    &[failure.clone(), code.clone(), partial.clone(), format]
  ));
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
    assert!(!reads_error(kind, std::slice::from_ref(&partial)), "{kind}");
  }
  assert!(reads_error("quota_exceeded", &[failure, code, partial]));
  assert!(reads_error("quota_exceeded", &[]));
}

#[test]
fn an_operating_system_code_is_a_64_bit_integer_in_canonical_decimal() {
  let failure = ("source_failure_kind", json!("not_found"));
  let with_code = |code: Value| {
    reads_error(
      "source_failed",
      &[failure.clone(), ("operating_system_error_code", code)],
    )
  };
  for code in [
    "2",
    "0",
    "-2147024894",
    "9223372036854775822",
    "18446744073709551615",
    "-9223372036854775808",
  ] {
    assert!(with_code(json!(code)), "{code}");
  }
  for code in [
    "02",
    "+2",
    "-0",
    "",
    "2.0",
    "0x2",
    "18446744073709551616",
    "-9223372036854775809",
  ] {
    assert!(!with_code(json!(code)), "{code}");
    assert!(
      !reads_error("quota_exceeded", &[("operating_system_error_code", json!(code))]),
      "{code}"
    );
  }
  assert!(!with_code(json!(2)), "a number");
  assert!(reads_error(
    "quota_exceeded",
    &[("operating_system_error_code", json!("-2147024894"))]
  ));
}

/// A completeness whose parts are complete but those given, with those kept reasons and that record.
fn completeness(incomplete: &[&str], reasons: Value, record: Value) -> Value {
  let mut completeness = with(with(complete(), "reasons", reasons), "record", record);
  for part in incomplete {
    completeness[*part] = json!(match *part {
      "timing" | "companions" => "cut_short",
      _ => "incomplete",
    });
  }
  completeness
}

fn reads(completeness: Value) -> bool {
  serde_json::from_value::<Completeness>(completeness).is_ok()
}

fn reason(part: &str, kind: &str) -> Value {
  json!({ "part": part, "reason": { "kind": kind } })
}

#[test]
fn a_reason_omitted_needs_a_first_reason() {
  assert!(!reads(completeness(&[], json!([]), record(json!([]), "1"))));
  assert!(reads(completeness(&[], json!([]), record(json!([]), "0"))));
}

#[test]
fn each_first_reason_that_the_kept_reasons_lack_or_change_is_omitted_on_its_own() {
  let first = json!([reason("tags", "truncated_input"), reason("chapters", "truncated_input")]);
  let parts = ["tags", "chapters"];
  let differing = json!([
    reason("tags", "range_unavailable"),
    reason("chapters", "range_unavailable")
  ]);
  assert!(!reads(completeness(
    &parts,
    differing.clone(),
    record(first.clone(), "1")
  )));
  assert!(reads(completeness(&parts, differing, record(first.clone(), "2"))));
  let one = json!([reason("chapters", "range_unavailable")]);
  assert!(!reads(completeness(&parts, one.clone(), record(first.clone(), "1"))));
  assert!(reads(completeness(&parts, one, record(first.clone(), "2"))));
  let kept = json!([reason("tags", "truncated_input"), reason("chapters", "truncated_input")]);
  assert!(reads(completeness(&parts, kept.clone(), record(first.clone(), "0"))));
  assert!(reads(completeness(&parts, kept, record(first, "1"))));
}

#[test]
fn every_known_status_that_records_a_shortfall_has_its_first_reason() {
  for (part, status) in [
    ("detection", "detection"),
    ("discovery", "discovery"),
    ("headers", "headers"),
    ("tags", "tags"),
    ("chapters", "chapters"),
    ("attachments", "attachments"),
    ("timing", "timing"),
    ("companions", "companions"),
  ] {
    assert!(
      !reads(completeness(&[status], json!([]), record(json!([]), "0"))),
      "{part}"
    );
    let first = json!([reason(part, "truncated_input")]);
    assert!(
      reads(completeness(&[status], json!([]), record(first.clone(), "1"))),
      "{part}"
    );
    assert!(!reads(completeness(&[], json!([]), record(first, "1"))), "{part}");
  }
  let mut newer = completeness(&[], json!([]), record(json!([reason("tags", "truncated_input")]), "1"));
  newer["tags"] = json!("partially_read");
  assert!(reads(newer.clone()));
  newer["record"] = empty_record();
  assert!(reads(newer));
  for (status, value) in [
    ("companions", "not_enabled"),
    ("tags", "not_requested"),
    ("timing", "not_available_by_design"),
  ] {
    let shortfall = status == "companions";
    let mut read = completeness(&[], json!([]), record(json!([]), "0"));
    read[status] = json!(value);
    assert_eq!(reads(read), !shortfall, "{status} {value}");
  }
}

#[test]
fn the_first_kept_reason_names_the_first_reasons_feature() {
  let disabled =
    |feature: &str| json!({ "part": "tags", "reason": { "kind": "feature_disabled", "feature": feature } });
  let first = json!([disabled("zlib")]);
  assert!(reads(completeness(
    &["tags"],
    json!([disabled("zlib")]),
    record(first.clone(), "0")
  )));
  let newer = json!([disabled("brotli")]);
  assert!(!reads(completeness(
    &["tags"],
    newer.clone(),
    record(first.clone(), "0")
  )));
  assert!(reads(completeness(&["tags"], newer, record(first, "1"))));
  let bare = json!([reason("tags", "feature_disabled")]);
  assert!(!reads(completeness(&["tags"], bare.clone(), record(bare, "0"))));
  let truncated = json!([{ "part": "tags", "reason": { "kind": "truncated_input", "feature": "zlib" } }]);
  assert!(!reads(completeness(
    &["tags"],
    truncated.clone(),
    record(truncated, "0")
  )));
  let limited = json!([{ "part": "companions", "reason": { "kind": "companion_limited", "limit": "sources_opened" } }]);
  let mut stopped = record(limited.clone(), "0");
  stopped["exhausted"] = json!(["sources_opened"]);
  stopped["first_stopped"] = json!([{ "limit": "sources_opened", "part": "companions" }]);
  assert!(reads(completeness(&["companions"], limited, stopped)));
  let unnamed = json!([reason("companions", "companion_limited")]);
  assert!(!reads(completeness(
    &["companions"],
    unnamed.clone(),
    record(unnamed, "0")
  )));
}

#[test]
fn a_limit_record_that_counts_an_omitted_reason_has_a_first_reason_wherever_it_is_read() {
  let record_value = record(json!([]), "1");
  assert!(serde_json::from_value::<LimitRecord>(record_value.clone()).is_err());
  assert!(!reads_error("format_not_determined", &[("record", record_value)]));
  assert!(serde_json::from_value::<LimitRecord>(record(json!([reason("tags", "truncated_input")]), "1")).is_ok());
  assert!(serde_json::from_value::<LimitRecord>(record(json!([]), "0")).is_ok());
}

/// A record that counts one omitted reference, whose retained data first stopped `stopped`, with those first reasons.
fn omitted_reference(stopped: &str, first_reasons: Value) -> Value {
  let mut record = with(empty_record(), "first_reasons", first_reasons);
  record["exhausted"] = json!(["retained_data"]);
  record["first_stopped"] = json!([{ "limit": "retained_data", "part": stopped }]);
  record["omitted_references"] = json!("1");
  record
}

fn retained(part: &str) -> Value {
  json!({ "part": part, "reason": { "kind": "limit_exhausted", "limit": "retained_data" } })
}

#[test]
fn a_limit_record_that_counts_an_omitted_reference_names_the_retained_data_and_the_part_that_found_it() {
  let bare = with(empty_record(), "omitted_references", json!("1"));
  assert!(serde_json::from_value::<LimitRecord>(bare.clone()).is_err());
  assert!(!reads_error("format_not_determined", &[("record", bare.clone())]));
  assert!(!reads(completeness(&[], json!([]), bare)));
  for part in ["discovery", "chapters", "attachments", "companions"] {
    let record = omitted_reference(part, json!([retained(part)]));
    assert!(serde_json::from_value::<LimitRecord>(record.clone()).is_ok(), "{part}");
    assert!(reads(completeness(&[part], json!([retained(part)]), record)), "{part}");
  }
  assert!(serde_json::from_value::<LimitRecord>(omitted_reference("tags", json!([retained("tags")]))).is_err());
  let later = omitted_reference("tags", json!([retained("discovery"), retained("tags")]));
  assert!(serde_json::from_value::<LimitRecord>(later).is_ok());
  let unexhausted = with(
    with(
      empty_record(),
      "first_reasons",
      json!([reason("discovery", "truncated_input")]),
    ),
    "omitted_references",
    json!("1"),
  );
  assert!(serde_json::from_value::<LimitRecord>(unexhausted).is_err());
}

#[test]
fn every_member_of_a_limit_record_is_required() {
  assert!(serde_json::from_value::<LimitRecord>(empty_record()).is_ok());
  for member in [
    "exhausted",
    "first_stopped",
    "first_reasons",
    "omitted_diagnostics",
    "omitted_declarations",
    "omitted_reasons",
    "omitted_references",
  ] {
    let error = serde_json::from_value::<LimitRecord>(without(empty_record(), member))
      .unwrap_err()
      .to_string();
    assert!(
      error.contains(&format!("missing field `{member}`")),
      "{member}: {error}"
    );
  }
  for count in [
    json!(0),
    json!("01"),
    json!("+1"),
    json!("-1"),
    json!("18446744073709551616"),
  ] {
    let malformed = with(empty_record(), "omitted_diagnostics", count.clone());
    assert!(serde_json::from_value::<LimitRecord>(malformed).is_err(), "{count}");
  }
  // Declarations are omitted only where the stream limit stopped discovery (plan section 7.5).
  let mut large = with(
    with(
      empty_record(),
      "first_reasons",
      json!([{ "part": "discovery", "reason": { "kind": "limit_exhausted", "limit": "streams" } }]),
    ),
    "omitted_declarations",
    json!("9007199254740993"),
  );
  large["exhausted"] = json!(["streams"]);
  large["first_stopped"] = json!([{ "limit": "streams", "part": "discovery" }]);
  let read: LimitRecord = serde_json::from_value(large.clone()).unwrap();
  assert_eq!(read.omitted_declarations(), 9_007_199_254_740_993);
  assert_eq!(serde_json::to_value(&read).unwrap(), large);
}

#[test]
fn the_lists_and_flags_of_version_1_0_are_required() {
  let tag_set = json!({
    "stream_targets": [],
    "edition_targets": [],
    "chapter_targets": [],
    "attachment_targets": [],
    "target_scope": "whole_file",
    "superseded": false,
    "tags": [],
  });
  assert!(serde_json::from_value::<TagSet>(tag_set.clone()).is_ok());
  for member in ["superseded", "tags", "stream_targets"] {
    let error = serde_json::from_value::<TagSet>(without(tag_set.clone(), member))
      .unwrap_err()
      .to_string();
    assert!(
      error.contains(&format!("missing field `{member}`")),
      "{member}: {error}"
    );
  }
  for member in ["candidates", "cause_messages", "usage", "record", "message"] {
    assert!(
      serde_json::from_value::<ErrorRecord>(without(source_failure("1.0"), member)).is_err(),
      "{member}"
    );
  }
}

/// A record whose retained data first stopped discovery and which omitted one reference, with that first reason and
/// that count of reasons omitted.
fn reference_omitted(first: Value, omitted_reasons: &str) -> Value {
  with(
    omitted_reference("discovery", Value::Array(vec![first])),
    "omitted_reasons",
    json!(omitted_reasons),
  )
}

#[test]
fn an_omitted_reference_has_the_retained_data_reason_of_the_part_that_found_it() {
  let truncated = reason("discovery", "truncated_input");
  let discovery = ["discovery"];
  assert!(!reads(completeness(
    &discovery,
    json!([truncated]),
    reference_omitted(truncated.clone(), "0")
  )));
  let kept = json!([truncated, retained("discovery")]);
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
  let first = retained("discovery");
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
  let mut record = omitted_reference("tags", json!([truncated, retained("tags")]));
  record["omitted_reasons"] = json!("0");
  let reasons = json!([truncated, retained("tags")]);
  assert!(!reads(completeness(&["discovery", "tags"], reasons, record)));
}
