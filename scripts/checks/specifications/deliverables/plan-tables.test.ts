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

// Tests of reading the tables of plan section 4.

import assert from 'node:assert/strict';
import { test } from 'node:test';

import { plannedTables, splitOutsideGroups } from './plan-tables.ts';

const CONFIGURATION = 'Decoder configuration record';

export const PLAN = [
  '### 4.1 Container and stream formats',
  '| Family | Formats (typical extensions) | Phase | Release |',
  '|---|---|---|---|',
  '| Matroska | Matroska (`mkv`, `mka`), WebM (`webm`) | 5 | 0.1.0 |',
  '| Detect-only | MicroDVD† | 3 | 0.1.0 |',
  '### 4.3 Codec-by-carriage checklist',
  '| Codec | Representation | Parser phase | Carriages (phase in which the pair ships) |',
  '|---|---|---|---|',
  '| H.264/AVC | Decoder configuration record | 4 | Matroska private data (5); FLV sequence header (13) |',
  '| Theora | Identification header | 9 | Ogg packets (9); Matroska laced private data (retrofit in 9) |',
  '| Codec | Why there is nothing to parse | Carried in (phase) |',
  '|---|---|---|',
  '| VobButtons | R22 | Matroska (5) |',
  '### 4.4 Detection inventory',
  '| Matroska | Not read | 5 | 0.1.0 |',
].join('\n');

test('splits at a separator outside parentheses and code spans', () => {
  assert.deepEqual(splitOutsideGroups('A (`a`, `b`), B, C (x; y)', ', '), ['A (`a`, `b`)', 'B', 'C (x; y)']);
  assert.deepEqual(splitOutsideGroups('One (1); Two (retrofit in 2)', '; '), ['One (1)', 'Two (retrofit in 2)']);
  assert.deepEqual(splitOutsideGroups('', ', '), []);
});

test('reads the formats, the pairs and the identified carriages in the plan\'s words', () => {
  const tables = plannedTables(PLAN);
  assert.deepEqual(tables.formats, [
    { family: 'Matroska', format: 'Matroska (`mkv`, `mka`)', phase: '5', release: '0.1.0' },
    { family: 'Matroska', format: 'WebM (`webm`)', phase: '5', release: '0.1.0' },
    { family: 'Detect-only', format: 'MicroDVD†', phase: '3', release: '0.1.0' },
  ]);
  assert.deepEqual(tables.pairs, [
    { codec: 'H.264/AVC', representation: CONFIGURATION, carriage: 'Matroska private data', phase: '5' },
    { codec: 'H.264/AVC', representation: CONFIGURATION, carriage: 'FLV sequence header', phase: '13' },
    { codec: 'Theora', representation: 'Identification header', carriage: 'Ogg packets', phase: '9' },
    {
      codec: 'Theora',
      representation: 'Identification header',
      carriage: 'Matroska laced private data',
      phase: 'retrofit in 9',
    },
  ]);
  assert.deepEqual(tables.identified, [{ codec: 'VobButtons', representation: '', carriage: 'Matroska', phase: '5' }]);
  assert.deepEqual(plannedTables('# Plan\n'), { formats: [], pairs: [], identified: [] });
});
