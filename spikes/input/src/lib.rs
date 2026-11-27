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

//! The I/O spike of plan Phase 3, task 1, for gate G3.1: one toy container walker, written once as `async` code over
//! an internal positional source (D8), driven from bytes, a seekable reader, a forward-only reader (D9), a custom
//! positional source, a file, a custom async source, and the seekable and forward-only readers of Tokio and of
//! futures-io (D15). Its header assembler and its toy codec follow the parsing contract (D17), and its companions
//! go through a blocking or an async resolver. `handwritten` is the same probe written as blocking code, the
//! baseline of the overhead measurement, and `machine` the same walk as D8's fallback, an explicit state machine
//! driven by a blocking loop or an async one. See `README.md`.

#![forbid(unsafe_code)]

mod assembler;
pub mod asynchronous;
pub mod blocking;
mod budget;
pub mod builder;
pub mod codec;
pub mod companion;
mod detection;
mod executor;
pub mod handwritten;
pub mod machine;
mod options;
pub mod report;
mod source;
pub mod toy;
mod utilities;
mod walker;

pub use budget::Usage;
pub use options::Options;
