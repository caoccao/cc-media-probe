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

// Tests of the errata table.

import assert from 'node:assert/strict';
import { test } from 'node:test';

import { readErrata } from './errata.ts';

const HEADER = 'rfc\terratum\tstatus\ttype\tsection\trelevance\tcomponents\tsummary\n';
const LISTED = new Set([8794, 9639]);

test('reads well-formed rows', () => {
  const text =
    HEADER +
    '8794\t7189\tverified\ttechnical\t17\tparsing\tEBML element IDs\tCorrects the ranges of element IDs.\n' +
    '9639\t8256\tverified\teditorial\t9.1\tnone\t\tFixes a typo.\n';
  const { rows, findings } = readErrata('errata.tsv', text, LISTED);
  assert.deepEqual(findings, []);
  assert.deepEqual(rows[0], {
    rfc: 8794,
    erratum: 7189,
    status: 'verified',
    type: 'technical',
    section: '17',
    relevance: 'parsing',
    components: 'EBML element IDs',
    summary: 'Corrects the ranges of element IDs.',
  });
  assert.equal(rows.length, 2);
});

test('reports every malformed cell', () => {
  const text =
    HEADER +
    'x\t1\tverified\ttechnical\t1\tnone\t\tSummary.\n' +
    '9639\t8256\tverified\ttechnical\t1\tnone\t\tSummary.\n' +
    '8794\t8256\treported\tminor\t1\tsome\tParser\t\n' +
    '1\t2\theld\ttechnical\t1\tparsing\t\tSummary.\n';
  assert.deepEqual(readErrata('errata.tsv', text, LISTED).findings, [
    'errata.tsv:2: not a number',
    'errata.tsv:4: erratum listed twice: 8256',
    'errata.tsv:4: reported is not one of verified, held',
    'errata.tsv:4: minor is not one of technical, editorial',
    'errata.tsv:4: some is not one of parsing, output, none',
    'errata.tsv:4: the summary is empty',
    'errata.tsv:4: rows are not in ascending order',
    'errata.tsv:5: names an RFC that the manifest does not list: 1',
    'errata.tsv:5: components are named exactly when the relevance is not `none`',
    'errata.tsv:5: rows are not in ascending order',
  ]);
});
