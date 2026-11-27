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

//! The measured heap peak of plan section 7.5 for the async probe, the hand-written one and D8's fallback, the state
//! machine, under an allocation-tracking allocator, beside the peak that the budget charged; and the size of each
//! async probe future, the state machine's among them. It prints one line per measurement and fails when the
//! measured peak exceeds the charged one by more than the stated fixed overhead, the probes' peaks differ, or a
//! probe did not walk the whole file: the waiting stream must see every packet, the first header must parse, and no
//! limit may stop the run, so a peak measured on a probe that stopped early never counts as evidence.

use std::io::Cursor;
use std::mem::size_of_val;

use cc_media_probe::Limits;
use input::Options;
use input::asynchronous::{self, futures, tokio as tokio_entry};
use input::blocking;
use input::builder::{ToyFile, payload_unit, setup_unit};
use input::codec::Framing;
use input::handwritten;
use input::machine;
use input::report::{HeaderStatus, Reason, ToyReport};
use peak_alloc::PeakAlloc;

#[global_allocator]
static ALLOCATOR: PeakAlloc = PeakAlloc;

/// The fixed overhead allowed beyond the charged peak: the report's own records, which retained data counts and
/// the spike does not, and the small fixed structures that it does not charge.
const FIXED_OVERHEAD: usize = 64 * 1024;

/// The packets of the waiting stream.
const PACKETS: u64 = 20_000;

/// A toy file of `PACKETS` packets for a waiting stream.
fn toy_file() -> Vec<u8> {
  let mut file = ToyFile::new()
    .title("Measured")
    .stream(1, Framing::Packet)
    .stream(2, Framing::Packet)
    .packet(1, &setup_unit(1920, 1080, 0, 0));
  for _ in 0..PACKETS {
    file = file.packet(2, &payload_unit(30));
  }
  file.build()
}

/// Whether `report` walked the whole toy file: the title read, the first header parsed, every packet offered to
/// the waiting stream until the input ended, and no limit exhausted.
fn walked_whole(report: &ToyReport) -> bool {
  let first = report.streams.first().map(|stream| &stream.header);
  let waiting = report.streams.get(1);
  report.titles.len() == 1
    && matches!(first, Some(HeaderStatus::Parsed(_)))
    && waiting.is_some_and(|stream| {
      stream.packets_offered == PACKETS && stream.header == HeaderStatus::NotFound(Reason::TruncatedInput)
    })
    && report.exhausted.is_empty()
}

/// Runs `probe` with the peak reset, and gives the measured heap peak above the heap in use before it, the charged
/// peak, and whether the probe walked the whole file.
fn measure(name: &str, probe: impl FnOnce() -> ToyReport) -> (usize, u64, bool) {
  let before = ALLOCATOR.current_usage();
  ALLOCATOR.reset_peak_usage();
  let report = probe();
  let measured = ALLOCATOR.peak_usage().saturating_sub(before);
  let charged = report.usage.peak_working_memory;
  let whole = walked_whole(&report);
  println!("{name}: measured heap peak {measured} bytes, charged working memory peak {charged} bytes");
  if !whole {
    println!("{name} did not walk the whole file: {report:?}");
  }
  (measured, charged, whole)
}

/// Measures each probe and prints the lines; exits with status 1 when a bound fails.
pub fn run() {
  let file = toy_file();
  let options = Options {
    limits: Limits::default().with_work_units(u64::MAX),
    ..Options::default()
  };
  let measurements = [
    measure("async, bytes", || blocking::probe_bytes(&file, &options).unwrap()),
    measure("hand-written, bytes", || {
      handwritten::probe_bytes(&file, &options).unwrap()
    }),
    measure("async, reader", || {
      blocking::probe_reader(Cursor::new(file.as_slice()), &options).unwrap()
    }),
    measure("hand-written, reader", || {
      handwritten::probe_reader(Cursor::new(file.as_slice()), &options).unwrap()
    }),
    measure("async, stream", || {
      blocking::probe_stream(Cursor::new(file.as_slice()), &options).unwrap()
    }),
    measure("state machine, bytes", || {
      machine::probe_bytes(&file, &options).unwrap()
    }),
    measure("state machine, reader", || {
      machine::probe_reader(Cursor::new(file.as_slice()), &options).unwrap()
    }),
  ];
  let mut failed = false;
  for (measured, charged, whole) in measurements {
    if !whole
      || measured
        > usize::try_from(charged)
          .unwrap_or(usize::MAX)
          .saturating_add(FIXED_OVERHEAD)
    {
      failed = true;
    }
  }
  let [
    async_bytes,
    hand_bytes,
    async_reader,
    hand_reader,
    _,
    machine_bytes,
    machine_reader,
  ] = measurements;
  if async_bytes.0 != hand_bytes.0 || async_reader.0 != hand_reader.0 {
    println!("the async and hand-written peaks differ");
    failed = true;
  }
  if machine_bytes.0 != hand_bytes.0 || machine_reader.0 != hand_reader.0 {
    println!("the state machine's and the hand-written peaks differ");
    failed = true;
  }
  let pending = std::iter::empty::<u8>().collect::<Vec<_>>();
  let custom = asynchronous::probe_source(Ready(pending), &options);
  println!("future of asynchronous::probe_source: {} bytes", size_of_val(&custom));
  let tokio_reader = tokio_entry::probe_reader(Cursor::new(file.clone()), &options);
  println!(
    "future of asynchronous::tokio::probe_reader: {} bytes",
    size_of_val(&tokio_reader)
  );
  let tokio_stream = tokio_entry::probe_stream(Cursor::new(file.clone()), &options);
  println!(
    "future of asynchronous::tokio::probe_stream: {} bytes",
    size_of_val(&tokio_stream)
  );
  let machine_source = machine::probe_source(Ready(Vec::new()), &options);
  println!(
    "future of machine::probe_source: {} bytes",
    size_of_val(&machine_source)
  );
  let futures_reader = futures::probe_reader(futures_lite::io::Cursor::new(file.clone()), &options);
  println!(
    "future of asynchronous::futures::probe_reader: {} bytes",
    size_of_val(&futures_reader)
  );
  if failed {
    std::process::exit(1);
  }
}

/// An async source that is always ready.
struct Ready(Vec<u8>);

impl cc_media_probe::asynchronous::AsyncPositionalSource for Ready {
  fn poll_read_at(
    self: std::pin::Pin<&mut Self>,
    _context: &mut std::task::Context<'_>,
    _position: u64,
    _buffer: &mut [u8],
  ) -> std::task::Poll<std::io::Result<usize>> {
    std::task::Poll::Ready(Ok(self.0.len()))
  }

  fn poll_length(
    self: std::pin::Pin<&mut Self>,
    _context: &mut std::task::Context<'_>,
  ) -> std::task::Poll<std::io::Result<Option<u64>>> {
    std::task::Poll::Ready(Ok(None))
  }
}
