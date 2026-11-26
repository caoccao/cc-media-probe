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

//! The limits that stopped a part and those that stopped none (`api.md` section 5.3; `json-output.md` section 7.2),
//! as the fifth review of the third slice's first part asked, which read a complete report refused once its
//! diagnostics limit was exhausted and one diagnostic omitted: `first_stopped` pairs a limit with a part only when it
//! stopped one, so an exhausted limit that only kept a diagnostic from being stored has no pair and leaves the report
//! complete. As the maintainer chose at that review, a limit that a reason names stopped a part and has its pair, and
//! so has the limit behind an omitted reference or declaration. As the maintainer chose at the sixth review, which
//! read discovery stopped by the diagnostics limit, that limit, which only bounds the diagnostics stored, names no
//! stopped part, in a reason or a pair, and is exhausted only beside a diagnostic omitted.
//!
//! A complete report is the entry outcome `ok`, which succeeds under `--strict`, and an incomplete one `partial`
//! (`cli.md` section 6.3), each decided by `is_complete()` alone; the document's entries and the tool's exit status
//! repeat these cases where they are written (plan section 12, task 11).

use cc_media_probe::completeness::{Completeness, LimitRecord, Part, Reason};
use cc_media_probe::error::ErrorRecord;
use cc_media_probe::{Detection, Limit, MediaReport};
use serde_json::{Value, json};

use crate::fixtures::{completeness, detection, error_record, failed_primary, record, report, with, without};

fn reads_record(record: &Value) -> bool {
  serde_json::from_value::<LimitRecord>(record.clone()).is_ok()
}

fn reads_completeness(completeness: &Value) -> bool {
  serde_json::from_value::<Completeness>(completeness.clone()).is_ok()
}

/// The report read, written and read again, which reads back as it was read.
fn round_trip(report: &Value) -> MediaReport {
  let read: MediaReport = serde_json::from_value(report.clone()).unwrap_or_else(|error| panic!("{report}: {error}"));
  let written = serde_json::to_value(&read).unwrap();
  assert_eq!(written["completeness"], report["completeness"]);
  assert_eq!(serde_json::from_value::<MediaReport>(written).unwrap(), read);
  read
}

/// A record with those exhausted limits, those pairs of a limit and the part it stopped first, those first reasons
/// and those omitted counts.
fn stops(exhausted: &[&str], pairs: &[(&str, &str)], first_reasons: Value, omitted: &[(&str, &str)]) -> Value {
  let mut stops = with(record(), "first_reasons", first_reasons);
  stops["exhausted"] = exhausted.iter().map(|limit| json!(limit)).collect();
  stops["first_stopped"] = pairs
    .iter()
    .map(|(limit, part)| json!({ "limit": limit, "part": part }))
    .collect();
  for (count, value) in omitted {
    stops[*count] = json!(value);
  }
  stops
}

/// A reason of that part that names that exhausted limit.
fn exhausted(part: &str, limit: &str) -> Value {
  json!({ "part": part, "reason": { "kind": "limit_exhausted", "limit": limit } })
}

#[test]
fn a_limit_that_stopped_no_part_has_no_pair_and_leaves_the_report_complete() {
  for limit in ["retained_data", "diagnostics"] {
    let mut complete = report("1.0");
    // The review's case: every part complete, the diagnostics limit exhausted and one diagnostic omitted.
    complete["completeness"]["record"] = stops(&[limit], &[], json!([]), &[("omitted_diagnostics", "1")]);
    let read = round_trip(&complete);
    let record = read.completeness().record();
    assert!(read.completeness().is_complete(), "{limit}");
    assert_eq!(record.omitted_diagnostics(), 1);
    let named = serde_json::from_value::<Limit>(json!(limit)).unwrap();
    assert!(record.exhausted(&named));
    assert_eq!(record.first_stopped(&named), None);
    // A complete report never says that a limit stopped one of its parts.
    complete["completeness"]["record"]["first_stopped"] = json!([{ "limit": limit, "part": "discovery" }]);
    assert!(serde_json::from_value::<MediaReport>(complete).is_err(), "{limit}");
  }
}

