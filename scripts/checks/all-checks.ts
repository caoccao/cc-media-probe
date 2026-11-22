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

import { existsSync, mkdirSync } from 'node:fs';
import { dirname, join } from 'node:path';

import { runCommand } from './command.ts';
import { CARGO, CHECKS_ARGUMENTS, TEST_FILE_SUFFIX } from './constants.ts';
import { checkHygiene } from './hygiene/hygiene.ts';
import { checkMarkdownLinks } from './markdown/links.ts';
import {
  ALL_CHECKS_PASSED,
  COVERAGE_STEP,
  DOCUMENTATION_STEP,
  DUPLICATES_STEP,
  FAILED_PREFIX,
  FORMATTING_STEP,
  HYGIENE_STEP,
  JOINT_PACKAGING_STEP,
  LINTS_STEP,
  MARKDOWN_LINKS_STEP,
  NODE_VERSION_STEP,
  NPM_TOOL_MISSING,
  PACKAGES_STEP,
  PURE_RUST_STEP,
  RUST_STRUCTURE_STEP,
  RUST_TESTS_STEP,
  SCRIPT_DUPLICATES_STEP,
  SCRIPT_STRUCTURE_STEP,
  SCRIPT_TESTS_STEP,
  SCRIPT_TYPES_STEP,
  SPECIFICATIONS_STEP,
  STEP_PREFIX,
  TARGETS_STEP,
} from './messages.ts';
import { checkNodeVersion } from './node-version.ts';
import { checkPackages } from './packages/packages.ts';
import { verifyPureRust } from './pure-rust/pure-rust.ts';
import { repositoryFiles } from './repository-files.ts';
import { checkScriptStructure } from './script-structure/structure.ts';
import { checkSpecifications } from './specifications/specifications.ts';
import {
  COVERAGE_ARGUMENTS,
  COVERAGE_COMMAND,
  COVERAGE_REPORT,
  DOCUMENTATION_ARGUMENTS,
  DUPLICATES_ARGUMENTS,
  FORMATTING_ARGUMENTS,
  JOINT_PACKAGING_ARGUMENTS,
  LINTS_ARGUMENTS,
  SCRIPT_DUPLICATES_CONFIGURATION,
  SCRIPT_DUPLICATES_CONFIGURATION_OPTION,
  SCRIPT_DUPLICATES_OPTIONS,
  SCRIPT_DUPLICATES_TOOL,
  SCRIPT_TESTS_ARGUMENTS,
  SCRIPTS_DIRECTORY,
  STRUCTURE_COMMAND,
  TARGETS_COMMAND,
  TESTS_ARGUMENTS,
  TYPE_CHECK_CONFIGURATION,
  TYPE_CHECK_OPTION,
  TYPE_CHECKER,
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

/**
 * Runs a tool that `npm ci` installs in `scripts`, given by the components of its entry point, with Node.js itself, so
 * that it needs no shell (D24).
 */
function runNpmTool(root: string, tool: readonly string[], toolArguments: readonly string[]): boolean {
  const entryPoint = join(root, ...tool);
  if (!existsSync(entryPoint)) {
    console.error(`${entryPoint}: ${NPM_TOOL_MISSING}`);
    return false;
  }
  return runCommand(process.execPath, [entryPoint, ...toolArguments], root);
}

/** Type-checks the scripts (D24). */
function typeCheck(root: string): boolean {
  return runNpmTool(root, TYPE_CHECKER, [TYPE_CHECK_OPTION, join(root, ...TYPE_CHECK_CONFIGURATION)]);
}

/** Looks for duplicate code in the scripts, with the thresholds of their configuration (R36). */
function scriptDuplicates(root: string): boolean {
  const configuration = join(root, ...SCRIPT_DUPLICATES_CONFIGURATION);
  return runNpmTool(root, SCRIPT_DUPLICATES_TOOL, [
    SCRIPT_DUPLICATES_CONFIGURATION_OPTION,
    configuration,
    ...SCRIPT_DUPLICATES_OPTIONS,
    join(root, ...SCRIPTS_DIRECTORY),
  ]);
}

/**
 * Measures coverage and applies the bar of R28 to the report (D23). The report's directory is created first, because
 * the coverage tool writes the file of `--output-path` without creating its directory, which a fresh checkout lacks.
 */
function coverage(root: string): boolean {
  const report = join(root, ...COVERAGE_REPORT);
  mkdirSync(dirname(report), { recursive: true });
  return (
    runCommand(CARGO, [...COVERAGE_ARGUMENTS, report], root) &&
    runCommand(CARGO, [...CHECKS_ARGUMENTS, COVERAGE_COMMAND, report], root)
  );
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
  { description: SPECIFICATIONS_STEP, run: (root) => passes(checkSpecifications(root, repositoryFiles(root))) },
  { description: SCRIPT_STRUCTURE_STEP, run: (root) => passes(checkScriptStructure(root, repositoryFiles(root))) },
  { description: SCRIPT_TYPES_STEP, run: (root) => typeCheck(root) },
  { description: SCRIPT_DUPLICATES_STEP, run: (root) => scriptDuplicates(root) },
  {
    description: SCRIPT_TESTS_STEP,
    run: (root) => runCommand(process.execPath, [...SCRIPT_TESTS_ARGUMENTS, ...scriptTests(root)], root),
  },
  { description: FORMATTING_STEP, run: (root) => runCommand(CARGO, FORMATTING_ARGUMENTS, root) },
  { description: LINTS_STEP, run: (root) => runCommand(CARGO, LINTS_ARGUMENTS, root) },
  {
    description: RUST_STRUCTURE_STEP,
    run: (root) => runCommand(CARGO, [...CHECKS_ARGUMENTS, STRUCTURE_COMMAND], root),
  },
  { description: DUPLICATES_STEP, run: (root) => runCommand(CARGO, DUPLICATES_ARGUMENTS, root) },
  { description: RUST_TESTS_STEP, run: (root) => runCommand(CARGO, TESTS_ARGUMENTS, root) },
  { description: COVERAGE_STEP, run: (root) => coverage(root) },
  { description: DOCUMENTATION_STEP, run: (root) => runCommand(CARGO, DOCUMENTATION_ARGUMENTS, root) },
  { description: TARGETS_STEP, run: (root) => runCommand(CARGO, [...CHECKS_ARGUMENTS, TARGETS_COMMAND], root) },
  { description: PACKAGES_STEP, run: (root) => passes(checkPackages(root)) },
  { description: JOINT_PACKAGING_STEP, run: (root) => runCommand(CARGO, JOINT_PACKAGING_ARGUMENTS, root) },
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
