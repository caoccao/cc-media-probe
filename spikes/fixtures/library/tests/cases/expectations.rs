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

//! Expectations in the library's JSON form (plan sections 9.1 and 9.2, `docs/design/test-kit.md` section 4): each
//! fixture probed by the real library in each of its cases through each blocking adapter, under equal conditions.
//! Each case's prober starts from `fuzzing_entry_points::prober`, the one that the `probe` target runs, with
//! lookup off unless the case asks for it; every adapter is given the fixture's file name, the file adapter as its
//! file's name and the others as the hint, or a hint-only case's hint, or a nameless case's none, without the file
//! adapter; and the file adapter's retained-data limit is raised by its path's charge, so that the rest of its probe
//! has the others' budget exactly. Each record is split into a semantic part, which the seekable adapters share and
//! `<name>[.<case>].expected.json` holds; a forward-only part, which `<name>[.<case>].forward.json` holds only where
//! it differs; and an adapter part for each adapter, which `<name>[.<case>].adapters.json` holds. The pass runs once
//! in a process, before the seed check reads the default cases' expectations, so that with the variable set they
//! are written before they are read. In Phase 2 the library reads no input, so every record is the error of an
//! unknown format, whatever the case.

use std::fs;
use std::io::{self, Cursor};
use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use std::sync::atomic::{AtomicU32, Ordering};

use cc_media_probe::source::PositionalSource;
use cc_media_probe::{Error, Limits, MediaReport};
use kit::fixture::{Fixture, layout, names};
use kit::readers::{CountingReader, Counts, FaultingReader, Pieces};
use kit::syntax::Case;
use library::fuzzing_entry_points::prober;
use serde_json::{Map, Value, json};

use crate::common::conditions::{Conditions, Lookup, Naming, conditions};
use crate::common::expectation::{check, check_absent, writing};
use crate::common::sources::{Handed, normalize};
use crate::common::split::split;

/// The length of every case's folder, in bytes of its display string, so that each path below it has the same
/// charge on every machine. A file name of 255 bytes below it stays under the 1,024 bytes of a path on macOS.
const FOLDER_LENGTH: usize = 512;
/// The most bytes of one component of the folder.
const MAXIMUM_COMPONENT: usize = 255;
/// The width, in hexadecimal digits, of the folder's components for the run and for the case.
const COMPONENT_WIDTH: usize = 8;

/// The cases that this process has given a folder, so that each has its own.
static CASES: AtomicU32 = AtomicU32::new(0);

/// A custom positional source over bytes in memory, which delivers in pieces and counts its reads.
struct Memory {
  /// The bytes.
  bytes: Vec<u8>,
  /// The pieces of the input.
  pieces: Pieces,
  /// Its reads so far.
  counts: Counts,
}

impl PositionalSource for Memory {
  fn read_at(&mut self, position: u64, buffer: &mut [u8]) -> io::Result<usize> {
    let length = u64::try_from(self.bytes.len()).unwrap();
    if position >= length {
      self.counts.record(0);
      return Ok(0);
    }
    let rest = &self.bytes[usize::try_from(position).unwrap()..];
    let piece = self.pieces.cut(position, buffer);
    let delivered = piece.len().min(rest.len());
    piece[..delivered].copy_from_slice(&rest[..delivered]);
    self.counts.record(delivered);
    Ok(delivered)
  }

  fn length(&mut self) -> io::Result<Option<u64>> {
    Ok(Some(u64::try_from(self.bytes.len()).unwrap()))
  }
}

/// A probe of a fixture through one adapter.
struct Probed {
  /// The adapter's name.
  adapter: &'static str,
  /// Whether the adapter is forward only.
  forward_only: bool,
  /// The report or error record in the JSON form.
  record: Value,
  /// The reads that the adapter's counter saw, where the test counts them. The file adapter's are counted by the
  /// repository-only counter of `docs/design/test-kit.md` section 7, which the I/O layer of task 6 brings.
  counts: Option<Counts>,
}

/// The folder of one case: the case's own component, removed when the case is done, and the padded folder below it.
struct CaseFolder {
  /// The case's component below the run's.
  root: PathBuf,
  /// The padded folder that the case writes its files into.
  path: PathBuf,
}

impl Drop for CaseFolder {
  fn drop(&mut self) {
    let _ = fs::remove_dir_all(&self.root);
  }
}

