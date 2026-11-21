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

// Tests of the RFC manifest: reading, formatting, and deriving it from Appendix A.

import assert from 'node:assert/strict';
import { test } from 'node:test';

import type { AppendixRfc } from './appendix.ts';
import {
  compareWithAppendix,
  formatManifest,
  manifestFromAppendix,
  metadataAddress,
  readManifest,
  type RfcEntry,
  rfcFileName,
  textAddress,
} from './manifest.ts';

const HEADER = 'number\ttitle\tstatus\turl\tsha256\tretrieved\trole\n';
const CHECKSUM = 'a'.repeat(64);
const EBML: AppendixRfc = { number: 8794, title: 'EBML', status: 'Proposed Standard', role: 'Matroska' };
const OGG: AppendixRfc = { number: 3533, title: 'Ogg', status: 'Informational', role: 'Ogg pages' };

function entry(rfc: AppendixRfc, checksum = '', retrieved = ''): RfcEntry {
  return { ...rfc, address: textAddress(rfc.number), checksum, retrieved };
}

test('names files and addresses', () => {
  assert.equal(rfcFileName(8794), 'rfc8794.txt');
  assert.equal(textAddress(8794), 'https://www.rfc-editor.org/rfc/rfc8794.txt');
  assert.equal(metadataAddress(8794), 'https://www.rfc-editor.org/rfc/rfc8794.json');
});

test('formats in ascending order and reads back the same entries', () => {
  const entries = [entry(EBML, CHECKSUM, '2026-11-21'), entry(OGG)];
  const text = formatManifest(entries);
  assert.equal(
    text,
    HEADER +
      '3533\tOgg\tInformational\thttps://www.rfc-editor.org/rfc/rfc3533.txt\t\t\tOgg pages\n' +
      `8794\tEBML\tProposed Standard\thttps://www.rfc-editor.org/rfc/rfc8794.txt\t${CHECKSUM}\t2026-11-21\tMatroska\n`,
  );
  assert.deepEqual(readManifest('manifest.tsv', text), {
    entries: [entry(OGG), entry(EBML, CHECKSUM, '2026-11-21')],
    findings: [],
  });
});

test('reports malformed numbers, checksums, dates and order', () => {
  const text =
    HEADER +
    'x\tTitle\tInformational\taddress\t\t\trole\n' +
    '8794\tEBML\tProposed Standard\taddress\tABC\t2026-11-21\trole\n' +
    '3533\tOgg\tInformational\taddress\t\t2026-11-21\trole\n';
  assert.deepEqual(readManifest('manifest.tsv', text).findings, [
    'manifest.tsv:2: not a number: x',
    'manifest.tsv:3: not a SHA-256 checksum in lower-case hexadecimal',
    'manifest.tsv:4: not a date written as YYYY-MM-DD: 2026-11-21',
    'manifest.tsv:4: rows are not in ascending order',
  ]);
});

test('derives the manifest from Appendix A and keeps what was recorded', () => {
  const recorded = [entry(EBML, CHECKSUM, '2026-11-21'), entry({ ...OGG, number: 1 })];
  const derived = manifestFromAppendix([EBML, OGG], recorded);
  assert.deepEqual(derived, [entry(OGG), entry(EBML, CHECKSUM, '2026-11-21')]);
});

test('compares the manifest with Appendix A', () => {
  assert.deepEqual(compareWithAppendix('manifest.tsv', [entry(OGG), entry(EBML)], [EBML, OGG]), []);
  const entries = [entry({ ...OGG, title: 'Old title' }), entry({ ...EBML, number: 9559 })];
  assert.deepEqual(compareWithAppendix('manifest.tsv', entries, [EBML, OGG]), [
    'manifest.tsv: RFC 3533: title differs from the expected value: Ogg',
    'manifest.tsv: RFC 8794: in Appendix A but not in the manifest',
    'manifest.tsv: RFC 9559: in the manifest but not in Appendix A',
  ]);
});
