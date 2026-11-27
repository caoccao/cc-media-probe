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

//! Tokio's readers as raw inputs: `AsyncRead + AsyncSeek` for the seekable path, read as a seek followed by a read,
//! and `AsyncRead` alone for the forward-only path (D15, plan section 8.7).

use std::future::poll_fn;
use std::io::{self, SeekFrom};
use std::pin::Pin;
use std::task::Poll;

use ::tokio::io::{AsyncRead, AsyncSeek, ReadBuf};

use super::{RawPositional, RawSequential};

/// A seekable Tokio reader.
pub(crate) struct TokioSeekable<Reader>(pub(crate) Reader);

impl<Reader: AsyncRead + AsyncSeek + Unpin> RawPositional for TokioSeekable<Reader> {
  async fn read_at(&mut self, position: u64, buffer: &mut [u8]) -> io::Result<usize> {
    seek(&mut self.0, SeekFrom::Start(position)).await?;
    read(&mut self.0, buffer).await
  }
}

/// A forward-only Tokio reader.
pub(crate) struct TokioForward<Reader>(pub(crate) Reader);

impl<Reader: AsyncRead + Unpin> RawSequential for TokioForward<Reader> {
  async fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
    read(&mut self.0, buffer).await
  }
}

/// Seeks as Tokio's own `seek` does: an operation still in flight completes first, then the seek starts and
/// completes. So a `tokio::fs::File` kept by its caller after a dropped probe is never asked to seek while busy.
async fn seek<Reader: AsyncSeek + Unpin>(reader: &mut Reader, target: SeekFrom) -> io::Result<u64> {
  poll_fn(|context| Pin::new(&mut *reader).poll_complete(context)).await?;
  Pin::new(&mut *reader).start_seek(target)?;
  poll_fn(|context| Pin::new(&mut *reader).poll_complete(context)).await
}

/// Reads into `buffer` where the reader stands.
async fn read<Reader: AsyncRead + Unpin>(reader: &mut Reader, buffer: &mut [u8]) -> io::Result<usize> {
  poll_fn(|context| {
    let mut filled = ReadBuf::new(buffer);
    match Pin::new(&mut *reader).poll_read(context, &mut filled) {
      Poll::Ready(Ok(())) => Poll::Ready(Ok(filled.filled().len())),
      Poll::Ready(Err(error)) => Poll::Ready(Err(error)),
      Poll::Pending => Poll::Pending,
    }
  })
  .await
}
