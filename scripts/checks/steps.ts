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
/**
 * The duplicate-code check of R36 over the repository's Rust code, with the thresholds of `dupes.toml`. The path is
 * the repository root, given as `.` so that the exclusions of `dupes.toml` match relative paths.
 */
export const DUPLICATES_ARGUMENTS = ['dupes', '--path', '.', 'check'];
/** The target-ownership check. */
export const TARGETS_COMMAND = 'targets';
/** The type checker's entry point, relative to the repository root, by component; `npm ci` in `scripts` installs it. */
export const TYPE_CHECKER = ['scripts', 'node_modules', 'typescript', 'bin', 'tsc'];
/** The option of the type checker that names its configuration; the configuration's path follows. */
export const TYPE_CHECK_OPTION = '--project';
/** The type checker's configuration, which emits nothing, relative to the repository root, by component. */
export const TYPE_CHECK_CONFIGURATION = ['scripts', 'tsconfig.json'];
/** The duplicate-code check of the scripts, by component; `npm ci` in `scripts` installs it (R36). */
export const SCRIPT_DUPLICATES_TOOL = ['scripts', 'node_modules', 'jscpd', 'run-jscpd.js'];
/** The option of the duplicate-code check that names its configuration; the configuration's path follows. */
export const SCRIPT_DUPLICATES_CONFIGURATION_OPTION = '--config';
/** The thresholds of the duplicate-code check of the scripts, relative to the repository root, by component. */
export const SCRIPT_DUPLICATES_CONFIGURATION = ['scripts', 'jscpd.json'];
/** Leaves out the tips that the duplicate-code check prints after its result. */
export const SCRIPT_DUPLICATES_OPTIONS = ['--no-tips'];
/** The directory of the scripts, relative to the repository root, by component. */
export const SCRIPTS_DIRECTORY = ['scripts'];
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
