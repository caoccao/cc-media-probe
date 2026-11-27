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

//! Scenario 9: scan a directory, handle each file's error, and use the partial data an error carries.

use std::io::{self, Write};
use std::num::NonZeroUsize;
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

use cc_media_probe::completeness::{Part, Reason};
use cc_media_probe::{Error, ErrorKind, Limits, Prober};

fn main() -> io::Result<()> {
  // One prober serves every file. It is `Sync`, so the threads share it.
  let prober = Prober::new().with_limits(Limits::default().with_bytes_read(8 * 1024 * 1024));
  let mut paths = Vec::new();
  for entry in std::fs::read_dir("media")? {
    let path = entry?.path();
    if path.is_file() {
      paths.push(path);
    }
  }
  // A fixed number of workers, however many files there are: each takes the next path until none is left.
  let workers = thread::available_parallelism().map_or(1, NonZeroUsize::get);
  let next = AtomicUsize::new(0);
  thread::scope(|scope| {
    let handles: Vec<_> = (0..workers.min(paths.len()))
      .map(|_| {
        scope.spawn(|| -> io::Result<()> {
          while let Some(path) = paths.get(next.fetch_add(1, Ordering::Relaxed)) {
            scan(&prober, path)?;
          }
          Ok(())
        })
      })
      .collect();
    // A worker whose output failed gives its error, and one that panicked passes the panic on.
    handles
      .into_iter()
      .try_for_each(|handle| handle.join().unwrap_or_else(|panic| std::panic::resume_unwind(panic)))
  })
}

/// Probes one file, and then writes all of its lines under one lock of standard output, so the lines of the files
/// that other workers finish meanwhile never come between them.
fn scan(prober: &Prober, path: &Path) -> io::Result<()> {
  let outcome = prober.probe_file(path);
  let mut output = io::stdout().lock();
  match outcome {
    Ok(report) => {
      let status = if report.completeness().is_complete() {
        "complete"
      } else {
        "incomplete"
      };
      writeln!(
        output,
        "{}: {} with {} streams, {status}",
        path.display(),
        report.format(),
        report.streams().len()
      )
    }
    Err(error) => {
      writeln!(output, "{}: {error}", path.display())?;
      print_partial_data(&mut output, &error)
    }
  }
}

fn print_partial_data(output: &mut impl Write, error: &Error) -> io::Result<()> {
  match error.kind() {
    // The source failed. If the stream inventory was reached first, the report so far comes with the error.
    ErrorKind::SourceFailed => {
      if let Some(partial) = error.partial_report() {
        writeln!(output, "  before the failure: {} streams", partial.streams().len())?;
      }
    }
    // The probe stopped before any stream list; what it saw is kept as observations.
    ErrorKind::NoStreamInventory => {
      for diagnostic in error
        .observations()
        .map_or(&[][..], |observations| observations.diagnostics())
      {
        writeln!(output, "  {}: {}", diagnostic.code(), diagnostic.message())?;
      }
    }
    // Detection could not decide. A limit that ended it, such as a read limit of zero, stopped the part `Detection`,
    // before stream discovery began.
    ErrorKind::FormatNotDetermined => {
      if let Some(limit) = error.reason().and_then(Reason::limit) {
        let part = error.record().first_stopped(limit).map_or("no part", Part::as_str);
        writeln!(output, "  the {} limit stopped {part}", limit.as_str())?;
      }
    }
    // Not a media file, or none this version reads, and the kinds of later versions: nothing more to show.
    _ => {}
  }
  // The usage is known for an error too, so a scan can total its reads.
  writeln!(output, "  read {} bytes", error.usage().bytes_read())
}
