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

//! What the approved `clap` makes of the command lines of `docs/design/cli.md` sections 2 and 3.

use std::ffi::OsString;

use cli_arguments::{Invocation, parse};

fn arguments(list: &[&str]) -> Vec<OsString> {
  list.iter().map(OsString::from).collect()
}

fn run(json: bool, inputs: &[&str]) -> Invocation {
  Invocation::Run {
    json,
    detect: false,
    strict: false,
    inputs: arguments(inputs),
  }
}

fn usage_error(json: bool, message: &str) -> Invocation {
  Invocation::UsageError {
    json,
    message: message.to_owned(),
  }
}

fn check(cases: &[(&[&str], Invocation)]) {
  for (case, expected) in cases {
    assert_eq!(&parse(&arguments(case)), expected, "{case:?}");
  }
}

#[test]
fn standard_input_is_an_input_once_before_and_after_the_separator() {
  check(&[
    (&["-"], run(false, &["-"])),
    (&["--", "-"], run(false, &["-"])),
    (&["-", "-"], usage_error(false, "standard input (-) was named twice")),
    (
      &["-", "--", "-"],
      usage_error(false, "standard input (-) was named twice"),
    ),
  ]);
}

#[test]
fn json_mode_is_the_exact_argument_before_the_separator() {
  check(&[
    (&["--json", "--json", "movie.mkv"], run(true, &["movie.mkv"])),
    (&["movie.mkv", "--json"], run(true, &["movie.mkv"])),
    (&["--", "--json"], run(false, &["--json"])),
    (&["--json", "--", "--bogus"], run(true, &["--bogus"])),
    (&["--json", "--jsno"], usage_error(true, "unknown option: --jsno")),
    (&["--jsno", "movie.mkv"], usage_error(false, "unknown option: --jsno")),
    (&["--JSON", "movie.mkv"], usage_error(false, "unknown option: --JSON")),
    (&["---json", "movie.mkv"], usage_error(false, "unknown option: ---json")),
    (
      &["--json=yes", "movie.mkv"],
      usage_error(false, "--json takes no value"),
    ),
    (&["--json=", "movie.mkv"], usage_error(false, "--json takes no value")),
  ]);
}

#[test]
fn long_options_are_never_abbreviated_and_there_are_no_short_options() {
  check(&[
    (&["--js", "movie.mkv"], usage_error(false, "unknown option: --js")),
    (&["-h"], usage_error(false, "unknown option: -h")),
    (&["-json", "movie.mkv"], usage_error(false, "unknown option: -j")),
    (
      &["--detect=1", "movie.mkv"],
      usage_error(false, "--detect takes no value"),
    ),
  ]);
}

#[test]
fn the_first_unknown_option_from_the_left_is_reported() {
  check(&[(
    &["--strict", "--bogus", "--jsno"],
    usage_error(false, "unknown option: --bogus"),
  )]);
}

#[test]
fn help_wins_over_everything_and_version_over_a_missing_input() {
  check(&[
    (&["--bogus", "--help"], Invocation::Help),
    (&["--json", "--help", "-", "-"], Invocation::Help),
    (&["--", "--help"], run(false, &["--help"])),
    (&["--help=1"], usage_error(false, "--help takes no value")),
    (&["--version"], Invocation::Version { json: false }),
    (
      &["--json", "--version", "movie.mkv"],
      Invocation::Version { json: true },
    ),
    (
      &["--version", "-", "-"],
      usage_error(false, "standard input (-) was named twice"),
    ),
    (&["--version", "--bogus"], usage_error(false, "unknown option: --bogus")),
  ]);
}

#[test]
fn an_input_is_required_and_any_text_is_an_input() {
  check(&[
    (&[], usage_error(false, "no input was named")),
    (&["--json"], usage_error(true, "no input was named")),
    (&[""], run(false, &[""])),
    (
      &["--detect", "--strict", "a", "b"],
      Invocation::Run {
        json: false,
        detect: true,
        strict: true,
        inputs: arguments(&["a", "b"]),
      },
    ),
  ]);
}

#[cfg(unix)]
#[test]
fn an_argument_that_is_not_unicode_is_kept_or_reported_lossily() {
  use std::os::unix::ffi::OsStringExt;
  let input = OsString::from_vec(vec![b'm', 0xff, b'.', b'm', b'k', b'v']);
  let option = OsString::from_vec(vec![b'-', b'-', 0xff]);
  assert_eq!(
    parse(std::slice::from_ref(&input)),
    Invocation::Run {
      json: false,
      detect: false,
      strict: false,
      inputs: vec![input]
    }
  );
  assert_eq!(
    parse(&[option, OsString::from("a")]),
    usage_error(false, "unknown option: --\u{fffd}")
  );
}

#[test]
fn an_unknown_option_with_a_value_is_named_as_written() {
  // The eighth review's cases: the message names the whole argument, in either mode.
  for json in [false, true] {
    let with_mode = |list: &[&'static str]| {
      let mut all = if json { vec!["--json"] } else { Vec::new() };
      all.extend_from_slice(list);
      all
    };
    check(&[
      (
        &with_mode(&["--bogus=value", "movie.mkv"]),
        usage_error(json, "unknown option: --bogus=value"),
      ),
      (
        &with_mode(&["--=x", "movie.mkv"]),
        usage_error(json, "unknown option: --=x"),
      ),
      (
        &with_mode(&["--bogus=", "movie.mkv"]),
        usage_error(json, "unknown option: --bogus="),
      ),
    ]);
  }
}

#[test]
fn a_message_holds_the_argument_decoded_and_unescaped() {
  // The ninth review's case: the message that goes into the document holds the newline itself, and only standard
  // error escapes it (`tests/process.rs`).
  for json in [false, true] {
    let mut list = if json { vec!["--json"] } else { Vec::new() };
    list.extend_from_slice(&["--bo\ngus", "movie.mkv"]);
    check(&[(&list, usage_error(json, "unknown option: --bo\ngus"))]);
  }
}
