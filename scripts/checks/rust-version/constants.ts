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

// Programs, arguments and messages of the tests with the declared `rust-version` (R12, plan section 9.8).

/** Describes the workspace's own packages as JSON, without resolving their dependencies. */
export const METADATA_ARGUMENTS = ['metadata', '--quiet', '--locked', '--no-deps', '--format-version', '1'];
/** The toolchain installer. */
export const RUSTUP = 'rustup';
/** Installs a toolchain; its name follows. */
export const INSTALL_ARGUMENTS = ['toolchain', 'install'];
/** Installs only what building and testing need. */
export const INSTALL_OPTIONS = ['--profile', 'minimal'];
/** Selects the toolchain of one Cargo command through rustup's proxy; the toolchain's name follows. */
export const TOOLCHAIN_PREFIX = '+';
/** Announces the run. */
export const TESTING_WITH = '==> tests with the declared rust-version';
/** `cargo metadata` failed. */
export const METADATA_FAILED = 'cargo metadata failed';
/** The output of `cargo metadata` has no list of packages. */
export const UNREADABLE_METADATA = 'cargo metadata printed no list of packages';
/** A published package is not in the workspace. */
export const MISSING_PACKAGE = 'is not in the workspace';
/** A published package declares no `rust-version`. */
export const NO_DECLARATION = 'declares no rust-version';
/** The published packages disagree. */
export const DIFFERENT_DECLARATIONS = 'the published packages declare different rust-version values';