/// The record of a probe's result in the JSON form.
fn record(result: Result<MediaReport, Error>) -> Value {
  match result {
    Ok(report) => serde_json::to_value(&report).unwrap(),
    Err(error) => serde_json::to_value(error.to_record()).unwrap(),
  }
}

/// A reader of `bytes` that delivers in the case's pieces and counts its reads.
fn reader(bytes: &[u8], conditions: &Conditions) -> CountingReader<FaultingReader<Cursor<Vec<u8>>>> {
  CountingReader::new(FaultingReader::new(Cursor::new(bytes.to_vec())).with_pieces(&conditions.pieces))
}

/// The limits of the file adapters: the case's, with the retained-data limit raised by the charge of the paths that
/// the test hands the library, which the library charges before anything else that the probe retains.
fn raised(limits: &Limits, charge: u64) -> Limits {
  limits
    .clone()
    .with_retained_data(limits.retained_data().saturating_add(charge))
}

/// The fixture probed through each blocking adapter under `conditions`: a file written into `folder` under the
/// fixture's file name, unless the case gives another name or none, bytes, a seekable reader, a custom positional
/// source and a forward-only reader.
fn probes(fixture: &Fixture, conditions: &Conditions, folder: &Path, handed: &[Handed]) -> Vec<Probed> {
  assert!(
    conditions.lookup == Lookup::Off,
    "{}: the library has no companion lookup before Phase 14, task 1",
    fixture.name
  );
  let bytes = &fixture.body.bytes;
  let hint = match &conditions.naming {
    Naming::FileName => Some(fixture.file_name()),
    Naming::Hint(hint) => Some(hint.as_str()),
    Naming::Absent => None,
  };
  let prober = prober(hint.map(Into::into)).with_limits(conditions.limits.clone());
  let result = |adapter, forward_only, record, counts| Probed {
    adapter,
    forward_only,
    record,
    counts,
  };
  let mut seekable = reader(bytes, conditions);
  let read = record(prober.probe_reader(&mut seekable));
  let mut forward = reader(bytes, conditions);
  let streamed = record(prober.probe_stream(&mut forward));
  let mut memory = Memory {
    bytes: bytes.clone(),
    pieces: Pieces::new(&conditions.pieces),
    counts: Counts::default(),
  };
  let sourced = record(prober.probe_source(&mut memory));
  let mut results = vec![
    result("bytes", false, record(prober.probe_bytes(bytes)), None),
    result("reader", false, read, Some(seekable.counts())),
    result("source", false, sourced, Some(memory.counts)),
    result("stream", true, streamed, Some(forward.counts())),
  ];
  if conditions.has_file() {
    let path = folder.join(fixture.file_name());
    fs::write(&path, bytes).unwrap();
    let charge = handed.iter().map(|path| path.length).sum();
    let file_prober = prober.with_limits(raised(&conditions.limits, charge));
    results.insert(1, result("file", false, record(file_prober.probe_file(&path)), None));
  }
  results
}

/// The adapter part of a probe, with the reads that its counter saw.
fn adapter_part(probe: &Probed, mut part: Map<String, Value>) -> Value {
  if let Some(counts) = probe.counts {
    part.insert("read_calls".to_owned(), json!(counts.read_calls));
    part.insert("bytes_delivered".to_owned(), json!(counts.delivered));
  }
  Value::Object(part)
}

