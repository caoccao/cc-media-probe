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
//! fixture probed by the library in each of its cases through each adapter, blocking and async, under equal
//! conditions (`probes`). Each record is split into a semantic part, which the seekable adapters share and
//! `<name>[.<case>].expected.json` holds; a forward-only part, which `<name>[.<case>].forward.json` holds only where
//! it differs; and an adapter part for each adapter, which `<name>[.<case>].adapters.json` holds. A case with
//! `outcomes=per_adapter`, or a default case whose `default:` line says so, keeps each adapter's semantic part in
//! `<name>[.<case>].outcomes.json` instead of the first two, where a limit explains each difference.
//!
//! Before anything is compared or written, and before its paths are netted, each adapter's usage, and its partial
//! report's, is read strictly and stays within the limits that the adapter was given, the file adapters' raised,
//! and the bytes that the test's counters measured stay within the read limit and are the usage's bytes read. Each
//! source table of the record then lists the primary input with exactly the path that its adapter was given: the
//! file adapters' file, whose charge their raised limit always holds, and none for the others.
//!
//! The pass runs once in a process, through `checked_expectations`, which the seed check of plan Phase 3, task 20,
//! calls before it reads the default cases' expectations, so that with the variable set they are written before
//! they are read. The target requires every feature of the library, so that its expectations have one feature set
//! (section 4.3).

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::sync::OnceLock;

use cc_media_probe::Limits;
use cc_media_probe_testkit::fixture::{Expectation, Fixture, expectation_path, layout};
use cc_media_probe_testkit::syntax::Case;
use serde_json::{Map, Value, json};

use crate::common::conditions::{Outcomes, conditions};
use crate::common::expectation::{WRITE, check, check_absent, writing};
use crate::common::outcomes::{Outcome, per_adapter, shared};
use crate::common::sources::{Handed, normalize, primary_path};
use crate::common::split::split;
use crate::common::usage::{within_limits, within_reads};
use crate::folder::case_folder;
use crate::probes::{Probed, probes, raised};
use crate::records::{entry, report};

/// Checks the record of `probe` as the library wrote it, against the limits that its adapter was given and the
/// reads that its counter saw, and its primary input against the path that the adapter was given, the first of
/// `handed` or none, and then writes its paths with the placeholder and its retained data net of the paths in
/// `handed`. Checked after the netting, against the case's limits, a file probe that passed its raised limit by
/// less than a built-in companion's charge would pass, as the sixth review of the proposal found; and a file
/// report without its path would have the whole raised limit for the rest, as the seventh found.
fn prepared(probe: &mut Probed, folder: &Path, handed: &[Handed]) -> Result<(), String> {
  within_limits(&probe.record, &probe.limits, probe.adapter)?;
  within_reads(&probe.record, probe.counts, &probe.limits, probe.adapter)?;
  let given = handed.first().filter(|_| probe.given_path);
  primary_path(&probe.record, given, probe.adapter)?;
  normalize(&mut probe.record, folder, handed);
  Ok(())
}

/// The adapter part of a probe, with the reads that its counter saw.
fn adapter_part(probe: &Probed, mut part: Map<String, Value>) -> Value {
  if let Some(counts) = probe.counts {
    part.insert("read_calls".to_owned(), json!(counts.read_calls));
    part.insert("bytes_delivered".to_owned(), json!(counts.delivered));
  }
  Value::Object(part)
}

/// The value of `result`, or a failure that names fixture `name`.
fn passed<Passed>(name: &str, result: Result<Passed, String>) -> Passed {
  result.unwrap_or_else(|message| panic!("{name}: {message}"))
}

/// Checks one case of `fixture`, and gives the parts that it handled: those that it compared or wrote, and the
/// forward-only part that it found absent or removed.
fn check_case(fixture: &Fixture, case: Option<&Case>) -> Vec<(Option<String>, &'static str)> {
  let name = case.map(|case| case.name.as_str());
  let conditions = conditions(case, &fixture.header.default_conditions)
    .unwrap_or_else(|refused| panic!("{}: {refused}", fixture.name));
  let folder = case_folder();
  let primary = Handed::new(&folder.path, fixture.file_name());
  let probed = probes(fixture, &conditions, &folder.path, primary.length);
  let handed = [primary];
  let mut adapters = Map::new();
  let mut outcomes = Vec::new();
  for mut probe in probed {
    passed(&fixture.name, prepared(&mut probe, &folder.path, &handed));
    let (semantic, part) = split(&probe.record);
    adapters.insert(probe.adapter.to_owned(), adapter_part(&probe, part));
    outcomes.push(Outcome {
      adapter: probe.adapter,
      forward_only: probe.forward_only,
      semantic,
    });
  }
  let case_name = name.map(str::to_owned);
  let mut accounted = vec![(case_name.clone(), "adapters")];
  if conditions.outcomes == Outcomes::PerAdapter {
    check(
      &fixture.name,
      name,
      "outcomes",
      &passed(&fixture.name, per_adapter(&outcomes)),
    );
    accounted.push((case_name, "outcomes"));
  } else {
    let (seekable, forward) = passed(&fixture.name, shared(&outcomes));
    check(&fixture.name, name, "expected", &seekable);
    accounted.push((case_name.clone(), "expected"));
    match forward.filter(|forward| *forward != seekable) {
      Some(forward) => check(&fixture.name, name, "forward", &forward),
      None => check_absent(&fixture.name, name, "forward"),
    }
    accounted.push((case_name, "forward"));
  }
  check(&fixture.name, name, "adapters", &Value::Object(adapters));
  accounted
}

