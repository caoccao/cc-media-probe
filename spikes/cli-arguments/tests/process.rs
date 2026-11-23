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
//! line that the program writes, and a usage error exits with 2.

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
      String::new(),
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
