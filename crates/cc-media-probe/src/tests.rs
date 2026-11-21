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

//! Tests that belong to the crate root.

/// The guard of the repository marker (plan section 7.1, option A). It is
/// compiled only when the marker is absent, and fails when the test kit sits
/// beside the crate: then this is a repository build whose compiler flags
/// were replaced, typically by a `RUSTFLAGS` variable.
#[cfg(not(cc_media_probe_repository))]
#[test]
fn repository_builds_carry_the_marker() {
  let test_kit = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../cc-media-probe-testkit/Cargo.toml");
  assert!(
    !test_kit.exists(),
    "the repository marker cc_media_probe_repository is missing; unset RUSTFLAGS, which replaces the flags of .cargo/config.toml"
  );
}
