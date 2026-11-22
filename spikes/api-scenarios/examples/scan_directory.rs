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

use std::num::NonZeroUsize;
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

use cc_media_probe::{Error, ErrorKind, Limits, Prober};

fn main() -> std::io::Result<()> {
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
    for _ in 0..workers.min(paths.len()) {
      scope.spawn(|| {
        while let Some(path) = paths.get(next.fetch_add(1, Ordering::Relaxed)) {
          scan(&prober, path);
        }
      });
    }
  });
  Ok(())
}

fn scan(prober: &Prober, path: &Path) {
  match prober.probe_file(path) {
    Ok(report) => {
      let status = if report.completeness().is_complete() {
        "complete"
      } else {
        "incomplete"
      };
      println!(
        "{}: {} with {} streams, {status}",
        path.display(),
        report.format(),
        report.streams().len()
      );
    }
    Err(error) => {
      println!("{}: {error}", path.display());
      print_partial_data(&error);
    }
  }
}

fn print_partial_data(error: &Error) {
  match error.kind() {
    // The source failed. If the stream inventory was reached first, the report so far comes with the error.
    ErrorKind::SourceFailed => {
      if let Some(partial) = error.partial_report() {
        println!("  before the failure: {} streams", partial.streams().len());
      }
    }
    // The probe stopped before any stream list; what it saw is kept as observations.
    ErrorKind::NoStreamInventory => {
      for diagnostic in error
        .observations()
        .map_or(&[][..], |observations| observations.diagnostics())
      {
        println!("  {}: {}", diagnostic.code(), diagnostic.message());
      }
    }
    // Not a media file, or none this version reads, and the kinds of later versions: nothing more to show.
    _ => {}
  }
  // The usage is known for an error too, so a scan can total its reads.
  println!("  read {} bytes", error.usage().bytes_read());
}
