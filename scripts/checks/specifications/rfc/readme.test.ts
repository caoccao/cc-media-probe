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

// Tests of the generated RFC index.

import assert from 'node:assert/strict';
import { test } from 'node:test';

import { textAddress } from './manifest.ts';
import { renderReadme } from './readme.ts';

test('lists the RFCs in ascending order between the fixed texts', () => {
  const entry = { checksum: '', retrieved: '', status: 'Informational' };
  const readme = renderReadme([
    { ...entry, number: 3533, title: 'Ogg', address: textAddress(3533), role: 'Ogg pages' },
    { ...entry, number: 1950, title: 'ZLIB', address: textAddress(1950), role: 'Compressed headers' },
  ]);
  const lines = readme.split('\n');
  assert.equal(lines[0], '# RFCs');
  const header = lines.indexOf('| RFC | Title | Status | Role |');
  assert.deepEqual(lines.slice(header + 2, header + 4), [
    '| [1950](rfc1950.txt) | ZLIB | Informational | Compressed headers |',
    '| [3533](rfc3533.txt) | Ogg | Informational | Ogg pages |',
  ]);
  assert.ok(lines.includes('## License notice'));
  assert.ok(readme.endsWith('.\n'));
});
