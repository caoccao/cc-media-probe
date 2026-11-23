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

//! The twelfth review's case, `cli.md` section 10: a pipe closed before the program starts fails its first write,
//! wherever buffering puts that write, so it cannot choose a byte inside an entry or the final line feed, and the exit
//! status cannot show which inputs were taken. A writer that fails at a chosen byte and inputs that count what was
//! taken can.

use std::cell::Cell;
use std::ffi::OsString;
use std::io::{self, Write};

use cli_arguments::{report_output_failure, write_inputs};

/// A writer that takes bytes up to a chosen count and fails on the next one, as a closed pipe would.
struct FailingAt {
  written: Vec<u8>,
  failing_byte: usize,
  flush_fails: bool,
}

impl FailingAt {
  fn new(failing_byte: usize, flush_fails: bool) -> FailingAt {
    FailingAt {
      written: Vec::new(),
      failing_byte,
      flush_fails,
    }
  }
}

impl Write for FailingAt {
  fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
    let room = self.failing_byte.saturating_sub(self.written.len());
    if room == 0 {
      return Err(io::Error::from(io::ErrorKind::BrokenPipe));
    }
    let taken = buffer.len().min(room);
    self.written.extend_from_slice(&buffer[..taken]);
    Ok(taken)
  }

  fn flush(&mut self) -> io::Result<()> {
    if self.flush_fails {
      return Err(io::Error::from(io::ErrorKind::BrokenPipe));
    }
    Ok(())
  }
}

/// Writes the lines of three inputs to `output`, and gives the result with the number of inputs taken.
fn write_three(output: &mut FailingAt) -> (io::Result<()>, usize) {
  let inputs: Vec<OsString> = ["a.mkv", "b.mkv", "c.mkv"].iter().map(OsString::from).collect();
  let taken = Cell::new(0);
  let result = write_inputs(output, inputs.iter().inspect(|_| taken.set(taken.get() + 1)));
  (result, taken.get())
}

/// Each input's line, `input a.mkv` and its line feed.
const LINE: usize = 12;

#[test]
fn a_failure_inside_an_entry_leaves_the_next_input_untaken() {
  let mut output = FailingAt::new(LINE + 6, false);
  let (result, taken) = write_three(&mut output);
  assert_eq!(result.unwrap_err().kind(), io::ErrorKind::BrokenPipe);
  assert_eq!(taken, 2, "the third input was never taken");
  assert_eq!(output.written, b"input a.mkv\ninput ");
  // The failure's line, as standard error writes it.
  let mut errors = Vec::new();
  report_output_failure(&mut errors, &io::Error::from(io::ErrorKind::BrokenPipe));
  assert_eq!(
    String::from_utf8(errors).unwrap(),
    "standard output failed: broken pipe\n"
  );
}

#[test]
fn a_failure_at_the_final_line_feed_fails_after_every_input_was_taken() {
  let mut output = FailingAt::new(3 * LINE - 1, false);
  let (result, taken) = write_three(&mut output);
  assert!(result.is_err());
  assert_eq!(taken, 3);
  assert_eq!(output.written.len(), 3 * LINE - 1);
  assert_eq!(output.written.last(), Some(&b'v'), "everything but the final line feed");
}

#[test]
fn a_flush_that_fails_fails_as_a_write_does_and_a_writer_that_takes_everything_succeeds() {
  let (flushed, taken) = write_three(&mut FailingAt::new(3 * LINE, true));
  assert!(flushed.is_err());
  assert_eq!(taken, 3);
  let mut output = FailingAt::new(3 * LINE, false);
  assert!(write_three(&mut output).0.is_ok());
  assert_eq!(output.written, b"input a.mkv\ninput b.mkv\ninput c.mkv\n");
}
