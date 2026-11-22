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

//! Names, files and messages of the pure-Rust verification.

/// The packages whose dependency graph must be pure Rust (R15).
pub const PUBLISHED_PACKAGES: &[&str] = &["cc-media-probe", "cc-media-probe-cli"];
/// Build helpers that compile or locate native code.
pub const NATIVE_HELPERS: &[&str] = &[
  "autotools",
  "bindgen",
  "cc",
  "cmake",
  "cxx-build",
  "gcc",
  "metadeps",
  "nasm-rs",
  "pkg-config",
  "system-deps",
  "vcpkg",
];
/// The arguments of `cargo metadata` for the full graph.
pub const FULL_METADATA_ARGUMENTS: &[&str] = &["metadata", "--format-version", "1", "--all-features", "--locked"];
/// The reviewed build scripts, relative to the repository root, by component.
pub const REVIEWED_SCRIPTS_FILE: &[&str] = &["crates", "cc-media-probe-checks", "data", "build-scripts.tsv"];
/// The allowed system libraries, relative to the repository root, by component.
pub const SYSTEM_LIBRARIES_FILE: &[&str] = &["crates", "cc-media-probe-checks", "data", "system-libraries.tsv"];
/// The platform name for libraries that a build script may name.
pub const BUILD_SCRIPT_PLATFORM: &str = "build-script";
/// Marks an allow-list entry that matches by prefix.
pub const PREFIX_WILDCARD: char = '*';
/// The target kind of a build script.
pub const BUILD_SCRIPT_KIND: &str = "custom-build";
/// The dependency kind of a build dependency.
pub const BUILD_DEPENDENCY_KIND: &str = "build";
/// Metadata members.
pub const IDENTIFIER_MEMBER: &str = "id";
/// The member with a package version.
pub const VERSION_MEMBER: &str = "version";
/// The member with a package's `links` key.
pub const LINKS_MEMBER: &str = "links";
/// The member with the resolved graph.
pub const RESOLVE_MEMBER: &str = "resolve";
/// The member with the nodes of the resolved graph.
pub const NODES_MEMBER: &str = "nodes";
/// The member with a node's dependencies.
pub const DEPENDENCIES_MEMBER: &str = "deps";
/// The member with a dependency's package.
pub const PACKAGE_MEMBER: &str = "pkg";
/// The member with a dependency's kinds.
pub const DEPENDENCY_KINDS_MEMBER: &str = "dep_kinds";
/// The file in which Cargo 1.100 stores a build script's output.
pub const RUN_OUTPUT_FILE: &str = "stdout";
/// The directory of a build script's run, which holds that file.
pub const RUN_DIRECTORY: &str = "run";
/// The file of a build script's output in the older layout.
pub const LEGACY_OUTPUT_FILE: &str = "output";
/// A build script's output directory.
pub const OUT_DIRECTORY: &str = "out";
/// The prefixes of build-script instructions.
pub const INSTRUCTION_PREFIXES: &[&str] = &["cargo::", "cargo:"];
/// Instructions that link something.
pub const LINK_INSTRUCTIONS: &[&str] = &[
  "rustc-link-lib",
  "rustc-link-search",
  "rustc-link-arg",
  "rustc-cdylib-link-arg",
  "rustc-flags",
];
/// The instruction that names a library to link.
pub const LINK_LIBRARY_INSTRUCTION: &str = "rustc-link-lib";
/// Separates an instruction from its value, and a link kind from a name.
pub const VALUE_SEPARATOR: char = '=';
/// Separates link modifiers from a link kind.
pub const MODIFIER_SEPARATOR: char = ':';
/// Extensions of native objects and libraries.
pub const NATIVE_EXTENSIONS: &[&str] = &["a", "dll", "dylib", "lib", "o", "obj", "so"];
/// A build script that is not in the reviewed list.
pub const UNREVIEWED_BUILD_SCRIPT: &str = "build script not in the reviewed list (D4)";
/// A package with a `links` key.
pub const DECLARES_LINKS: &str = "package declares a links key (D4)";
/// A build dependency on a native helper.
pub const NATIVE_HELPER_DEPENDENCY: &str = "build dependency on a native-toolchain helper (D4)";
/// A link instruction outside the allow-list.
pub const LINK_INSTRUCTION_FOUND: &str = "build script emits a link instruction (D4)";
/// A native object in a build script's output directory.
pub const NATIVE_OBJECT_FOUND: &str = "native object in a build script's output directory (D4)";
/// A dynamic library outside the allow-list.
pub const LIBRARY_NOT_ALLOWED: &str = "dynamic library outside the allow-list of system libraries (D4)";
/// The columns of a reviewed build script: name, version, runs, emits.
pub const REVIEWED_SCRIPT_COLUMNS: usize = 4;
/// The columns of an allowed library: platform, library.
pub const ALLOWED_LIBRARY_COLUMNS: usize = 2;
/// A line of a list that cannot be read.
pub const MALFORMED_LIST_LINE: &str = "malformed line";
/// Marks a crate with a build script in the summary.
pub const BUILD_SCRIPT_MARK: &str = "build script";
/// Marks a crate's `links` key in the summary.
pub const LINKS_MARK: &str = "links";
