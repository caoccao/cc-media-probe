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

//! The parser's description of the command line. `clap`'s own help and version flags are off: `--help` and
//! `--version` are plain flags, and the program writes their text itself, so `clap` never writes to an output.

use std::ffi::OsString;

use clap::{Arg, ArgAction, Command, value_parser};

use crate::constants::{DETECT_NAME, HELP_NAME, INPUT_NAME, JSON_NAME, PROGRAM_NAME, STRICT_NAME, VERSION_NAME};

/// The command line of `docs/design/cli.md` section 2.
pub fn command() -> Command {
  let flags = [JSON_NAME, DETECT_NAME, STRICT_NAME, HELP_NAME, VERSION_NAME];
  flags
    .into_iter()
    .fold(base(), |command, name| command.arg(flag(name)))
    .arg(inputs())
}

fn base() -> Command {
  Command::new(PROGRAM_NAME)
    .no_binary_name(true)
    .disable_help_flag(true)
    .disable_version_flag(true)
    .args_override_self(true)
}

fn flag(name: &'static str) -> Arg {
  Arg::new(name).long(name).action(ArgAction::SetTrue)
}

fn inputs() -> Arg {
  Arg::new(INPUT_NAME)
    .action(ArgAction::Append)
    .value_parser(value_parser!(OsString))
}
