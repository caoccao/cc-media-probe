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

//! The record's omitted counts and the limits that explain them (plan section 7.5; `api.md` section 5.3), as the
//! fourth review of the third slice's first part asked, which read a declaration omitted beside complete discovery
//! and no exhausted limit: a declaration is omitted only where the stream limit or the retained data stopped
//! discovery, a diagnostic only where the diagnostics limit or the retained data kept it, and a reason only where the
//! retained data kept it; and a report keeps, gives first or counts the reason of discovery that names the limit of
//! an omitted declaration, as the maintainer chose. A limit that a reason of detection names stopped detection first.

use cc_media_probe::MediaReport;
use cc_media_probe::completeness::{Completeness, LimitRecord};
use serde_json::{Value, json};

use crate::fixtures::{completeness, record, report, with};

fn reads_record(record: &Value) -> bool {
  serde_json::from_value::<LimitRecord>(record.clone()).is_ok()
}

fn reads_completeness(completeness: &Value) -> bool {
  serde_json::from_value::<Completeness>(completeness.clone()).is_ok()
}

/// A reason of that part that names that exhausted limit.
fn exhausted(part: &str, limit: &str) -> Value {
  json!({ "part": part, "reason": { "kind": "limit_exhausted", "limit": limit } })
}

/// A record with those first reasons, whose limits each stopped the part given first, and that omitted count.
fn stopped(first_reasons: Value, limits: &[(&str, &str)], count: (&str, &str)) -> Value {
  let mut record = with(record(), "first_reasons", first_reasons);
  record["exhausted"] = limits.iter().map(|(limit, _)| json!(limit)).collect();
  record["first_stopped"] = limits
    .iter()
    .map(|(limit, part)| json!({ "limit": limit, "part": part }))
    .collect();
  record[count.0] = json!(count.1);
  record
}

#[test]
fn an_omitted_declaration_needs_discovery_stopped_by_the_stream_limit_or_the_retained_data() {
  let declarations = ("omitted_declarations", "1");
  // The review's case: a declaration omitted beside complete discovery and no exhausted limit.
  assert!(!reads_record(&stopped(json!([]), &[], declarations)));
  for limit in ["streams", "retained_data"] {
    let record = stopped(
      json!([exhausted("discovery", limit)]),
      &[(limit, "discovery")],
      declarations,
    );
    assert!(reads_record(&record), "{limit}");
  }
  let truncated = json!([{ "part": "discovery", "reason": { "kind": "truncated_input" } }]);
  assert!(!reads_record(&stopped(truncated.clone(), &[], declarations)));
  let read = stopped(
    json!([exhausted("discovery", "bytes_read")]),
    &[("bytes_read", "discovery")],
    declarations,
  );
  assert!(!reads_record(&read));
  let elsewhere = stopped(
    json!([exhausted("tags", "streams")]),
    &[("streams", "tags")],
    declarations,
  );
  assert!(!reads_record(&elsewhere));
}

#[test]
fn an_omitted_diagnostic_or_reason_needs_the_limit_that_kept_it() {
  let first = || json!([exhausted("discovery", "streams")]);
  let alone = [("streams", "discovery")];
  let retained = [("retained_data", "discovery"), ("streams", "discovery")];
  // The diagnostics limit stops no part, so it has no pair, and it is exhausted beside a diagnostic that it kept.
  let diagnostics = |count: (&str, &str)| {
    let mut record = stopped(
      first(),
      &[("streams", "discovery"), ("diagnostics", "discovery")],
      count,
    );
    record["first_stopped"] = json!([{ "limit": "streams", "part": "discovery" }]);
    record["omitted_diagnostics"] = json!("1");
    record
  };
  let omitted_diagnostic = ("omitted_diagnostics", "1");
  assert!(!reads_record(&stopped(first(), &alone, omitted_diagnostic)));
  assert!(reads_record(&stopped(first(), &retained, omitted_diagnostic)));
  assert!(reads_record(&diagnostics(omitted_diagnostic)));
  let omitted_reason = ("omitted_reasons", "1");
  assert!(!reads_record(&stopped(first(), &alone, omitted_reason)));
  assert!(reads_record(&stopped(first(), &retained, omitted_reason)));
  assert!(!reads_record(&diagnostics(omitted_reason)));
}

#[test]
fn a_report_keeps_gives_first_or_counts_the_reason_of_an_omitted_declaration() {
  let cut = |kept: Value, first: Value, omitted_reasons: &str| {
    let limits = [
      ("bytes_read", "discovery"),
      ("retained_data", "discovery"),
      ("streams", "discovery"),
    ];
    let mut record = stopped(first, &limits, ("omitted_declarations", "1"));
    record["omitted_reasons"] = json!(omitted_reasons);
    let mut completeness = with(with(completeness(), "reasons", kept), "record", record);
    completeness["discovery"] = json!("incomplete");
    completeness
  };
  let read = exhausted("discovery", "bytes_read");
  let streams = exhausted("discovery", "streams");
  assert!(reads_completeness(&cut(json!([read, streams]), json!([read]), "0")));
  assert!(reads_completeness(&cut(json!([streams]), json!([streams]), "0")));
  // Neither kept nor first: the reason that names the stream limit was counted among the omitted reasons.
  assert!(!reads_completeness(&cut(json!([read]), json!([read]), "0")));
  assert!(reads_completeness(&cut(json!([read]), json!([read]), "1")));
  // One omitted reason of discovery naming the retained data can stand for the declaration and a reference alike.
  let mut both = cut(json!([read]), json!([read]), "1");
  both["record"]["omitted_references"] = json!("1");
  assert!(reads_completeness(&both));
  let mut in_report = report("1.0");
  in_report["completeness"] = cut(json!([read]), json!([read]), "0");
  assert!(serde_json::from_value::<MediaReport>(in_report).is_err());
}

#[test]
fn a_limit_that_a_reason_of_detection_names_stopped_detection_first() {
  let none = ("omitted_reasons", "0");
  let first = || {
    json!([
      exhausted("detection", "work_units"),
      exhausted("discovery", "work_units")
    ])
  };
  assert!(reads_record(&stopped(first(), &[("work_units", "detection")], none)));
  assert!(!reads_record(&stopped(first(), &[("work_units", "discovery")], none)));
  // A kept reason of detection too: the nesting depth stopped discovery first, so it stopped no check of a candidate.
  let record = stopped(
    json!([
      exhausted("detection", "work_units"),
      exhausted("discovery", "nesting_depth")
    ]),
    &[("work_units", "detection"), ("nesting_depth", "discovery")],
    none,
  );
  let reasons = json!([
    exhausted("detection", "work_units"),
    exhausted("detection", "nesting_depth"),
    exhausted("discovery", "nesting_depth"),
  ]);
  let mut kept = with(with(completeness(), "reasons", reasons), "record", record);
  kept["detection"] = json!("incomplete");
  kept["discovery"] = json!("incomplete");
  assert!(!reads_completeness(&kept));
  kept["reasons"] = json!([
    exhausted("detection", "work_units"),
    exhausted("discovery", "nesting_depth")
  ]);
  assert!(reads_completeness(&kept));
}
