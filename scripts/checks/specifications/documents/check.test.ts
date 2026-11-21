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

// Tests of the offline check of the manifest of non-RFC documents.

import assert from 'node:assert/strict';
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { test } from 'node:test';

import { checksum } from '../downloads.ts';
import { checkDocuments } from './check.ts';
import { ETSI } from './fixtures.test.ts';
import { formatDocuments } from './manifest.ts';

const DRAFT = 'https://www.ietf.org/archive/id/draft-ietf-cellar-codec-21.txt';
const PLAN =
  '### A.7 Internet-Drafts\n' +
  '| [draft-ietf-cellar-codec](https://datatracker.ietf.org/doc/draft-ietf-cellar-codec/) | Codecs | -21 | ' +
  `[archive](${DRAFT}) | In IESG evaluation |\n`;
const CODEC = { ...ETSI, identifier: 'ietf-draft-cellar-codec', artifact: DRAFT, contentType: 'text/plain' };

function repository(manifest: string): string {
  const root = mkdtempSync(join(tmpdir(), 'documents-check-'));
  mkdirSync(join(root, 'docs', 'plans'), { recursive: true });
  mkdirSync(join(root, 'docs', 'specs'));
  writeFileSync(join(root, 'docs', 'plans', 'plan-init-project.md'), PLAN);
  writeFileSync(join(root, 'docs', 'specs', 'manifest.tsv'), manifest);
  return root;
}

test('passes a well-formed manifest without stored files', () => {
  const root = repository(formatDocuments([ETSI, CODEC]));
  assert.deepEqual(checkDocuments(root), []);
  rmSync(root, { recursive: true });
});

test('verifies stored files and reports unlisted ones', () => {
  const recorded = { ...ETSI, checksum: checksum(Buffer.from('pdf')), retrieved: '2026-11-21' };
  const root = repository(formatDocuments([recorded, CODEC]));
  const files = join(root, 'docs', 'specs', 'files');
  mkdirSync(join(files, 'audio', 'etsi'), { recursive: true });
  writeFileSync(join(files, 'audio', 'etsi', 'etsi-ts-102366.pdf'), 'pdf');
  assert.deepEqual(checkDocuments(root), []);
  writeFileSync(join(files, 'audio', 'etsi', 'etsi-ts-102366.pdf'), 'changed');
  writeFileSync(join(files, 'audio', 'etsi', 'ietf-draft-cellar-codec.txt'), 'draft');
  writeFileSync(join(files, 'stray.pdf'), 'stray');
  const label = (name: string): string => join('docs', 'specs', 'files', name);
  assert.deepEqual(checkDocuments(root), [
    `${label(join('audio', 'etsi', 'etsi-ts-102366.pdf'))}: the stored file differs from the manifest's checksum`,
    `${label(join('audio', 'etsi', 'ietf-draft-cellar-codec.txt'))}: present but not recorded in the manifest; ` +
      'delete it and run again',
    `${label('stray.pdf')}: stored file that no manifest row names`,
  ]);
  rmSync(root, { recursive: true });
});

test('reports a missing draft row, a malformed row and a missing manifest', () => {
  const root = repository(formatDocuments([{ ...ETSI, availability: 'cheap' }]));
  const manifest = join('docs', 'specs', 'manifest.tsv');
  assert.deepEqual(checkDocuments(root), [
    `${manifest}:2: cheap is not one of free, paid, none`,
    `${manifest}: no manifest row fetches the pinned revision of this draft: draft-ietf-cellar-codec-21`,
  ]);
  rmSync(join(root, 'docs', 'specs', 'manifest.tsv'));
  assert.deepEqual(checkDocuments(root), [`${manifest}: the manifest is missing`]);
  rmSync(root, { recursive: true });
});
