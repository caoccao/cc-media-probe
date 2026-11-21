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

// Tests of the comparison with the RFC Editor's metadata and errata.

import assert from 'node:assert/strict';
import { test } from 'node:test';

import type { ErratumRow } from './errata.ts';
import { textAddress } from './manifest.ts';
import { compareErrata, compareMetadata, parseErrata, parseMetadata, templateRow } from './refresh.ts';

const ENTRY = {
  number: 8794,
  title: 'Extensible Binary Meta Language',
  status: 'Proposed Standard',
  address: textAddress(8794),
  checksum: '',
  retrieved: '',
  role: 'EBML',
};

function row(erratum: number, status = 'verified', type = 'technical', section = '17'): ErratumRow {
  return { rfc: 8794, erratum, status, type, section, relevance: 'parsing', components: 'EBML', summary: 'Summary.' };
}

test('reads the RFC Editor metadata', () => {
  const document = {
    doc_id: 'RFC8794',
    title: 'Extensible Binary Meta Language',
    status: 'PROPOSED STANDARD',
    obsoleted_by: [],
    updated_by: ['RFC9559', 'not an RFC'],
  };
  assert.deepEqual(parseMetadata(document), {
    number: 8794,
    title: 'Extensible Binary Meta Language',
    status: 'PROPOSED STANDARD',
    obsoletedBy: [],
    updatedBy: [9559],
  });
  assert.equal(parseMetadata({ doc_id: 'BCP78' }), undefined);
  assert.equal(parseMetadata('text'), undefined);
  assert.equal(parseMetadata(null), undefined);
});

test('reports changed metadata and notes unlisted updates', () => {
  const current = { number: 8794, title: ENTRY.title, status: 'PROPOSED STANDARD', obsoletedBy: [], updatedBy: [9559] };
  assert.deepEqual(compareMetadata(ENTRY, current, new Set([8794, 9559])), { problems: [], notes: [] });
  const changed = { ...current, title: 'New title', status: 'INTERNET STANDARD', obsoletedBy: [9999] };
  assert.deepEqual(compareMetadata(ENTRY, changed, new Set([8794])), {
    problems: [
      'RFC 8794: the RFC Editor now gives the title: New title',
      'RFC 8794: the RFC Editor now gives the status: INTERNET STANDARD',
      'RFC 8794 is obsoleted by RFC 9999',
    ],
    notes: ['RFC 8794 is updated by an RFC that Appendix A does not list: RFC 9559'],
  });
});

/** A record of the RFC Editor's errata list for RFC 8794. */
function record(erratum: string, status: string, type: string, section: string): object {
  return { errata_id: erratum, 'doc-id': 'RFC8794', errata_status_code: status, errata_type_code: type, section };
}

test('reads the errata list and joins sections onto one line', () => {
  const records = parseErrata([
    record('7189', 'Verified', 'Technical', '17'),
    record('1', 'Reported', 'Editorial', 'A\n  B'),
    { errata_id: 'x', 'doc-id': 'RFC8794' },
    { errata_id: '2', 'doc-id': 'BCP14' },
    'not a record',
  ]);
  assert.deepEqual(records, [
    { rfc: 8794, erratum: 7189, status: 'Verified', type: 'Technical', section: '17' },
    { rfc: 8794, erratum: 1, status: 'Reported', type: 'Editorial', section: 'A B' },
  ]);
  assert.deepEqual(parseErrata({}), []);
});

test('compares the errata table with the list', () => {
  const listed = new Set([8794]);
  const verified = { rfc: 8794, erratum: 7189, status: 'Verified', type: 'Technical', section: '17' };
  const held = { rfc: 8794, erratum: 7190, status: 'Held for Document Update', type: 'Editorial', section: '5' };
  const reported = { rfc: 8794, erratum: 7191, status: 'Reported', type: 'Technical', section: '5' };
  const elsewhere = { rfc: 1, erratum: 7192, status: 'Verified', type: 'Technical', section: '5' };
  const records = [verified, held, reported, elsewhere];
  assert.deepEqual(compareErrata(listed, [row(7189)], records), {
    problems: [],
    notes: ['RFC 8794, erratum 7190: erratum held for document update, not in the table; review it if it is new'],
  });
  assert.deepEqual(compareErrata(listed, [row(7190, 'verified'), row(7193)], records).problems, [
    'RFC 8794, erratum 7189: verified erratum missing from the errata table; classify it and add the row: ' +
      '8794\t7189\tverified\ttechnical\t17\t\t\t',
    'RFC 8794, erratum 7190: the RFC Editor now gives other values; the table has verified, technical, 17: ' +
      'held, editorial, 5',
    'RFC 8794, erratum 7193: the RFC Editor lists no such erratum for this RFC',
  ]);
});

test('writes a row to fill in for a missing erratum', () => {
  const record = { rfc: 8794, erratum: 7189, status: 'Verified', type: 'Editorial', section: '17' };
  assert.equal(templateRow(record), '8794\t7189\tverified\teditorial\t17\t\t\t');
});
