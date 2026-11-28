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

//! The seed check (plan section 9.5), which is also the corpus tool: every seed of every fixture, in its target's
//! input encoding, reaches the parser that it was written for and gives the outcome that its fixture records. Each
//! fixture with a target is also a seed of the full public probe, which probes bytes in memory, so its outcome is
//! that of the bytes adapter in the default case's expectation: `expected`, or the `bytes` member of `outcomes`
//! where the default case's adapters keep their own; `report`, or the error record's kind. So a whole fixture
//! never names `probe` as its target, while a region may. The check
//! reads those expectations only after the golden pass has run in the same process, so that with
//! `CC_MEDIA_PROBE_WRITE_EXPECTATIONS` set it reads what that pass wrote. Every seed is within the fuzz run's input
//! length whatever its target, before a target that a fixture names ahead of the library is let through on the
//! list of later targets. With `CC_MEDIA_PROBE_WRITE_CORPUS` set, the check then writes every seed into
//! `fuzz/corpus/<target>/`, so the corpus holds only seeds that passed it. It runs as an ordinary test of a
//! repository build, where `fuzzing_entry_points` is compiled too.

use std::fs;
use std::path::Path;

use kit::fixture::{Fixture, expectation_path, names};
use kit::syntax::parse;
use library::fuzzing_entry_points::{EncodingError, MAXIMUM_INPUT_LENGTH, Parser, Target, encode, run};
use serde_json::Value;

use crate::common::conditions::{Outcomes, conditions};
use crate::common::corpus::{self, Checked, file_name};
use crate::common::expectation::stored;
use crate::expectations::checked_expectations;

/// The targets that a fixture may name before the library has them, each with the phase that adds it.
const LATER_TARGETS: [(&str, &str); 2] = [("flac", "Phase 9"), ("flac_codec", "Phase 4")];

/// The outcome of the full public probe, which probes bytes in memory, that the default case's expectation of
/// `fixture` records: its `expected`, or the `bytes` member of its `outcomes` where its adapters keep their own, as
/// the fifth review of the proposal asked, which found a reader stopped by the default read limit beside bytes in
/// memory that completed.
fn probe_outcome(fixture: &Fixture) -> String {
  bytes_outcome(fixture, |part| stored(&expectation_path(&fixture.name, None, part)))
}

/// The outcome of the bytes adapter in the default case of `fixture`, whose expectation `part` `stored` gives.
fn bytes_outcome(fixture: &Fixture, stored: impl Fn(&str) -> Value) -> String {
  let default = conditions(None, &fixture.header.default_conditions, fixture.file_name())
    .unwrap_or_else(|refused| panic!("{}: {refused}", fixture.name));
  let expected = match default.outcomes {
    Outcomes::Shared => stored("expected"),
    Outcomes::PerAdapter => stored("outcomes")["bytes"].clone(),
  };
  expected
    .get("kind")
    .and_then(Value::as_str)
    .unwrap_or("report")
    .to_owned()
}

/// Every seed of every fixture, checked, once the golden pass has compared or written the expectations.
fn checked_seeds() -> Vec<Checked> {
  checked_expectations();
  names()
    .unwrap()
    .iter()
    .flat_map(|name| checked_seeds_of(&Fixture::load(name).unwrap()))
    .collect()
}

/// Every seed of `fixture`, checked: its own, then the whole fixture's of the `probe` target.
fn checked_seeds_of(fixture: &Fixture) -> Vec<Checked> {
  let name = &fixture.name;
  let mut checked = Vec::new();
  if let Some(route) = &fixture.header.route {
    assert_ne!(
      route.target,
      Target::Probe.name(),
      "{name}: the whole fixture is a seed of `probe` already; route a region to it, or name its format's target"
    );
  }
  for seed in fixture.seeds() {
    let route = seed.route;
    assert!(
      seed.payload.len() <= MAXIMUM_INPUT_LENGTH,
      "{}: {} bytes, beyond the fuzz run's input length of {MAXIMUM_INPUT_LENGTH} for any target",
      seed.name,
      seed.payload.len()
    );
    let Some(target) = Target::from_name(&route.target) else {
      assert!(
        LATER_TARGETS.iter().any(|(later, _)| *later == route.target),
        "{}: no target {}, and it is not on the list of later targets",
        seed.name,
        route.target
      );
      continue;
    };
    let input = encode(
      target,
      route.representation.as_deref(),
      &route.context,
      fixture.file_name(),
      seed.payload,
    )
    .unwrap_or_else(|error| panic!("{}: {error:?}", seed.name));
    let reached = run(target, &input);
    assert_eq!(
      (reached.parser, reached.outcome.as_str()),
      (Some(target.parser()), route.outcome.as_str()),
      "{}",
      seed.name
    );
    checked.push(Checked {
      target: target.name(),
      name: seed.name,
      input,
    });
  }
  if fixture.header.route.is_some() {
    let input = encode(Target::Probe, None, &[], fixture.file_name(), &fixture.body.bytes)
      .unwrap_or_else(|error| panic!("{name}: {error:?}"));
    let reached = run(Target::Probe, &input);
    assert_eq!(
      (reached.parser, reached.outcome),
      (Some(Parser::PublicProbe), probe_outcome(fixture)),
      "{name}: the public probe"
    );
    checked.push(Checked {
      target: Target::Probe.name(),
      name: name.clone(),
      input,
    });
  }
  checked
}

