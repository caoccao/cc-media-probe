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

//! The source entries of a companion's requests and what names them (`api.md` sections 6.15, 10.1 and 13), as the
//! third review of the third slice's first part asked, which read an entry not found beside the reason of a decline
//! and a source failure that named a declined request: a reason that names a request is its entry's; the reason of
//! each entry not opened is kept, or counted among the omitted reasons; and a source failure names a source that
//! failed.

use cc_media_probe::MediaReport;
use cc_media_probe::completeness::PartReason;
use cc_media_probe::error::ErrorRecord;
use serde_json::{Value, json};

use crate::fixtures::{
  error_record, failed_primary, partial_report, primary_source, record_omitting, report, with, with_partial,
};

fn reads(report: &Value) -> bool {
  serde_json::from_value::<MediaReport>(report.clone()).is_ok()
}

fn reads_error(record: &Value) -> bool {
  serde_json::from_value::<ErrorRecord>(record.clone()).is_ok()
}

/// A companion's entry of that outcome, with a reason of that kind when one is given.
fn entry(identifier: &str, outcome: &str, kind: Option<&str>) -> Value {
  let mut entry = json!({ "identifier": identifier, "role": "subtitle_index", "outcome": outcome });
  if let Some(kind) = kind {
    entry["reason"] = json!({ "kind": kind });
  }
  entry
}

/// A reason of the companions that names that source.
fn reason(source: &str, kind: &str) -> Value {
  json!({ "part": "companions", "source_identifier": source, "reason": { "kind": kind } })
}

/// A report whose companions were cut short, with those entries beside the primary input, those kept reasons, that
/// first reason of the companions and that count of omitted reasons.
fn cut(entries: &[&Value], kept: &[&Value], first: &Value, omitted: &str) -> Value {
  let mut report = report("1.0");
  let mut sources = vec![primary_source()];
  sources.extend(entries.iter().map(|entry| (*entry).clone()));
  report["sources"] = Value::Array(sources);
  report["completeness"]["companions"] = json!("cut_short");
  report["completeness"]["reasons"] = Value::Array(kept.iter().map(|reason| (*reason).clone()).collect());
  report["completeness"]["record"] = record_omitting(json!([first]), omitted);
  report
}

#[test]
fn a_reason_that_names_a_request_is_its_entrys_reason() {
  let not_found = entry("1", "not_found", Some("companion_not_found"));
  let its_own = reason("1", "companion_not_found");
  assert!(reads(&cut(&[&not_found], &[&its_own], &its_own, "0")));
  let mut detailed = its_own.clone();
  detailed["reason"]["detail"] = json!("no such file");
  assert!(reads(&cut(&[&not_found], &[&detailed], &its_own, "0")));
  // The review's case: an entry not found beside the reason of a decline of the same source, kept and first, with
  // no reason omitted.
  let declined = reason("1", "companion_declined");
  assert!(!reads(&cut(&[&not_found], &[&declined], &declined, "0")));
  assert!(!reads(&cut(&[&not_found], &[&its_own, &declined], &its_own, "0")));
  // A reason of a request that names an entry that was opened.
  let opened = entry("1", "opened", None);
  assert!(!reads(&cut(&[&opened], &[&its_own], &its_own, "0")));
  // An outcome of a newer version is not checked.
  let newer = entry("1", "future_outcome", Some("companion_not_found"));
  assert!(reads(&cut(&[&newer], &[&declined], &declined, "0")));
}

#[test]
fn the_reason_of_each_entry_not_opened_is_kept_or_counted() {
  let not_found = entry("1", "not_found", Some("companion_not_found"));
  let declined = entry("2", "declined", Some("companion_declined"));
  let first = reason("1", "companion_not_found");
  let second = reason("2", "companion_declined");
  let entries = [&not_found, &declined];
  assert!(reads(&cut(&entries, &[&first, &second], &first, "0")));
  assert!(!reads(&cut(&entries, &[&first], &first, "0")));
  assert!(reads(&cut(&entries, &[&first], &first, "1")));
  // Neither kept: the first reason of the companions is the first entry's, counted once, and the second's once more.
  assert!(!reads(&cut(&entries, &[], &first, "1")));
  assert!(reads(&cut(&entries, &[], &first, "2")));
}

