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

// Tests of storing and verifying one RFC text.

import assert from 'node:assert/strict';
import { existsSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { test } from 'node:test';

import { checksum, type Download } from '../downloads.ts';
import { textAddress } from './manifest.ts';
import { storeText, textProblem } from './texts.ts';

const TEXT = Buffer.from('Network Working Group\nRequest for Comments: 3533\n\nThe Ogg Encapsulation Format\n');
const ENTRY = {
  number: 3533,
  title: 'Ogg',
  status: 'Informational',
  address: textAddress(3533),
  checksum: '',
  retrieved: '',
  role: 'Ogg pages',
};

function serving(download: Download): (address: string) => Promise<Download> {
  return (address) =>
    address === ENTRY.address ? Promise.resolve(download) : Promise.reject(new Error(`no such address: ${address}`));
}

test('refuses another media type, another RFC and other bytes', () => {
  const text = { mediaType: 'text/plain', bytes: TEXT };
  assert.equal(textProblem(ENTRY, text), undefined);
  assert.equal(textProblem(ENTRY, { ...text, mediaType: 'text/html' }), 'unexpected media type: text/html');
  assert.equal(
    textProblem({ ...ENTRY, number: 353 }, text),
    'the downloaded text does not name this RFC near its top',
  );
  assert.equal(
    textProblem({ ...ENTRY, checksum: 'a'.repeat(64) }, text),
    'the checksum differs from the manifest',
  );
});

test('stores a new text and records its checksum and date', async () => {
  const directory = mkdtempSync(join(tmpdir(), 'rfc-texts-'));
  const problems: string[] = [];
  const stored = await storeText(
    directory,
    ENTRY,
    serving({ mediaType: 'text/plain', bytes: TEXT }),
    '2026-11-21',
    problems,
  );
  assert.deepEqual(problems, []);
  assert.deepEqual(stored, { ...ENTRY, checksum: checksum(TEXT), retrieved: '2026-11-21' });
  assert.deepEqual(readFileSync(join(directory, 'rfc3533.txt')), TEXT);
  const again = await storeText(directory, stored, () => Promise.reject(new Error('offline')), '2026-12-01', problems);
  assert.deepEqual(problems, []);
  assert.deepEqual(again, stored);
  rmSync(directory, { recursive: true });
});

test('reports a changed file, an unrecorded file and a failed download', async () => {
  const directory = mkdtempSync(join(tmpdir(), 'rfc-texts-'));
  const problems: string[] = [];
  writeFileSync(join(directory, 'rfc3533.txt'), 'changed');
  const recorded = { ...ENTRY, checksum: checksum(TEXT), retrieved: '2026-11-21' };
  const text = serving({ mediaType: 'text/plain', bytes: TEXT });
  await storeText(directory, recorded, text, '2026-11-21', problems);
  await storeText(directory, ENTRY, text, '2026-11-21', problems);
  rmSync(join(directory, 'rfc3533.txt'));
  const offline = (): Promise<Download> => Promise.reject(new Error('offline'));
  const unchanged = await storeText(directory, ENTRY, offline, '2026-11-21', problems);
  await storeText(directory, ENTRY, serving({ mediaType: 'text/html', bytes: TEXT }), '2026-11-21', problems);
  assert.deepEqual(problems, [
    'RFC 3533: the checksum differs from the manifest',
    'RFC 3533: present but not recorded in the manifest; delete it and run again',
    'RFC 3533: download failed: Error: offline',
    'RFC 3533: unexpected media type: text/html',
  ]);
  assert.deepEqual(unchanged, ENTRY);
  assert.equal(existsSync(join(directory, 'rfc3533.txt')), false);
  rmSync(directory, { recursive: true });
});
