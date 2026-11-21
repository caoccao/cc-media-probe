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

// Tests of the Markdown anchors and links.

import assert from 'node:assert/strict';
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { test } from 'node:test';

import { collectAnchors, headingAnchor } from './anchors.ts';
import { checkMarkdownLinks, linkReferences } from './links.ts';

test('derives anchors as GitHub does', () => {
  assert.equal(headingAnchor('Appendix E — Revision history'), 'appendix-e--revision-history');
  assert.equal(headingAnchor('1. How to use this plan'), '1-how-to-use-this-plan');
  assert.equal(headingAnchor('The `cc_media` crate'), 'the-cc_media-crate');
  assert.equal(headingAnchor('Données écrites'), 'données-écrites');
  const anchors = collectAnchors('# Title\n## Notes\n```\n# not a heading\n```\n## Notes ##\n');
  assert.deepEqual([...anchors], ['title', 'notes', 'notes-1']);
});

test('finds inline links and reference definitions outside code', () => {
  const references = linkReferences('[a](one.md) `[b](two.md)` [c](<three.md> "title")\n```\n[d](four.md)\n```\n[e]: five.md\n');
  assert.deepEqual(references, [
    { line: 1, target: 'one.md' },
    { line: 1, target: '<three.md> "title"' },
    { line: 5, target: 'five.md' },
  ]);
});

test('reports missing files and headings', () => {
  const root = mkdtempSync(join(tmpdir(), 'markdown-links-'));
  mkdirSync(join(root, 'docs'));
  writeFileSync(join(root, 'docs', 'plan.md'), '# Plan\n## Gate summary\n');
  writeFileSync(
    join(root, 'README.md'),
    '[plan](docs/plan.md#gate-summary) [self](#readme) [web](https://example.org/x)\n' +
      '[gone](missing.md) [bad](docs/plan.md#no-such-heading) [ok](docs/) [encoded](docs/plan.md#gate%2Dsummary)\n# README\n',
  );
  const findings = checkMarkdownLinks(root, ['README.md', 'docs/plan.md']);
  rmSync(root, { recursive: true });
  assert.deepEqual(findings, [
    'README.md:2: link to a missing file: missing.md',
    'README.md:2: link to a missing heading: docs/plan.md#no-such-heading',
  ]);
});
