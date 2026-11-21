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

//! Names of files and directories with a fixed meaning.

/// The extension of Rust source files.
pub const RUST_EXTENSION: &str = "rs";
/// The root of a library crate.
pub const LIBRARY_ROOT: &str = "lib.rs";
/// The root of a binary crate.
pub const BINARY_ROOT: &str = "main.rs";
/// The root of a directory module.
pub const DIRECTORY_MODULE_ROOT: &str = "mod.rs";
/// The file that holds the tests of a module root (R30).
pub const TESTS_FILE: &str = "tests.rs";
/// The file name of a constants module (R33).
pub const CONSTANTS_FILE: &str = "constants.rs";
/// The directory name of a constants module split by group (R33).
pub const CONSTANTS_DIRECTORY: &str = "constants";
/// The file stem of a constants module.
pub const CONSTANTS_STEM: &str = "constants";
/// The stem of a directory module root.
pub const DIRECTORY_MODULE_STEM: &str = "mod";
/// The stem of a library crate root.
pub const LIBRARY_STEM: &str = "lib";
/// The stem of a binary crate root.
pub const BINARY_STEM: &str = "main";
/// A package manifest.
pub const MANIFEST_FILE: &str = "Cargo.toml";
/// Integration tests of a package.
pub const TESTS_DIRECTORY: &str = "tests";
/// Benchmarks of a package.
pub const BENCHMARKS_DIRECTORY: &str = "benches";
/// Example programs of a package.
pub const EXAMPLES_DIRECTORY: &str = "examples";
/// The source directory of a package.
pub const SOURCE_DIRECTORY: &str = "src";
/// The directory of the workspace's member crates.
pub const CRATES_DIRECTORY: &str = "crates";
/// Build output, never checked.
pub const TARGET_DIRECTORY: &str = "target";
/// Installed JavaScript packages, never checked.
pub const NODE_MODULES_DIRECTORY: &str = "node_modules";
/// Hidden directories start with this character and are never checked.
pub const HIDDEN_PREFIX: char = '.';
/// Separates the components of a relative path in findings.
pub const PATH_SEPARATOR: char = '/';
