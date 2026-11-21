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

// Tests of reading the RFCs of Appendix A.

import assert from 'node:assert/strict';
import { test } from 'node:test';

import { appendixRfcs } from './appendix.ts';

const PLAN = [
  '# Plan',
  '| [1](https://www.rfc-editor.org/info/rfc1) | Before the appendix | PS | Not read | 1 |',
  '## Appendix A — RFC list',
  '| RFC | Title | Status | Role | Phase |',
  '|---|---|---|---|---|',
  '| [8794](https://www.rfc-editor.org/info/rfc8794) | Extensible Binary Meta Language | PS | EBML structure | 5 |',
  '### A.6 Conventions',
  '| [2119](https://www.rfc-editor.org/info/rfc2119) | Key words for use in RFCs | BCP | Requirement levels |',
  '| [draft-ietf-cellar-codec](https://datatracker.ietf.org/doc/draft-ietf-cellar-codec/) | Codecs | -21 | x | y |',
  '| 8794 (EBML) | [7189](https://www.rfc-editor.org/errata/eid7189) | Element ID ranges |',
  '## Appendix B — Non-RFC specifications',
  '| [3533](https://www.rfc-editor.org/info/rfc3533) | After the appendix | Info | Not read | 9 |',
].join('\n');

test('reads the RFC rows of Appendix A only', () => {
  const { rfcs, findings } = appendixRfcs('plan.md', PLAN);
  assert.deepEqual(findings, []);
  assert.deepEqual(rfcs, [
    { number: 8794, title: 'Extensible Binary Meta Language', status: 'Proposed Standard', role: 'EBML structure' },
    { number: 2119, title: 'Key words for use in RFCs', status: 'Best Current Practice', role: 'Requirement levels' },
  ]);
});

test('reports a missing appendix, a wrong link, an unknown status and a repeated RFC', () => {
  assert.deepEqual(appendixRfcs('plan.md', '# Plan\n').findings, ['plan.md: Appendix A was not found']);
  const plan = [
    '## Appendix A',
    '| [8794](https://www.rfc-editor.org/info/rfc8795) | Title | PS | Role |',
    '| [9559](https://www.rfc-editor.org/info/rfc9559) | Title | XS | Role |',
    '| [3533](https://www.rfc-editor.org/info/rfc3533) | Title | Info | Role |',
    '| [3533](https://www.rfc-editor.org/info/rfc3533) | Title | Info | Role |',
  ].join('\n');
  const { rfcs, findings } = appendixRfcs('plan.md', plan);
  assert.deepEqual(findings, [
    'plan.md:2: the link names another RFC than the label',
    'plan.md:3: unknown status abbreviation',
    'plan.md:5: RFC listed twice: 3533',
  ]);
  assert.deepEqual(rfcs.map((rfc) => rfc.number), [3533]);
});
