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

// Tests of storing and verifying one document.

import assert from 'node:assert/strict';
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { test } from 'node:test';

import { checksum, type Download, type DownloadOptions } from '../downloads.ts';
import type { Report } from '../report.ts';
import { ETSI, SNAPSHOT } from './fixtures.test.ts';
import { storeDocument } from './retrieval.ts';

const PDF = Buffer.from('%PDF-1.7 document');

function serving(responses: Map<string, Download>, calls: [string, DownloadOptions | undefined][] = []) {
  return (address: string, options?: DownloadOptions): Promise<Download> => {
    calls.push([address, options]);
    const response = responses.get(address);
    return response === undefined ? Promise.reject(new Error('not found')) : Promise.resolve(response);
  };
}

function empty(): Report {
  return { problems: [], notes: [] };
}

test('stores a new document and records it, then verifies it without downloading', async () => {
  const directory = mkdtempSync(join(tmpdir(), 'documents-'));
  const report = empty();
  const responses = new Map([[ETSI.artifact, { mediaType: 'application/pdf', bytes: PDF }]]);
  const stored = await storeDocument(directory, ETSI, serving(responses), '2026-11-21', report);
  assert.deepEqual(stored, { ...ETSI, checksum: checksum(PDF), retrieved: '2026-11-21' });
  assert.deepEqual(readFileSync(join(directory, 'audio', 'etsi', 'etsi-ts-102366.pdf')), PDF);
  const calls: [string, DownloadOptions | undefined][] = [];
  assert.deepEqual(await storeDocument(directory, stored, serving(responses, calls), '2026-12-01', report), stored);
  assert.deepEqual(calls, []);
  assert.deepEqual(report, empty());
  rmSync(directory, { recursive: true });
});

test('fetches a snapshot from its reference and refuses redirects of archive captures', async () => {
  const directory = mkdtempSync(join(tmpdir(), 'documents-'));
  const capture = 'https://web.archive.org/web/20261026025506id_/https://xiph.org/ogg/doc/framing.html';
  const calls: [string, DownloadOptions | undefined][] = [];
  const responses = new Map([
    [SNAPSHOT.immutableReference, { mediaType: 'text/plain', bytes: Buffer.from('framing') }],
    [capture, { mediaType: 'text/plain', bytes: Buffer.from('framing') }],
  ]);
  const report = empty();
  await storeDocument(directory, SNAPSHOT, serving(responses, calls), '2026-11-21', report);
  const captured = { ...SNAPSHOT, identifier: 'captured', immutableReference: capture };
  await storeDocument(directory, captured, serving(responses, calls), '2026-11-21', report);
  assert.deepEqual(calls, [
    [SNAPSHOT.immutableReference, { redirects: true }],
    [capture, { redirects: false }],
  ]);
  assert.deepEqual(report, empty());
  rmSync(directory, { recursive: true });
});

test('reports a changed document, a wrong media type, a failed download and a missing manual file', async () => {
  const directory = mkdtempSync(join(tmpdir(), 'documents-'));
  const report = empty();
  const recorded = { ...ETSI, checksum: 'a'.repeat(64), retrieved: '2026-11-21' };
  const responses = new Map([[ETSI.artifact, { mediaType: 'application/pdf', bytes: PDF }]]);
  assert.deepEqual(await storeDocument(directory, recorded, serving(responses), '2026-12-01', report), recorded);
  const html = new Map([[ETSI.artifact, { mediaType: 'text/html', bytes: PDF }]]);
  await storeDocument(directory, ETSI, serving(html), '2026-11-21', report);
  await storeDocument(directory, ETSI, serving(new Map()), '2026-11-21', report);
  const manual = { ...ETSI, identifier: 'iso-iec-14496-12', availability: 'paid', retrieval: 'manual', artifact: '' };
  await storeDocument(directory, manual, serving(new Map()), '2026-11-21', report);
  assert.deepEqual(report.problems, [
    'etsi-ts-102366: the downloaded document differs from the recorded checksum; ' +
      'review the change and supersede the row through GS.2',
    'etsi-ts-102366: unexpected media type: text/html',
    'etsi-ts-102366: download failed: Error: not found',
  ]);
  const placeHere = join(directory, 'audio', 'etsi', 'iso-iec-14496-12.pdf');
  assert.deepEqual(report.notes, [`iso-iec-14496-12: place the document here by hand: ${placeHere}`]);
  rmSync(directory, { recursive: true });
});

test('records a placed manual file, and reports changed and unrecorded files', async () => {
  const directory = mkdtempSync(join(tmpdir(), 'documents-'));
  const report = empty();
  const manual = { ...ETSI, availability: 'paid', retrieval: 'manual', artifact: '' };
  mkdirSync(join(directory, 'audio', 'etsi'), { recursive: true });
  writeFileSync(join(directory, 'audio', 'etsi', 'etsi-ts-102366.pdf'), PDF);
  const placed = await storeDocument(directory, manual, serving(new Map()), '2026-11-21', report);
  assert.deepEqual(placed, { ...manual, checksum: checksum(PDF), retrieved: '2026-11-21' });
  await storeDocument(directory, ETSI, serving(new Map()), '2026-11-21', report);
  await storeDocument(directory, { ...placed, checksum: 'b'.repeat(64) }, serving(new Map()), '2026-11-21', report);
  assert.deepEqual(report.problems, [
    'etsi-ts-102366: present but not recorded in the manifest; delete it and run again',
    "etsi-ts-102366: the stored file differs from the manifest's checksum",
  ]);
  const link = { ...ETSI, retrieval: 'link' };
  assert.deepEqual(await storeDocument(directory, link, serving(new Map()), '', report), link);
  rmSync(directory, { recursive: true });
});
