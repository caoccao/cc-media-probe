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

//! The fixtures' layout and the seeds that a fixture gives (plan section 9.5): the whole fixture to its header's
//! target, and each region to its own, with the representation and context of a codec target, by names that no
//! other seed has.

use std::fs;
use std::path::Path;

use kit::fixture::{Expectation, Fixture, FixtureError, layout, layout_of, names};
use kit::syntax::is_hyphenated_name;

/// What a test compares of a seed: its name, target, representation, context, outcome and length.
type Summary<'seed> = (
  &'seed str,
  &'seed str,
  Option<&'seed str>,
  &'seed [u8],
  &'seed str,
  usize,
);

#[test]
fn the_fixtures_folder_holds_the_two_samples_among_its_fixtures() {
  let names = names().unwrap();
  for sample in ["flac/decoding-example-1", "toy/setup-and-parameters"] {
    assert!(names.iter().any(|name| name == sample), "{sample}");
  }
  let layout = layout().unwrap();
  assert!(layout.expectations.contains(&Expectation {
    fixture: "toy/setup-and-parameters".to_owned(),
    case: Some("short-reads".to_owned()),
    part: "adapters".to_owned(),
  }));
}

#[test]
fn a_file_or_folder_that_the_layout_does_not_allow_is_an_error() {
  let base = Path::new(env!("CARGO_TARGET_TMPDIR")).join("layout");
  let refused = |relative: &str| {
    let _ = fs::remove_dir_all(&base);
    fs::create_dir_all(base.join("toy")).unwrap();
    fs::write(base.join("toy").join("sample.hex"), "").unwrap();
    let path = base.join(relative);
    if relative.ends_with('/') {
      fs::create_dir_all(&path).unwrap();
    } else {
      fs::create_dir_all(path.parent().unwrap()).unwrap();
      fs::write(&path, "").unwrap();
    }
    match layout_of(&base) {
      Err(FixtureError::Misplaced(found, _)) => found.strip_prefix(&base).unwrap().to_path_buf(),
      other => panic!("{relative}: {other:?}"),
    }
  };
  for (relative, reported) in [
    ("loose.hex", "loose.hex"),
    ("toy/deeper/", "toy/deeper"),
    ("Flac/sample.hex", "Flac"),
    ("toy/Other.hex", "toy/Other.hex"),
    ("toy/sample.txt", "toy/sample.txt"),
    (
      "toy/sample.split.header.expected.json",
      "toy/sample.split.header.expected.json",
    ),
    ("toy/sample.answers.json", "toy/sample.answers.json"),
  ] {
    assert_eq!(refused(relative), Path::new(reported), "{relative}");
  }
  assert_eq!(refused("toy/other.expected.json"), Path::new("toy/other.expected.json"));
  let _ = fs::remove_dir_all(&base);
  fs::create_dir_all(base.join("toy")).unwrap();
  for file in [
    "sample.hex",
    "sample.expected.json",
    "sample.split-header.adapters.json",
  ] {
    fs::write(base.join("toy").join(file), "").unwrap();
  }
  let layout = layout_of(&base).unwrap();
  assert_eq!(layout.fixtures, ["toy/sample"]);
  assert_eq!(layout.expectations.len(), 2);
  fs::remove_dir_all(&base).unwrap();
}

#[cfg(unix)]
#[test]
fn a_file_name_that_is_not_unicode_is_an_error() {
  use std::ffi::OsStr;
  use std::os::unix::ffi::OsStrExt;

  let base = Path::new(env!("CARGO_TARGET_TMPDIR")).join("layout-unicode");
  let _ = fs::remove_dir_all(&base);
  fs::create_dir_all(base.join("toy")).unwrap();
  let name = OsStr::from_bytes(b"sample\xff.hex");
  if fs::write(base.join("toy").join(name), "").is_err() {
    return;
  }
  assert!(matches!(layout_of(&base), Err(FixtureError::Misplaced(..))));
  fs::remove_dir_all(&base).unwrap();
}

#[test]
fn a_fixture_is_loaded_only_by_a_name_of_the_layout() {
  for name in [
    "toy",
    "toy/Sample",
    "toy/a/b",
    "../toy/setup-and-parameters",
    "toy/nul",
    "com1/sample",
  ] {
    assert!(
      matches!(Fixture::load(name), Err(FixtureError::MalformedName(_))),
      "{name}"
    );
  }
}

#[test]
fn a_name_of_the_layout_is_no_device_name_that_windows_reserves() {
  assert!(is_hyphenated_name("nul-a"));
  assert!(is_hyphenated_name("com10"));
  for name in ["nul", "con", "prn", "aux", "com1", "lpt9"] {
    assert!(!is_hyphenated_name(name), "{name}");
  }
}

#[test]
fn a_container_fixture_gives_its_embedded_codec_header_as_a_seed_of_its_own() {
  let fixture = Fixture::load("toy/setup-and-parameters").unwrap();
  let seeds = fixture.seeds();
  let routes: Vec<Summary<'_>> = seeds
    .iter()
    .map(|seed| {
      (
        seed.name.as_str(),
        seed.route.target.as_str(),
        seed.route.representation.as_deref(),
        seed.route.context.as_slice(),
        seed.route.outcome.as_str(),
        seed.payload.len(),
      )
    })
    .collect();
  assert_eq!(
    routes,
    [
      ("toy/setup-and-parameters", "toy", None, &[][..], "report", 48),
      (
        "toy/setup-and-parameters#1",
        "toy_codec",
        Some("one_byte_lengths"),
        &[0, 0, 0, 0, 12][..],
        "parsed",
        12
      ),
    ]
  );
  assert_eq!(fixture.file_name(), "setup-and-parameters");
}
