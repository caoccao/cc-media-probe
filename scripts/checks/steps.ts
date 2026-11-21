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

// The commands of the check runner's steps.

/** Runs the scripts' own tests with the built-in test runner; the test files follow. */
export const SCRIPT_TESTS_ARGUMENTS = ['--test'];
/** Checks the formatting of every crate. */
export const FORMATTING_ARGUMENTS = ['fmt', '--all', '--check'];
/** Lints every target of every crate with every feature. */
export const LINTS_ARGUMENTS = ['clippy', '--quiet', '--locked', '--workspace', '--all-targets', '--all-features'];
/** Runs every test of every crate. */
export const TESTS_ARGUMENTS = ['test', '--quiet', '--locked', '--workspace', '--all-features'];
/** Builds the documentation of every crate. */
export const DOCUMENTATION_ARGUMENTS = ['doc', '--quiet', '--locked', '--workspace', '--no-deps', '--all-features'];
/** The Rust structure check. */
export const STRUCTURE_COMMAND = 'structure';
/** The target-ownership check. */
export const TARGETS_COMMAND = 'targets';
