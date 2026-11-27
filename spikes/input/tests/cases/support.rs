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

//! What the cases share: options with small buffers, a typical toy file, sources and readers that record their
//! requests or return pending, a temporary folder, and the semantic part of a report.

use std::fs;
use std::io;
use std::num::NonZeroUsize;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::sync::atomic::{AtomicU64, Ordering};
use std::task::{Context, Poll};

use cc_media_probe::asynchronous::AsyncPositionalSource;
use cc_media_probe::source::PositionalSource;
use input::Options;
use input::builder::{ToyFile, parameters_unit, payload_unit, setup_unit};
use input::codec::{Framing, Header, Setup};
use input::report::{HeaderStatus, ToyReport};

/// Options with small buffers, so that small files reach every edge. The window still covers the typical file, so
/// every input gives its report.
pub fn small_options() -> Options {
  Options {
    block_size: size(64),
    cached_blocks: size(2),
    window: size(512),
    prefix: size(64),
    chunk: size(16),
    piece: size(16),
    ..Options::default()
  }
}

/// A positive size of a buffer.
pub fn size(value: usize) -> NonZeroUsize {
  NonZeroUsize::new(value).unwrap()
}

/// The header of a setup without flags and the sum of its parameters.
pub fn parsed(width: u16, height: u16, parameters: &[u16]) -> HeaderStatus {
  HeaderStatus::Parsed(Header {
    setup: Setup {
      width,
      height,
      parameter_count: u8::try_from(parameters.len()).unwrap_or(u8::MAX),
      flags: 0,
    },
    parameter_sum: parameters.iter().map(|parameter| u32::from(*parameter)).sum(),
  })
}

/// A typical toy file: a title, a stream in packet mode and one in stream mode whose setup two deliveries split,
/// packets of both interleaved, a list with a title and a reference to the first title, and a stream of another
/// codec.
pub fn typical_file() -> Vec<u8> {
  let setup = setup_unit(640, 480, 3, 0);
  let (head, tail) = setup.split_at(5);
  ToyFile::new()
    .title("Concert")
    .stream(1, Framing::Packet)
    .stream(2, Framing::Stream)
    .packet(1, &[setup_unit(1920, 1080, 2, 0), payload_unit(10)].concat())
    .packet(2, &[payload_unit(4), head.to_vec()].concat())
    .packet(2, &[tail.to_vec(), parameters_unit(&[1, 2, 3]).unwrap()].concat())
    .packet(1, &parameters_unit(&[7, 8]).unwrap())
    .open_list()
    .title("Encore")
    .title_reference(12)
    .close_list()
    .stream_with_codec(3, Framing::Packet, *b"OTHR")
    .packet(3, b"opaque")
    .build()
}

/// The report without what depends on how the input was read: the usage and the bytes of each source.
pub fn semantic(report: &ToyReport) -> ToyReport {
  let mut report = report.clone();
  report.usage.bytes_read = 0;
  report.usage.peak_working_memory = 0;
  for source in &mut report.sources {
    source.bytes_read = 0;
  }
  report
}

/// A blocking positional source over bytes, of unknown length, that records the position and length of every
/// request and fills each buffer as far as the bytes go.
pub struct RecordingSource {
  data: Vec<u8>,
  /// Every request, in order.
  pub requests: Vec<(u64, usize)>,
}

impl RecordingSource {
  /// The source over `data`.
  pub fn new(data: &[u8]) -> RecordingSource {
    RecordingSource {
      data: data.to_vec(),
      requests: Vec::new(),
    }
  }
}

impl PositionalSource for RecordingSource {
  fn read_at(&mut self, position: u64, buffer: &mut [u8]) -> io::Result<usize> {
    self.requests.push((position, buffer.len()));
    Ok(copy_from(&self.data, position, buffer))
  }

  fn length(&mut self) -> io::Result<Option<u64>> {
    Ok(None)
  }
}

/// An async positional source over bytes that answers `Poll::Pending` to every other call, starting with the
/// first, after waking the call's waker at once, and records every call.
pub struct PendingSource {
  data: Vec<u8>,
  pending_next: bool,
  /// Every call, in order.
  pub requests: Vec<(u64, usize)>,
}

impl PendingSource {
  /// The source over `data`.
  pub fn new(data: &[u8]) -> PendingSource {
    PendingSource {
      data: data.to_vec(),
      pending_next: true,
      requests: Vec::new(),
    }
  }
}

impl AsyncPositionalSource for PendingSource {
  fn poll_read_at(
    mut self: Pin<&mut Self>,
    context: &mut Context<'_>,
    position: u64,
    buffer: &mut [u8],
  ) -> Poll<io::Result<usize>> {
    self.requests.push((position, buffer.len()));
    if self.pending_next {
      self.pending_next = false;
      context.waker().wake_by_ref();
      return Poll::Pending;
    }
    self.pending_next = true;
    Poll::Ready(Ok(copy_from(&self.data, position, buffer)))
  }

  fn poll_length(self: Pin<&mut Self>, _context: &mut Context<'_>) -> Poll<io::Result<Option<u64>>> {
    Poll::Ready(Ok(None))
  }
}

/// Copies what `data` holds from `position` on into `buffer`.
pub fn copy_from(data: &[u8], position: u64, buffer: &mut [u8]) -> usize {
  let start = usize::try_from(position).map_or(data.len(), |start| start.min(data.len()));
  let available = &data[start..];
  let count = available.len().min(buffer.len());
  buffer[..count].copy_from_slice(&available[..count]);
  count
}

/// A folder of its own below the system's temporary folder, removed when dropped.
pub struct TemporaryFolder {
  path: PathBuf,
}

impl TemporaryFolder {
  /// A new, empty folder.
  pub fn new() -> io::Result<TemporaryFolder> {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let number = COUNTER.fetch_add(1, Ordering::Relaxed);
    let name = format!("input-spike-{}-{number}", std::process::id());
    let path = std::env::temp_dir().join(name);
    fs::create_dir_all(&path)?;
    Ok(TemporaryFolder {
      path: fs::canonicalize(path)?,
    })
  }

  /// The folder's path.
  pub fn path(&self) -> &Path {
    &self.path
  }
}

impl Drop for TemporaryFolder {
  fn drop(&mut self) {
    let _ = fs::remove_dir_all(&self.path);
  }
}
