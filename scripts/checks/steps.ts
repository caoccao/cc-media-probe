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
/** Packages both published crates in one command, with their verification builds (Appendix C.2.1, step 3). The
 * support crates are never selected. Uncommitted changes are allowed, so that the check runs before a commit. */
export const JOINT_PACKAGING_ARGUMENTS = [
  'package',
  '--quiet',
  '--locked',
  '--allow-dirty',
  '--package',
  'cc-media-probe',
  '--package',
  'cc-media-probe-cli',
];
/** The Rust structure check. */
export const STRUCTURE_COMMAND = 'structure';
/** The target-ownership check. */
export const TARGETS_COMMAND = 'targets';
/** The type checker's entry point, relative to the repository root, by component; `npm ci` in `scripts` installs it. */
export const TYPE_CHECKER = ['scripts', 'node_modules', 'typescript', 'bin', 'tsc'];
/** The option of the type checker that names its configuration; the configuration's path follows. */
export const TYPE_CHECK_OPTION = '--project';
/** The type checker's configuration, which emits nothing, relative to the repository root, by component. */
export const TYPE_CHECK_CONFIGURATION = ['scripts', 'tsconfig.json'];
/** Measures line coverage over every test of every crate and writes an LCOV report; the report's path follows. */
export const COVERAGE_ARGUMENTS = [
  'llvm-cov',
  '--quiet',
  '--locked',
  '--workspace',
  '--all-features',
  '--lcov',
  '--output-path',
];
/** The LCOV report, relative to the repository root, by component. */
export const COVERAGE_REPORT = ['target', 'coverage', 'lcov.info'];
/** The coverage check of the checks crate; the report's path follows. */
export const COVERAGE_COMMAND = 'coverage';
