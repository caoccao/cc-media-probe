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

//! The source protocol of `api.md` sections 4.4 and 9.3 through the block cache and the window, as G3.1 is to
//! confirm (`api.md` section 9.3): a pending request is repeated with the same range, a source reused after a
//! dropped probe answers the new request, no read asks for zero bytes, and each probe future is `Send` for a `Send`
//! input. The cases of `docs/design/deferred-cases.md` section 5, run against the spike.

use std::future::poll_fn;
use std::io::{self, Cursor};
use std::pin::{Pin, pin};
use std::task::{Context, Poll, Waker};

use cc_media_probe::Limits;
use cc_media_probe::asynchronous::AsyncPositionalSource;
use futures_lite::future::block_on;
use input::asynchronous::{self, futures, tokio as tokio_entry};
use input::blocking;
use input::report::DetectionOutcome;
use tokio::io::{AsyncRead, ReadBuf};

use crate::support::{PendingSource, RecordingSource, semantic, small_options, typical_file};

#[test]
fn a_pending_read_is_asked_again_with_the_same_request() {
  let options = small_options();
  let mut source = PendingSource::new(&typical_file());
  let report = block_on(asynchronous::probe_source(&mut source, &options)).unwrap();
  assert_eq!(report.detection, DetectionOutcome::Identified);
  assert!(source.requests.len() >= 4);
  assert_eq!(source.requests.len() % 2, 0);
  for pair in source.requests.chunks(2) {
    assert_eq!(pair[0], pair[1]);
  }
  assert!(source.requests.iter().all(|(_, length)| *length > 0));
}

/// A Tokio reader that answers `Poll::Pending` before every read, waking at once, and records the room of every
/// call's buffer.
struct HesitantReader {
  inner: Cursor<Vec<u8>>,
  pending_next: bool,
  rooms: Vec<usize>,
}

impl AsyncRead for HesitantReader {
  fn poll_read(mut self: Pin<&mut Self>, context: &mut Context<'_>, buffer: &mut ReadBuf<'_>) -> Poll<io::Result<()>> {
    self.rooms.push(buffer.remaining());
    if self.pending_next {
      self.pending_next = false;
      context.waker().wake_by_ref();
      return Poll::Pending;
    }
    self.pending_next = true;
    Pin::new(&mut self.inner).poll_read(context, buffer)
  }
}

#[test]
fn the_window_repeats_a_pending_read_with_the_same_buffer() {
  let options = small_options();
  let mut reader = HesitantReader {
    inner: Cursor::new(typical_file()),
    pending_next: true,
    rooms: Vec::new(),
  };
  let runtime = tokio::runtime::Builder::new_current_thread().build().unwrap();
  let report = runtime
    .block_on(tokio_entry::probe_stream(&mut reader, &options))
    .unwrap();
  let expected = blocking::probe_bytes(&typical_file(), &options).unwrap();
  assert_eq!(semantic(&report), semantic(&expected));
  assert_eq!(reader.rooms.len() % 2, 0);
  for pair in reader.rooms.chunks(2) {
    assert_eq!(pair[0], pair[1]);
  }
  assert!(reader.rooms.iter().all(|room| *room > 0));
}

/// An async source over the bytes 0 to 255 that keeps the range of its request in flight, as the source of
/// `api.md` section 15.7 does: a call for another range starts a request and is pending; a call for the range in
/// flight answers it.
struct ReusableSource {
  data: Vec<u8>,
  in_flight: Option<(u64, usize)>,
  calls: Vec<(u64, usize)>,
}

impl AsyncPositionalSource for ReusableSource {
  fn poll_read_at(
    mut self: Pin<&mut Self>,
    context: &mut Context<'_>,
    position: u64,
    buffer: &mut [u8],
  ) -> Poll<io::Result<usize>> {
    self.calls.push((position, buffer.len()));
    if self.in_flight == Some((position, buffer.len())) {
      self.in_flight = None;
      return Poll::Ready(Ok(crate::support::copy_from(&self.data, position, buffer)));
    }
    self.in_flight = Some((position, buffer.len()));
    context.waker().wake_by_ref();
    Poll::Pending
  }

  fn poll_length(self: Pin<&mut Self>, _context: &mut Context<'_>) -> Poll<io::Result<Option<u64>>> {
    Poll::Ready(Ok(Some(256)))
  }
}

