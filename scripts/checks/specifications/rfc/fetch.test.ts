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

// Tests of the RFC fetch script, with the RFC Editor replaced by a table of responses.

import assert from 'node:assert/strict';
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { type TestContext, test } from 'node:test';

import { checksum, type Download, type Downloader } from '../downloads.ts';
import { checkRfcs } from './check.ts';
import { fetchRfcs } from './fetch.ts';

const PLAN = '## Appendix A\n| [3533](https://www.rfc-editor.org/info/rfc3533) | Ogg | Info | Ogg pages | 9 |\n';
const TEXT = 'Network Working Group\nRequest for Comments: 3533\n';
const METADATA = {
  doc_id: 'RFC3533',
  title: 'Ogg',
  status: 'INFORMATIONAL',
  obsoleted_by: [],
  updated_by: ['RFC9999'],
};
const ERRATUM = {
  errata_id: '6796',
  'doc-id': 'RFC3533',
  errata_status_code: 'Verified',
  errata_type_code: 'Technical',
  section: '6',
};
const TEXT_ADDRESS = 'https://www.rfc-editor.org/rfc/rfc3533.txt';
const METADATA_ADDRESS = 'https://www.rfc-editor.org/rfc/rfc3533.json';
const ERRATA_ADDRESS = 'https://www.rfc-editor.org/api/v1/errata.json';
const ERRATA_HEADER = 'rfc\terratum\tstatus\ttype\tsection\trelevance\tcomponents\tsummary\n';

/** A downloader that answers from `responses` by address, and the addresses it was asked for. */
function editor(responses: Map<string, Download>): { downloader: Downloader; calls: string[] } {
  const calls: string[] = [];
  const downloader = (address: string): Promise<Download> => {
    calls.push(address);
    const response = responses.get(address);
    return response === undefined ? Promise.reject(new Error('not found')) : Promise.resolve(response);
  };
  return { downloader, calls };
}

function json(value: unknown): Download {
  return { mediaType: 'application/json', bytes: Buffer.from(JSON.stringify(value)) };
}

function responses(errata: readonly object[]): Map<string, Download> {
  return new Map([
    [TEXT_ADDRESS, { mediaType: 'text/plain', bytes: Buffer.from(TEXT) }],
    [METADATA_ADDRESS, json(METADATA)],
    [ERRATA_ADDRESS, json(errata)],
  ]);
}

/** Keeps the script's report out of the test output; returns the problems it reports. */
function quiet(context: TestContext): string[] {
  const problems: string[] = [];
  context.mock.method(console, 'log', () => undefined);
  context.mock.method(console, 'error', (problem: string) => problems.push(problem));
  return problems;
}

function repository(): string {
  const root = mkdtempSync(join(tmpdir(), 'rfc-fetch-'));
  mkdirSync(join(root, 'docs', 'plans'), { recursive: true });
  writeFileSync(join(root, 'docs', 'plans', 'plan-init-project.md'), PLAN);
  return root;
}

test('writes the manifest and index, stores the text, and reports a missing erratum', async (context) => {
  const problems = quiet(context);
  const root = repository();
  const { downloader, calls } = editor(responses([ERRATUM]));
  assert.equal(await fetchRfcs(root, downloader, '2026-11-21'), 1);
  assert.deepEqual(calls, [TEXT_ADDRESS, METADATA_ADDRESS, ERRATA_ADDRESS]);
  assert.deepEqual(problems, [
    'RFC 3533, erratum 6796: verified erratum missing from the errata table; classify it and add the row: ' +
      '3533\t6796\tverified\ttechnical\t6\t\t\t',
  ]);
  assert.equal(readFileSync(join(root, 'docs', 'rfc', 'rfc3533.txt'), 'utf8'), TEXT);
  assert.equal(
    readFileSync(join(root, 'docs', 'rfc', 'manifest.tsv'), 'utf8').split('\n')[1],
    `3533\tOgg\tInformational\t${TEXT_ADDRESS}\t${checksum(Buffer.from(TEXT))}\t2026-11-21\tOgg pages`,
  );
  const row = '3533\t6796\tverified\ttechnical\t6\tparsing\tOgg pages\tFixes a field.\n';
  writeFileSync(join(root, 'docs', 'rfc', 'errata.tsv'), ERRATA_HEADER + row);
  assert.deepEqual(checkRfcs(root, ['docs/rfc/rfc3533.txt']), []);
  rmSync(root, { recursive: true });
});

test('verifies a stored text without downloading it again, and passes a complete table', async (context) => {
  quiet(context);
  const root = repository();
  const first = editor(responses([]));
  assert.equal(await fetchRfcs(root, first.downloader, '2026-11-21'), 0);
  writeFileSync(join(root, 'docs', 'rfc', 'errata.tsv'), ERRATA_HEADER);
  const second = editor(responses([]));
  assert.equal(await fetchRfcs(root, second.downloader, '2026-12-01'), 0);
  assert.deepEqual(second.calls, [METADATA_ADDRESS, ERRATA_ADDRESS]);
  assert.match(readFileSync(join(root, 'docs', 'rfc', 'manifest.tsv'), 'utf8'), /\t2026-11-21\t/u);
  rmSync(root, { recursive: true });
});

test('reports unlisted texts, unreadable metadata and a failed errata download', async (context) => {
  const problems = quiet(context);
  const root = repository();
  mkdirSync(join(root, 'docs', 'rfc'));
  writeFileSync(join(root, 'docs', 'rfc', 'rfc1.txt'), 'old');
  const partial = responses([]);
  partial.set(METADATA_ADDRESS, { mediaType: 'text/html', bytes: Buffer.from('<html>') });
  partial.delete(ERRATA_ADDRESS);
  assert.equal(await fetchRfcs(root, editor(partial).downloader, '2026-11-21'), 1);
  assert.deepEqual(problems, [
    `${join('docs', 'rfc', 'rfc1.txt')}: RFC text that the manifest does not list`,
    'RFC 3533: unexpected media type: text/html',
    'RFC 3533: the RFC Editor metadata could not be read',
    `${ERRATA_ADDRESS}: download failed: Error: not found`,
  ]);
  rmSync(root, { recursive: true });
});

test('stops before downloading when the plan or the manifest is malformed', async (context) => {
  const problems = quiet(context);
  const root = repository();
  mkdirSync(join(root, 'docs', 'rfc'));
  writeFileSync(join(root, 'docs', 'rfc', 'manifest.tsv'), 'wrong header\n');
  const { downloader, calls } = editor(responses([]));
  assert.equal(await fetchRfcs(root, downloader, '2026-11-21'), 1);
  assert.deepEqual(calls, []);
  assert.equal(problems.length, 1);
  rmSync(root, { recursive: true });
});