#[test]
fn a_limit_that_stopped_a_part_has_its_pair_and_leaves_the_report_partial() {
  let mut partial = report("1.0");
  let stopped = exhausted("discovery", "bytes_read");
  partial["completeness"]["discovery"] = json!("incomplete");
  partial["completeness"]["reasons"] = json!([stopped]);
  partial["completeness"]["record"] = stops(
    &["bytes_read", "diagnostics"],
    &[("bytes_read", "discovery")],
    json!([stopped]),
    &[("omitted_diagnostics", "1")],
  );
  let read = round_trip(&partial);
  let record = read.completeness().record();
  assert!(!read.completeness().is_complete());
  assert_eq!(record.first_stopped(&Limit::BytesRead), Some(&Part::Discovery));
  assert!(record.exhausted(&Limit::Diagnostics));
  assert_eq!(record.first_stopped(&Limit::Diagnostics), None);
  // The read limit stopped discovery, which its reason says, so the record pairs it.
  partial["completeness"]["record"]["first_stopped"] = json!([]);
  assert!(serde_json::from_value::<MediaReport>(partial).is_err());
}

#[test]
fn the_pairs_name_some_of_the_exhausted_limits_each_once_in_their_order() {
  let first = || json!([exhausted("discovery", "working_memory")]);
  let both = ["bytes_read", "working_memory"];
  assert!(reads_record(&stops(
    &both,
    &[("working_memory", "discovery")],
    first(),
    &[]
  )));
  assert!(reads_record(&stops(
    &both,
    &[("bytes_read", "discovery"), ("working_memory", "discovery")],
    first(),
    &[]
  )));
  assert!(!reads_record(&stops(
    &both,
    &[("working_memory", "discovery"), ("bytes_read", "discovery")],
    first(),
    &[]
  )));
  assert!(!reads_record(&stops(
    &["working_memory"],
    &[("working_memory", "discovery"), ("working_memory", "discovery")],
    first(),
    &[]
  )));
  assert!(!reads_record(&stops(
    &["working_memory"],
    &[("bytes_read", "discovery")],
    first(),
    &[]
  )));
  // A first reason that names a limit beside the limit exhausted but not paired.
  assert!(!reads_record(&stops(
    &both,
    &[("bytes_read", "discovery")],
    first(),
    &[]
  )));
}

#[test]
fn an_omitted_reference_or_declaration_has_the_pair_of_the_limit_behind_it() {
  let truncated = || json!([{ "part": "discovery", "reason": { "kind": "truncated_input" } }]);
  let paired = [("retained_data", "discovery")];
  for count in ["omitted_references", "omitted_declarations"] {
    assert!(
      reads_record(&stops(&["retained_data"], &paired, truncated(), &[(count, "1")])),
      "{count}"
    );
    assert!(
      !reads_record(&stops(&["retained_data"], &[], truncated(), &[(count, "1")])),
      "{count}"
    );
  }
  let streams = [("streams", "discovery")];
  let declarations = [("omitted_declarations", "1")];
  assert!(reads_record(&stops(&["streams"], &streams, truncated(), &declarations)));
  assert!(!reads_record(&stops(&["streams"], &[], truncated(), &declarations)));
  // The retained data stopped no part where it only kept a reason or a diagnostic from being stored.
  let reasons_and_diagnostics = [("omitted_reasons", "1"), ("omitted_diagnostics", "1")];
  assert!(reads_record(&stops(
    &["retained_data"],
    &[],
    truncated(),
    &reasons_and_diagnostics
  )));
}

