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

//! The test's own custom positional sources over a fixture's bytes in memory (plan section 7.1): the blocking
//! `PositionalSource` and the async `AsyncPositionalSource`, which implement the library's own traits, so they live
//! in the library's tests rather than in the test kit. Each applies the test kit's `Faults` at the positions of its
//! own reads, a case's pieces among them, and counts its reads with the kit's `Counts`. The async source is pending
//! once before it serves each read, waking its task at once, so that every case shows parity across suspension. Each
//! borrows the fixture's bytes, which it never copies: the golden pass's readers and sources had kept five copies
//! of a fixture alive at once, about 105 MiB for one of 16 MiB, before the third review of plan Phase 3, task 4,
//! found it.

use std::io;
use std::pin::Pin;
use std::task::{Context, Poll};

use cc_media_probe::asynchronous::AsyncPositionalSource;
use cc_media_probe::source::PositionalSource;
use cc_media_probe_testkit::readers::{Counts, Faults};

/// A custom positional source over bytes in memory, whose length it knows.
#[derive(Debug)]
pub struct Memory<'bytes> {
  /// The bytes.
  bytes: &'bytes [u8],
  /// What its reads do at fixed positions.
  faults: Faults,
  /// Its reads so far.
  counts: Counts,
}

impl<'bytes> Memory<'bytes> {
  /// A source of `bytes` whose reads are cut into `pieces`.
  pub fn new(bytes: &'bytes [u8], pieces: &[usize]) -> Memory<'bytes> {
    Memory::with_faults(bytes, Faults::new().with_pieces(pieces))
  }

  /// A source of `bytes` with `faults`.
  pub fn with_faults(bytes: &'bytes [u8], faults: Faults) -> Memory<'bytes> {
    Memory {
      bytes,
      faults,
      counts: Counts::default(),
    }
  }

  /// Its reads so far.
  pub fn counts(&self) -> Counts {
    self.counts
  }

  /// Serves a read at `position` into `buffer`, uncounted. The faults apply before the end is known, so that an
  /// error at the end is given, and an empty buffer meets none, so that it returns 0 as the trait requires and
  /// leaves an interruption to the next read: the end was checked first, and an empty read consumed the
  /// interruption, before the second review of plan Phase 3, task 4, found both.
  fn serve(&mut self, position: u64, buffer: &mut [u8]) -> io::Result<usize> {
    let piece = self.faults.apply(position, buffer)?;
    let start = usize::try_from(position).map_or(self.bytes.len(), |start| start.min(self.bytes.len()));
    let rest = &self.bytes[start..];
    let delivered = piece.len().min(rest.len());
    piece[..delivered].copy_from_slice(&rest[..delivered]);
    Ok(delivered)
  }
}

impl PositionalSource for Memory<'_> {
  /// Counts every call, one that fails as a call that delivered nothing, as the test kit's `CountingReader` counts
  /// a reader's. A failed read went uncounted before the review of plan Phase 3, task 4, found it.
  fn read_at(&mut self, position: u64, buffer: &mut [u8]) -> io::Result<usize> {
    let delivered = self.serve(position, buffer);
    self.counts.record(*delivered.as_ref().unwrap_or(&0));
    delivered
  }

  fn length(&mut self) -> io::Result<Option<u64>> {
    Ok(Some(u64::try_from(self.bytes.len()).unwrap()))
  }
}

/// The async form of `Memory`, pending once before each read.
#[derive(Debug)]
pub struct AsyncMemory<'bytes> {
  /// The source that serves the reads.
  memory: Memory<'bytes>,
  /// Whether the next poll of a read is pending.
  pending: bool,
}

impl<'bytes> AsyncMemory<'bytes> {
  /// A source of `bytes` whose reads are cut into `pieces`.
  pub fn new(bytes: &'bytes [u8], pieces: &[usize]) -> AsyncMemory<'bytes> {
    AsyncMemory {
      memory: Memory::new(bytes, pieces),
      pending: true,
    }
  }

  /// Its reads so far.
  pub fn counts(&self) -> Counts {
    self.memory.counts()
  }
}

impl AsyncPositionalSource for AsyncMemory<'_> {
  fn poll_read_at(
    self: Pin<&mut Self>,
    context: &mut Context<'_>,
    position: u64,
    buffer: &mut [u8],
  ) -> Poll<io::Result<usize>> {
    let this = self.get_mut();
    if owed(&mut this.pending, context) {
      return Poll::Pending;
    }
    Poll::Ready(this.memory.read_at(position, buffer))
  }

  fn poll_length(self: Pin<&mut Self>, _context: &mut Context<'_>) -> Poll<io::Result<Option<u64>>> {
    Poll::Ready(self.get_mut().memory.length())
  }
}

/// Whether this poll of a read is the pending one: every other poll is, starting with the first, after it wakes the
/// task of `context` so that the read is polled again at once.
pub fn owed(pending: &mut bool, context: &Context<'_>) -> bool {
  let owe = *pending;
  if owe {
    context.waker().wake_by_ref();
  }
  *pending = !owe;
  owe
}
