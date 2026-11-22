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

//! Opens a file with `tokio::fs` and hands it to a function bounded by the two traits the library uses, as
//! `asynchronous::tokio::probe_file` would. Built with Tokio's `fs` feature and nothing else.

#![forbid(unsafe_code)]

mod opening;

pub use opening::{accepts_seekable_reader, future_is_send, open_for_probing};
