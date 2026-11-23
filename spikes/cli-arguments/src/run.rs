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

//! The spike's program: one line for the invocation on standard output, one line for each input, and a usage
//! error on standard error with its exit status. It probes nothing.

use std::env;
use std::io::{self, Write};
use std::process::ExitCode;

use crate::constants::{HELP_LINE, INPUT_LINE, RUN_LINE, USAGE_ERROR_LINE, USAGE_ERROR_STATUS, VERSION_LINE};
use crate::invocation::{Invocation, parse};

/// Reads the process's arguments and prints what they ask for.
pub fn run() -> ExitCode {
  match print(&parse(env::args_os().skip(1).collect())) {
    Ok(status) => status,
    Err(_) => ExitCode::FAILURE,
  }
}

fn print(invocation: &Invocation) -> io::Result<ExitCode> {
  let mut output = io::stdout().lock();
  match invocation {
    Invocation::Help => writeln!(output, "{HELP_LINE}")?,
    Invocation::Version { json } => writeln!(output, "{VERSION_LINE} {json}")?,
    Invocation::Run {
      json,
      detect,
      strict,
      inputs,
    } => {
      writeln!(output, "{RUN_LINE} {json} {detect} {strict}")?;
      for input in inputs {
        writeln!(output, "{INPUT_LINE} {}", input.to_string_lossy())?;
      }
    }
    Invocation::UsageError { json, message } => {
      writeln!(io::stderr().lock(), "{USAGE_ERROR_LINE} {json}: {message}")?;
      return Ok(ExitCode::from(USAGE_ERROR_STATUS));
    }
  }
  output.flush()?;
  Ok(ExitCode::SUCCESS)
}
