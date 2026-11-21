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

// Tests of the rules of one manifest row.

import assert from 'node:assert/strict';
import { test } from 'node:test';

import { ETSI, SNAPSHOT } from './fixtures.test.ts';
import { ruleFindings } from './rules.ts';

test('accepts well-formed rows of every retrieval', () => {
  assert.deepEqual(ruleFindings('row', ETSI, undefined), []);
  assert.deepEqual(ruleFindings('row', SNAPSHOT, ETSI), []);
  const paid = { ...ETSI, availability: 'paid', retrieval: 'manual', artifact: '' };
  assert.deepEqual(ruleFindings('row', paid, undefined), []);
  const link = { ...ETSI, retrieval: 'link', artifact: '', contentType: '' };
  assert.deepEqual(ruleFindings('row', link, undefined), []);
  const tree = { ...SNAPSHOT, retrieval: 'tree', immutableReference: 'none', contentType: 'application/json' };
  assert.deepEqual(ruleFindings('row', tree, undefined), []);
  const stored = { ...ETSI, checksum: 'a'.repeat(64), retrieved: '2026-11-21', approved: 'G1.4' };
  assert.deepEqual(ruleFindings('row', stored, undefined), []);
});

test('reports malformed cells and rows that break their retrieval', () => {
  const broken = {
    ...ETSI,
    identifier: 'ETSI TS',
    folder: 'drafts/etsi',
    title: '',
    landing: 'www.etsi.org',
    availability: 'cheap',
    classification: 'draft',
    approved: 'soon',
  };
  assert.deepEqual(ruleFindings('row', broken, ETSI), [
    'row: not an identifier of lower-case words joined by hyphens or dots: ETSI TS',
    'row: not a folder of a topic (containers, video, audio, subtitles, tags-and-registries) and a publisher: ' +
      'drafts/etsi',
    'row: rows are not in ascending order of identifier',
    'row: title is empty',
    'row: cheap is not one of free, paid, none',
    'row: draft is not one of normative, secondary, descriptive',
    'row: not an address: www.etsi.org',
    'row: not a gate: soon',
  ]);
  const bare = { ...ETSI, artifact: '', immutableReference: 'none', contentType: '' };
  assert.deepEqual(ruleFindings('row', bare, undefined), [
    'row: a fetched, snapshot or tree row needs an artifact address',
    'row: a snapshot needs an immutable reference or `none`, a tree needs `none`, and other rows have none',
    'row: a stored document needs a media type, and a link has none: ',
  ]);
  assert.deepEqual(ruleFindings('row', { ...SNAPSHOT, immutableReference: '' }, undefined), [
    'row: a snapshot needs an immutable reference or `none`, a tree needs `none`, and other rows have none',
  ]);
  assert.deepEqual(ruleFindings('row', { ...ETSI, availability: 'paid' }, undefined), [
    'row: a paid document has no artifact and is not downloaded',
  ]);
  const recorded = { checksum: 'a'.repeat(64), retrieved: '2026-11-21' };
  const link = { ...ETSI, retrieval: 'link', artifact: '', contentType: '', ...recorded };
  assert.deepEqual(ruleFindings('row', link, undefined), ['row: a link is not stored, so it has no checksum or date']);
  assert.deepEqual(ruleFindings('row', { ...ETSI, checksum: 'abc' }, undefined), [
    'row: the checksum must be SHA-256 in lower-case hexadecimal, with its date',
  ]);
});
