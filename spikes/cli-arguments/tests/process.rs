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

//! The spike's program, run as a process: `clap` writes nothing to either output, so each one holds only the
//! line that the program writes, and a usage error exits with 2. An output that cannot be written exits with 1.

use std::process::Command;

fn outputs(arguments: &[&str]) -> (String, String, Option<i32>) {
  let output = Command::new(env!("CARGO_BIN_EXE_cli-arguments"))
    .args(arguments)
    .output()
    .unwrap();
  (
    String::from_utf8(output.stdout).unwrap(),
    String::from_utf8(output.stderr).unwrap(),
    output.status.code(),
  )
}

#[test]
fn the_parser_writes_nothing_itself() {
  assert_eq!(outputs(&["--help"]), ("help\n".to_owned(), String::new(), Some(0)));
  assert_eq!(
    outputs(&["--json", "--version"]),
    ("version true\n".to_owned(), String::new(), Some(0))
  );
  assert_eq!(
    outputs(&["--jsno"]),
    (
      String::new(),
      "usage error false: unknown option: --jsno\n".to_owned(),
      Some(2)
    )
  );
  assert_eq!(
    outputs(&["--json", "-", "-"]),
    (
      "usage error document: standard input (-) was named twice\n".to_owned(),
      "usage error true: standard input (-) was named twice\n".to_owned(),
      Some(2)
    )
  );
  assert_eq!(
    outputs(&[]),
    (
      String::new(),
      "usage error false: no input was named\n".to_owned(),
      Some(2)
    )
  );
  assert_eq!(
    outputs(&["--json", "a"]),
    ("run true false false\ninput a\n".to_owned(), String::new(), Some(0))
  );
}

#[test]
fn an_output_whose_reader_has_gone_fails_with_status_1() {
  // The eighth review's case. A pipe whose read end is closed before the program writes fails every write, so the
  // status is 1 on every run. A reader that is still open, as `head` can be for a short output, lets the writes
  // succeed, and the program then ends normally. Since the twelfth review, standard error also gets the failure's line
  // (`cli.md` section 9); which byte failed and which inputs were taken are for `tests/output.rs`.
  let (reader, writer) = std::io::pipe().unwrap();
  drop(reader);
  let output = Command::new(env!("CARGO_BIN_EXE_cli-arguments"))
    .args(["a", "b"])
    .stdout(writer)
    .output()
    .unwrap();
  assert_eq!(output.status.code(), Some(1));
  let errors = String::from_utf8(output.stderr).unwrap();
  assert_eq!(errors.lines().count(), 1);
  assert!(errors.starts_with("standard output failed: "), "{errors}");
}

/// The status and standard output of the program, run with a standard error whose read end is closed.
fn without_standard_error(arguments: &[&str]) -> (String, Option<i32>) {
  let (reader, writer) = std::io::pipe().unwrap();
  drop(reader);
  let output = Command::new(env!("CARGO_BIN_EXE_cli-arguments"))
    .args(arguments)
    .stderr(writer)
    .output()
    .unwrap();
  (String::from_utf8(output.stdout).unwrap(), output.status.code())
}

#[test]
fn a_usage_error_keeps_status_2_when_standard_error_fails() {
  // The tenth review's case: a failure to write standard error changes nothing (`cli.md` section 9), so a usage
  // error still exits with 2, and in JSON mode standard output still gets its document.
  assert_eq!(without_standard_error(&["--jsno"]), (String::new(), Some(2)));
  assert_eq!(
    without_standard_error(&["--json", "--jsno"]),
    ("usage error document: unknown option: --jsno\n".to_owned(), Some(2))
  );
  assert_eq!(
    without_standard_error(&["a"]),
    ("run false false false\ninput a\n".to_owned(), Some(0))
  );
}

#[test]
fn a_usage_error_keeps_status_2_when_its_document_cannot_be_written() {
  // `cli.md` section 4: the status stays 2, and standard error also reports the output failure, on the line before
  // the usage error's, as the maintainer chose at the eleventh review.
  let (reader, writer) = std::io::pipe().unwrap();
  drop(reader);
  let output = Command::new(env!("CARGO_BIN_EXE_cli-arguments"))
    .args(["--json", "--jsno"])
    .stdout(writer)
    .output()
    .unwrap();
  assert_eq!(output.status.code(), Some(2));
  let errors = String::from_utf8(output.stderr).unwrap();
  let lines: Vec<&str> = errors.lines().collect();
  assert_eq!(lines.len(), 2);
  assert!(lines[0].starts_with("standard output failed: "));
  assert_eq!(lines[1], "usage error true: unknown option: --jsno");
}

#[test]
fn standard_error_escapes_what_a_terminal_acts_on() {
  // The ninth review's case: a newline and an escape inside an unknown option are written as `\u{…}` on standard
  // error, in either mode, while the message itself keeps them (`tests/arguments.rs`).
  assert_eq!(
    outputs(&["--bo\ngus\u{1b}[2J", "movie.mkv"]),
    (
      String::new(),
      "usage error false: unknown option: --bo\\u{a}gus\\u{1b}[2J\n".to_owned(),
      Some(2)
    )
  );
  assert_eq!(
    outputs(&["--json", "--x\u{202e}y"]).1,
    "usage error true: unknown option: --x\\u{202e}y\n"
  );
}
