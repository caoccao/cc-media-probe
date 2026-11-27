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

//! `Options`.

use std::num::NonZeroUsize;
use std::sync::Arc;

use cc_media_probe::Limits;

use super::constants::{BLOCK_SIZE, CACHED_BLOCKS, CHUNK, PIECE, PREFIX, WINDOW};
use crate::companion::{AsyncCompanionResolver, CompanionLookup};
use crate::source::WindowSizes;

/// The options of a probe. The sizes are the spike's proposals, each positive by its type, so no size of 0 can stall
/// a read or a packet's pieces.
#[derive(Clone)]
pub struct Options {
  /// The limits, the library's own type.
  pub limits: Limits,
  /// The bytes of one block of the seekable path's cache.
  pub block_size: NonZeroUsize,
  /// The blocks that the cache holds.
  pub cached_blocks: NonZeroUsize,
  /// The bytes that the forward-only window keeps behind the read position.
  pub window: NonZeroUsize,
  /// The most bytes that detection may read on a forward-only input.
  pub prefix: NonZeroUsize,
  /// The bytes of one read of a forward-only input.
  pub chunk: NonZeroUsize,
  /// The bytes of one piece of a packet offered to an assembler.
  pub piece: NonZeroUsize,
  /// Lookup for the blocking entry points.
  pub companions: CompanionLookup,
  /// Lookup for the async entry points: only a caller's own resolver.
  pub async_companions: Option<Arc<dyn AsyncCompanionResolver>>,
}

impl Default for Options {
  fn default() -> Options {
    Options {
      limits: Limits::default(),
      block_size: BLOCK_SIZE,
      cached_blocks: CACHED_BLOCKS,
      window: WINDOW,
      prefix: PREFIX,
      chunk: CHUNK,
      piece: PIECE,
      companions: CompanionLookup::Off,
      async_companions: None,
    }
  }
}

impl Options {
  /// The sizes of the forward-only window.
  pub(crate) fn window_sizes(&self) -> WindowSizes {
    WindowSizes {
      prefix: self.prefix.get(),
      window: self.window.get(),
      chunk: self.chunk.get(),
    }
  }
}
