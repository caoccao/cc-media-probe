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

//! The async positional source, `api.md` section 9.3, and the function that probes it.

use std::io;
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::error::Error;
use crate::model::MediaReport;
use crate::prober::Prober;

/// An async source that is neither a file nor a reader, such as ranges of a remote object.
///
/// Within one probe, after a call returns `Poll::Pending`, the probe's next call to the same method repeats the same
/// request: the same `position` and a buffer of the same length. The promise ends with the probe: a source that
/// outlives a dropped probe can still hold its request, and keeps the range of that request to tell it apart.
pub trait AsyncPositionalSource {
  /// Reads at most `buffer.len()` bytes starting at `position`. Ready with 0 only at or past the end.
  fn poll_read_at(
    self: Pin<&mut Self>,
    context: &mut Context<'_>,
    position: u64,
    buffer: &mut [u8],
  ) -> Poll<io::Result<usize>>;

  /// The total length, or `None` when the source does not know it.
  fn poll_length(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<io::Result<Option<u64>>>;
}

impl<Source: AsyncPositionalSource + Unpin + ?Sized> AsyncPositionalSource for &mut Source {
  fn poll_read_at(
    self: Pin<&mut Self>,
    context: &mut Context<'_>,
    position: u64,
    buffer: &mut [u8],
  ) -> Poll<io::Result<usize>> {
    Pin::new(&mut **self.get_mut()).poll_read_at(context, position, buffer)
  }

  fn poll_length(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<io::Result<Option<u64>>> {
    Pin::new(&mut **self.get_mut()).poll_length(context)
  }
}

impl<Source: AsyncPositionalSource + ?Sized> AsyncPositionalSource for Pin<Box<Source>> {
  fn poll_read_at(
    self: Pin<&mut Self>,
    context: &mut Context<'_>,
    position: u64,
    buffer: &mut [u8],
  ) -> Poll<io::Result<usize>> {
    self.get_mut().as_mut().poll_read_at(context, position, buffer)
  }

  fn poll_length(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<io::Result<Option<u64>>> {
    self.get_mut().as_mut().poll_length(context)
  }
}

/// Probes the caller's own async positional source with the default options.
pub async fn probe_source<Source: AsyncPositionalSource + Unpin>(source: Source) -> Result<MediaReport, Error> {
  Prober::new().asynchronous().probe_source(source).await
}
