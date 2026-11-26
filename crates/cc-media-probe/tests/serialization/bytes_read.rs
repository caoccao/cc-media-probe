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

//! The bytes that one probe read, which every source charges to the one budget (plan section 7.5), as the maintainer
//! chose at the third review of the third slice's first part, which read an error of 0 bytes read beside a partial
//! report of 64, and a report of 64 bytes read beside a primary input of 150: the entries of a source table read at
//! most the usage's bytes read, in a report and in an error's observations, and an error's bytes read are at least
//! its partial report's.

use cc_media_probe::MediaReport;
use cc_media_probe::error::ErrorRecord;
use serde_json::{Value, json};

use crate::fixtures::{error_record, failed_primary, partial_report, primary_source, report, with, with_partial};

fn reads(report: &Value) -> bool {
  serde_json::from_value::<MediaReport>(report.clone()).is_ok()
}

fn reads_error(record: &Value) -> bool {
  serde_json::from_value::<ErrorRecord>(record.clone()).is_ok()
}

/// The record with `bytes_read` set to that count, as a number within 2^53 - 1 and as its twin beyond.
fn reading(record: Value, bytes: u64) -> Value {
  if bytes > 9_007_199_254_740_991 {
    with(record, "bytes_read_exact", json!(bytes.to_string()))
  } else {
    with(record, "bytes_read", json!(bytes))
  }
}

/// A report whose usage counts `total` bytes read and whose sources read those counts, the first the primary input's.
fn report_reading(total: u64, read: &[u64]) -> Value {
  let mut report = report("1.0");
  report["usage"] = reading(crate::fixtures::without(report["usage"].clone(), "bytes_read"), total);
  let sources = (0_u64..).zip(read).map(|(identifier, bytes)| {
    let entry = if identifier == 0 {
      primary_source()
    } else {
      json!({ "identifier": identifier.to_string(), "role": "subtitle_index", "outcome": "opened" })
    };
    reading(entry, *bytes)
  });
  report["sources"] = Value::Array(sources.collect());
  if read.len() > 1 {
    report["completeness"]["companions"] = json!("complete");
  }
  report
}

#[test]
fn the_sources_of_a_report_read_at_most_its_bytes_read() {
  assert!(reads(&report_reading(64, &[64])));
  assert!(reads(&report_reading(300, &[150, 64])));
  assert!(reads(&report_reading(u64::MAX, &[u64::MAX])));
  // The review's case: a primary input of 150 bytes beside a usage of 64.
  assert!(!reads(&report_reading(64, &[150])));
  assert!(!reads(&report_reading(150, &[100, 100])));
  assert!(!reads(&report_reading(u64::MAX, &[u64::MAX, 1])));
}

#[test]
fn an_error_counts_the_bytes_of_its_partial_report_and_of_its_observed_sources() {
  let partial = with_partial(
    error_record("1.0"),
    with(partial_report("1.0"), "usage", {
      let mut usage = crate::fixtures::usage();
      usage["bytes_read"] = json!(64);
      usage
    }),
  );
  let counting = |record: &Value, bytes: u64| {
    let mut record = record.clone();
    record["usage"]["bytes_read"] = json!(bytes);
    record
  };
  assert!(reads_error(&counting(&partial, 64)));
  assert!(reads_error(&counting(&partial, 100)));
  // The review's case: an error of 0 bytes read beside a partial report of 64.
  assert!(!reads_error(&counting(&partial, 0)));
  let observed = with(
    error_record("1.0"),
    "observations",
    json!({ "diagnostics": [], "sources": [reading(failed_primary(), 64)] }),
  );
  assert!(reads_error(&counting(&observed, 64)));
  assert!(!reads_error(&counting(&observed, 63)));
}
