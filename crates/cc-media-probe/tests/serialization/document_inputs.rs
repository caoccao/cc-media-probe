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

//! An entry's input (`api.md` sections 6.15 and 12; `json-output.md` section 5.8): its display string under
//! `input`, and for a path that is not valid Unicode the flag `input_lossy` and the raw form `input_raw` beside it,
//! the raw form only where the writing platform has one. The three members describe one path, by the rules of every
//! portable path, and a foreign path or one of an encoding of a newer version is kept as written.

use std::path::Path;

use cc_media_probe::document::{Document, DocumentEntry, Generator};
use cc_media_probe::source::PortablePath;
use serde_json::{Value, json};

use crate::fixtures::{document, error_record, reads_document};

/// A document of one error entry with these members of its input.
fn input_document(members: &[(&str, Value)]) -> Value {
  let mut entry = json!({ "index": 0, "outcome": "error", "error": error_record("1.0") });
  for (name, value) in members {
    entry[*name] = value.clone();
  }
  document("1.0", json!([entry]))
}

fn lossy(input: &str, raw: Option<(&str, &str)>) -> Value {
  let mut members = vec![("input", json!(input)), ("input_lossy", json!(true))];
  if let Some((encoding, data)) = raw {
    members.push(("input_raw", json!({ "encoding": encoding, "data": data })));
  }
  input_document(&members)
}

/// The document of one entry for that path, as this version writes it.
fn written_for(path: &Path) -> Value {
  let record = cc_media_probe::probe_bytes(b"").unwrap_err().to_record();
  let entry = DocumentEntry::from_error(PortablePath::from_path(path), record);
  serde_json::to_value(Document::from_results(Generator::new("catalog", "1"), vec![entry])).unwrap()
}

#[test]
fn a_valid_input_is_its_display_string_alone() {
  let written = written_for(Path::new("folder/movie.mkv"));
  let entry = &written["results"][0];
  assert_eq!(entry["input"], json!("folder/movie.mkv"));
  assert!(entry.get("input_lossy").is_none() && entry.get("input_raw").is_none());
}

#[cfg(unix)]
#[test]
fn a_lossy_input_has_its_flag_and_its_raw_form_beside_it() {
  use std::ffi::OsStr;
  use std::os::unix::ffi::OsStrExt;
  let path = Path::new(OsStr::from_bytes(b"caf\xe9.mkv"));
  let written = written_for(path);
  let entry = &written["results"][0];
  assert_eq!(entry["input"], json!("caf\u{FFFD}.mkv"));
  assert_eq!(entry["input_lossy"], json!(true));
  assert_eq!(
    entry["input_raw"],
    json!({ "encoding": "unix_bytes", "data": "636166e92e6d6b76" })
  );
  let read: Document = serde_json::from_value(written).unwrap();
  let input = read.results()[0].input();
  assert!(input.is_lossy());
  assert_eq!(input.to_native().as_deref(), Some(path));
}

#[cfg(windows)]
#[test]
fn a_lossy_input_has_its_flag_and_its_raw_form_beside_it() {
  use std::ffi::OsString;
  use std::os::windows::ffi::OsStringExt;
  let path = std::path::PathBuf::from(OsString::from_wide(&[0x0061, 0xd800]));
  let written = written_for(&path);
  let entry = &written["results"][0];
  assert_eq!(entry["input"], json!("a\u{FFFD}"));
  assert_eq!(
    entry["input_raw"],
    json!({ "encoding": "windows_wide", "data": "0061d800" })
  );
  let read: Document = serde_json::from_value(written).unwrap();
  assert_eq!(read.results()[0].input().to_native(), Some(path));
}

