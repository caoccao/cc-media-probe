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

//! The readers of futures-io as raw inputs: `AsyncRead + AsyncSeek` for the seekable path and `AsyncRead` alone for
//! the forward-only path (D15, plan section 8.7).

use std::future::poll_fn;
use std::io::{self, SeekFrom};
use std::pin::Pin;

use futures_io::{AsyncRead, AsyncSeek};

use super::{RawPositional, RawSequential};

/// A seekable reader of futures-io.
pub(crate) struct FuturesSeekable<Reader>(pub(crate) Reader);

impl<Reader: AsyncRead + AsyncSeek + Unpin> RawPositional for FuturesSeekable<Reader> {
  async fn read_at(&mut self, position: u64, buffer: &mut [u8]) -> io::Result<usize> {
    poll_fn(|context| Pin::new(&mut self.0).poll_seek(context, SeekFrom::Start(position))).await?;
    poll_fn(|context| Pin::new(&mut self.0).poll_read(context, buffer)).await
  }
}

/// A forward-only reader of futures-io.
pub(crate) struct FuturesForward<Reader>(pub(crate) Reader);

impl<Reader: AsyncRead + Unpin> RawSequential for FuturesForward<Reader> {
  async fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
    poll_fn(|context| Pin::new(&mut self.0).poll_read(context, buffer)).await
  }
}
