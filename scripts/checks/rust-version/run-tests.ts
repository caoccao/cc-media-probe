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

// Builds and tests the workspace with exactly the `rust-version` that its published packages declare (R12, plan
// section 9.8), on the toolchain that rustup installs for that release.

import { captureCommand, runCommand } from '../command.ts';
import { CARGO } from '../constants.ts';
import { TESTS_ARGUMENTS } from '../steps.ts';
import {
  INSTALL_ARGUMENTS,
  INSTALL_OPTIONS,
  METADATA_ARGUMENTS,
  METADATA_FAILED,
  RUSTUP,
  TESTING_WITH,
  TOOLCHAIN_PREFIX,
} from './constants.ts';
import { declaredRustVersion } from './declared.ts';

/** Installs the declared release and runs every test with it; returns whether all of it passed. */
export function testDeclaredRustVersion(root: string): boolean {
  const metadata = captureCommand(CARGO, METADATA_ARGUMENTS, root);
  if (metadata === undefined) {
    console.error(METADATA_FAILED);
    return false;
  }
  const { version, findings } = declaredRustVersion(metadata);
  if (version === undefined) {
    for (const finding of findings) {
      console.error(finding);
    }
    return false;
  }
  console.log(`${TESTING_WITH}: ${version}`);
  return (
    runCommand(RUSTUP, [...INSTALL_ARGUMENTS, version, ...INSTALL_OPTIONS], root) &&
    runCommand(CARGO, [`${TOOLCHAIN_PREFIX}${version}`, ...TESTS_ARGUMENTS], root)
  );
}