/// Checks every case of every fixture against its expectations, or writes them, and refuses or removes the files
/// that no case needs. The layout is read once, before any file is written, and its expectation files are grouped
/// by fixture: the pass read the whole layout again for each fixture before the review of plan Phase 3, task 4,
/// found the work growing with the square of the fixtures.
fn check_every_fixture() {
  let layout = layout().unwrap();
  let mut existing: BTreeMap<&str, Vec<&Expectation>> = BTreeMap::new();
  for file in &layout.expectations {
    existing.entry(file.fixture.as_str()).or_default().push(file);
  }
  for name in &layout.fixtures {
    let fixture = Fixture::load(name).unwrap();
    let mut accounted = check_case(&fixture, None);
    for case in &fixture.header.cases {
      accounted.extend(check_case(&fixture, Some(case)));
    }
    let stale = existing.get(name.as_str()).into_iter().flatten().filter(|file| {
      !accounted
        .iter()
        .any(|(case, part)| *case == file.case && *part == file.part)
    });
    for file in stale {
      let path = expectation_path(name, file.case.as_deref(), &file.part);
      assert!(
        writing(),
        "{}: no case needs it; set {WRITE} to remove it",
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
fn every_fixture_meets_its_expectations_in_every_case_through_every_adapter() {
  checked_expectations();
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

/// The file adapter's probe of a file below `/tmp/abcdefg` under a case's retained-data limit of 128 bytes, raised
/// by its path's 18, whose report retains `retained` bytes and lists `sources`.
fn file_probe(retained: u64, sources: &[Value]) -> Probed {
  Probed {
    adapter: "file",
    forward_only: false,
    record: report(retained, sources),
    counts: None,
    limits: raised(&Limits::new().with_retained_data(128), 18),
    given_path: true,
  }
}

#[test]
fn a_record_is_checked_against_its_adapters_limits_before_its_paths_are_netted() {
  let folder = Path::new("/tmp/abcdefg");
  let handed = [Handed::new(folder, "x.bin"), Handed::new(folder, "x.idx")];
  assert_eq!((handed[0].length, handed[1].length), (18, 18));
  // A report whose companion was opened, as the seventh review asked: 18 bytes for each path and 110 for the rest.
  let file = |retained| {
    file_probe(
      retained,
      &[
        entry("0", "opened", &handed[0].members),
        entry("1", "opened", &handed[1].members),
      ],
    )
  };
  let error = prepared(&mut file(147), folder, &handed).unwrap_err();
  assert!(error.ends_with("file: /usage/retained_data 147 > 146"), "{error}");
  let mut within = file(146);
  prepared(&mut within, folder, &handed).unwrap();
  assert_eq!(within.record["usage"]["retained_data"], json!(110));
  assert_eq!(within.record["sources"][1]["path"], "{folder}/x.idx");
}

#[test]
fn a_file_report_keeps_the_path_that_its_raised_limit_pays_for() {
  let folder = Path::new("/tmp/abcdefg");
  let handed = [Handed::new(folder, "x.bin")];
  // The review's case: 146 bytes retained within the raised limit, and the primary input listed without its path.
  let mut pathless = file_probe(146, &[entry("0", "opened", &Map::new())]);
  let error = prepared(&mut pathless, folder, &handed).unwrap_err();
  assert!(
    error.contains("not the one that its adapter was given at /sources of file"),
    "{error}"
  );
  let mut kept = file_probe(146, &[entry("0", "opened", &handed[0].members)]);
  prepared(&mut kept, folder, &handed).unwrap();
  assert_eq!(kept.record["usage"]["retained_data"], json!(128));
  assert_eq!(kept.record["sources"][0]["path"], "{folder}/x.bin");
}
