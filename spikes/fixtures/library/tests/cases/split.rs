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

//! The split of a record at the places that the form defines (`docs/design/test-kit.md` section 4.2): a size, the
//! usage's bytes read and peak memory, and the source entries' bytes read and paths go to the adapter part; an
//! error's native details become placeholders; everything else, a typed detail's own members included, stays
//! semantic.

use serde_json::{Value, json};

use crate::common::split::split;

#[test]
fn the_adapter_part_holds_the_size_the_bytes_read_the_peak_memory_and_the_paths() {
  let report = json!({
    "schema_version": "1.0",
    "format": "matroska",
    "size": 57,
    "streams": [],
    "usage": { "bytes_read": 57, "peak_working_memory": 4096, "work_units": 3, "retained_data": 12 },
    "sources": [{
      "identifier": "0", "role": "primary", "outcome": "opened", "bytes_read": 57, "path": "{folder}/b.bin",
      "attempted_identity": { "identifier": "b" },
    }],
  });
  let (semantic, adapter) = split(&report);
  assert_eq!(
    semantic,
    json!({
      "schema_version": "1.0",
      "format": "matroska",
      "streams": [],
      "usage": { "work_units": 3, "retained_data": 12 },
      "sources": [{ "identifier": "0", "role": "primary", "outcome": "opened", "attempted_identity": { "identifier": "b" } }],
    })
  );
  assert_eq!(
    Value::Object(adapter),
    json!({
      "/size": 57,
      "/usage/bytes_read": 57,
      "/usage/peak_working_memory": 4096,
      "/sources/0/bytes_read": 57,
      "/sources/0/path": "{folder}/b.bin",
    })
  );
}

#[test]
fn a_typed_details_own_usage_and_sources_stay_semantic() {
  let report = json!({
    "schema_version": "1.0",
    "streams": [{ "details": { "usage": { "bytes_read": 9 }, "sources": [{ "path": "x" }], "size": 3 } }],
  });
  let (semantic, adapter) = split(&report);
  assert_eq!(semantic, report);
  assert!(adapter.is_empty());
}

#[test]
fn an_errors_partial_report_and_observations_split_and_its_native_details_become_placeholders() {
  let record = json!({
    "schema_version": "1.0",
    "kind": "source_failed",
    "source_failure_kind": "not_found",
    "operating_system_error_code": "2",
    "cause_messages": ["No such file or directory (os error 2)"],
    "attempted_identity": { "identifier": "movie", "requested_name": "movie.sub" },
    "usage": { "bytes_read": 10, "work_units": 1 },
    "partial_report": {
      "size": 10,
      "usage": { "peak_working_memory": 64 },
      "sources": [{ "identifier": "0", "bytes_read": 10 }],
    },
  });
  let (semantic, adapter) = split(&record);
  assert_eq!(semantic["operating_system_error_code"], "{native}");
  assert_eq!(semantic["cause_messages"], json!(["{native}"]));
  assert_eq!(semantic["source_failure_kind"], "not_found");
  assert_eq!(semantic["attempted_identity"], record["attempted_identity"]);
  assert_eq!(
    semantic["partial_report"],
    json!({ "usage": {}, "sources": [{ "identifier": "0" }] })
  );
  assert_eq!(
    Value::Object(adapter),
    json!({
      "/usage/bytes_read": 10,
      "/partial_report/size": 10,
      "/partial_report/usage/peak_working_memory": 64,
      "/partial_report/sources/0/bytes_read": 10,
    })
  );
  let observed = json!({
    "schema_version": "1.0",
    "kind": "unknown_format",
    "cause_messages": [],
    "observations": { "sources": [{ "identifier": "0", "path": "{folder}/a" }] },
  });
  let (semantic, adapter) = split(&observed);
  assert_eq!(semantic["observations"], json!({ "sources": [{ "identifier": "0" }] }));
  assert_eq!(semantic["cause_messages"], json!([]));
  assert_eq!(
    Value::Object(adapter),
    json!({ "/observations/sources/0/path": "{folder}/a" })
  );
}