#[test]
fn a_foreign_input_is_kept_although_this_platform_cannot_rebuild_it() {
  // Each data spells `caf`, an ill-formed byte or an unpaired surrogate, and `.mkv`, so it decodes to the display.
  for (encoding, data, native_here) in [
    ("unix_bytes", "636166e92e6d6b76", cfg!(unix)),
    ("windows_wide", "006300610066d800002e006d006b0076", cfg!(windows)),
  ] {
    let text = lossy("caf\u{FFFD}.mkv", Some((encoding, data)));
    let read: Document = serde_json::from_value(text.clone()).unwrap();
    let input = read.results()[0].input();
    assert_eq!(input.raw().unwrap().data(), data);
    assert_eq!(input.to_native().is_some(), native_here, "{encoding}");
    assert_eq!(serde_json::to_value(&read).unwrap(), text);
  }
  // The data of an encoding that a newer version wrote is kept as written, unchecked.
  let newer = lossy("caf\u{FFFD}.mkv", Some(("vms_records", "Any Text")));
  let read: Document = serde_json::from_value(newer.clone()).unwrap();
  assert_eq!(read.results()[0].input().to_native(), None);
  assert_eq!(serde_json::to_value(&read).unwrap(), newer);
  // The flag alone is what a platform without an exact form of a path writes, and it reads on every platform.
  let flag_alone: Document = serde_json::from_value(lossy("caf\u{FFFD}.mkv", None)).unwrap();
  let input = flag_alone.results()[0].input();
  assert!(input.is_lossy() && input.raw().is_none() && input.to_native().is_none());
}

#[test]
fn an_input_whose_members_describe_different_paths_is_malformed() {
  let message = |document: Value| serde_json::from_value::<Document>(document).unwrap_err().to_string();
  let flag = "a path's raw form stands only beside its lossy flag, which is written only as true";
  let raw = json!({ "encoding": "unix_bytes", "data": "ff2e62696e" });
  for (members, expected) in [
    (vec![("input", json!("\u{fffd}.bin")), ("input_raw", raw.clone())], flag),
    (vec![("input", json!("movie.mkv")), ("input_lossy", json!(false))], flag),
    (
      vec![("input", json!("input.bin")), ("input_lossy", json!(true))],
      "a lossy path's display string holds no U+FFFD",
    ),
  ] {
    assert!(message(input_document(&members)).starts_with(expected), "{members:?}");
  }
  // Bytes that are valid UTF-8, or units that are valid UTF-16, were never lossy, and a display that is not the
  // lossy decoding of the raw data describes another path.
  for (input, encoding, data) in [
    ("\u{fffd}", "unix_bytes", "6f74686572"),
    ("\u{fffd}", "windows_wide", "0061"),
    ("\u{fffd}x.bin", "unix_bytes", "ff2e62696e"),
  ] {
    assert!(
      !reads_document(&lossy(input, Some((encoding, data)))),
      "{encoding} {data}"
    );
  }
  assert!(reads_document(&lossy(
    "\u{fffd}.bin",
    Some(("unix_bytes", "ff2e62696e"))
  )));
  assert!(reads_document(&lossy("\u{fffd}", Some(("windows_wide", "d800")))));
}

#[test]
fn the_raw_data_of_a_known_encoding_is_whole_units_of_lowercase_hexadecimal() {
  // Two lowercase hexadecimal digits a byte, and four a 16-bit unit, as the writer writes them.
  for (encoding, data) in [
    ("unix_bytes", "gg"),
    ("unix_bytes", "636"),
    ("unix_bytes", "+f"),
    ("unix_bytes", "E9"),
    ("windows_wide", "0061d8"),
    ("windows_wide", "+061"),
    ("windows_wide", "D800"),
  ] {
    assert!(
      !reads_document(&lossy("caf\u{FFFD}.mkv", Some((encoding, data)))),
      "{encoding} {data}"
    );
  }
}

#[test]
fn an_input_member_that_holds_null_is_malformed() {
  for members in [
    vec![("input", json!("movie.mkv")), ("input_lossy", Value::Null)],
    vec![("input", json!("movie.mkv")), ("input_raw", Value::Null)],
    vec![("input", Value::Null)],
    vec![("input", json!("movie.mkv")), ("report", Value::Null)],
  ] {
    assert!(!reads_document(&input_document(&members)), "{members:?}");
  }
  assert!(!reads_document(&input_document(&[])), "an entry names its input");
}
