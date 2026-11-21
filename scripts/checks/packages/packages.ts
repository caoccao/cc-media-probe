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

// The published packages contain exactly the expected files (plan section 7.1), and the crates' README and LICENSE
// copies equal the root files.

import { readFileSync } from 'node:fs';
import { join } from 'node:path';

import { captureCommand } from '../command.ts';
import { CARGO, LINE_BREAK_PATTERN, PUBLISHED_PACKAGES, TEXT_ENCODING } from '../constants.ts';
import {
  COPIED_FILES,
  EXPECTED_LIST_DIRECTORY,
  EXPECTED_LIST_EXTENSION,
  EXPECTED_LIST_PREFIX,
  LISTING_FAILED,
  MISSING_FILE,
  NOT_A_COPY,
  PACKAGE_LIST_ARGUMENTS,
  UNEXPECTED_FILE,
} from './constants.ts';

/** The differences between an expected and an actual list of files. */
export function compareFileLists(label: string, expected: readonly string[], actual: readonly string[]): string[] {
  const expectedSet = new Set(expected);
  const actualSet = new Set(actual);
  return [
    ...actual.filter((file) => !expectedSet.has(file)).map((file) => `${label}: ${UNEXPECTED_FILE}: ${file}`),
    ...expected.filter((file) => !actualSet.has(file)).map((file) => `${label}: ${MISSING_FILE}: ${file}`),
  ];
}

/** The findings of the package-contents check. */
export function checkPackages(root: string): string[] {
  const findings: string[] = [];
  for (const packageName of PUBLISHED_PACKAGES) {
    const listing = captureCommand(CARGO, [...PACKAGE_LIST_ARGUMENTS, packageName], root);
    if (listing === undefined) {
      findings.push(`${packageName}: ${LISTING_FAILED}`);
      continue;
    }
    const listFile = join(...EXPECTED_LIST_DIRECTORY, `${EXPECTED_LIST_PREFIX}${packageName}${EXPECTED_LIST_EXTENSION}`);
    const expected = lines(readFileSync(join(root, listFile), TEXT_ENCODING));
    findings.push(...compareFileLists(listFile, expected, lines(listing)));
  }
  for (const { copy, original } of COPIED_FILES) {
    const copyPath = join(...copy);
    const originalPath = join(...original);
    if (!readFileSync(join(root, copyPath)).equals(readFileSync(join(root, originalPath)))) {
      findings.push(`${copyPath}: ${NOT_A_COPY} ${originalPath}`);
    }
  }
  return findings;
}

function lines(text: string): string[] {
  return text.split(LINE_BREAK_PATTERN).filter((line) => line.length > 0);
}
