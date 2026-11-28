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

//! An async reader over a standard-library reader for the async adapters of both runtimes, Tokio's `AsyncRead` and
//! `AsyncSeek` and those of futures-io, which is pending once before it serves each read, waking its task at once,
//! so that every case shows parity across suspension (`docs/design/test-kit.md` section 4.1). The bytes, the pieces
//! and the counts are those of the reader inside it, the test kit's counting and faulting readers, as the blocking
//! adapters have them. A runtime's traits are no part of the test kit, which uses the standard library alone, so
//! the reader lives in the library's tests (plan section 7.1).

use std::io::{self, Read, Seek, SeekFrom};
use std::pin::Pin;
use std::task::{Context, Poll};

use super::memory::owed;

/// An async reader of `Reader`, pending once before each read.
#[derive(Debug)]
pub struct Pending<Reader> {
  /// The reader that serves the reads and seeks.
  reader: Reader,
  /// Whether the next poll of a read is the pending one.
  suspends: bool,
  /// The seek that Tokio's `start_seek` asked for, which `poll_complete` makes.
  seek: Option<SeekFrom>,
  /// The position after the last read or seek, which `poll_complete` gives without a seek to make.
  position: u64,
}

impl<Reader> Pending<Reader> {
  /// An async reader of `reader`, at its byte 0.
  pub fn new(reader: Reader) -> Pending<Reader> {
    Pending {
      reader,
      suspends: true,
      seek: None,
      position: 0,
    }
  }

  /// The reader that serves the reads.
  pub fn get_ref(&self) -> &Reader {
    &self.reader
  }
}

impl<Reader: Read> Pending<Reader> {
  /// Serves a read into `buffer`, unless this poll is the pending one.
  fn poll_read_into(&mut self, context: &Context<'_>, buffer: &mut [u8]) -> Poll<io::Result<usize>> {
    if owed(&mut self.suspends, context) {
      return Poll::Pending;
    }
    let read = self.reader.read(buffer);
    if let Ok(delivered) = &read {
      self.position = self.position.saturating_add(u64::try_from(*delivered).unwrap());
    }
    Poll::Ready(read)
  }
}

impl<Reader: Seek> Pending<Reader> {
  /// Makes the seek to `position` at once.
  fn seek_to(&mut self, position: SeekFrom) -> io::Result<u64> {
    let reached = self.reader.seek(position)?;
    self.position = reached;
    Ok(reached)
  }
}

impl<Reader: Read + Unpin> tokio::io::AsyncRead for Pending<Reader> {
  fn poll_read(
    self: Pin<&mut Self>,
    context: &mut Context<'_>,
    buffer: &mut tokio::io::ReadBuf<'_>,
  ) -> Poll<io::Result<()>> {
    let this = self.get_mut();
    match this.poll_read_into(context, buffer.initialize_unfilled()) {
      Poll::Ready(Ok(delivered)) => {
        buffer.advance(delivered);
        Poll::Ready(Ok(()))
      }
      Poll::Ready(Err(error)) => Poll::Ready(Err(error)),
      Poll::Pending => Poll::Pending,
    }
  }
}

impl<Reader: Seek + Unpin> tokio::io::AsyncSeek for Pending<Reader> {
  /// Takes the seek that `poll_complete` makes, or refuses it with `ErrorKind::Other` while another one waits, as
  /// Tokio's trait allows: a second seek replaced the first before the second review of plan Phase 3, task 4, found
  /// it.
  fn start_seek(self: Pin<&mut Self>, position: SeekFrom) -> io::Result<()> {
    let this = self.get_mut();
    if this.seek.is_some() {
      return Err(io::Error::other("a seek is in progress; poll_complete makes it first"));
    }
    this.seek = Some(position);
    Ok(())
  }

  fn poll_complete(self: Pin<&mut Self>, _context: &mut Context<'_>) -> Poll<io::Result<u64>> {
    let this = self.get_mut();
    Poll::Ready(match this.seek.take() {
      Some(position) => this.seek_to(position),
      None => Ok(this.position),
    })
  }
}

impl<Reader: Read + Unpin> futures_io::AsyncRead for Pending<Reader> {
  fn poll_read(self: Pin<&mut Self>, context: &mut Context<'_>, buffer: &mut [u8]) -> Poll<io::Result<usize>> {
    self.get_mut().poll_read_into(context, buffer)
  }
}

impl<Reader: Seek + Unpin> futures_io::AsyncSeek for Pending<Reader> {
  fn poll_seek(self: Pin<&mut Self>, _context: &mut Context<'_>, position: SeekFrom) -> Poll<io::Result<u64>> {
    Poll::Ready(self.get_mut().seek_to(position))
  }
}