/// The fixture `name` of `text`, which no file holds.
fn fixture_of(name: &str, text: &str) -> Fixture {
  let parsed = parse(text).unwrap();
  Fixture {
    name: name.to_owned(),
    header: parsed.header,
    body: parsed.body,
  }
}

#[test]
fn the_probe_targets_outcome_is_the_bytes_adapters_in_the_default_case() {
  let text =
    |default: &str| format!("description: a\nsource: b\ntarget: toy\noutcome: report\n{default}---\n## a\n00 # f\n");
  let parts = |part: &str| match part {
    "expected" => serde_json::json!({ "kind": "unknown_format" }),
    "outcomes" => serde_json::json!({ "bytes": { "format": "toy" }, "reader": { "kind": "format_not_determined" } }),
    other => panic!("{other}: no such part"),
  };
  let shared = fixture_of("test/shared", &text(""));
  assert_eq!(bytes_outcome(&shared, parts), "unknown_format");
  let each = fixture_of("test/each", &text("default: outcomes=per_adapter\n"));
  assert_eq!(bytes_outcome(&each, parts), "report");
}

#[test]
fn a_region_may_route_to_the_probe_target_with_its_own_outcome() {
  let fixture = fixture_of(
    "test/region-probe",
    "description: a\nsource: b\ntarget: none\n---\n## a\n\"ab\" # f\n@seed target=probe outcome=unknown_format\n00 # f\n@end\n",
  );
  let checked = checked_seeds_of(&fixture);
  assert_eq!(checked.len(), 1);
  assert_eq!(
    (checked[0].target, checked[0].name.as_str()),
    ("probe", "test/region-probe#1")
  );
  assert_eq!(checked[0].input, b"\x0cregion-probe\x00");
}

#[test]
#[should_panic(expected = "beyond the fuzz run's input length")]
fn a_seed_beyond_the_input_length_fails_before_its_target_is_deferred() {
  let fixture = fixture_of(
    "test/large-region",
    "description: a\nsource: b\ntarget: none\n---\n## a\n@seed target=flac outcome=report\n00 * 1048577 # f\n@end\n",
  );
  let _checked = checked_seeds_of(&fixture);
}

#[test]
#[should_panic(expected = "the whole fixture is a seed of `probe` already")]
fn a_whole_fixture_routed_to_the_probe_target_is_refused() {
  let fixture = fixture_of(
    "test/whole-probe",
    "description: a\nsource: b\ntarget: probe\noutcome: unknown_format\n---\n## a\n00 # f\n",
  );
  let _checked = checked_seeds_of(&fixture);
}

#[test]
fn every_seed_reaches_its_parser_with_the_outcome_that_its_fixture_records() {
  let checked = checked_seeds();
  for (target, name) in [
    ("toy", "toy/setup-and-parameters"),
    ("toy", "toy/header-across-packets"),
    ("toy_codec", "toy/setup-and-parameters#1"),
    ("probe", "toy/setup-and-parameters"),
    ("probe", "flac/decoding-example-1"),
  ] {
    assert!(
      checked.iter().any(|seed| seed.target == target && seed.name == name),
      "{target}: {name}"
    );
  }
  if corpus::writing() {
    corpus::write(&corpus::folder(), &checked);
  }
}

#[test]
fn a_seed_routed_wrongly_fails_the_check() {
  let fixture = Fixture::load("toy/setup-and-parameters").unwrap();
  let seeds = fixture.seeds();
  let codec = &seeds[1];
  let routed = |representation: &str| {
    encode(
      Target::ToyCodec,
      Some(representation),
      &codec.route.context,
      "",
      codec.payload,
    )
  };
  let other = routed("two_byte_lengths").unwrap();
  assert_eq!(run(Target::ToyCodec, &other).outcome, "malformed");
  let mut unknown = routed("one_byte_lengths").unwrap();
  unknown[0] = 9;
  assert_eq!(run(Target::ToyCodec, &unknown).parser, None);
  assert_eq!(run(Target::Toy, codec.payload).parser, None);
  assert_eq!(
    routed("units"),
    Err(EncodingError::UnknownRepresentation("units".to_owned()))
  );
  assert_eq!(
    encode(Target::ToyCodec, None, &[], "", b"x"),
    Err(EncodingError::MissingRepresentation)
  );
  assert_eq!(
    encode(Target::ToyCodec, Some("one_byte_lengths"), &[0], "", b"x"),
    Err(EncodingError::ContextLength(1))
  );
  assert_eq!(
    encode(Target::Toy, Some("one_byte_lengths"), &[], "", b"x"),
    Err(EncodingError::UnexpectedRoute)
  );
}

