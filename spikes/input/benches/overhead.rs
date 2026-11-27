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

//! The overhead of writing the walker once as `async` code (D8): the async probe driven by the minimal executor
//! against the hand-written blocking probe, and D8's fallback, the state machine through its blocking driver,
//! beside both, on the same toy files, from memory, through the block cache over a reader in memory, and from a
//! file, with few and with many packets. G3.1's bar, as the maintainer chose on
//! 2026-11-27: within 10% on the in-memory cases, where no I/O hides the cost, by the median of each of three runs.
//!
//! Before any case is timed, every probe of it runs once and must give the expected report, and a fixture that
//! cannot be written ends the run with status 1, so a skipped case or an error path is never timed as a success.
//! Every timed probe is checked too: one that fails ends the run with status 1, so no sample counts a failure.

use std::fs;
use std::hint::black_box;
use std::io::Cursor;
use std::path::Path;
use std::process;

use cc_media_probe::Limits;
use criterion::{Criterion, criterion_group, criterion_main};
use input::Options;
use input::blocking;
use input::builder::{ToyFile, parameters_unit, payload_unit, setup_unit};
use input::codec::{Framing, Header, Setup};
use input::handwritten;
use input::machine;
use input::report::{HeaderStatus, ProbeError, Reason, ToyReport};

/// A toy file of `packets` packets: two streams of the toy codec, whose headers come early, and a third, waiting
/// one in packet mode, whose packets carry no header, so every one of its packets is offered to its assembler.
fn toy_file(packets: usize) -> Vec<u8> {
  let mut file = ToyFile::new()
    .title("Measured")
    .stream(1, Framing::Packet)
    .stream(2, Framing::Stream)
    .stream(3, Framing::Packet)
    .packet(1, &[setup_unit(1920, 1080, 2, 0), payload_unit(16)].concat())
    .packet(
      2,
      &[setup_unit(640, 480, 1, 0), parameters_unit(&[9]).unwrap_or_default()].concat(),
    )
    .packet(1, &parameters_unit(&[7, 8]).unwrap_or_default());
  for number in [1, 2, 3].into_iter().cycle().take(packets) {
    file = file.packet(number, &payload_unit(30));
  }
  file.build()
}

/// The headers that every probe of `toy_file` gives: the first two parsed, the third not found by the input's end.
fn expected_headers() -> [HeaderStatus; 3] {
  let parsed = |width, height, parameter_count, parameter_sum| {
    HeaderStatus::Parsed(Header {
      setup: Setup {
        width,
        height,
        parameter_count,
        flags: 0,
      },
      parameter_sum,
    })
  };
  [
    parsed(1920, 1080, 2, 15),
    parsed(640, 480, 1, 9),
    HeaderStatus::NotFound(Reason::TruncatedInput),
  ]
}

/// Ends the run with status 1 and `message`.
fn fail(message: &str) -> ! {
  eprintln!("overhead: {message}");
  process::exit(1);
}

/// The report of a timed probe, or the end of the run with status 1 when it failed.
fn timed(result: Result<ToyReport, ProbeError>) -> ToyReport {
  result.unwrap_or_else(|error| fail(&format!("a timed probe failed: {error:?}")))
}

/// The report without what depends on how the input was read: the bytes read and the peak of working memory.
fn semantic(report: &ToyReport) -> ToyReport {
  let mut report = report.clone();
  report.usage.bytes_read = 0;
  report.usage.peak_working_memory = 0;
  for source in &mut report.sources {
    source.bytes_read = 0;
  }
  report
}

/// Checks that `result`, a probe of the case `name`, gave the report of the reference probe from memory, but for
/// what depends on how the input was read.
fn check(name: &str, result: Result<ToyReport, ProbeError>, reference: &ToyReport) {
  match result {
    Ok(report) if semantic(&report) == semantic(reference) => {}
    Ok(_) => fail(&format!("{name} gave another report")),
    Err(error) => fail(&format!("{name} failed: {error:?}")),
  }
}

/// The reference report of `file`, from memory, which must hold the expected headers.
fn reference(name: &str, file: &[u8], options: &Options) -> ToyReport {
  let report = blocking::probe_bytes(file, options).unwrap_or_else(|error| fail(&format!("{name}: {error:?}")));
  let headers: Vec<HeaderStatus> = report.streams.iter().map(|stream| stream.header.clone()).collect();
  if headers != expected_headers() {
    fail(&format!("{name} gave the headers {headers:?}"));
  }
  report
}

