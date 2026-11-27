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

//! The fuzz target of the full public probe: each input, a file-name hint and the bytes, goes through
//! `fuzzing_entry_points::probe` to the library's own `probe_bytes`, as a caller's would. The crate root only
//! declares, as R30 asks of a binary's root, so the target's macro sits in `target.rs`.

#![no_main]

mod constants;
mod target;
