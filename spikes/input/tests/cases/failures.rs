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

//! A source that fails while headers are read (R19; plan section 7.4, "End of input, limits"). The error's partial
//! report keeps each waiting stream's fields with the reason "source failed", says "not attempted" of a header
//! that no packet began, returns every charge, and records the failed source as failed. This holds on every
//! blocking input, the async one and the hand-written probe, and for a companion.

use std::io::{self, Read, Seek, SeekFrom};
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

use cc_media_probe::asynchronous::AsyncPositionalSource;
use cc_media_probe::source::PositionalSource;
use futures_lite::future::block_on;
use input::builder::{ToyFile, parameters_unit, payload_unit, setup_unit};
use input::codec::{Framing, Setup};
use input::companion::{Answer, CompanionLookup, CompanionRequest, CompanionResolver, Resolution};
use input::report::{HeaderStatus, ProbeError, Reason, SourceOutcome, ToyReport};
use input::{asynchronous, blocking, handwritten, machine};

use crate::support::{TemporaryFolder, copy_from, parsed, size, small_options};

/// The position from which every read fails.
const FAILING_AT: u64 = 128;

/// Bytes that are read up to a failing position, `FAILING_AT` unless given, and fail from there on, at a position or
/// from where a reader stands.
struct Failing {
  data: Vec<u8>,
  position: u64,
  fails_from: u64,
}

impl Failing {
  fn new(data: Vec<u8>) -> Failing {
    Failing::at(data, FAILING_AT)
  }

  fn at(data: Vec<u8>, failing_at: u64) -> Failing {
    Failing {
      data,
      position: 0,
      fails_from: failing_at,
    }
  }

  fn read_from(&self, position: u64, buffer: &mut [u8]) -> io::Result<usize> {
    if position >= self.fails_from {
      return Err(io::Error::other("the disk failed"));
    }
    let length = buffer
      .len()
      .min(usize::try_from(self.fails_from.saturating_sub(position)).unwrap());
    Ok(copy_from(&self.data, position, &mut buffer[..length]))
  }
}

impl PositionalSource for Failing {
  fn read_at(&mut self, position: u64, buffer: &mut [u8]) -> io::Result<usize> {
    self.read_from(position, buffer)
  }

  fn length(&mut self) -> io::Result<Option<u64>> {
    Ok(None)
  }
}

impl AsyncPositionalSource for Failing {
  fn poll_read_at(
    self: Pin<&mut Self>,
    _context: &mut Context<'_>,
    position: u64,
    buffer: &mut [u8],
  ) -> Poll<io::Result<usize>> {
    Poll::Ready(self.read_from(position, buffer))
  }

  fn poll_length(self: Pin<&mut Self>, _context: &mut Context<'_>) -> Poll<io::Result<Option<u64>>> {
    Poll::Ready(Ok(None))
  }
}

impl Read for Failing {
  fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
    let count = self.read_from(self.position, buffer)?;
    self.position = self.position.saturating_add(u64::try_from(count).unwrap());
    Ok(count)
  }
}

impl Seek for Failing {
  fn seek(&mut self, position: SeekFrom) -> io::Result<u64> {
    let SeekFrom::Start(position) = position else {
      return Err(io::Error::other("only seeks from the start"));
    };
    self.position = position;
    Ok(position)
  }
}

/// Three streams: one whose setup comes before the failure, one with nothing before it, and one in stream mode with
/// only payload before it; then a long title, whose read fails, and the rest of the headers.
fn failing_file() -> Vec<u8> {
  ToyFile::new()
    .stream(1, Framing::Packet)
    .stream(2, Framing::Packet)
    .stream(3, Framing::Stream)
    .packet(1, &setup_unit(320, 240, 2, 0))
    .packet(3, &payload_unit(2))
    .title(&"p".repeat(200))
    .packet(1, &parameters_unit(&[1, 2]).unwrap())
    .packet(2, &setup_unit(640, 480, 0, 0))
    .build()
}

/// The setup of stream 1 of `failing_file`.
fn setup(width: u16, height: u16, parameter_count: u8) -> Setup {
  Setup {
    width,
    height,
    parameter_count,
    flags: 0,
  }
}

/// Checks the error of a probe of `failing_file` whose primary input failed.
fn check(input: &str, result: Result<ToyReport, ProbeError>) {
  let Err(ProbeError::SourceFailed { source, cause, partial }) = result else {
    panic!("{input}: the probe did not fail");
  };
  assert_eq!(source, 0, "{input}");
  assert_eq!(cause.to_string(), "the disk failed", "{input}");
  let headers: Vec<HeaderStatus> = partial.streams.iter().map(|stream| stream.header.clone()).collect();
  assert_eq!(
    headers,
    [
      HeaderStatus::PartiallyParsedCutShort(setup(320, 240, 2), Reason::SourceFailed),
      HeaderStatus::NotAttempted(Reason::SourceFailed),
      HeaderStatus::NotFound(Reason::SourceFailed),
    ],
    "{input}"
  );
  assert_eq!(partial.sources[0].outcome, SourceOutcome::Failed, "{input}");
  assert_eq!(partial.usage.working_memory_at_end, 0, "{input}");
}

