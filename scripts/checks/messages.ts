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

// Messages and step names of the check runner.

/** The running Node.js is not a long-term-support release. */
export const NOT_LONG_TERM_SUPPORT =
  'the scripts need a long-term-support release of Node.js (R34); run them with `lts/*`';
/** Marks the start of a step. */
export const STEP_PREFIX = '==> ';
/** Marks a failed step. */
export const FAILED_PREFIX = 'failed: ';
/** Closes a successful run. */
export const ALL_CHECKS_PASSED = '==> all checks passed';
/** The step names, in order. */
export const NODE_VERSION_STEP = 'Node.js version';
/** File hygiene. */
export const HYGIENE_STEP = 'file hygiene';
/** Markdown links. */
export const MARKDOWN_LINKS_STEP = 'Markdown links';
/** The specification tables. */
export const SPECIFICATIONS_STEP = 'specification tables (section 5.2)';
/** Script structure. */
export const SCRIPT_STRUCTURE_STEP = 'script structure (R34)';
/** Script types. */
export const SCRIPT_TYPES_STEP = 'script types (D24)';
/** Script tests. */
export const SCRIPT_TESTS_STEP = 'script tests';
/** Rust formatting. */
export const FORMATTING_STEP = 'formatting';
/** Rust lints. */
export const LINTS_STEP = 'lints';
/** The Rust structure check. */
export const RUST_STRUCTURE_STEP = 'structure (D23)';
/** Rust tests. */
export const RUST_TESTS_STEP = 'tests';
/** Coverage. */
export const COVERAGE_STEP = 'coverage (R28)';
/** Rust documentation. */
export const DOCUMENTATION_STEP = 'documentation';
/** Target ownership. */
export const TARGETS_STEP = 'target ownership';
/** Package contents. */
export const PACKAGES_STEP = 'package contents';
/** Joint packaging. */
export const JOINT_PACKAGING_STEP = 'joint packaging (Appendix C.2.1)';
/** The pure-Rust verification. */
export const PURE_RUST_STEP = 'pure Rust (D4)';
/** The type checker is not installed. */
export const TYPE_CHECKER_MISSING = 'the type checker is missing; run `npm ci` in the `scripts` directory (D24)';
