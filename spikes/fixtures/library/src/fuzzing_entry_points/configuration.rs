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

//! Which configuration names the build had, so a test or a target can tell a repository build from a fuzz build.

/// The configuration names of the build.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Configuration {
  /// Whether `cfg(fuzzing)` was set, as cargo-fuzz sets it for every crate it builds.
  pub fuzzing: bool,
  /// Whether the repository marker was set, as the root `.cargo/config.toml` sets it.
  pub repository: bool,
}

/// The configuration names of this build.
#[must_use]
pub fn configuration() -> Configuration {
  Configuration {
    fuzzing: cfg!(fuzzing),
    repository: cfg!(cc_media_probe_repository),
  }
}
