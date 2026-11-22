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

//! Scenario 7, a custom async source: an object in a remote store, read by ranges. Needs no feature; any executor
//! can poll the probe.

use std::future::Future;
use std::io;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll, ready};

use cc_media_probe::asynchronous::{AsyncPositionalSource, probe_source};

/// A stand-in for a store's client, whose `fetch` resolves to the bytes of one range.
#[derive(Clone)]
struct Store {
  object: Arc<Vec<u8>>,
}

impl Store {
  fn fetch(&self, position: u64, length: usize) -> impl Future<Output = io::Result<Vec<u8>>> + Send + 'static {
    let object = Arc::clone(&self.object);
    async move {
      let start = usize::try_from(position).unwrap_or(usize::MAX).min(object.len());
      let end = start.saturating_add(length).min(object.len());
      Ok(object[start..end].to_vec())
    }
  }
}

/// One range request in flight.
type RangeRequest = Pin<Box<dyn Future<Output = io::Result<Vec<u8>>> + Send>>;

/// One range request in flight, with the range it asks for.
struct PendingRange {
  position: u64,
  length: usize,
  request: RangeRequest,
}

/// The source: one range request in flight at a time.
struct RemoteObject {
  store: Store,
  length: u64,
  pending: Option<PendingRange>,
}

impl AsyncPositionalSource for RemoteObject {
  fn poll_read_at(
    self: Pin<&mut Self>,
    context: &mut Context<'_>,
    position: u64,
    buffer: &mut [u8],
  ) -> Poll<io::Result<usize>> {
    let this = self.get_mut();
    // Within one probe the library repeats a request after `Pending`. A dropped probe can leave a request for
    // another range behind, so the request in flight is kept only while it asks for this range.
    if this
      .pending
      .as_ref()
      .is_some_and(|pending| (pending.position, pending.length) != (position, buffer.len()))
    {
      this.pending = None;
    }
    let pending = this.pending.get_or_insert_with(|| PendingRange {
      position,
      length: buffer.len(),
      request: Box::pin(this.store.fetch(position, buffer.len())),
    });
    let bytes = ready!(pending.request.as_mut().poll(context));
    this.pending = None;
    let bytes = bytes?;
    let count = bytes.len().min(buffer.len());
    buffer[..count].copy_from_slice(&bytes[..count]);
    Poll::Ready(Ok(count))
  }

  fn poll_length(self: Pin<&mut Self>, _context: &mut Context<'_>) -> Poll<io::Result<Option<u64>>> {
    // The store gave the length when the object was opened.
    Poll::Ready(Ok(Some(self.length)))
  }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  // A file that cannot be read is reported as itself, never as a format the probe could not recognize.
  let object = Arc::new(std::fs::read("movie.mkv")?);
  let source = RemoteObject {
    length: object.len() as u64,
    store: Store { object },
    pending: None,
  };
  let report = futures_lite::future::block_on(probe_source(source))?;
  println!("{}: read {} bytes", report.format(), report.usage().bytes_read());
  Ok(())
}
