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

//! Parsing the arguments.

use std::path::PathBuf;

use crate::command_line::constants::{
  BUILD_OUTPUT_COMMAND, COVERAGE_COMMAND, CURRENT_DIRECTORY, DYNAMIC_LIBRARIES_COMMAND, HELP_OPTION, INVENTORY_COMMAND,
  MISSING_ROOT, ROOT_OPTION, SHORT_HELP_OPTION, STRUCTURE_COMMAND, TARGETS_COMMAND, UNKNOWN_COMMAND,
};

/// What to run, and where.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Command {
  /// The structure check.
  Structure {
    /// The repository root.
    root: PathBuf,
  },
  /// The coverage check over an LCOV report.
  Coverage {
    /// The LCOV report.
    report: PathBuf,
    /// The repository root.
    root: PathBuf,
  },
  /// The target-ownership check.
  Targets {
    /// The repository root.
    root: PathBuf,
  },
  /// The inventory of the published crates' graph.
  Inventory {
    /// The repository root.
    root: PathBuf,
  },
  /// The inspection of build-script outputs.
  BuildOutput {
    /// The target directory of the build.
    target_directory: PathBuf,
    /// The repository root.
    root: PathBuf,
  },
  /// The comparison of dynamic libraries with the allow-list.
  DynamicLibraries {
    /// The platform: `macos`, `linux` or `windows`.
    platform: String,
    /// The libraries the binary loads.
    libraries: Vec<String>,
    /// The repository root.
    root: PathBuf,
  },
  /// Print the usage.
  Help,
}

/// Parses the arguments after the program name.
///
/// # Errors
///
/// Returns a message for an unknown command, wrong arguments, or `--root`
/// without a value.
pub fn parse_arguments(arguments: &[String]) -> Result<Command, &'static str> {
  let mut root = PathBuf::from(CURRENT_DIRECTORY);
  let mut positional: Vec<&str> = Vec::new();
  let mut remaining = arguments.iter();
  while let Some(argument) = remaining.next() {
    match argument.as_str() {
      ROOT_OPTION => root = PathBuf::from(remaining.next().ok_or(MISSING_ROOT)?),
      HELP_OPTION | SHORT_HELP_OPTION => return Ok(Command::Help),
      other => positional.push(other),
    }
  }
  let command = match positional.as_slice() {
    [STRUCTURE_COMMAND] => Command::Structure { root },
    [COVERAGE_COMMAND, report] => Command::Coverage {
      report: PathBuf::from(report),
      root,
    },
    [TARGETS_COMMAND] => Command::Targets { root },
    [INVENTORY_COMMAND] => Command::Inventory { root },
    [BUILD_OUTPUT_COMMAND, target_directory] => Command::BuildOutput {
      target_directory: PathBuf::from(target_directory),
      root,
    },
    [DYNAMIC_LIBRARIES_COMMAND, platform, libraries @ ..] => Command::DynamicLibraries {
      platform: (*platform).to_owned(),
      libraries: libraries.iter().map(|library| (*library).to_owned()).collect(),
      root,
    },
    _ => return Err(UNKNOWN_COMMAND),
  };
  Ok(command)
}

#[cfg(test)]
mod tests {
  use super::{Command, parse_arguments};
  use std::path::PathBuf;

  fn parse(arguments: &[&str]) -> Result<Command, &'static str> {
    let owned: Vec<String> = arguments.iter().map(|argument| (*argument).to_owned()).collect();
    parse_arguments(&owned)
  }

  #[test]
  fn parses_every_command() {
    assert_eq!(
      parse(&["structure"]),
      Ok(Command::Structure {
        root: PathBuf::from(".")
      })
    );
    assert_eq!(
      parse(&["coverage", "lcov.info", "--root", "/repository"]),
      Ok(Command::Coverage {
        report: PathBuf::from("lcov.info"),
        root: PathBuf::from("/repository"),
      })
    );
    assert_eq!(
      parse(&["targets"]),
      Ok(Command::Targets {
        root: PathBuf::from(".")
      })
    );
    assert_eq!(
      parse(&["inventory"]),
      Ok(Command::Inventory {
        root: PathBuf::from(".")
      })
    );
    assert_eq!(
      parse(&["build-output", "target"]),
      Ok(Command::BuildOutput {
        target_directory: PathBuf::from("target"),
        root: PathBuf::from("."),
      })
    );
    assert_eq!(
      parse(&["dynamic-libraries", "macos", "/usr/lib/libSystem.B.dylib"]),
      Ok(Command::DynamicLibraries {
        platform: "macos".to_owned(),
        libraries: vec!["/usr/lib/libSystem.B.dylib".to_owned()],
        root: PathBuf::from("."),
      })
    );
    assert_eq!(parse(&["structure", "--help"]), Ok(Command::Help));
    assert_eq!(parse(&["-h"]), Ok(Command::Help));
  }

  #[test]
  fn rejects_wrong_arguments() {
    assert!(parse(&[]).is_err());
    assert!(parse(&["structure", "extra"]).is_err());
    assert!(parse(&["coverage"]).is_err());
    assert!(parse(&["unknown"]).is_err());
    assert!(parse(&["structure", "--root"]).is_err());
  }
}
