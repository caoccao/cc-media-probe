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

// Tests of the file hygiene check.

import assert from 'node:assert/strict';
import { test } from 'node:test';

import { checkFile } from './hygiene.ts';

const encoder = new TextEncoder();

test('accepts clean text and tab-separated data', () => {
  assert.deepEqual(checkFile('docs/notes.md', encoder.encode('# Notes\n\nText.\n')), []);
  assert.deepEqual(checkFile('data/list.tsv', encoder.encode('name\tvalue\n')), []);
  assert.deepEqual(checkFile('data/list.tsv', encoder.encode('name\tvalue\tnote\none\t\t\n')), []);
  assert.deepEqual(checkFile('data/list.tsv', encoder.encode('name\tvalue\none \t\n')), ['data/list.tsv:2: trailing whitespace']);
  assert.deepEqual(checkFile('empty.txt', encoder.encode('')), []);
});

test('rejects scripts in other languages', () => {
  for (const file of ['scripts/check.sh', 'tool.py', 'run.mjs', 'build.ps1', 'links.awk']) {
    assert.match(checkFile(file, encoder.encode('x\n'))[0] ?? '', /TypeScript/u, file);
  }
});

test('reports each hygiene problem', () => {
  const findings = checkFile('notes.md', encoder.encode('trailing \r\n\tindented\nlast'));
  assert.deepEqual(findings, [
    'notes.md: carriage return; use LF line endings',
    'notes.md: no newline at the end',
    'notes.md:1: trailing whitespace',
    'notes.md:2: tab character',
  ]);
});

test('reports binary and invalid text, and exempts verbatim texts', () => {
  assert.deepEqual(checkFile('image.png', new Uint8Array([137, 0, 1])), ['image.png: binary file; nothing binary is committed here']);
  assert.deepEqual(checkFile('latin.txt', new Uint8Array([0xe9, 0x0a])), ['latin.txt: not valid UTF-8']);
  assert.deepEqual(checkFile('LICENSE', encoder.encode('text \n')), []);
  assert.deepEqual(checkFile('docs/rfc/rfc9559.txt', encoder.encode('text \r\n')), []);
});
