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

//! The paths of a record before the split (`docs/design/test-kit.md` section 4.2): the primary input listed with
//! the path that its adapter was given, a path below the test's folder written with the placeholder, the charge of
//! a handed path, and the retained data net of it, in the record's own usage and its partial report's.

use std::path::Path;

use cc_media_probe::Limits;
use serde_json::{Map, Value, json};

use crate::common::sources::{Handed, normalize, primary_path};
use crate::common::usage::within_limits;
use crate::records::{entry, failed, report, stopped, unknown};

/// A usage of the form whose retained data is `retained` and whose other counters are 0.
fn usage(retained: u64) -> Value {
  json!({
    "bytes_read": 0, "peak_working_memory": 0, "retained_data": retained, "retained_payloads": 0,
    "decompressed_output": 0, "work_units": 0, "companion_requests": 0, "sources_opened": 0,
  })
}

#[test]
fn a_path_below_the_folder_is_written_with_the_placeholder_and_taken_out_of_the_retained_data() {
  let folder = Path::new(env!("CARGO_TARGET_TMPDIR")).join("placeholder");
  let path = folder.join("movie.idx");
  let sibling = Path::new(env!("CARGO_TARGET_TMPDIR"))
    .join("placeholder2")
    .join("movie.idx");
  let handed = Handed::new(&folder, "movie.idx");
  let at = |path: &str| Map::from_iter([("path".to_owned(), json!(path))]);
  let mut record = report(
    1000,
    &[
      entry("0", "opened", &handed.members),
      entry("1", "opened", &at("/elsewhere/movie.sub")),
      entry("2", "opened", &at(sibling.to_str().unwrap())),
    ],
  );
  normalize(&mut record, &folder, &[handed]);
  let length = u64::try_from(path.as_os_str().len()).unwrap();
  assert_eq!(
    record["sources"][0],
    json!({ "identifier": "0", "role": "primary", "outcome": "opened", "path": "{folder}/movie.idx" })
  );
  assert_eq!(record["sources"][1]["path"], "/elsewhere/movie.sub");
  assert_eq!(record["sources"][2]["path"], sibling.to_str().unwrap());
  assert_eq!(record["usage"]["retained_data"], json!(1000 - length));
}

#[test]
fn a_partial_reports_path_is_taken_out_of_the_errors_usage_too() {
  let folder = Path::new(env!("CARGO_TARGET_TMPDIR")).join("placeholder");
  let path = folder.join("movie.idx");
  let handed = Handed::new(&folder, "movie.idx");
  let mut record = failed(900, &[entry("0", "failed", &handed.members)]);
  // The error's usage covers its partial report's.
  record["usage"]["retained_data"] = json!(1000);
  normalize(&mut record, &folder, &[handed]);
  let length = u64::try_from(path.as_os_str().len()).unwrap();
  assert_eq!(record["usage"]["retained_data"], json!(1000 - length));
  assert_eq!(record["partial_report"]["usage"]["retained_data"], json!(900 - length));
}

#[test]
#[should_panic(expected = "written as its form gives it")]
fn a_unicode_path_written_as_lossy_fails_the_comparison() {
  let folder = Path::new(env!("CARGO_TARGET_TMPDIR")).join("placeholder");
  let mut record = json!({
    "usage": usage(1000),
    "sources": [{
      "identifier": "0",
      "path": folder.join("movie.idx").to_str().unwrap(),
      "path_lossy": true,
      "path_raw": { "encoding": "unix_bytes", "data": "6d" },
    }],
  });
  normalize(&mut record, &folder, &[Handed::new(&folder, "movie.idx")]);
}

