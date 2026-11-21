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

// Every repository check that applies to what exists (plan section 1.2), in order. The run stops at the first step
// that fails. Warnings are denied by the workspace lint table, never through RUSTFLAGS, which would drop the
// repository marker.

import { join } from 'node:path';

import { runCommand } from './command.ts';
import { CARGO, CHECKS_ARGUMENTS, TEST_FILE_SUFFIX } from './constants.ts';
import { checkHygiene } from './hygiene/hygiene.ts';
import { checkMarkdownLinks } from './markdown/links.ts';
import {
  ALL_CHECKS_PASSED,
  DOCUMENTATION_STEP,
  FAILED_PREFIX,
  FORMATTING_STEP,
  HYGIENE_STEP,
  LINTS_STEP,
  MARKDOWN_LINKS_STEP,
  NODE_VERSION_STEP,
  PACKAGES_STEP,
  PURE_RUST_STEP,
  RUST_STRUCTURE_STEP,
  RUST_TESTS_STEP,
  SCRIPT_STRUCTURE_STEP,
  SCRIPT_TESTS_STEP,
  STEP_PREFIX,
  TARGETS_STEP,
} from './messages.ts';
import { checkNodeVersion } from './node-version.ts';
import { checkPackages } from './packages/packages.ts';
import { verifyPureRust } from './pure-rust/pure-rust.ts';
import { repositoryFiles } from './repository-files.ts';
import { checkScriptStructure } from './script-structure/structure.ts';
import {
  DOCUMENTATION_ARGUMENTS,
  FORMATTING_ARGUMENTS,
  LINTS_ARGUMENTS,
  SCRIPT_TESTS_ARGUMENTS,
  STRUCTURE_COMMAND,
  TARGETS_COMMAND,
  TESTS_ARGUMENTS,
} from './steps.ts';

/** One check: what it is called and how it runs. */
interface Step {
  readonly description: string;
  readonly run: (root: string) => boolean;
}

/** Prints the findings; returns whether there were none. */
function passes(findings: readonly string[]): boolean {
  for (const finding of findings) {
    console.log(finding);
  }
  return findings.length === 0;
}

/** The scripts' test files, as file-system paths. */
function scriptTests(root: string): string[] {
  return repositoryFiles(root)
    .filter((file) => file.endsWith(TEST_FILE_SUFFIX))
    .map((file) => join(root, file));
}

const STEPS: readonly Step[] = [
  { description: NODE_VERSION_STEP, run: () => passes(checkNodeVersion(process.version, process.release.lts)) },
  { description: HYGIENE_STEP, run: (root) => passes(checkHygiene(root, repositoryFiles(root))) },
  { description: MARKDOWN_LINKS_STEP, run: (root) => passes(checkMarkdownLinks(root, repositoryFiles(root))) },
  { description: SCRIPT_STRUCTURE_STEP, run: (root) => passes(checkScriptStructure(root, repositoryFiles(root))) },
  { description: SCRIPT_TESTS_STEP, run: (root) => runCommand(process.execPath, [...SCRIPT_TESTS_ARGUMENTS, ...scriptTests(root)], root) },
  { description: FORMATTING_STEP, run: (root) => runCommand(CARGO, FORMATTING_ARGUMENTS, root) },
  { description: LINTS_STEP, run: (root) => runCommand(CARGO, LINTS_ARGUMENTS, root) },
  { description: RUST_STRUCTURE_STEP, run: (root) => runCommand(CARGO, [...CHECKS_ARGUMENTS, STRUCTURE_COMMAND], root) },
  { description: RUST_TESTS_STEP, run: (root) => runCommand(CARGO, TESTS_ARGUMENTS, root) },
  { description: DOCUMENTATION_STEP, run: (root) => runCommand(CARGO, DOCUMENTATION_ARGUMENTS, root) },
  { description: TARGETS_STEP, run: (root) => runCommand(CARGO, [...CHECKS_ARGUMENTS, TARGETS_COMMAND], root) },
  { description: PACKAGES_STEP, run: (root) => passes(checkPackages(root)) },
  { description: PURE_RUST_STEP, run: (root) => verifyPureRust(root) },
];

/** Runs every step in `root`; returns the exit status: 0 when all pass, 1 otherwise. */
export function runAllChecks(root: string): number {
  for (const step of STEPS) {
    console.log(`${STEP_PREFIX}${step.description}`);
    if (!step.run(root)) {
      console.error(`${FAILED_PREFIX}${step.description}`);
      return 1;
    }
  }
  console.log(ALL_CHECKS_PASSED);
  return 0;
}