/// A folder of its own for the files of one case, below Cargo's folder for integration tests: a component for the
/// run, the process's identifier, and one for the case, its number in the process, each of `COMPONENT_WIDTH`
/// hexadecimal digits, so that concurrent runs and cases never share a folder and no name of a fixture or a case
/// counts. Components of `_` pad it to `FOLDER_LENGTH` bytes, at most `MAXIMUM_COMPONENT` each, so that each path
/// below it has the same charge on every machine. The folder must be valid Unicode, so that no path below it is
/// lossy, and short enough to pad.
fn case_folder() -> CaseFolder {
  let width = COMPONENT_WIDTH;
  let run = format!("{:0width$x}", std::process::id());
  let mut component = format!("{:0width$x}", CASES.fetch_add(1, Ordering::Relaxed));
  let mut path = Path::new(env!("CARGO_TARGET_TMPDIR")).join("expectations").join(run);
  let root = path.join(&component);
  let length = path
    .to_str()
    .unwrap_or_else(|| panic!("{}: the folder for tests is valid Unicode", path.display()))
    .len();
  let mut remaining = FOLDER_LENGTH
    .checked_sub(length)
    .filter(|remaining| *remaining > component.len())
    .unwrap_or_else(|| {
      panic!(
        "{}: no room below it for a folder of {FOLDER_LENGTH} bytes; set CARGO_TARGET_DIR to a shorter folder",
        path.display()
      )
    });
  while remaining > 0 {
    let available = remaining.saturating_sub(1);
    let mut taken = available.min(MAXIMUM_COMPONENT);
    if available.saturating_sub(taken) == 1 {
      taken = taken.saturating_sub(1);
    }
    component.push_str(&"_".repeat(taken.saturating_sub(component.len())));
    path.push(&component);
    remaining = available.saturating_sub(taken);
    component = String::new();
  }
  let _ = fs::remove_dir_all(&root);
  fs::create_dir_all(&path).unwrap();
  CaseFolder { root, path }
}

/// Checks one case of `fixture`, and gives the parts that it compared.
fn check_case(fixture: &Fixture, case: Option<&Case>) -> Vec<(Option<String>, &'static str)> {
  let name = case.map(|case| case.name.as_str());
  let conditions =
    conditions(case, fixture.file_name()).unwrap_or_else(|refused| panic!("{}: {refused}", fixture.name));
  let folder = case_folder();
  let handed = [Handed::new(&folder.path, fixture.file_name())];
  let mut adapters = Map::new();
  let mut seekable: Option<Value> = None;
  let mut forward = None;
  for mut probe in probes(fixture, &conditions, &folder.path, &handed) {
    normalize(&mut probe.record, &folder.path, &handed);
    let (semantic, part) = split(&probe.record);
    adapters.insert(probe.adapter.to_owned(), adapter_part(&probe, part));
    if probe.forward_only {
      forward = Some(semantic);
    } else if let Some(first) = &seekable {
      assert_eq!(
        &semantic, first,
        "{}: the semantic part through {} differs",
        fixture.name, probe.adapter
      );
    } else {
      seekable = Some(semantic);
    }
  }
  let (seekable, forward) = (seekable.unwrap(), forward.unwrap());
  let case_name = name.map(str::to_owned);
  let mut compared = vec![(case_name.clone(), "expected"), (case_name.clone(), "adapters")];
  check(&fixture.name, name, "expected", &seekable);
  if forward == seekable {
    check_absent(&fixture.name, name, "forward");
  } else {
    check(&fixture.name, name, "forward", &forward);
    compared.push((case_name, "forward"));
  }
  check(&fixture.name, name, "adapters", &Value::Object(adapters));
  compared
}

/// Checks every case of every fixture against its expectations, or writes them, and refuses or removes the files
/// that no case needs.
fn check_every_fixture() {
  for name in names().unwrap() {
    let fixture = Fixture::load(&name).unwrap();
    let mut compared = check_case(&fixture, None);
    for case in &fixture.header.cases {
      compared.extend(check_case(&fixture, Some(case)));
    }
    let stale: Vec<_> = layout()
      .unwrap()
      .expectations
      .into_iter()
      .filter(|file| file.fixture == name)
      .filter(|file| {
        !compared
          .iter()
          .any(|(case, part)| *case == file.case && *part == file.part)
      })
      .collect();
    for file in stale {
      let path = kit::fixture::expectation_path(&name, file.case.as_deref(), &file.part);
      assert!(
        writing(),
        "{}: no case needs it; set the variable to remove it",
        path.display()
      );
      fs::remove_file(path).unwrap();
    }
  }
}

/// Runs the golden pass once in this process, before anything reads the expectations. A pass that fails fails
/// every caller, the seed check among them, since what it would read is wrong or missing.
pub fn checked_expectations() {
  static CHECKED: OnceLock<()> = OnceLock::new();
  CHECKED.get_or_init(check_every_fixture);
}

#[test]
fn every_fixture_meets_its_expectations_in_every_case_through_every_blocking_adapter() {
  checked_expectations();
}

