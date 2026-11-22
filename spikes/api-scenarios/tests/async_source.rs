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

//! The contract of `api.md` section 9.3: after `Pending`, the same request is repeated within one probe, and a
//! source that outlives a dropped probe answers the next request for its own range.

use std::future::{Future, poll_fn};
use std::io;
use std::pin::{Pin, pin};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};

use cc_media_probe::ErrorKind;
use cc_media_probe::asynchronous::{AsyncPositionalSource, probe_source};
use futures_lite::future::block_on;

/// Answers `Pending` once before each answer, and records every request.
struct HesitantSource {
  requests: Arc<Mutex<Vec<(u64, usize)>>>,
  ready: bool,
}

impl AsyncPositionalSource for HesitantSource {
  fn poll_read_at(
    self: Pin<&mut Self>,
    context: &mut Context<'_>,
    position: u64,
    buffer: &mut [u8],
  ) -> Poll<io::Result<usize>> {
    let this = self.get_mut();
    this.requests.lock().unwrap().push((position, buffer.len()));
    if !this.ready {
      this.ready = true;
      context.waker().wake_by_ref();
      return Poll::Pending;
    }
    this.ready = false;
    buffer.fill(0x1A);
    Poll::Ready(Ok(buffer.len()))
  }

  fn poll_length(self: Pin<&mut Self>, _context: &mut Context<'_>) -> Poll<io::Result<Option<u64>>> {
    Poll::Ready(Ok(None))
  }
}

/// Keeps one request in flight with its range, as the source of scenario 7 does. A request for a new range is
/// pending once before its bytes are ready.
struct RangedObject {
  object: Vec<u8>,
  pending: Option<(u64, usize)>,
  requests: Vec<(u64, usize)>,
}

impl AsyncPositionalSource for RangedObject {
  fn poll_read_at(
    self: Pin<&mut Self>,
    context: &mut Context<'_>,
    position: u64,
    buffer: &mut [u8],
  ) -> Poll<io::Result<usize>> {
    let this = self.get_mut();
    this.requests.push((position, buffer.len()));
    if this.pending != Some((position, buffer.len())) {
      // A new range, or a request that a dropped probe left behind: start the request for this range.
      this.pending = Some((position, buffer.len()));
      context.waker().wake_by_ref();
      return Poll::Pending;
    }
    this.pending = None;
    let start = usize::try_from(position).unwrap().min(this.object.len());
    let end = (start + buffer.len()).min(this.object.len());
    buffer[..end - start].copy_from_slice(&this.object[start..end]);
    Poll::Ready(Ok(end - start))
  }

  fn poll_length(self: Pin<&mut Self>, _context: &mut Context<'_>) -> Poll<io::Result<Option<u64>>> {
    Poll::Ready(Ok(Some(self.object.len() as u64)))
  }
}

#[test]
fn a_pending_read_is_asked_again_with_the_same_request() {
  let requests = Arc::new(Mutex::new(Vec::new()));
  let source = HesitantSource {
    requests: Arc::clone(&requests),
    ready: false,
  };
  let error = block_on(probe_source(source)).unwrap_err();
  assert_eq!(error.kind(), &ErrorKind::UnsupportedFormat);
  let requests = requests.lock().unwrap();
  assert_eq!(requests.len(), 2);
  assert_eq!(requests[0], requests[1]);
}

#[test]
fn a_source_reused_after_a_dropped_probe_answers_the_new_request() {
  let mut source = RangedObject {
    object: (0..=255).collect(),
    pending: None,
    requests: Vec::new(),
  };
  // The first probe is dropped while its read is pending, and never calls again.
  {
    let mut probe = pin!(probe_source(&mut source));
    assert!(
      probe
        .as_mut()
        .poll(&mut Context::from_waker(Waker::noop()))
        .is_pending()
    );
  }
  assert_eq!(source.requests, [(0, 16)]);
  // The next request asks for another range and gets that range's bytes, not those of the abandoned request.
  let mut buffer = [0_u8; 4];
  let count = block_on(poll_fn(|context| {
    Pin::new(&mut source).poll_read_at(context, 100, &mut buffer)
  }))
  .unwrap();
  assert_eq!(buffer[..count], [100, 101, 102, 103]);
  // A second probe on the same source runs to its ordinary end.
  let error = block_on(probe_source(&mut source)).unwrap_err();
  assert_eq!(error.kind(), &ErrorKind::UnsupportedFormat);
  assert_eq!(source.requests, [(0, 16), (100, 4), (100, 4), (0, 16), (0, 16)]);
}
