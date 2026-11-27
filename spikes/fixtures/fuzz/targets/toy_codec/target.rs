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

//! The target. A fuzz build sets `cfg(fuzzing)`, and cargo-fuzz's `RUSTFLAGS` replace the repository marker of the
//! root `.cargo/config.toml`, so the target checks both on every input: an input that ran in another build would
//! fail here, not in the parser.

use libfuzzer_sys::fuzz_target;
use library::fuzzing_entry_points::{configuration, toy_codec};

use crate::constants::NOT_A_FUZZ_BUILD;

fuzz_target!(|input: &[u8]| {
  let built = configuration();
  assert!(built.fuzzing && !built.repository, "{NOT_A_FUZZ_BUILD}: {built:?}");
  let _reached = toy_codec(input);
});
