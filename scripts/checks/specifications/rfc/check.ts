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

// The offline check of the RFC set (plan section 5.2): the manifest equals what Appendix A implies, every listed RFC
// is stored with its recorded checksum and nothing else is, the index is current, and the errata table is well
// formed. It needs no network, so `node scripts/check.ts` and CI run it.

import { existsSync, readFileSync } from 'node:fs';
import { join } from 'node:path';

import { TEXT_ENCODING } from '../../constants.ts';
import { EMPTY, PLAN_FILE } from '../constants.ts';
import { checksum } from '../downloads.ts';
import { appendixRfcs } from './appendix.ts';
import { ERRATA_FILE, MANIFEST_FILE, README_FILE, RFC_DIRECTORY, RFC_FILE_PATTERN, RFC_LABEL } from './constants.ts';
import { readErrata } from './errata.ts';
import { compareWithAppendix, readManifest, type RfcEntry, rfcFileName } from './manifest.ts';
import {
  CHECKSUM_MISMATCH,
  MISSING_ERRATA,
  MISSING_MANIFEST,
  MISSING_TEXT,
  STALE_README,
  UNLISTED_TEXT,
  UNRECORDED_TEXT,
} from './messages.ts';
import { renderReadme } from './readme.ts';

/** The findings of the RFC set in `root`, whose files Git lists as `files`. */
export function checkRfcs(root: string, files: readonly string[]): string[] {
  const planFile = join(...PLAN_FILE);
  const appendix = appendixRfcs(planFile, readFileSync(join(root, planFile), TEXT_ENCODING));
  const manifestFile = join(...RFC_DIRECTORY, MANIFEST_FILE);
  if (!existsSync(join(root, manifestFile))) {
    return [...appendix.findings, `${manifestFile}: ${MISSING_MANIFEST}`];
  }
  const manifest = readManifest(manifestFile, readFileSync(join(root, manifestFile), TEXT_ENCODING));
  const findings = [
    ...appendix.findings,
    ...manifest.findings,
    ...compareWithAppendix(manifestFile, manifest.entries, appendix.rfcs),
    ...checkTexts(root, files, manifest.entries),
  ];
  const readmeFile = join(...RFC_DIRECTORY, README_FILE);
  const readmePath = join(root, readmeFile);
  if (!existsSync(readmePath) || readFileSync(readmePath, TEXT_ENCODING) !== renderReadme(manifest.entries)) {
    findings.push(`${readmeFile}: ${STALE_README}`);
  }
  const errataFile = join(...RFC_DIRECTORY, ERRATA_FILE);
  if (existsSync(join(root, errataFile))) {
    const listed = new Set(manifest.entries.map((entry) => entry.number));
    findings.push(...readErrata(errataFile, readFileSync(join(root, errataFile), TEXT_ENCODING), listed).findings);
  } else {
    findings.push(`${errataFile}: ${MISSING_ERRATA}`);
  }
  return findings;
}

/** Every listed RFC is present with its recorded checksum, and no other RFC text is present. */
function checkTexts(root: string, files: readonly string[], entries: readonly RfcEntry[]): string[] {
  const present = new Set(
    files.flatMap((file) => {
      const match = RFC_FILE_PATTERN.exec(file);
      return match === null ? [] : [Number(match[1])];
    }),
  );
  const findings: string[] = [];
  for (const entry of entries) {
    const label = join(...RFC_DIRECTORY, rfcFileName(entry.number));
    if (!present.has(entry.number)) {
      findings.push(`${label}: ${MISSING_TEXT}`);
    } else if (entry.checksum === EMPTY) {
      findings.push(`${label}: ${UNRECORDED_TEXT}`);
    } else if (checksum(readFileSync(join(root, label))) !== entry.checksum) {
      findings.push(`${label}: ${CHECKSUM_MISMATCH}`);
    }
    present.delete(entry.number);
  }
  for (const number of present) {
    findings.push(`${RFC_LABEL}${number}: ${UNLISTED_TEXT}`);
  }
  return findings;
}
