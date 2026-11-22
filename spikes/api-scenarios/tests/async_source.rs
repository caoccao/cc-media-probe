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

//! The contract of `api.md` section 9.3: after `Pending`, the same request is repeated.

use std::io;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};

use cc_media_probe::ErrorKind;
use cc_media_probe::asynchronous::{AsyncPositionalSource, probe_source};

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

#[test]
fn a_pending_read_is_asked_again_with_the_same_request() {
  let requests = Arc::new(Mutex::new(Vec::new()));
  let source = HesitantSource {
    requests: Arc::clone(&requests),
    ready: false,
  };
  let error = futures_lite::future::block_on(probe_source(source)).unwrap_err();
  assert_eq!(error.kind(), &ErrorKind::UnsupportedFormat);
  let requests = requests.lock().unwrap();
  assert_eq!(requests.len(), 2);
  assert_eq!(requests[0], requests[1]);
}
