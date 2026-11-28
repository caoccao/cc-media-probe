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

//! The usage of each record against the limits that its adapter was given (`docs/design/test-kit.md` section 4.1),
//! as the fifth review of the proposal asked: read by the library's strict reader, the partial report's too, which
//! the sixth review found could be missing, beside the bytes that the test's counters measured.

use cc_media_probe::Limits;
use kit::readers::Counts;
use serde_json::{Value, json};

use crate::common::usage::{within_limits, within_reads};

/// A usage whose counters are all 0 but those of `counters`.
fn usage(counters: &Value) -> Value {
  let mut usage = json!({
    "bytes_read": 0, "peak_working_memory": 0, "retained_data": 0, "retained_payloads": 0,
    "decompressed_output": 0, "work_units": 0, "companion_requests": 0, "sources_opened": 0,
  });
  for (member, value) in counters.as_object().unwrap() {
    let twin = member.strip_suffix("_exact");
    usage.as_object_mut().unwrap().remove(twin.unwrap_or(member));
    usage[member] = value.clone();
  }
  usage
}

/// The record of an adapter whose usage has `counters`.
fn record(counters: &Value) -> Value {
  json!({ "kind": "unknown_format", "usage": usage(counters) })
}

#[test]
fn every_counter_of_the_usage_stays_within_its_limit() {
  let limits = Limits::default().with_working_memory(0).with_bytes_read(100);
  within_limits(&record(&json!({ "bytes_read": 100 })), &limits, "reader").unwrap();
  let error = within_limits(&record(&json!({ "peak_working_memory": 1 })), &limits, "reader").unwrap_err();
  assert!(error.ends_with("reader: /usage/peak_working_memory 1 > 0"), "{error}");
  let twin = record(&json!({ "bytes_read_exact": "9007199254740993" }));
  let error = within_limits(&twin, &limits, "file").unwrap_err();
  assert!(
    error.ends_with("file: /usage/bytes_read 9007199254740993 > 100"),
    "{error}"
  );
}

#[test]
fn a_partial_reports_usage_stays_within_the_limits_too() {
  let limits = Limits::default().with_working_memory(0);
  let mut failed = record(&json!({}));
  failed["kind"] = json!("source_failed");
  failed["partial_report"] = json!({ "usage": usage(&json!({ "peak_working_memory": 3 })) });
  let error = within_limits(&failed, &limits, "reader").unwrap_err();
  assert!(
    error.ends_with("reader: /partial_report/usage/peak_working_memory 3 > 0"),
    "{error}"
  );
  failed["partial_report"]["usage"] = json!({ "peak_working_memory": 0 });
  let error = within_limits(&failed, &limits, "reader").unwrap_err();
  assert!(
    error.contains("not the form's at /partial_report/usage of reader"),
    "{error}"
  );
  for partial in [json!({ "sources": [] }), json!(null)] {
    failed["partial_report"] = partial;
    let error = within_limits(&failed, &limits, "reader").unwrap_err();
    assert!(error.contains("no usage at /partial_report/usage of reader"), "{error}");
  }
  failed.as_object_mut().unwrap().remove("partial_report");
  within_limits(&failed, &limits, "reader").unwrap();
}

#[test]
fn a_usage_that_is_missing_or_not_the_forms_fails() {
  let limits = Limits::default().with_working_memory(0);
  let error = within_limits(&json!({ "kind": "unknown_format" }), &limits, "bytes").unwrap_err();
  assert!(error.contains("no usage at /usage of bytes"), "{error}");
  let mut absent = record(&json!({}));
  absent["usage"].as_object_mut().unwrap().remove("work_units");
  let both = usage(&json!({ "peak_working_memory": 0 }));
  let mut both = json!({ "usage": both });
  both["usage"]["peak_working_memory_exact"] = json!("9007199254740993");
  for malformed in [
    absent,
    record(&json!({ "peak_working_memory_exact": "x" })),
    record(&json!({ "peak_working_memory_exact": "9007199254740991" })),
    record(&json!({ "peak_working_memory_exact": "09007199254740993" })),
    both,
  ] {
    let error = within_limits(&malformed, &limits, "reader").unwrap_err();
    assert!(
      error.contains("not the form's at /usage of reader"),
      "{malformed}: {error}"
    );
  }
}

#[test]
fn the_bytes_delivered_stay_within_the_read_limit_and_are_the_bytes_read() {
  let limits = Limits::default().with_bytes_read(1);
  let counted = |delivered| {
    Some(Counts {
      read_calls: 1,
      delivered,
      seeks: 0,
    })
  };
  within_reads(&record(&json!({ "bytes_read": 1 })), counted(1), &limits, "reader").unwrap();
  within_reads(&record(&json!({ "bytes_read": 1 })), None, &limits, "bytes").unwrap();
  let error = within_reads(&record(&json!({ "bytes_read": 1 })), counted(2), &limits, "stream").unwrap_err();
  assert!(error.ends_with("stream: 2 > 1"), "{error}");
  let error = within_reads(&record(&json!({ "bytes_read": 0 })), counted(1), &limits, "source").unwrap_err();
  assert!(error.ends_with("source: 0 != 1"), "{error}");
}
