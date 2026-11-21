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

//! Commands, files and messages of the target-ownership check.

/// The variable through which Cargo names its own executable.
pub const CARGO_VARIABLE: &str = "CARGO";
/// The executable used when the variable is not set.
pub const CARGO_EXECUTABLE: &str = "cargo";
/// The arguments of `cargo metadata` for the workspace's own packages.
pub const METADATA_ARGUMENTS: &[&str] = &["metadata", "--format-version", "1", "--no-deps", "--offline"];
/// The argument before the path of a manifest.
pub const MANIFEST_PATH_ARGUMENT: &str = "--manifest-path";
/// The expected targets, relative to the repository root, by component.
pub const EXPECTED_TARGETS_FILE: &[&str] = &["crates", "cc-media-probe-checks", "data", "targets.tsv"];
/// The directories whose packages are separate workspaces.
pub const EXCLUDED_DIRECTORIES: &[&str] = &["spikes", "readers"];
/// The name of the probe package created in each excluded directory.
pub const PROBE_PACKAGE: &str = "exclusion-probe";
/// The manifest of the probe package: its own workspace.
pub const PROBE_MANIFEST: &str =
  "[package]\nname = \"exclusion-probe\"\nversion = \"0.0.0\"\nedition = \"2024\"\npublish = false\n\n[workspace]\n";
/// The library root of the probe package.
pub const PROBE_LIBRARY: &str = "//! Probe.\n";
/// Files copied from the root into the temporary workspace.
pub const ROOT_FILES: &[&str] = &["Cargo.toml", "Cargo.lock", "rust-toolchain.toml"];
/// The directory of the temporary workspace inside the system's.
pub const TEMPORARY_PREFIX: &str = "cc-media-probe-exclusion-";
/// A target that is not expected.
pub const UNEXPECTED_TARGET: &str = "target not listed in the expected targets";
/// An expected target that does not exist.
pub const MISSING_TARGET: &str = "expected target is missing";
/// A line of the expected targets that cannot be read.
pub const MALFORMED_EXPECTATION: &str = "expected three tab-separated columns: package, kind, name";
/// `cargo metadata` failed.
pub const METADATA_FAILED: &str = "cargo metadata failed";
/// A probe package became a member of the root workspace.
pub const PROBE_IS_MEMBER: &str = "a package below an excluded directory joined the root workspace";
/// The number of columns of the expected targets.
pub const EXPECTED_COLUMNS: usize = 3;
