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

// Tests of the fetch script of the non-RFC documents, with the sites replaced by a table of responses.

import assert from 'node:assert/strict';
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { type TestContext, test } from 'node:test';

import { checksum, type Download } from '../downloads.ts';
import { fetchDocuments } from './fetch.ts';
import { ETSI, SNAPSHOT } from './fixtures.test.ts';
import { formatDocuments, readDocuments } from './manifest.ts';

const PDF = Buffer.from('%PDF-1.7');
const HTML = Buffer.from('<html>framing</html>');

function quiet(context: TestContext): string[] {
  const problems: string[] = [];
  context.mock.method(console, 'log', () => undefined);
  context.mock.method(console, 'error', (problem: string) => problems.push(problem));
  return problems;
}

function repository(): string {
  const root = mkdtempSync(join(tmpdir(), 'documents-fetch-'));
  mkdirSync(join(root, 'docs', 'specs'), { recursive: true });
  writeFileSync(join(root, 'docs', 'specs', 'manifest.tsv'), formatDocuments([ETSI, SNAPSHOT]));
  return root;
}

function sites(calls: string[]): (address: string) => Promise<Download> {
  const responses = new Map([
    [ETSI.artifact, { mediaType: 'application/pdf', bytes: PDF }],
    [SNAPSHOT.immutableReference, { mediaType: 'text/plain', bytes: HTML }],
  ]);
  return (address) => {
    calls.push(address);
    const response = responses.get(address);
    return response === undefined ? Promise.reject(new Error('not found')) : Promise.resolve(response);
  };
}

function manifest(root: string): ReturnType<typeof readDocuments>['entries'] {
  return readDocuments('manifest.tsv', readFileSync(join(root, 'docs', 'specs', 'manifest.tsv'), 'utf8')).entries;
}

test('stores every document and records it in the manifest', async (context) => {
  quiet(context);
  const root = repository();
  const calls: string[] = [];
  assert.equal(await fetchDocuments(root, [], sites(calls), '2026-11-21'), 0);
  assert.deepEqual(calls, [ETSI.artifact, SNAPSHOT.immutableReference]);
  assert.deepEqual(manifest(root).map((entry) => [entry.checksum, entry.retrieved]), [
    [checksum(PDF), '2026-11-21'],
    [checksum(HTML), '2026-11-21'],
  ]);
  const again: string[] = [];
  assert.equal(await fetchDocuments(root, [], sites(again), '2026-12-01'), 0);
  assert.deepEqual(again, []);
  rmSync(root, { recursive: true });
});

test('handles only the identifiers named, and rejects unknown ones', async (context) => {
  const problems = quiet(context);
  const root = repository();
  const calls: string[] = [];
  assert.equal(await fetchDocuments(root, ['xiph-ogg-framing'], sites(calls), '2026-11-21'), 0);
  assert.deepEqual(calls, [SNAPSHOT.immutableReference]);
  assert.deepEqual(manifest(root).map((entry) => entry.checksum === ''), [true, false]);
  assert.equal(await fetchDocuments(root, ['unknown'], sites(calls), '2026-11-21'), 1);
  assert.deepEqual(problems, ['unknown: no manifest row has this identifier']);
  rmSync(root, { recursive: true });
});

test('reports a failed download and keeps the row unrecorded', async (context) => {
  const problems = quiet(context);
  const root = repository();
  const failing = (): Promise<Download> => Promise.reject(new Error('offline'));
  assert.equal(await fetchDocuments(root, ['etsi-ts-102366'], failing, '2026-11-21'), 1);
  assert.deepEqual(problems, ['etsi-ts-102366: download failed: Error: offline']);
  assert.equal(manifest(root)[0]?.checksum, '');
  rmSync(join(root, 'docs', 'specs', 'manifest.tsv'));
  assert.equal(await fetchDocuments(root, [], failing, '2026-11-21'), 1);
  rmSync(root, { recursive: true });
});