#[test]
fn a_first_reason_that_names_an_observed_request_is_its_entrys_reason() {
  let observed = |first: Value| {
    let mut record = with(
      error_record("1.0"),
      "observations",
      json!({ "diagnostics": [], "sources": [failed_primary(), entry("1", "not_found", Some("companion_not_found"))] }),
    );
    record["record"]["first_reasons"] = json!([first]);
    record
  };
  assert!(reads_error(&observed(reason("1", "companion_not_found"))));
  assert!(!reads_error(&observed(reason("1", "companion_declined"))));
}

#[test]
fn a_source_failure_names_a_source_that_failed() {
  let failing = |companion: Value| {
    let record = with(
      error_record("1.0"),
      "observations",
      json!({ "diagnostics": [], "sources": [primary_source(), companion] }),
    );
    with(record, "source_identifier", json!("1"))
  };
  assert!(reads_error(&failing(entry("1", "failed", None))));
  assert!(reads_error(&failing(entry("1", "future_outcome", None))));
  // The review's case: a source failure that names a request that a resolver declined.
  assert!(!reads_error(&failing(entry(
    "1",
    "declined",
    Some("companion_declined")
  ))));
  assert!(!reads_error(&failing(entry("1", "opened", None))));
  // The primary input in a partial report.
  assert!(reads_error(&with_partial(error_record("1.0"), partial_report("1.0"))));
  assert!(!reads_error(&with_partial(error_record("1.0"), report("1.0"))));
}

#[test]
fn a_companions_reason_stands_against_the_companions_and_names_its_request() {
  let read = |reason: Value| serde_json::from_value::<PartReason>(reason).is_ok();
  let part_reason = |part: &str, source: Option<&str>, reason: Value| {
    let mut part_reason = json!({ "part": part, "reason": reason });
    if let Some(source) = source {
      part_reason["source_identifier"] = json!(source);
    }
    part_reason
  };
  let not_found = || json!({ "kind": "companion_not_found" });
  assert!(read(part_reason("companions", Some("1"), not_found())));
  // The review's cases: a companion not found without a source, and against the tags.
  assert!(!read(part_reason("companions", None, not_found())));
  assert!(!read(part_reason("tags", Some("1"), not_found())));
  assert!(!read(part_reason(
    "companions",
    None,
    json!({ "kind": "companion_declined" })
  )));
  let limited = json!({ "kind": "companion_limited", "limit": "companion_requests" });
  assert!(read(part_reason("companions", None, limited.clone())));
  assert!(!read(part_reason("discovery", None, limited)));
  assert!(!read(part_reason(
    "discovery",
    None,
    json!({ "kind": "companions_not_enabled" })
  )));
  // A part of a newer version may hold it, and the request that it names still names its source.
  assert!(read(part_reason("future_part", Some("1"), not_found())));
  assert!(!read(part_reason("future_part", None, not_found())));
  // The review's case in a report: a companion not found beside no entry and no source.
  let lost = part_reason("companions", None, not_found());
  let mut report = report("1.0");
  report["completeness"]["companions"] = json!("cut_short");
  report["completeness"]["reasons"] = json!([lost]);
  report["completeness"]["record"]["first_reasons"] = json!([lost]);
  assert!(!reads(&report));
}

#[test]
fn an_observed_refusal_keeps_the_first_reason_of_the_companions() {
  let observed = |first_reasons: Value| {
    let mut record = with(
      error_record("1.0"),
      "observations",
      json!({ "diagnostics": [], "sources": [failed_primary(), entry("1", "not_found", Some("companion_not_found"))] }),
    );
    record["record"]["first_reasons"] = first_reasons;
    record
  };
  assert!(reads_error(&observed(json!([reason("1", "companion_not_found")]))));
  // The review's case: a companion observed not found beside no first reason at all.
  assert!(!reads_error(&observed(json!([]))));
}