/// The three forms of the walker.
#[derive(Clone, Copy)]
enum Form {
  /// The async walker, written once.
  Async,
  /// D8's fallback, the state machine.
  Machine,
  /// The hand-written blocking walker, the baseline.
  Handwritten,
}

/// Probes the file at `path` through the block cache, with the walker of `form`.
fn probe_path(path: &Path, options: &Options, form: Form) -> Result<ToyReport, ProbeError> {
  let file = fs::File::open(path).unwrap_or_else(|error| fail(&format!("{}: {error}", path.display())));
  match form {
    Form::Async => blocking::probe_reader(file, options),
    Form::Machine => machine::probe_reader(file, options),
    Form::Handwritten => handwritten::probe_reader(file, options),
  }
}

fn overhead(criterion: &mut Criterion) {
  let options = Options {
    limits: Limits::default().with_work_units(u64::MAX).with_bytes_read(u64::MAX),
    ..Options::default()
  };
  let inputs = [("few", toy_file(16)), ("many", toy_file(100_000))];
  let references: Vec<ToyReport> = inputs
    .iter()
    .map(|(name, file)| reference(name, file, &options))
    .collect();
  let mut memory = criterion.benchmark_group("memory");
  for ((name, file), reference) in inputs.iter().zip(&references) {
    check(name, handwritten::probe_bytes(file, &options), reference);
    check(name, machine::probe_bytes(file, &options), reference);
    memory.bench_function(format!("async/{name}"), |bencher| {
      bencher.iter(|| timed(blocking::probe_bytes(black_box(file), &options)));
    });
    memory.bench_function(format!("machine/{name}"), |bencher| {
      bencher.iter(|| timed(machine::probe_bytes(black_box(file), &options)));
    });
    memory.bench_function(format!("handwritten/{name}"), |bencher| {
      bencher.iter(|| timed(handwritten::probe_bytes(black_box(file), &options)));
    });
  }
  memory.finish();
  let mut cached = criterion.benchmark_group("cursor");
  for ((name, file), reference) in inputs.iter().zip(&references) {
    check(
      name,
      blocking::probe_reader(Cursor::new(file.as_slice()), &options),
      reference,
    );
    check(
      name,
      handwritten::probe_reader(Cursor::new(file.as_slice()), &options),
      reference,
    );
    check(
      name,
      machine::probe_reader(Cursor::new(file.as_slice()), &options),
      reference,
    );
    cached.bench_function(format!("async/{name}"), |bencher| {
      bencher.iter(|| {
        timed(blocking::probe_reader(
          Cursor::new(black_box(file.as_slice())),
          &options,
        ))
      });
    });
    cached.bench_function(format!("machine/{name}"), |bencher| {
      bencher.iter(|| timed(machine::probe_reader(Cursor::new(black_box(file.as_slice())), &options)));
    });
    cached.bench_function(format!("handwritten/{name}"), |bencher| {
      bencher.iter(|| {
        timed(handwritten::probe_reader(
          Cursor::new(black_box(file.as_slice())),
          &options,
        ))
      });
    });
  }
  cached.finish();
  let folder = std::env::temp_dir().join(format!("input-spike-bench-{}", process::id()));
  if let Err(error) = fs::create_dir_all(&folder) {
    fail(&format!("{}: {error}", folder.display()));
  }
  let mut files = criterion.benchmark_group("file");
  for ((name, file), reference) in inputs.iter().zip(&references) {
    let path = folder.join(format!("{name}.toy"));
    if let Err(error) = fs::write(&path, file) {
      fail(&format!("{}: {error}", path.display()));
    }
    for form in [Form::Async, Form::Machine, Form::Handwritten] {
      check(name, probe_path(&path, &options, form), reference);
    }
    files.bench_function(format!("async/{name}"), |bencher| {
      bencher.iter(|| timed(probe_path(&path, &options, Form::Async)));
    });
    files.bench_function(format!("machine/{name}"), |bencher| {
      bencher.iter(|| timed(probe_path(&path, &options, Form::Machine)));
    });
    files.bench_function(format!("handwritten/{name}"), |bencher| {
      bencher.iter(|| timed(probe_path(&path, &options, Form::Handwritten)));
    });
  }
  files.finish();
  let _ = fs::remove_dir_all(&folder);
}

criterion_group!(benches, overhead);
criterion_main!(benches);
