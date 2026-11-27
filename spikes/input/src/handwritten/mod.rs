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

//! The same probe written by hand as blocking code, the baseline of the overhead measurement of plan Phase 3, task
//! 1: bytes read in place and a seekable reader through the same block bookkeeping, with the same walker state,
//! element decisions, assembler and parser, and no companions. Only the reads differ from the async walker, so the
//! comparison measures what writing the walker once as `async` code costs.

mod entry;
mod source;
mod walk;

pub use entry::{probe_bytes, probe_reader};
pub(crate) use source::{BlockingSource, HandCache, HandMemory};
