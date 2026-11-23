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

//! What a command line asks for, decided in the order of `docs/design/cli.md` section 3: the JSON-mode scan,
//! the help scan, the parser, the checks of the inputs, and the version.

use std::ffi::OsString;

use clap::ArgMatches;

use crate::command::command;
use crate::constants::{
  DETECT_NAME, HELP_ARGUMENT, INPUT_NAME, JSON_ARGUMENT, NO_INPUT, STANDARD_INPUT, STANDARD_INPUT_TWICE, STRICT_NAME,
  VERSION_NAME,
};
use crate::scan::appears_before_separator;
use crate::usage::message;

/// What a command line asks for.
#[derive(Debug, PartialEq, Eq)]
pub enum Invocation {
  /// Print the help text, whatever else the command line holds.
  Help,
  /// Print the version, as text or as a document.
  Version {
    /// Whether JSON mode is on.
    json: bool,
  },
  /// Probe or detect each input in order.
  Run {
    /// Whether JSON mode is on.
    json: bool,
    /// Whether `--detect` was given.
    detect: bool,
    /// Whether `--strict` was given.
    strict: bool,
    /// The inputs, as written.
    inputs: Vec<OsString>,
  },
  /// A command line that cannot be run.
  UsageError {
    /// Whether JSON mode is on.
    json: bool,
    /// The message, without the program's name.
    message: String,
  },
}

/// Reads a command line, without the program's name.
#[must_use]
pub fn parse(arguments: &[OsString]) -> Invocation {
  let json = appears_before_separator(arguments, JSON_ARGUMENT);
  if appears_before_separator(arguments, HELP_ARGUMENT) {
    return Invocation::Help;
  }
  match command().try_get_matches_from(arguments) {
    Ok(matches) => decide(json, &matches),
    Err(error) => Invocation::UsageError {
      json,
      message: message(&error, arguments),
    },
  }
}

fn decide(json: bool, matches: &ArgMatches) -> Invocation {
  let inputs: Vec<OsString> = matches
    .get_many::<OsString>(INPUT_NAME)
    .into_iter()
    .flatten()
    .cloned()
    .collect();
  let standard_inputs = inputs
    .iter()
    .filter(|input| input.as_os_str() == STANDARD_INPUT)
    .count();
  if standard_inputs > 1 {
    return usage_error(json, STANDARD_INPUT_TWICE);
  }
  if matches.get_flag(VERSION_NAME) {
    return Invocation::Version { json };
  }
  if inputs.is_empty() {
    return usage_error(json, NO_INPUT);
  }
  Invocation::Run {
    json,
    detect: matches.get_flag(DETECT_NAME),
    strict: matches.get_flag(STRICT_NAME),
    inputs,
  }
}

fn usage_error(json: bool, message: &str) -> Invocation {
  Invocation::UsageError {
    json,
    message: message.to_owned(),
  }
}
