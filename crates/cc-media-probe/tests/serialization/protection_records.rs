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

//! A protection that contradicts itself or its report (`api.md` section 6.3), as the fourth review of the third
//! slice's first part asked, which read a declaration of a scope that was not read beside complete discovery and
//! beside a protection examined in full, and description 1 kept as clear beside a declaration of description 1.

use cc_media_probe::MediaReport;
use cc_media_probe::model::Protection;
use serde_json::{Value, json};

use crate::fixtures::{report, stream, with};

fn reads(report: &Value) -> bool {
  serde_json::from_value::<MediaReport>(report.clone()).is_ok()
}

fn reads_protection(protection: &Value) -> bool {
  serde_json::from_value::<Protection>(protection.clone()).is_ok()
}

/// A Matroska track's protection whose one declaration is of the scope `unread`.
fn unread(fully_examined: bool) -> Value {
  json!({
    "declarations": [{ "scope": "unread" }],
    "fully_examined": fully_examined,
    "has_sample_descriptions": false,
    "sample_descriptions": [],
  })
}

/// An ISO track's protection of two descriptions, with those entries and those declarations of a description.
fn described(entries: Value, declared: &[u64]) -> Value {
  let declarations: Vec<Value> = declared
    .iter()
    .map(|index| json!({ "scope": "sample_description", "sample_description_index": index }))
    .collect();
  let mut protection = json!({
    "declarations": declarations,
    "fully_examined": false,
    "has_sample_descriptions": true,
    "sample_description_count": 2,
  });
  protection["sample_descriptions"] = entries;
  protection
}

#[test]
fn a_scope_that_was_not_read_leaves_discovery_incomplete() {
  let protected = with(stream(0, "audio"), "protection", unread(false));
  let mut report = with(report("1.0"), "streams", json!([protected]));
  // The review's case: the declaration beside complete discovery.
  assert!(!reads(&report));
  let cut = json!({ "part": "discovery", "stream": 0, "reason": { "kind": "truncated_input" } });
  report["completeness"]["discovery"] = json!("incomplete");
  report["completeness"]["reasons"] = json!([cut]);
  report["completeness"]["record"]["first_reasons"] = json!([cut]);
  assert!(reads(&report));
  // Beside a status of a newer version, the cut needs the first reason of discovery.
  report["completeness"]["discovery"] = json!("partially_discovered");
  assert!(reads(&report));
  report["completeness"]["reasons"] = json!([]);
  report["completeness"]["record"]["first_reasons"] = json!([]);
  assert!(!reads(&report));
}

#[test]
fn a_protection_holds_no_two_records_that_contradict_each_other() {
  assert!(reads_protection(&unread(false)));
  // The review's cases: a scope that was not read in a protection examined in full, and a description kept as
  // clear beside a declaration of it.
  assert!(!reads_protection(&unread(true)));
  let entry = |index: u64, state: &str| json!({ "index": index, "state": state });
  assert!(!reads_protection(&described(json!([entry(1, "clear")]), &[1])));
  assert!(reads_protection(&described(json!([entry(1, "declared")]), &[1])));
  assert!(reads_protection(&described(json!([entry(2, "clear")]), &[1])));
  assert!(reads_protection(&described(json!([entry(1, "future_state")]), &[1])));
}
