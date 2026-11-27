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

//! D8's fallback, written for comparison at G3.1 because the async walker missed the bar of 10% once the review's
//! fixes were in (`README.md` section 5): the toy walker as an explicit state machine. The machine never reads; it
//! says which range it needs, takes in what a read delivered, and runs the same plain code between reads as the
//! async walker and the hand-written one. A blocking loop and an async loop drive the one machine, so it serves
//! both modes from one code base, as D8's fallback must. Like the hand-written probe, it has no companions and no
//! forward-only window.

mod asynchronous;
mod blocking;
mod phase;
mod state_machine;

pub use asynchronous::probe_source;
pub use blocking::{probe_bytes, probe_reader};
