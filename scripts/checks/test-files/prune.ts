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

// Removing generated files that no recipe produces any more, and the folders they leave empty.

import { existsSync, readdirSync, rmdirSync, rmSync } from 'node:fs';
import { join } from 'node:path';

import { PARTIAL_DIRECTORY, PATH_SEPARATOR, STATE_FILE } from './constants.ts';

/** Removes every file below `directory` whose path is not in `wanted`, except the state file, and every unfinished
 * file; returns the paths of the removed files, with `/`, in sorted order. */
export function removeUnusedFiles(directory: string, wanted: ReadonlySet<string>): string[] {
  if (!existsSync(directory)) {
    return [];
  }
  rmSync(join(directory, PARTIAL_DIRECTORY), { recursive: true, force: true });
  const removed: string[] = [];
  const folders: string[][] = [];
  const pending: string[][] = [[]];
  for (let components = pending.pop(); components !== undefined; components = pending.pop()) {
    folders.push(components);
    for (const entry of readdirSync(join(directory, ...components), { withFileTypes: true })) {
      const entryComponents = [...components, entry.name];
      const path = entryComponents.join(PATH_SEPARATOR);
      if (entry.isDirectory()) {
        pending.push(entryComponents);
      } else if (path !== STATE_FILE && !wanted.has(path)) {
        rmSync(join(directory, ...entryComponents), { force: true });
        removed.push(path);
      }
    }
  }
  for (const components of folders.filter((folder) => folder.length > 0).reverse()) {
    const folder = join(directory, ...components);
    if (readdirSync(folder).length === 0) {
      rmdirSync(folder);
    }
  }
  return removed.sort();
}
