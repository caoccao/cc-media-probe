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

// The offline check of the manifest of non-RFC documents (plan section 5.2): every row follows the rules, every
// draft of Appendix A.7 is fetched at its pinned revision, and every document stored locally matches its recorded
// checksum. The stored documents are git-ignored, so CI checks the manifest alone.

import { existsSync, readdirSync, readFileSync } from 'node:fs';
import { join, relative } from 'node:path';

import { TEXT_ENCODING } from '../../constants.ts';
import { EMPTY, PLAN_FILE } from '../constants.ts';
import { checksum } from '../downloads.ts';
import { FILES_DIRECTORY, MANIFEST_FILE, SPECIFICATIONS_DIRECTORY } from './constants.ts';
import { compareDrafts, pinnedDrafts } from './drafts.ts';
import { type DocumentEntry, readDocuments, storedPath } from './manifest.ts';
import { CHECKSUM_MISMATCH, MISSING_MANIFEST, UNLISTED_FILE, UNRECORDED_FILE } from './messages.ts';

/** The findings of the manifest of non-RFC documents in `root`. */
export function checkDocuments(root: string): string[] {
  const manifestFile = join(...SPECIFICATIONS_DIRECTORY, MANIFEST_FILE);
  if (!existsSync(join(root, manifestFile))) {
    return [`${manifestFile}: ${MISSING_MANIFEST}`];
  }
  const { entries, findings } = readDocuments(manifestFile, readFileSync(join(root, manifestFile), TEXT_ENCODING));
  const plan = readFileSync(join(root, ...PLAN_FILE), TEXT_ENCODING);
  return [...findings, ...compareDrafts(manifestFile, pinnedDrafts(plan), entries), ...checkStored(root, entries)];
}

/** Every stored document matches its checksum, and every stored file, in any folder, belongs to a row. */
function checkStored(root: string, entries: readonly DocumentEntry[]): string[] {
  const directory = join(root, ...SPECIFICATIONS_DIRECTORY, FILES_DIRECTORY);
  if (!existsSync(directory)) {
    return [];
  }
  const findings: string[] = [];
  const present = new Set(
    readdirSync(directory, { recursive: true, withFileTypes: true })
      .filter((file) => file.isFile())
      .map((file) => relative(directory, join(file.parentPath, file.name))),
  );
  for (const entry of entries) {
    const name = join(...storedPath(entry));
    if (!present.delete(name)) {
      continue;
    }
    const label = join(...SPECIFICATIONS_DIRECTORY, FILES_DIRECTORY, name);
    if (entry.checksum === EMPTY) {
      findings.push(`${label}: ${UNRECORDED_FILE}`);
    } else if (checksum(readFileSync(join(directory, name))) !== entry.checksum) {
      findings.push(`${label}: ${CHECKSUM_MISMATCH}`);
    }
  }
  for (const name of present) {
    findings.push(`${join(...SPECIFICATIONS_DIRECTORY, FILES_DIRECTORY, name)}: ${UNLISTED_FILE}`);
  }
  return findings;
}
