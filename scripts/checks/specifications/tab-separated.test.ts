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

// Tests of the tab-separated tables.

import assert from 'node:assert/strict';
import { test } from 'node:test';

import { cellAt, formatTable, parseTable } from './tab-separated.ts';

test('parses rows by column and numbers their lines', () => {
  const table = parseTable('table.tsv', 'name\tvalue\none\t1\ntwo\t\n', ['name', 'value']);
  assert.deepEqual(table.findings, []);
  assert.deepEqual(table.rows, [
    { line: 2, cells: ['one', '1'] },
    { line: 3, cells: ['two', ''] },
  ]);
  assert.equal(cellAt(table.rows[0]!, 1), '1');
  assert.equal(cellAt(table.rows[0]!, 2), '');
});

test('reports a different header and rows of the wrong width', () => {
  assert.deepEqual(parseTable('table.tsv', 'name\n', ['name', 'value']).findings, [
    'table.tsv:1: the header row differs from the expected columns: name\tvalue',
  ]);
  const table = parseTable('table.tsv', 'name\tvalue\none\n\nthree\t3\textra\nfour\t4\n', ['name', 'value']);
  assert.deepEqual(table.findings, [
    'table.tsv:2: the row does not have one cell per column',
    'table.tsv:3: the row does not have one cell per column',
    'table.tsv:4: the row does not have one cell per column',
  ]);
  assert.deepEqual(table.rows, [{ line: 5, cells: ['four', '4'] }]);
});

test('formats a table that parses back to the same rows', () => {
  const text = formatTable(['name', 'value'], [['one', '1'], ['two', '']]);
  assert.equal(text, 'name\tvalue\none\t1\ntwo\t\n');
  assert.deepEqual(
    parseTable('table.tsv', text, ['name', 'value']).rows.map((row) => row.cells),
    [['one', '1'], ['two', '']],
  );
});