#[test]
fn a_path_below_the_folder_is_written_with_the_placeholder_and_taken_out_of_the_retained_data() {
  let folder = Path::new(env!("CARGO_TARGET_TMPDIR")).join("placeholder");
  let path = folder.join("movie.idx");
  let sibling = Path::new(env!("CARGO_TARGET_TMPDIR"))
    .join("placeholder2")
    .join("movie.idx");
  let mut record = json!({
    "schema_version": "1.0",
    "usage": { "retained_data": 1000 },
    "sources": [
      { "identifier": "0", "path": path.to_str().unwrap() },
      { "identifier": "1", "path": "/elsewhere/movie.sub" },
      { "identifier": "2", "path": sibling.to_str().unwrap() },
    ],
  });
  normalize(&mut record, &folder, &[Handed::new(&folder, "movie.idx")]);
  let length = u64::try_from(path.as_os_str().len()).unwrap();
  assert_eq!(
    record["sources"][0],
    json!({ "identifier": "0", "path": "{folder}/movie.idx" })
  );
  assert_eq!(record["sources"][1]["path"], "/elsewhere/movie.sub");
  assert_eq!(record["sources"][2]["path"], sibling.to_str().unwrap());
  assert_eq!(record["usage"]["retained_data"], json!(1000 - length));
}

#[test]
fn a_partial_reports_path_is_taken_out_of_the_errors_usage_too() {
  let folder = Path::new(env!("CARGO_TARGET_TMPDIR")).join("placeholder");
  let path = folder.join("movie.idx");
  let mut record = json!({
    "kind": "source_failed",
    "usage": { "retained_data": 1000 },
    "partial_report": {
      "usage": { "retained_data": 900 },
      "sources": [{ "identifier": "0", "path": path.to_str().unwrap() }],
    },
  });
  normalize(&mut record, &folder, &[Handed::new(&folder, "movie.idx")]);
  let length = u64::try_from(path.as_os_str().len()).unwrap();
  assert_eq!(record["usage"]["retained_data"], json!(1000 - length));
  assert_eq!(record["partial_report"]["usage"]["retained_data"], json!(900 - length));
}

#[test]
#[should_panic(expected = "written as its form gives it")]
fn a_unicode_path_written_as_lossy_fails_the_comparison() {
  let folder = Path::new(env!("CARGO_TARGET_TMPDIR")).join("placeholder");
  let mut record = json!({
    "usage": { "retained_data": 1000 },
    "sources": [{
      "identifier": "0",
      "path": folder.join("movie.idx").to_str().unwrap(),
      "path_lossy": true,
      "path_raw": { "encoding": "unix_bytes", "data": "6d" },
    }],
  });
  normalize(&mut record, &folder, &[Handed::new(&folder, "movie.idx")]);
}

#[test]
fn every_case_folder_has_the_same_length_and_one_of_its_own() {
  let folders = [case_folder(), case_folder()];
  assert_ne!(folders[0].path, folders[1].path);
  for folder in &folders {
    assert_eq!(folder.path.to_str().unwrap().len(), FOLDER_LENGTH);
    assert!(folder.path.is_dir());
    let below = folder
      .path
      .strip_prefix(Path::new(env!("CARGO_TARGET_TMPDIR")))
      .unwrap();
    assert!(below.iter().all(|component| component.len() <= MAXIMUM_COMPONENT));
  }
  let root = folders[0].root.clone();
  drop(folders);
  assert!(!root.exists());
}

#[test]
fn the_file_adapters_limit_is_raised_by_the_paths_charge_alone() {
  let limits = Limits::new().with_retained_data(10).with_streams(3);
  let file = raised(&limits, 7);
  assert_eq!((file.retained_data(), file.streams()), (17, 3));
  assert_eq!(
    raised(&Limits::new().with_retained_data(u64::MAX), 7).retained_data(),
    u64::MAX
  );
}

#[test]
fn a_custom_source_gives_no_bytes_at_and_beyond_its_end() {
  let mut memory = Memory {
    bytes: vec![1, 2, 3],
    pieces: Pieces::new(&[2]),
    counts: Counts::default(),
  };
  let mut buffer = [0_u8; 8];
  assert_eq!(memory.read_at(0, &mut buffer).unwrap(), 2);
  assert_eq!(memory.read_at(2, &mut buffer).unwrap(), 1);
  assert_eq!(memory.read_at(3, &mut buffer).unwrap(), 0);
  assert_eq!(memory.read_at(u64::MAX, &mut buffer).unwrap(), 0);
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
