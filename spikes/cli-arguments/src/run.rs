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
//! error on standard error with its exit status. In JSON mode a usage error also writes one line on standard output,
//! which stands for its document. It probes nothing.

use std::env;
use std::ffi::OsString;
use std::fmt;
use std::io::{self, Write};
use std::process::ExitCode;

use crate::constants::{
  HELP_LINE, INPUT_LINE, OUTPUT_FAILED_LINE, RUN_LINE, USAGE_DOCUMENT_LINE, USAGE_ERROR_LINE, USAGE_ERROR_STATUS,
  VERSION_LINE,
};
use crate::escape::for_terminal;
use crate::invocation::{Invocation, parse};

/// Reads the process's arguments and prints what they ask for.
pub fn run() -> ExitCode {
  let arguments: Vec<OsString> = env::args_os().skip(1).collect();
  match parse(&arguments) {
    Invocation::UsageError { json, message } => usage_error(json, &message),
    invocation => match print(&invocation) {
      Ok(()) => ExitCode::SUCCESS,
      Err(failure) => {
        report_output_failure(&mut io::stderr().lock(), &failure);
        ExitCode::FAILURE
      }
    },
  }
}

fn print(invocation: &Invocation) -> io::Result<()> {
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
      return write_inputs(&mut output, inputs);
    }
    Invocation::UsageError { .. } => {}
  }
  output.flush()
}

/// Writes one line for each input and then flushes, taking an input only once the line before it was written. The
/// first failure ends the output, so no further input is taken, as the tool probes no further input (`cli.md`
/// section 9). The tests drive it with a writer that fails at a chosen byte and inputs that count what was taken,
/// which a closed pipe cannot choose (section 10, twelfth review).
///
/// # Errors
///
/// The first failure of a write or of the final flush, after which no further input is taken.
pub fn write_inputs<'input>(
  output: &mut impl Write,
  inputs: impl IntoIterator<Item = &'input OsString>,
) -> io::Result<()> {
  for input in inputs {
    writeln!(output, "{INPUT_LINE} {}", input.to_string_lossy())?;
  }
  output.flush()
}

/// Writes the line of an output failure to `errors`, escaped. A failure to write it changes nothing (`cli.md`
/// section 9), so its result is ignored.
pub fn report_output_failure(errors: &mut impl Write, failure: &io::Error) {
  let _ = writeln!(errors, "{OUTPUT_FAILED_LINE}: {}", for_terminal(&failure.to_string()));
}

/// A usage error exits with 2 whatever its outputs do (`cli.md` section 9, tenth review). In JSON mode its document
/// goes to standard output, and a failure there is reported on standard error, on the line before the usage error's
/// (eleventh review). The message goes to standard error, escaped there only.
fn usage_error(json: bool, message: &str) -> ExitCode {
  if json {
    let mut output = io::stdout().lock();
    if let Err(failure) = writeln!(output, "{USAGE_DOCUMENT_LINE}: {message}").and_then(|()| output.flush()) {
      report_output_failure(&mut io::stderr().lock(), &failure);
    }
  }
  write_error_line(format_args!("{USAGE_ERROR_LINE} {json}: {}", for_terminal(message)));
  ExitCode::from(USAGE_ERROR_STATUS)
}

/// Writes one line to standard error. A failure to write standard error changes nothing (`cli.md` section 9), so its
/// result is ignored.
fn write_error_line(line: fmt::Arguments<'_>) {
  let _ = writeln!(io::stderr().lock(), "{line}");
}
