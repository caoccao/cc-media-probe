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

//! A path in the source table (`json-output.md` section 5.8): its display string under its own name, with the flag
//! and the raw form of a lossy path beside it. The three members describe one path, on every platform: the raw form
//! stands only beside the flag, which is only `true`; a lossy display string holds U+FFFD; and the data of a known
//! encoding is whole units of lowercase digits, is not valid Unicode, and decodes to the display string. Data of an
//! encoding of a newer version is kept as written.

use cc_media_probe::source::{RawPathEncoding, SourceEntry};
use serde_json::{Value, json};

fn entry(path: &[(&str, Value)]) -> Value {
  let mut entry = json!({ "identifier": "0", "role": "primary", "outcome": "opened" });
  for (name, value) in path {
    entry[*name] = value.clone();
  }
  entry
}

fn read(path: &[(&str, Value)]) -> Result<SourceEntry, serde_json::Error> {
  serde_json::from_value(entry(path))
}

fn raw(encoding: &str, data: &str) -> Value {
  json!({ "encoding": encoding, "data": data })
}

#[test]
fn a_path_that_is_not_lossy_is_its_display_string_alone() {
  let read_back = read(&[("path", json!("movie.mkv"))]).unwrap();
  let path = read_back.path().unwrap();
  assert_eq!(
    (path.display(), path.is_lossy(), path.raw()),
    ("movie.mkv", false, None)
  );
  assert_eq!(
    serde_json::to_value(&read_back).unwrap(),
    entry(&[("path", json!("movie.mkv"))])
  );
  assert!(read(&[]).unwrap().path().is_none());
}

#[test]
fn a_lossy_path_describes_one_path_on_unix_and_windows_data_alike() {
  for (display, encoding, data) in [
    ("\u{fffd}.bin", "unix_bytes", "ff2e62696e"),
    ("\u{fffd}.bin", "windows_wide", "d800002e00620069006e"),
  ] {
    let members = [
      ("path", json!(display)),
      ("path_lossy", json!(true)),
      ("path_raw", raw(encoding, data)),
    ];
    let read_back = read(&members).unwrap();
    let path = read_back.path().unwrap();
    assert!(path.is_lossy());
    assert_eq!(path.raw().map(cc_media_probe::source::RawPath::data), Some(data));
    assert_eq!(serde_json::to_value(&read_back).unwrap(), entry(&members));
  }
  let path = read(&[
    ("path", json!("\u{fffd}.bin")),
    ("path_raw", raw("unix_bytes", "ff2e62696e")),
    ("path_lossy", json!(true)),
  ])
  .unwrap();
  assert_eq!(
    path
      .path()
      .and_then(|path| path.raw())
      .map(cc_media_probe::source::RawPath::encoding),
    Some(&RawPathEncoding::UnixBytes)
  );
}

#[test]
fn the_three_members_that_disagree_are_malformed() {
  let lossy = ("path_lossy", json!(true));
  for members in [
    vec![
      ("path", json!("\u{fffd}.bin")),
      ("path_raw", raw("unix_bytes", "ff2e62696e")),
    ],
    vec![("path", json!("movie.mkv")), ("path_lossy", json!(false))],
    vec![
      ("path", json!("input.bin")),
      lossy.clone(),
      ("path_raw", raw("unix_bytes", "ff2e62696e")),
    ],
    vec![
      ("path", json!("\u{fffd}.bin")),
      lossy.clone(),
      ("path_raw", raw("unix_bytes", "6f74686572")),
    ],
    vec![
      ("path", json!("\u{fffd}x.bin")),
      lossy.clone(),
      ("path_raw", raw("unix_bytes", "ff2e62696e")),
    ],
    vec![
      ("path", json!("\u{fffd}")),
      lossy.clone(),
      ("path_raw", raw("unix_bytes", "gg")),
    ],
    vec![
      ("path", json!("\u{fffd}")),
      lossy.clone(),
      ("path_raw", raw("unix_bytes", "fff")),
    ],
    vec![
      ("path", json!("\u{fffd}")),
      lossy.clone(),
      ("path_raw", raw("windows_wide", "d80")),
    ],
    vec![
      ("path", json!("\u{fffd}")),
      lossy.clone(),
      ("path_raw", raw("unix_bytes", "FF")),
    ],
    vec![("path", json!("plain")), lossy.clone()],
    vec![
      ("path", json!("\u{fffd}")),
      lossy.clone(),
      ("path_raw", raw("future_bytes", "anything")),
    ]
    .into_iter()
    .map(|(name, value)| {
      if name == "path" {
        (name, json!("plain"))
      } else {
        (name, value)
      }
    })
    .collect(),
    vec![lossy.clone()],
    vec![("path_raw", raw("unix_bytes", "ff"))],
    vec![("path", json!("\u{fffd}")), ("path_lossy", Value::Null)],
  ] {
    assert!(read(&members).is_err(), "{members:?}");
  }
}

#[test]
fn a_lossy_path_without_a_raw_form_or_of_a_newer_encoding_reads_beside_u_fffd() {
  let flag_alone = read(&[("path", json!("\u{fffd}.bin")), ("path_lossy", json!(true))]).unwrap();
  let path = flag_alone.path().unwrap();
  assert!(path.is_lossy() && path.raw().is_none() && path.to_native().is_none());
  let newer = [
    ("path", json!("\u{fffd}.bin")),
    ("path_lossy", json!(true)),
    ("path_raw", raw("future_bytes", "Any Text")),
  ];
  let read_back = read(&newer).unwrap();
  assert_eq!(serde_json::to_value(&read_back).unwrap(), entry(&newer));
  assert_eq!(
    read_back
      .path()
      .and_then(cc_media_probe::source::PortablePath::to_native),
    None
  );
}

#[test]
fn bytes_read_is_a_number_or_beyond_the_exact_range_its_twin() {
  let large = read(&[("bytes_read_exact", json!("9007199254740993"))]).unwrap();
  assert_eq!(large.bytes_read(), Some(9_007_199_254_740_993));
  assert_eq!(
    serde_json::to_value(&large).unwrap()["bytes_read_exact"],
    json!("9007199254740993")
  );
  assert!(read(&[("bytes_read_exact", json!("1"))]).is_err());
  assert!(read(&[("bytes_read", json!(9_007_199_254_740_993_u64))]).is_err());
}
