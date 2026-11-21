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

// Tests of the offline check of the RFC set.

import assert from 'node:assert/strict';
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { test } from 'node:test';

import { checksum } from '../downloads.ts';
import { checkRfcs } from './check.ts';
import { formatManifest, textAddress } from './manifest.ts';
import { renderReadme } from './readme.ts';

const PLAN = '## Appendix A\n| [3533](https://www.rfc-editor.org/info/rfc3533) | Ogg | Info | Ogg pages | 9 |\n';
const TEXT = 'Request for Comments: 3533\n';
const ENTRY = {
  number: 3533,
  title: 'Ogg',
  status: 'Informational',
  address: textAddress(3533),
  checksum: checksum(Buffer.from(TEXT)),
  retrieved: '2026-11-21',
  role: 'Ogg pages',
};
const ERRATA = 'rfc\terratum\tstatus\ttype\tsection\trelevance\tcomponents\tsummary\n';
const FILES = ['docs/specs/rfc/rfc3533.txt'];
const STALE = 'differs from the index generated from the manifest; run `node scripts/fetch-rfcs.ts`';

/** A repository root with a consistent RFC set of one RFC. */
function repository(): string {
  const root = mkdtempSync(join(tmpdir(), 'rfc-check-'));
  mkdirSync(join(root, 'docs', 'plans'), { recursive: true });
  mkdirSync(join(root, 'docs', 'specs', 'rfc'), { recursive: true });
  writeFileSync(join(root, 'docs', 'plans', 'plan-init-project.md'), PLAN);
  writeFileSync(join(root, 'docs', 'specs', 'rfc', 'rfc3533.txt'), TEXT);
  writeFileSync(join(root, 'docs', 'specs', 'rfc', 'manifest.tsv'), formatManifest([ENTRY]));
  writeFileSync(join(root, 'docs', 'specs', 'rfc', 'README.md'), renderReadme([ENTRY]));
  writeFileSync(join(root, 'docs', 'specs', 'rfc', 'errata.tsv'), ERRATA);
  return root;
}

test('passes a consistent RFC set', () => {
  const root = repository();
  assert.deepEqual(checkRfcs(root, FILES), []);
  rmSync(root, { recursive: true });
});

test('reports a changed text, a missing text, an unlisted text and a stale index', () => {
  const root = repository();
  writeFileSync(join(root, 'docs', 'specs', 'rfc', 'rfc3533.txt'), 'changed');
  writeFileSync(join(root, 'docs', 'specs', 'rfc', 'README.md'), 'stale');
  assert.deepEqual(checkRfcs(root, [...FILES, 'docs/specs/rfc/rfc9559.txt']), [
    `${join('docs', 'specs', 'rfc', 'rfc3533.txt')}: the checksum differs from the manifest`,
    'RFC 9559: RFC text that the manifest does not list',
    `${join('docs', 'specs', 'rfc', 'README.md')}: ${STALE}`,
  ]);
  assert.deepEqual(checkRfcs(root, []).slice(0, 1), [
    `${join('docs', 'specs', 'rfc', 'rfc3533.txt')}: the RFC text is missing; run \`node scripts/fetch-rfcs.ts\``,
  ]);
  rmSync(root, { recursive: true });
});

test('reports a manifest that differs from Appendix A, and missing tables', () => {
  const root = repository();
  const unrecorded = { ...ENTRY, title: 'Old', checksum: '', retrieved: '' };
  writeFileSync(join(root, 'docs', 'specs', 'rfc', 'manifest.tsv'), formatManifest([unrecorded]));
  rmSync(join(root, 'docs', 'specs', 'rfc', 'errata.tsv'));
  assert.deepEqual(checkRfcs(root, FILES), [
    `${join('docs', 'specs', 'rfc', 'manifest.tsv')}: RFC 3533: title differs from the expected value: Ogg`,
    `${join('docs', 'specs', 'rfc', 'rfc3533.txt')}: present but not recorded in the manifest; delete it and run again`,
    `${join('docs', 'specs', 'rfc', 'README.md')}: ${STALE}`,
    `${join('docs', 'specs', 'rfc', 'errata.tsv')}: the errata table is missing`,
  ]);
  rmSync(join(root, 'docs', 'specs', 'rfc', 'manifest.tsv'));
  assert.deepEqual(checkRfcs(root, FILES), [
    `${join('docs', 'specs', 'rfc', 'manifest.tsv')}: the manifest is missing; run \`node scripts/fetch-rfcs.ts\``,
  ]);
  rmSync(root, { recursive: true });
});