#[test]
fn a_kept_reason_names_a_limit_with_its_pair() {
  let kept = |pairs: &[(&str, &str)]| {
    let truncated = json!({ "part": "discovery", "reason": { "kind": "truncated_input" } });
    let mut cut = with(
      completeness(),
      "reasons",
      json!([truncated, exhausted("discovery", "bytes_read")]),
    );
    cut["discovery"] = json!("incomplete");
    cut["record"] = stops(&["bytes_read"], pairs, json!([truncated]), &[]);
    cut
  };
  assert!(reads_completeness(&kept(&[("bytes_read", "discovery")])));
  assert!(!reads_completeness(&kept(&[])));
}

#[test]
fn an_observed_entry_names_a_limit_with_its_pair() {
  let observed = |pairs: &[(&str, &str)]| {
    let refused = json!({
      "identifier": "1",
      "role": "subtitle_index",
      "outcome": "limit_reached",
      "reason": { "kind": "companion_limited", "limit": "sources_opened" },
    });
    let first = json!([{ "part": "companions", "reason": { "kind": "truncated_input" } }]);
    let mut error = with(
      error_record("1.0"),
      "observations",
      json!({ "diagnostics": [], "sources": [failed_primary(), refused] }),
    );
    error["record"] = stops(&["sources_opened"], pairs, first, &[]);
    error
  };
  let reads_error = |error: &Value| serde_json::from_value::<ErrorRecord>(error.clone()).is_ok();
  assert!(reads_error(&observed(&[("sources_opened", "companions")])));
  assert!(!reads_error(&observed(&[])));
}

#[test]
fn the_diagnostics_limit_names_no_stopped_part() {
  let diagnostics = json!({ "kind": "limit_exhausted", "limit": "diagnostics" });
  let limited = json!({ "kind": "companion_limited", "limit": "diagnostics" });
  // A reason of either kind that names the diagnostics limit is malformed wherever it stands, alone too.
  for reason in [&diagnostics, &limited] {
    assert!(serde_json::from_value::<Reason>(reason.clone()).is_err(), "{reason}");
  }
  // The review's case: discovery incomplete with the diagnostics limit as its only reason.
  let mut stopped = report("1.0");
  let cause = json!({ "part": "discovery", "reason": diagnostics });
  stopped["completeness"]["discovery"] = json!("incomplete");
  stopped["completeness"]["reasons"] = json!([cause]);
  stopped["completeness"]["record"] = stops(
    &["diagnostics"],
    &[("diagnostics", "discovery")],
    json!([cause]),
    &[("omitted_diagnostics", "1")],
  );
  assert!(serde_json::from_value::<MediaReport>(stopped).is_err());
  // Paired with a part that the input's end stopped, the diagnostics limit is still refused.
  let truncated = || json!([{ "part": "discovery", "reason": { "kind": "truncated_input" } }]);
  let omitted = [("omitted_diagnostics", "1")];
  assert!(reads_record(&stops(&["diagnostics"], &[], truncated(), &omitted)));
  assert!(!reads_record(&stops(
    &["diagnostics"],
    &[("diagnostics", "discovery")],
    truncated(),
    &omitted
  )));
  let insufficient = |reason: Value| {
    let detection = with(detection("1.0"), "outcome", json!("insufficient_data"));
    let detection = without(
      without(with(detection, "insufficient_reason", reason), "format"),
      "confidence",
    );
    serde_json::from_value::<Detection>(detection)
  };
  assert!(insufficient(json!({ "kind": "limit_exhausted", "limit": "bytes_read" })).is_ok());
  assert!(insufficient(diagnostics).is_err());
}

#[test]
fn the_diagnostics_limit_is_exhausted_only_beside_a_diagnostic_that_it_kept() {
  assert!(!reads_record(&stops(&["diagnostics"], &[], json!([]), &[])));
  assert!(reads_record(&stops(
    &["diagnostics"],
    &[],
    json!([]),
    &[("omitted_diagnostics", "1")]
  )));
  // The retained data can keep a diagnostic too, so an omitted diagnostic needs either limit.
  assert!(reads_record(&stops(
    &["retained_data"],
    &[],
    json!([]),
    &[("omitted_diagnostics", "1")]
  )));
}
