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

//! The values that a recipe states for its generated file (`docs/design/test-kit.md` section 4.4), matched with a
//! report in the library's JSON form: a report read through the library's strict reader and written again, as a
//! probe's report would be written.

use cc_media_probe::MediaReport;
use serde_json::{Value, json};

use crate::common::matcher::mismatches;

/// A complete Matroska report with a video stream of 320 by 240 and an audio stream of two channels.
fn report() -> Value {
  let stream = |index: usize, kind: &str, information: Value| {
    let mut stream = json!({
      "index": index,
      "kind": kind,
      "codec": { "candidates": [], "has_every_candidate": true, "raw_identifiers": [] },
      "header_status": { "state": "parsed" },
      "languages": [],
      "titles": [],
      "disposition": {},
    });
    stream[kind] = information;
    stream
  };
  let record = json!({
    "exhausted": [], "first_stopped": [], "first_reasons": [], "omitted_diagnostics": "0",
    "omitted_declarations": "0", "omitted_reasons": "0", "omitted_references": "0",
  });
  let usage = json!({
    "bytes_read": 0, "peak_working_memory": 0, "retained_data": 0, "retained_payloads": 0,
    "decompressed_output": 0, "work_units": 0, "companion_requests": 0, "sources_opened": 0,
  });
  let written = json!({
    "schema_version": "1.0",
    "format": "matroska",
    "streams": [
      stream(0, "video", json!({
        "frame_rate_mode": "constant",
        "coded_size": { "width": 320, "height": 240 },
        "coded_size_provenance": "bitstream",
      })),
      stream(1, "audio", json!({ "channel_count": 2, "channel_count_provenance": "declared" })),
    ],
    "programs": [], "chapter_editions": [], "tag_sets": [], "attachments": [], "external_references": [],
    "diagnostics": [],
    "completeness": {
      "detection": "complete", "discovery": "complete", "headers": "complete", "tags": "complete",
      "chapters": "complete", "attachments": "complete", "timing": "established", "companions": "not_applicable",
      "reasons": [], "record": record,
    },
    "inventory_class": "declared",
    "usage": usage,
    "sources": [{ "identifier": "0", "role": "primary", "outcome": "opened" }],
  });
  let read: MediaReport = serde_json::from_value(written).unwrap();
  serde_json::to_value(&read).unwrap()
}

/// What a recipe of a Matroska file with an H.264 stream of 320 by 240 and a stereo Opus stream fixes, in the
/// member `expected` that `docs/guide/test-files.md` section 3 proposes.
fn expected() -> Value {
  json!({
    "format": "matroska",
    "streams": [
      { "kind": "video", "video": { "coded_size": { "width": 320, "height": 240 } } },
      { "kind": "audio", "audio": { "channel_count": 2 } },
    ],
    "creation_time": null,
  })
}

#[test]
fn a_recipe_states_only_the_values_that_it_fixes() {
  assert_eq!(mismatches(&expected(), &report()), Vec::<String>::new());
}

#[test]
fn each_value_that_differs_is_named_by_its_place() {
  let mut expected = expected();
  expected["streams"][0]["video"]["coded_size"]["width"] = json!(640);
  expected["inventory_class"] = Value::Null;
  expected["media_type"] = json!("video/x-matroska");
  assert_eq!(
    mismatches(&expected, &report()),
    [
      "/inventory_class: expected no value, found \"declared\"",
      "/media_type: expected \"video/x-matroska\", found no value",
      "/streams/0/video/coded_size/width: expected 640, found 320",
    ]
  );
  expected["streams"] = json!([{ "kind": "video" }]);
  assert_eq!(
    mismatches(&expected, &report())[2],
    "/streams: expected 1 entries, found 2"
  );
}

#[test]
fn a_member_name_with_a_slash_or_a_tilde_is_escaped_in_its_place() {
  let found = mismatches(
    &json!({ "a/b": 1, "a": { "b": 2 }, "c~d": 3 }),
    &json!({ "a/b": 4, "a": { "b": 5 }, "c~d": 6 }),
  );
  assert_eq!(
    found,
    [
      "/a/b: expected 2, found 5",
      "/a~1b: expected 1, found 4",
      "/c~0d: expected 3, found 6"
    ]
  );
}

#[test]
fn the_differences_come_in_the_order_of_their_pointers_with_indexes_as_numbers() {
  let found = mismatches(
    &json!([0, 9, 9, 3, 4, 5, 6, 7, 8, 9, 9]),
    &json!([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]),
  );
  assert_eq!(
    found,
    [
      "/1: expected 9, found 1",
      "/2: expected 9, found 2",
      "/10: expected 9, found 10"
    ]
  );
}

#[test]
fn an_array_of_another_length_still_compares_the_entries_that_both_have() {
  // The review of plan Phase 3, task 4, compared [1, 2] with [9] and read the length alone.
  assert_eq!(
    mismatches(&json!([1, 2]), &json!([9])),
    [": expected 2 entries, found 1", "/0: expected 1, found 9"]
  );
  assert_eq!(
    mismatches(&json!({ "a": [1] }), &json!({ "a": [1, 2] })),
    ["/a: expected 1 entries, found 2"]
  );
}
