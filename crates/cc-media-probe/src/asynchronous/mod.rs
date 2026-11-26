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

//! The async surface (`api.md` section 9): the trait of a caller's own async source with its two functions, which
//! need no feature, and one module for each runtime's I/O traits, `tokio` and `futures`, behind their features. No
//! function is generic over either runtime, and enabling both features changes neither module.
//!
//! # Cancellation
//!
//! Dropping a future stops its probe: nothing is polled again, and the library issues no further read. Work that
//! was already handed elsewhere is not recalled:
//!
//! - Tokio runs every file operation on its blocking pool, where an operation that was started runs until it
//!   ends. What becomes of its result depends on who owns the file:
//!   - A file that the dropped future owns is dropped with it: a file that a path function of the Tokio module
//!     opens, its open included, and a `tokio::fs::File` passed by value. The result is discarded, and the file
//!     closes once no operation on it is in flight.
//!   - A `tokio::fs::File` passed as `&mut file` stays open with the caller and keeps the result: its next read
//!     returns the bytes that a read in flight fetched. Its position is unspecified, as after any probe, so seek
//!     it before reading from it again. Tokio's `seek` waits for the operation in flight first.
//! - A caller's own source decides the same for its own work, such as a range request in flight.
//! - The library spawns no task of its own.

mod entry;
#[cfg(feature = "futures-io")]
pub mod futures;
mod handle;
#[cfg(any(feature = "tokio", feature = "futures-io"))]
mod reader_methods;
mod source;
#[cfg(feature = "tokio")]
pub mod tokio;

pub use entry::{detect_source, probe_source};
pub use handle::AsynchronousProber;
pub use source::AsyncPositionalSource;
#[cfg(test)]
pub(crate) use source::tests::ReadyRanges;
