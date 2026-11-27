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

//! `probe_source` for a caller's own async positional source.

use cc_media_probe::asynchronous::AsyncPositionalSource;

use crate::companion::AsyncLookup;
use crate::options::Options;
use crate::report::{ProbeError, ToyReport};
use crate::source::{AsyncCustom, BlockCache};
use crate::walker::{Primary, probe};

/// Probes the caller's own async positional source from its position 0, through the block cache. Pass
/// `&mut source` to keep it, or `Box::pin(source)` for a source that is not `Unpin`. The future is `Send` exactly
/// when the source is.
///
/// # Errors
///
/// `SourceFailed` when the source or a companion fails.
pub async fn probe_source<Source: AsyncPositionalSource + Unpin>(
  source: Source,
  options: &Options,
) -> Result<ToyReport, ProbeError> {
  let source = BlockCache::new(
    AsyncCustom(source),
    options.block_size.get(),
    options.cached_blocks.get(),
  );
  probe(source, lookup(options), options, Primary::default()).await
}

/// The async lookup of `options`.
pub(crate) fn lookup(options: &Options) -> AsyncLookup {
  AsyncLookup(options.async_companions.clone())
}
