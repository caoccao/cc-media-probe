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

// Tests of the manifest of non-RFC documents: reading, formatting, sources and file names.

import assert from 'node:assert/strict';
import { test } from 'node:test';

import { ETSI, SNAPSHOT } from './fixtures.test.ts';
import { formatDocuments, readDocuments, sourceAddress, storedFileName, storedPath } from './manifest.ts';

test('formats in order of identifier and reads back the same entries', () => {
  const text = formatDocuments([SNAPSHOT, ETSI]);
  const lines = text.split('\n');
  assert.equal(lines[0]?.split('\t')[0], 'identifier');
  assert.equal(lines[1]?.split('\t')[0], 'etsi-ts-102366');
  assert.deepEqual(readDocuments('manifest.tsv', text), { entries: [ETSI, SNAPSHOT], findings: [] });
});

test('downloads a snapshot from its immutable reference, or from the page when it has none', () => {
  assert.equal(sourceAddress(ETSI), ETSI.artifact);
  assert.equal(sourceAddress(SNAPSHOT), SNAPSHOT.immutableReference);
  assert.equal(sourceAddress({ ...SNAPSHOT, immutableReference: 'none' }), SNAPSHOT.artifact);
});

test('names stored files by the address, or by the media type, in the folder of their topic and publisher', () => {
  assert.equal(storedFileName(ETSI), 'etsi-ts-102366.pdf');
  assert.deepEqual(storedPath(ETSI), ['audio', 'etsi', 'etsi-ts-102366.pdf']);
  assert.equal(storedFileName(SNAPSHOT), 'xiph-ogg-framing.html');
  const page = { ...SNAPSHOT, immutableReference: 'https://example.org/registry?view=all', contentType: 'text/html' };
  assert.equal(storedFileName(page), 'xiph-ogg-framing.html');
  assert.equal(storedFileName({ ...page, contentType: 'image/png' }), 'xiph-ogg-framing.bin');
  const tree = { ...SNAPSHOT, retrieval: 'tree', immutableReference: 'none', contentType: 'application/json' };
  const root = { ...tree, artifact: 'https://example.org/data/documentation/root.json' };
  assert.equal(storedFileName(root), 'xiph-ogg-framing.json');
});
