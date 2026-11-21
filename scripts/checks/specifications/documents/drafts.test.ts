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

// Tests of the pinned drafts of Appendix A.7.

import assert from 'node:assert/strict';
import { test } from 'node:test';

import { compareDrafts, pinnedDrafts } from './drafts.ts';
import { ETSI } from './fixtures.test.ts';

const PLAN = [
  '### A.7 Internet-Drafts',
  '| Draft | Subject | Pinned revision | Exact text | State |',
  '|---|---|---|---|---|',
  '| [draft-ietf-cellar-codec](https://datatracker.ietf.org/doc/draft-ietf-cellar-codec/) | Codecs | -21 | ' +
    '[archive](https://www.ietf.org/archive/id/draft-ietf-cellar-codec-21.txt) | In IESG evaluation |',
  '### A.8 Verified errata',
  '| [draft-ietf-cellar-tags](https://datatracker.ietf.org/doc/draft-ietf-cellar-tags/) | Tags | -25 | ' +
    '[archive](https://www.ietf.org/archive/id/draft-ietf-cellar-tags-25.txt) | Not read |',
].join('\n');

test('reads the pinned drafts of A.7 only', () => {
  assert.deepEqual(pinnedDrafts(PLAN), [
    {
      name: 'draft-ietf-cellar-codec',
      revision: '21',
      address: 'https://www.ietf.org/archive/id/draft-ietf-cellar-codec-21.txt',
    },
  ]);
  assert.deepEqual(pinnedDrafts('# Plan\n'), []);
});

test('reports a draft that no row fetches at its revision', () => {
  const drafts = pinnedDrafts(PLAN);
  const row = { ...ETSI, artifact: 'https://www.ietf.org/archive/id/draft-ietf-cellar-codec-21.txt' };
  assert.deepEqual(compareDrafts('manifest.tsv', drafts, [row]), []);
  assert.deepEqual(compareDrafts('manifest.tsv', drafts, [{ ...row, retrieval: 'link' }]), [
    'manifest.tsv: no manifest row fetches the pinned revision of this draft: draft-ietf-cellar-codec-21',
  ]);
});
