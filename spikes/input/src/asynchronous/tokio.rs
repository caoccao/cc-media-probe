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

//! The entry points for Tokio's readers.

use ::tokio::io::{AsyncRead, AsyncSeek};

use super::lookup;
use crate::options::Options;
use crate::report::{ProbeError, ToyReport};
use crate::source::{BlockCache, TokioForward, TokioSeekable, Window};
use crate::walker::{Primary, probe};

/// Probes a seekable Tokio reader from its byte 0, through the block cache.
///
/// # Errors
///
/// `SourceFailed` when the reader or a companion fails.
pub async fn probe_reader<Reader: AsyncRead + AsyncSeek + Unpin>(
  reader: Reader,
  options: &Options,
) -> Result<ToyReport, ProbeError> {
  let source = BlockCache::new(
    TokioSeekable(reader),
    options.block_size.get(),
    options.cached_blocks.get(),
  );
  probe(source, lookup(options), options, Primary::default()).await
}

/// Probes a forward-only Tokio reader from where it stands, through the sliding window.
///
/// # Errors
///
/// `SourceFailed` when the reader or a companion fails.
pub async fn probe_stream<Reader: AsyncRead + Unpin>(
  reader: Reader,
  options: &Options,
) -> Result<ToyReport, ProbeError> {
  let source = Window::new(TokioForward(reader), options.window_sizes());
  probe(source, lookup(options), options, Primary::default()).await
}
