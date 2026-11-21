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

// The repository root and its files.

import { statSync } from 'node:fs';
import { join, resolve } from 'node:path';

import { captureCommand } from './command.ts';
import { GIT, GIT_FAILED, LIST_FILES_ARGUMENTS, NUL_CHARACTER, PARENT_DIRECTORY } from './constants.ts';

/** The repository root: two directories above this file. */
export function repositoryRoot(): string {
  return resolve(import.meta.dirname, PARENT_DIRECTORY, PARENT_DIRECTORY);
}

/** Tracked files and new files that Git does not ignore, that exist on disk. Git names them relative to the root
 * with `/` between components on every platform; callers turn a name into a file-system path only with `join`. */
export function repositoryFiles(root: string): string[] {
  const listing = captureCommand(GIT, LIST_FILES_ARGUMENTS, root);
  if (listing === undefined) {
    throw new Error(GIT_FAILED);
  }
  return listing
    .split(NUL_CHARACTER)
    .filter((file) => file.length > 0 && statSync(join(root, file), { throwIfNoEntry: false })?.isFile() === true)
    .sort();
}
