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

//! The placeholder behavior of the tool: print its name and version.

use std::io::Write;
use std::process::ExitCode;

/// Writes `cc-media-probe <version>` to `output`; fails when it cannot be
/// written.
pub fn run(output: &mut dyn Write) -> ExitCode {
  let name = env!("CARGO_BIN_NAME");
  let version = env!("CARGO_PKG_VERSION");
  match writeln!(output, "{name} {version}") {
    Ok(()) => ExitCode::SUCCESS,
    Err(_) => ExitCode::FAILURE,
  }
}

#[cfg(test)]
mod tests {
  use super::run;
  use std::io::{self, Write};
  use std::process::ExitCode;

  struct ClosedOutput;

  impl Write for ClosedOutput {
    fn write(&mut self, _: &[u8]) -> io::Result<usize> {
      Err(io::Error::other("closed"))
    }

    fn flush(&mut self) -> io::Result<()> {
      Ok(())
    }
  }

  #[test]
  fn prints_name_and_version() {
    let mut output = Vec::new();
    assert_eq!(run(&mut output), ExitCode::SUCCESS);
    assert_eq!(
      String::from_utf8(output).unwrap(),
      format!("cc-media-probe {}\n", env!("CARGO_PKG_VERSION"))
    );
  }

  #[test]
  fn fails_when_output_is_closed() {
    assert_eq!(run(&mut ClosedOutput), ExitCode::FAILURE);
  }
}
