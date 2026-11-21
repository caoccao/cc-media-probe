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

// Tests of walking a documentation tree.

import assert from 'node:assert/strict';
import { test } from 'node:test';

import type { Download } from '../downloads.ts';
import { childPaths, collectTree } from './tree.ts';

const BASE = 'https://example.org/data';

function page(children: readonly string[]): object {
  const references = Object.fromEntries(children.map((child) => ['doc://' + child, { url: child }]));
  return { topicSections: [{ identifiers: children.map((child) => 'doc://' + child) }], references };
}

const PAGES = new Map<string, object>([
  ['/documentation/format', page(['/documentation/format/b', '/documentation/format/a', '/documentation/other'])],
  ['/documentation/format/a', page(['/documentation/format/b'])],
  ['/documentation/format/b', page(['/documentation/format'])],
]);

function serve(calls: string[]): (address: string) => Promise<Download> {
  return (address) => {
    calls.push(address);
    const found = PAGES.get(address.slice(BASE.length, -'.json'.length));
    return found === undefined
      ? Promise.reject(new Error('not found'))
      : Promise.resolve({ mediaType: 'application/json', bytes: Buffer.from(JSON.stringify(found)) });
  };
}

test('names the child pages below the root only', () => {
  assert.deepEqual(childPaths(PAGES.get('/documentation/format'), '/documentation/format'), [
    '/documentation/format/b',
    '/documentation/format/a',
  ]);
  assert.deepEqual(childPaths('not a page', '/documentation/format'), []);
});

test('visits every page once and stores them in order of path', async () => {
  const calls: string[] = [];
  const bytes = await collectTree(BASE + '/documentation/format.json', serve(calls));
  assert.equal(calls.length, 3);
  const stored = JSON.parse(bytes.toString()) as { root: string; pages: { path: string }[] };
  assert.equal(stored.root, BASE + '/documentation/format.json');
  assert.deepEqual(stored.pages.map((entry) => entry.path), [
    '/documentation/format',
    '/documentation/format/a',
    '/documentation/format/b',
  ]);
  assert.ok(bytes.toString().endsWith('}\n'));
});

test('fails on a page that is not JSON', async () => {
  const text = (): Promise<Download> => Promise.resolve({ mediaType: 'text/html', bytes: Buffer.from('<html>') });
  await assert.rejects(collectTree(BASE + '/documentation/format.json', text), /not JSON: \/documentation\/format$/u);
});