#[test]
fn an_input_beyond_the_fuzz_runs_length_is_no_seed() {
  let bytes = vec![0_u8; MAXIMUM_INPUT_LENGTH];
  assert_eq!(
    encode(Target::Toy, None, &[], "", &bytes).map(|input| input.len()),
    Ok(MAXIMUM_INPUT_LENGTH)
  );
  assert_eq!(
    encode(Target::Probe, None, &[], "a", &bytes),
    Err(EncodingError::TooLong(MAXIMUM_INPUT_LENGTH + 2))
  );
  let context = [0, 0, 0, 0, 1];
  assert_eq!(
    encode(Target::ToyCodec, Some("one_byte_lengths"), &context, "", &bytes),
    Err(EncodingError::TooLong(MAXIMUM_INPUT_LENGTH + 6))
  );
  assert_eq!(
    encode(Target::Probe, None, &[], &"a".repeat(256), b"x"),
    Err(EncodingError::FileNameLength(256))
  );
}

#[test]
fn the_detection_and_probe_targets_carry_the_file_name_hint() {
  let input = encode(Target::Detection, None, &[], "movie.mkv", b"not a toy").unwrap();
  assert_eq!(&input[..10], b"\x09movie.mkv");
  assert_eq!(run(Target::Detection, &input).parser, Some(Parser::Detection));
  assert_eq!(run(Target::Detection, &input).outcome, "unknown");
  assert_eq!(run(Target::Probe, &input).outcome, "unknown_format");
  assert_eq!(run(Target::Probe, b"\x09movie").parser, None);
}

#[test]
fn the_corpus_names_each_seed_once_and_keeps_the_fuzzers_inputs() {
  assert_eq!(file_name("test/sample#1"), "seed.test.sample.1");
  assert_eq!(file_name("test/sample-1"), "seed.test.sample-1");
  let corpus = Path::new(env!("CARGO_TARGET_TMPDIR")).join("corpus");
  let _ = fs::remove_dir_all(&corpus);
  let seed = |name: &str| Checked {
    target: "toy",
    name: name.to_owned(),
    input: name.as_bytes().to_vec(),
  };
  corpus::write(&corpus, &[seed("test/old"), seed("test/sample#1")]);
  let found = corpus.join("toy").join("0123456789abcdef0123456789abcdef01234567");
  fs::write(&found, b"found").unwrap();
  corpus::write(&corpus, &[seed("test/sample#1"), seed("test/sample-1")]);
  let mut files: Vec<String> = fs::read_dir(corpus.join("toy"))
    .unwrap()
    .map(|file| file.unwrap().file_name().into_string().unwrap())
    .collect();
  files.sort();
  assert_eq!(
    files,
    [
      "0123456789abcdef0123456789abcdef01234567",
      "seed.test.sample-1",
      "seed.test.sample.1"
    ]
  );
  fs::remove_dir_all(&corpus).unwrap();
}

#[test]
#[should_panic(expected = "two seeds write it")]
fn two_seeds_of_one_file_are_refused_before_anything_is_written() {
  let corpus = Path::new(env!("CARGO_TARGET_TMPDIR")).join("corpus-twice");
  let seed = || Checked {
    target: "toy",
    name: "test/sample".to_owned(),
    input: Vec::new(),
  };
  corpus::write(&corpus, &[seed(), seed()]);
}

#[cfg(unix)]
#[test]
fn an_unreadable_folder_of_the_corpus_fails_the_writer_with_its_path() {
  use std::os::unix::fs::PermissionsExt;

  let corpus = Path::new(env!("CARGO_TARGET_TMPDIR")).join("corpus-unreadable");
  let _ = fs::remove_dir_all(&corpus);
  let old = corpus.join("toy");
  fs::create_dir_all(&old).unwrap();
  fs::write(old.join("seed.test.old"), b"old").unwrap();
  fs::set_permissions(&old, fs::Permissions::from_mode(0o300)).unwrap();
  let readable = fs::read_dir(&old).is_ok();
  let seeds = [Checked {
    target: "probe",
    name: "test/new".to_owned(),
    input: Vec::new(),
  }];
  let written = std::panic::catch_unwind(|| corpus::write(&corpus, &seeds));
  fs::set_permissions(&old, fs::Permissions::from_mode(0o755)).unwrap();
  fs::remove_dir_all(&corpus).unwrap();
  if readable {
    // A user who may read every folder, such as root, cannot make the fault.
    return;
  }
  let message = written.unwrap_err().downcast::<String>().unwrap();
  assert!(message.starts_with(&old.display().to_string()), "{message}");
}
