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

// The RFC fetch script (plan Phase 1, task 7, and section 5.3): it writes the manifest from Appendix A, stores and
// verifies every text, regenerates the index, and compares the set with the RFC Editor's metadata and errata.

import { existsSync, mkdirSync, readdirSync, readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

import { TEXT_ENCODING } from '../../constants.ts';
import { PLAN_FILE } from '../constants.ts';
import { download, type Downloader, today } from '../downloads.ts';
import { finish, merge, type Report } from '../report.ts';
import { appendixRfcs } from './appendix.ts';
import {
  ERRATA_ADDRESS,
  ERRATA_FILE,
  JSON_MEDIA_TYPE,
  MANIFEST_FILE,
  README_FILE,
  RFC_DIRECTORY,
  RFC_LABEL,
  STORED_TEXT_PATTERN,
} from './constants.ts';
import { readErrata } from './errata.ts';
import { formatManifest, manifestFromAppendix, metadataAddress, type RfcEntry, readManifest } from './manifest.ts';
import { DOWNLOAD_FAILED, UNLISTED_TEXT, UNREADABLE_METADATA, UP_TO_DATE, WRONG_MEDIA_TYPE } from './messages.ts';
import { renderReadme } from './readme.ts';
import { compareErrata, compareMetadata, parseErrata, parseMetadata } from './refresh.ts';
import { storeText } from './texts.ts';

/** Runs the fetch script in `root`; returns the exit status: 0 when nothing needs attention, 1 otherwise. */
export async function fetchRfcs(
  root: string,
  downloader: Downloader = download,
  date: string = today(),
): Promise<number> {
  const report: Report = { problems: [], notes: [] };
  const directory = join(root, ...RFC_DIRECTORY);
  const planFile = join(...PLAN_FILE);
  const appendix = appendixRfcs(planFile, readFileSync(join(root, planFile), TEXT_ENCODING));
  const manifestFile = join(...RFC_DIRECTORY, MANIFEST_FILE);
  const existing = existsSync(join(root, manifestFile))
    ? readManifest(manifestFile, readFileSync(join(root, manifestFile), TEXT_ENCODING))
    : { entries: [], findings: [] };
  report.problems.push(...appendix.findings, ...existing.findings);
  if (report.problems.length > 0) {
    return finish(report, UP_TO_DATE);
  }
  mkdirSync(directory, { recursive: true });
  const entries: RfcEntry[] = [];
  for (const entry of manifestFromAppendix(appendix.rfcs, existing.entries)) {
    entries.push(await storeText(directory, entry, downloader, date, report.problems));
  }
  writeFileSync(join(root, manifestFile), formatManifest(entries));
  writeFileSync(join(directory, README_FILE), renderReadme(entries));
  const listed = new Set(entries.map((entry) => entry.number));
  for (const name of readdirSync(directory)) {
    const match = STORED_TEXT_PATTERN.exec(name);
    if (match !== null && !listed.has(Number(match[1]))) {
      report.problems.push(`${join(...RFC_DIRECTORY, name)}: ${UNLISTED_TEXT}`);
    }
  }
  await compareWithEditor(root, entries, downloader, report);
  return finish(report, UP_TO_DATE);
}

/** Adds the comparison with the RFC Editor's metadata and errata to `report`. */
async function compareWithEditor(
  root: string,
  entries: readonly RfcEntry[],
  downloader: Downloader,
  report: Report,
): Promise<void> {
  const listed = new Set(entries.map((entry) => entry.number));
  for (const entry of entries) {
    const label = `${RFC_LABEL}${entry.number}`;
    const metadata = parseMetadata(await downloadJson(label, metadataAddress(entry.number), downloader, report));
    if (metadata?.number === entry.number) {
      merge(report, compareMetadata(entry, metadata, listed));
    } else {
      report.problems.push(`${label}: ${UNREADABLE_METADATA}`);
    }
  }
  const errataFile = join(...RFC_DIRECTORY, ERRATA_FILE);
  const errata = existsSync(join(root, errataFile))
    ? readErrata(errataFile, readFileSync(join(root, errataFile), TEXT_ENCODING), listed)
    : { rows: [], findings: [] };
  report.problems.push(...errata.findings);
  const list = await downloadJson(ERRATA_ADDRESS, ERRATA_ADDRESS, downloader, report);
  if (list !== undefined) {
    merge(report, compareErrata(listed, errata.rows, parseErrata(list)));
  }
}

/** Downloads and parses a JSON document; a failure becomes a problem labeled `label`. */
async function downloadJson(
  label: string,
  address: string,
  downloader: Downloader,
  report: Report,
): Promise<unknown> {
  try {
    const downloaded = await downloader(address);
    if (downloaded.mediaType !== JSON_MEDIA_TYPE) {
      report.problems.push(`${label}: ${WRONG_MEDIA_TYPE}: ${downloaded.mediaType}`);
      return undefined;
    }
    return JSON.parse(downloaded.bytes.toString(TEXT_ENCODING)) as unknown;
  } catch (error) {
    report.problems.push(`${label}: ${DOWNLOAD_FAILED}: ${String(error)}`);
    return undefined;
  }
}