#[cfg(unix)]
#[test]
fn a_lossy_path_is_charged_at_its_display_string_and_its_raw_forms_data() {
  use std::ffi::OsStr;
  use std::os::unix::ffi::OsStrExt;

  let folder = Path::new(OsStr::from_bytes(b"/f\xff"));
  let handed = Handed::new(folder, "a");
  assert_eq!(folder.join("a").as_os_str().len(), 5);
  assert_eq!("/f\u{fffd}/a".len() + "2f66ff2f61".len(), 17);
  assert_eq!(handed.length, 17);
  assert_eq!(handed.members["path_lossy"], json!(true));
  assert_eq!(handed.members["path_raw"]["data"], "2f66ff2f61");
}

#[test]
fn retained_data_beyond_the_largest_number_is_netted_in_its_twin_and_written_canonically() {
  let folder = Path::new("/tmp/abcdefg");
  let handed = [Handed::new(folder, "x.bin")];
  assert_eq!(handed[0].length, 18);
  // The record lists the path as the library writes it: `Path::join` gives Windows its own separator.
  let gross_of = |retained| report(retained, &[entry("0", "opened", &handed[0].members)]);
  let mut gross = gross_of(9_007_199_254_741_009);
  assert_eq!(gross["usage"]["retained_data_exact"], json!("9007199254741009"));
  normalize(&mut gross, folder, &handed);
  assert_eq!(gross["usage"]["retained_data"], json!(9_007_199_254_740_991_u64));
  assert_eq!(gross["usage"].get("retained_data_exact"), None);
  let limits = Limits::default().with_retained_data(9_007_199_254_740_991);
  within_limits(&gross, &limits, "file").unwrap();
  let mut beyond = gross_of(9_007_199_254_741_012);
  normalize(&mut beyond, folder, &handed);
  assert_eq!(beyond["usage"]["retained_data_exact"], json!("9007199254740994"));
  assert_eq!(beyond["usage"].get("retained_data"), None);
}

#[test]
fn each_source_table_lists_the_primary_input_with_the_path_that_its_adapter_was_given() {
  let folder = Path::new("/tmp/abcdefg");
  let handed = Handed::new(folder, "x.bin");
  let none = Map::new();
  let listing = |outcome, path: &Map<String, Value>| vec![entry("0", outcome, path)];
  primary_path(&report(18, &listing("opened", &handed.members)), Some(&handed), "file").unwrap();
  primary_path(&report(0, &listing("opened", &none)), None, "reader").unwrap();
  // The seventh review's case: a file report without the path that its raised limit holds.
  let error = primary_path(&report(0, &listing("opened", &none)), Some(&handed), "file").unwrap_err();
  assert!(error.contains("given at /sources of file: {} != {\"path\":"), "{error}");
  // A name hint is never a path, so an adapter given none lists none.
  let error = primary_path(&report(18, &listing("opened", &handed.members)), None, "reader").unwrap_err();
  assert!(error.contains("given at /sources of reader"), "{error}");
  // An error lists its sources in its partial report or its observations, where they stand, and has none at detection.
  primary_path(&failed(18, &listing("failed", &handed.members)), Some(&handed), "file").unwrap();
  let error = primary_path(&failed(0, &listing("failed", &none)), Some(&handed), "file").unwrap_err();
  assert!(error.contains("given at /partial_report/sources of file"), "{error}");
  primary_path(&stopped(18, &listing("opened", &handed.members)), Some(&handed), "file").unwrap();
  let error = primary_path(&stopped(0, &listing("opened", &none)), Some(&handed), "file").unwrap_err();
  assert!(error.contains("given at /observations/sources of file"), "{error}");
  primary_path(&unknown(0), Some(&handed), "file").unwrap();
  // Records that the strict reader refuses: a report without its table, and observations without the primary input.
  let error = primary_path(&json!({ "format": "matroska" }), None, "bytes").unwrap_err();
  assert!(error.contains("no source table at /sources of bytes"), "{error}");
  let observed = json!({ "kind": "no_stream_inventory", "observations": { "sources": [] } });
  let error = primary_path(&observed, None, "bytes").unwrap_err();
  assert!(
    error.contains("no primary input at /observations/sources of bytes"),
    "{error}"
  );
}