#[test]
fn a_source_reused_after_a_dropped_probe_answers_the_new_request() {
  let options = small_options();
  let mut source = ReusableSource {
    data: (0..=255).collect(),
    in_flight: None,
    calls: Vec::new(),
  };
  {
    let mut probe = pin!(asynchronous::probe_source(&mut source, &options));
    let mut context = Context::from_waker(Waker::noop());
    assert!(probe.as_mut().poll(&mut context).is_pending());
  }
  let mut bytes = [0_u8; 4];
  let count = block_on(poll_fn(|context| {
    Pin::new(&mut source).poll_read_at(context, 100, &mut bytes)
  }))
  .unwrap();
  assert_eq!((count, bytes), (4, [100, 101, 102, 103]));
  let report = block_on(asynchronous::probe_source(&mut source, &options)).unwrap();
  assert_eq!(report.detection, DetectionOutcome::Unknown);
  assert_eq!(source.calls, [(0, 64), (100, 4), (100, 4), (0, 64), (0, 64)]);
}

#[test]
fn a_blocking_probe_never_asks_for_zero_bytes() {
  let file = typical_file();
  for limit in [0, 1, 7, 13, 64, 65, 200, u64::MAX] {
    let options = input::Options {
      limits: Limits::default().with_bytes_read(limit),
      ..small_options()
    };
    let mut source = RecordingSource::new(&file);
    let report = blocking::probe_source(&mut source, &options).unwrap();
    assert!(source.requests.iter().all(|(_, length)| *length > 0), "limit {limit}");
    assert!(report.usage.bytes_read <= limit);
    let delivered: u64 = source
      .requests
      .iter()
      .map(|(position, length)| {
        let end = usize::try_from(*position)
          .unwrap()
          .saturating_add(*length)
          .min(file.len());
        u64::try_from(end.saturating_sub(usize::try_from(*position).unwrap())).unwrap()
      })
      .sum();
    assert_eq!(delivered, report.usage.bytes_read, "limit {limit}");
    if limit == 0 {
      assert_eq!(source.requests, [] as [(u64, usize); 0]);
    }
  }
}

/// Whether `value` is `Send`, which the call compiles only for.
fn is_send<Value: Send>(_: &Value) -> bool {
  true
}

#[test]
fn every_probe_future_is_send_for_a_send_input() {
  let options = small_options();
  let file = typical_file();
  let mut pending = PendingSource::new(&file);
  let custom = asynchronous::probe_source(&mut pending, &options);
  assert!(is_send(&custom));
  let tokio_reader = tokio_entry::probe_reader(Cursor::new(file.clone()), &options);
  assert!(is_send(&tokio_reader));
  let tokio_stream = tokio_entry::probe_stream(Cursor::new(file.clone()), &options);
  assert!(is_send(&tokio_stream));
  let futures_reader = futures::probe_reader(futures_lite::io::Cursor::new(file.clone()), &options);
  assert!(is_send(&futures_reader));
  let futures_stream = futures::probe_stream(futures_lite::io::Cursor::new(file), &options);
  assert!(is_send(&futures_stream));
}

#[test]
fn a_probe_future_over_a_source_that_is_not_send_still_runs() {
  let options = small_options();
  let shared = std::rc::Rc::new(typical_file());
  let source = LocalSource { data: shared };
  let report = block_on(asynchronous::probe_source(source, &options)).unwrap();
  assert_eq!(report.detection, DetectionOutcome::Identified);
}

/// A source that holds an `Rc`, so neither it nor its probe future is `Send`.
struct LocalSource {
  data: std::rc::Rc<Vec<u8>>,
}

impl AsyncPositionalSource for LocalSource {
  fn poll_read_at(
    self: Pin<&mut Self>,
    _context: &mut Context<'_>,
    position: u64,
    buffer: &mut [u8],
  ) -> Poll<io::Result<usize>> {
    Poll::Ready(Ok(crate::support::copy_from(&self.data, position, buffer)))
  }

  fn poll_length(self: Pin<&mut Self>, _context: &mut Context<'_>) -> Poll<io::Result<Option<u64>>> {
    Poll::Ready(Ok(None))
  }
}
