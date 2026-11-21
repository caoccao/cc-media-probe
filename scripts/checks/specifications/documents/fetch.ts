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

// The fetch script of the non-RFC documents (plan Phase 1, task 8): it stores or verifies every document of the
// manifest, or of the identifiers named, records the checksum and date of what it stored, and reports manual
// documents that are still missing.

import { existsSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

import { TEXT_ENCODING } from '../../constants.ts';
import { download, type Downloader, today } from '../downloads.ts';
import { finish, type Report } from '../report.ts';
import { FILES_DIRECTORY, MANIFEST_FILE, SPECIFICATIONS_DIRECTORY } from './constants.ts';
import { type DocumentEntry, formatDocuments, readDocuments } from './manifest.ts';
import { MISSING_MANIFEST, UNKNOWN_IDENTIFIER, UP_TO_DATE } from './messages.ts';
import { storeDocument } from './retrieval.ts';

/** Runs the fetch script in `root` for the identifiers in `only`, or for every row when it is empty; returns the
 * exit status: 0 when nothing needs attention, 1 otherwise. */
export async function fetchDocuments(
  root: string,
  only: readonly string[],
  downloader: Downloader = download,
  date: string = today(),
): Promise<number> {
  const report: Report = { problems: [], notes: [] };
  const manifestFile = join(...SPECIFICATIONS_DIRECTORY, MANIFEST_FILE);
  if (!existsSync(join(root, manifestFile))) {
    report.problems.push(`${manifestFile}: ${MISSING_MANIFEST}`);
    return finish(report, UP_TO_DATE);
  }
  const { entries, findings } = readDocuments(manifestFile, readFileSync(join(root, manifestFile), TEXT_ENCODING));
  const known = new Set(entries.map((entry) => entry.identifier));
  report.problems.push(
    ...findings,
    ...only.filter((identifier) => !known.has(identifier)).map((identifier) => `${identifier}: ${UNKNOWN_IDENTIFIER}`),
  );
  if (report.problems.length > 0) {
    return finish(report, UP_TO_DATE);
  }
  const directory = join(root, ...SPECIFICATIONS_DIRECTORY, FILES_DIRECTORY);
  mkdirSync(directory, { recursive: true });
  const selected = new Set(only.length > 0 ? only : known);
  const recorded: DocumentEntry[] = [];
  for (const entry of entries) {
    const selectedEntry = selected.has(entry.identifier);
    recorded.push(selectedEntry ? await storeDocument(directory, entry, downloader, date, report) : entry);
    writeFileSync(join(root, manifestFile), formatDocuments([...recorded, ...entries.slice(recorded.length)]));
  }
  return finish(report, UP_TO_DATE);
}