#[test]
fn a_failing_primary_input_keeps_the_fields_read_on_every_input() {
  let options = small_options();
  check("source", blocking::probe_source(Failing::new(failing_file()), &options));
  check("reader", blocking::probe_reader(Failing::new(failing_file()), &options));
  check("stream", blocking::probe_stream(Failing::new(failing_file()), &options));
  check(
    "async source",
    block_on(asynchronous::probe_source(Failing::new(failing_file()), &options)),
  );
  check(
    "hand-written reader",
    handwritten::probe_reader(Failing::new(failing_file()), &options),
  );
}

#[test]
fn a_file_that_cannot_be_opened_is_a_failed_source() {
  let folder = TemporaryFolder::new().unwrap();
  let Err(ProbeError::SourceFailed { source, partial, .. }) =
    blocking::probe_file(folder.path().join("missing.toy"), &small_options())
  else {
    panic!("the probe did not fail");
  };
  assert_eq!(source, 0);
  assert_eq!(partial.sources[0].outcome, SourceOutcome::Failed);
}

/// A resolver whose one companion fails after its setup.
struct FailingCompanion;

impl CompanionResolver for FailingCompanion {
  fn resolve(&self, _request: &CompanionRequest) -> Resolution {
    let data = ToyFile::new()
      .stream(1, Framing::Packet)
      .packet(1, &setup_unit(800, 600, 1, 0))
      .title(&"p".repeat(200))
      .packet(1, &parameters_unit(&[3]).unwrap())
      .build();
    Answer::Found {
      source: Box::new(Failing::new(data)),
      identity: None,
    }
  }
}

#[test]
fn a_companion_that_fails_keeps_its_fields_and_leaves_the_primary_input_opened() {
  let primary = ToyFile::new()
    .companion("movie")
    .stream(1, Framing::Packet)
    .packet(1, &setup_unit(320, 240, 0, 0))
    .build();
  let options = input::Options {
    companions: CompanionLookup::Custom(Arc::new(FailingCompanion)),
    ..small_options()
  };
  let Err(ProbeError::SourceFailed { source, partial, .. }) = blocking::probe_bytes(&primary, &options) else {
    panic!("the probe did not fail");
  };
  assert_eq!(source, 1);
  assert_eq!(partial.streams[0].header, parsed(320, 240, &[]));
  assert_eq!(
    partial.streams[1].header,
    HeaderStatus::PartiallyParsedCutShort(setup(800, 600, 1), Reason::SourceFailed)
  );
  let outcomes: Vec<SourceOutcome> = partial.sources.iter().map(|entry| entry.outcome.clone()).collect();
  assert_eq!(outcomes, [SourceOutcome::Opened, SourceOutcome::Failed]);
  assert_eq!(partial.usage.working_memory_at_end, 0);
}

#[test]
fn bytes_delivered_before_a_read_fails_are_offered_to_their_stream() {
  let file = ToyFile::new().stream(1, Framing::Packet);
  let payload = file.position() + 8 + 1;
  let file = file
    .packet(1, &[setup_unit(320, 240, 1, 0), payload_unit(20)].concat())
    .packet(1, &parameters_unit(&[7]).unwrap())
    .build();
  let failing_at = payload + 12;
  let failing = || Failing::at(file.clone(), failing_at);
  for piece in [8, 16] {
    let options = input::Options {
      piece: size(piece),
      ..small_options()
    };
    let results = [
      ("source", blocking::probe_source(failing(), &options)),
      ("reader", blocking::probe_reader(failing(), &options)),
      ("stream", blocking::probe_stream(failing(), &options)),
      (
        "async source",
        block_on(asynchronous::probe_source(failing(), &options)),
      ),
      ("hand-written reader", handwritten::probe_reader(failing(), &options)),
      ("state machine reader", machine::probe_reader(failing(), &options)),
      (
        "async state machine",
        block_on(machine::probe_source(failing(), &options)),
      ),
    ];
    for (input, result) in results {
      let Err(ProbeError::SourceFailed { partial, .. }) = result else {
        panic!("{input}: the probe did not fail");
      };
      assert_eq!(
        partial.streams[0].header,
        HeaderStatus::PartiallyParsedCutShort(setup(320, 240, 1), Reason::SourceFailed),
        "{input}, pieces of {piece} bytes"
      );
      assert_eq!(
        partial.streams[0].packets_offered, 1,
        "{input}, pieces of {piece} bytes"
      );
      assert_eq!(
        partial.sources[0].bytes_read, failing_at,
        "{input}, pieces of {piece} bytes"
      );
      assert_eq!(
        partial.usage.working_memory_at_end, 0,
        "{input}, pieces of {piece} bytes"
      );
    }
  }
}
