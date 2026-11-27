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

//! The internal positional source of D8, which the walker reads, and what lies under it: the raw inputs, the block
//! cache of the seekable path, and the sliding window of the forward-only path (D9) with its pinned detection prefix.

mod asynchronous;
mod blocking;
mod blocks;
mod cache;
mod delivery;
mod fill;
mod futures;
mod holdings;
mod internal;
mod memory;
mod raw;
mod tokio;
mod window;

pub(crate) use asynchronous::AsyncCustom;
pub(crate) use blocking::{BlockingPositional, Forward, Seekable};
pub(crate) use blocks::{Blocks, Step};
pub(crate) use cache::BlockCache;
pub(crate) use delivery::Delivery;
pub(crate) use fill::{Failure, Filled, Progress, fill, fill_now, fill_rest, settle};
pub(crate) use futures::{FuturesForward, FuturesSeekable};
pub(crate) use holdings::Holdings;
pub(crate) use internal::InternalSource;
pub(crate) use memory::{Memory, known_before, read_memory};
pub(crate) use raw::{RawPositional, RawSequential};
pub(crate) use tokio::{TokioForward, TokioSeekable};
pub(crate) use window::{Window, WindowSizes};
