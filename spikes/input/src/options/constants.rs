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

//! The default sizes of the spike's buffers, proposals that G3.1 reviews with the measurements.

use std::num::NonZeroUsize;

use crate::utilities::positive;

/// The bytes of one block of the seekable path's cache.
pub const BLOCK_SIZE: NonZeroUsize = positive(65_536);
/// The blocks that the cache holds.
pub const CACHED_BLOCKS: NonZeroUsize = positive(8);
/// The bytes that the forward-only window keeps behind the read position.
pub const WINDOW: NonZeroUsize = positive(1_048_576);
/// The most bytes that detection may read on a forward-only input, all of which stay pinned.
pub const PREFIX: NonZeroUsize = positive(65_536);
/// The bytes of one read of a forward-only input.
pub const CHUNK: NonZeroUsize = positive(65_536);
/// The bytes of one piece of a packet offered to an assembler.
pub const PIECE: NonZeroUsize = positive(4_096);
