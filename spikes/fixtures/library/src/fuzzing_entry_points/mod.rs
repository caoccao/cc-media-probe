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

//! The hidden access path of the fuzz targets (D19, as `docs/design/test-kit.md` section 6.3 proposes it): compiled
//! under `cfg(fuzzing)`, which cargo-fuzz sets, and in repository builds, so that the seed check runs as an
//! ordinary test (the maintainer's choice of 2026-11-27). A published build has neither name, so the module is
//! absent there, and it is no part of the public API.

mod configuration;
mod constants;
mod encoding;
mod reached;
mod targets;

pub use configuration::{Configuration, configuration};
pub use constants::MAXIMUM_INPUT_LENGTH;
pub use encoding::{EncodingError, encode};
pub use reached::{Parser, Reached};
pub use targets::{Target, detection, probe, prober, run, toy, toy_codec};
