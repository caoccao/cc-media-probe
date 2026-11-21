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

//! From arguments to output and an exit status.

use std::ffi::OsString;
use std::io::{self, Write};
use std::process::ExitCode;

use crate::command_line::arguments::{Command, parse_arguments};
use crate::command_line::constants::{FAILURE_STATUS, FINDINGS_WORD, SUCCESS_STATUS, USAGE, USAGE_ERROR_STATUS};
use crate::command_line::execution::execute;

/// Runs the command that `arguments` names, writing to standard output and
/// standard error.
pub fn run(arguments: impl IntoIterator<Item = OsString>) -> ExitCode {
  let mut output = io::stdout().lock();
  let mut errors = io::stderr().lock();
  ExitCode::from(run_with(arguments, &mut output, &mut errors))
}

/// Runs the command that `arguments` names (the first argument is the
/// program) and returns the exit status: 0 when the check passes, 1 when it
/// finds problems or cannot run, 2 on a usage error.
pub fn run_with(arguments: impl IntoIterator<Item = OsString>, output: &mut dyn Write, errors: &mut dyn Write) -> u8 {
  let arguments: Vec<String> = arguments
    .into_iter()
    .skip(1)
    .map(|argument| argument.to_string_lossy().into_owned())
    .collect();
  let command = match parse_arguments(&arguments) {
    Ok(command) => command,
    Err(message) => {
      return match writeln!(errors, "{message}\n\n{USAGE}") {
        Ok(()) => USAGE_ERROR_STATUS,
        Err(_) => FAILURE_STATUS,
      };
    }
  };
  if command == Command::Help {
    return match writeln!(output, "{USAGE}") {
      Ok(()) => SUCCESS_STATUS,
      Err(_) => FAILURE_STATUS,
    };
  }
  match execute(&command) {
    Ok(outcome) => match report(output, &outcome.summary, &outcome.findings) {
      Ok(()) if outcome.findings.is_empty() => SUCCESS_STATUS,
      _ => FAILURE_STATUS,
    },
    Err(error) => {
      let _ = writeln!(errors, "{error}");
      FAILURE_STATUS
    }
  }
}

fn report(output: &mut dyn Write, summary: &[String], findings: &[crate::findings::Finding]) -> io::Result<()> {
  for line in summary {
    writeln!(output, "{line}")?;
  }
  for finding in findings {
    writeln!(output, "{finding}")?;
  }
  let count = findings.len();
  writeln!(output, "{count} {FINDINGS_WORD}")
}

#[cfg(test)]
mod tests {
  use super::run_with;
  use std::ffi::OsString;
  use std::fs;
  use std::io::{self, Write};

  fn run(arguments: &[&str]) -> (u8, String, String) {
    let mut all = vec![OsString::from("cc-media-probe-checks")];
    all.extend(arguments.iter().map(OsString::from));
    let mut output = Vec::new();
    let mut errors = Vec::new();
    let status = run_with(all, &mut output, &mut errors);
    (
      status,
      String::from_utf8(output).unwrap(),
      String::from_utf8(errors).unwrap(),
    )
  }

  struct FailingWriter;

  impl Write for FailingWriter {
    fn write(&mut self, _: &[u8]) -> io::Result<usize> {
      Err(io::Error::other("closed"))
    }

    fn flush(&mut self) -> io::Result<()> {
      Ok(())
    }
  }

  #[test]
  fn prints_usage() {
    let (status, output, _) = run(&["--help"]);
    assert_eq!(status, 0);
    assert!(output.starts_with("usage: cc-media-probe-checks"));
    let (status, _, errors) = run(&["unknown"]);
    assert_eq!(status, 2);
    assert!(errors.starts_with("unknown command or wrong arguments"));
  }

  #[test]
  fn reports_findings_and_errors() {
    let root = std::env::temp_dir().join(format!("checks-dispatch-{}", std::process::id()));
    fs::create_dir_all(&root).unwrap();
    fs::write(root.join("broken.rs"), "fn broken() {}\n").unwrap();
    let root_text = root.to_string_lossy().into_owned();
    let (status, output, _) = run(&["structure", "--root", &root_text]);
    fs::write(root.join("broken.rs"), "").unwrap();
    fs::remove_dir_all(&root).unwrap();
    assert_eq!(status, 1);
    assert!(output.ends_with("1 findings\n"), "{output}");
    let (status, _, errors) = run(&["structure", "--root", "/nonexistent/root"]);
    assert_eq!(status, 1);
    assert_ne!(errors, "");
  }

  #[test]
  fn passes_a_clean_tree() {
    let root = std::env::temp_dir().join(format!("checks-dispatch-clean-{}", std::process::id()));
    fs::create_dir_all(&root).unwrap();
    let root_text = root.to_string_lossy().into_owned();
    let (status, output, _) = run(&["structure", "--root", &root_text]);
    fs::remove_dir_all(&root).unwrap();
    assert_eq!(status, 0);
    assert_eq!(output, "0 findings\n");
  }

  #[test]
  fn fails_when_output_cannot_be_written() {
    let arguments = vec![OsString::from("checks"), OsString::from("--help")];
    assert_eq!(run_with(arguments, &mut FailingWriter, &mut Vec::new()), 1);
    let unknown = vec![OsString::from("checks"), OsString::from("unknown")];
    assert_eq!(run_with(unknown, &mut Vec::new(), &mut FailingWriter), 1);
    let root = std::env::temp_dir();
    let structure = vec![
      OsString::from("checks"),
      OsString::from("structure"),
      OsString::from("--root"),
      root.join("checks-missing-root").into_os_string(),
    ];
    assert_eq!(run_with(structure, &mut FailingWriter, &mut Vec::new()), 1);
  }
}
