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

// Tests of the script structure check: tokens, statements, literals, header and line limit.

import assert from 'node:assert/strict';
import { test } from 'node:test';

import { HEADER_BODY } from './constants.ts';
import { checkLiterals } from './literals.ts';
import { topLevelStatements } from './statements.ts';
import { checkHeader, checkScript } from './structure.ts';
import { tokenize } from './tokenizer.ts';

const HEADER = ['/*', '* Copyright (c) 2026. caoccao.com Sam Cao', ...HEADER_BODY, ''].join('\n');

test('tokenizes literals, templates, regular expressions and comments', () => {
  const tokens = tokenize("// note\nconst value = `a${b + `c${d}`}e` / 2; /* x */ f(/[/]+/u, 'q\\'', 1_000n);\n");
  const kinds = tokens.map((token) => `${token.kind}:${token.text}`);
  assert.deepEqual(kinds.slice(0, 4), ['identifier:const', 'identifier:value', 'punctuation:=', 'template:`a${']);
  assert.ok(kinds.includes('template:`c${'));
  assert.ok(kinds.includes('template:}`'));
  assert.ok(kinds.includes('template:}e`'));
  assert.ok(kinds.includes('punctuation:/'));
  assert.ok(kinds.includes('regular-expression:/[/]+/u'));
  assert.ok(kinds.includes("string:'q\\''"));
  assert.ok(kinds.includes('number:1_000n'));
  assert.equal(tokens.at(-1)?.line, 2);
});

test('reads numbers with exponents and tracks lines across comments and templates', () => {
  const tokens = tokenize('/*\n\n*/ 1.5e-3 .5 `\n` end');
  assert.deepEqual(
    tokens.map((token) => [token.text, token.line]),
    [['1.5e-3', 3], ['.5', 3], ['`\n`', 3], ['end', 4]],
  );
});

test('splits top-level statements', () => {
  const statements = topLevelStatements(tokenize("import { a } from 'b';\nexport const c = { d: 1 };\nfunction e() { return; }\nconst f = 2;\n"));
  assert.deepEqual(
    statements.map((statement) => [statement.first.text, statement.isImport, statement.isConstant]),
    [['import', true, false], ['export', false, true], ['function', false, false], ['const', false, true]],
  );
});

test('rejects literals in ordinary scripts', () => {
  const source = "import { x } from 'node:fs';\nfunction run(): void { console.log('text', 42, /pattern/u, `${x}: ${x}`, `label ${x}`, 0, 1); }\n";
  assert.deepEqual(checkLiterals('scripts/run.ts', tokenize(source)), [
    "scripts/run.ts:2: literal outside a constants module; name it as a constant (R34): 'text'",
    'scripts/run.ts:2: literal outside a constants module; name it as a constant (R34): 42',
    'scripts/run.ts:2: literal outside a constants module; name it as a constant (R34): /pattern/u',
    'scripts/run.ts:2: template literal holds text; move the text into a constant (R34)',
  ]);
});

test('accepts constants modules, data tables and tests, and checks constants modules', () => {
  const table = "import { join } from 'node:path';\nexport const NAME = 'value';\nconst LIMIT = 500;\n";
  assert.deepEqual(checkLiterals('scripts/checks/constants.ts', tokenize(table)), []);
  assert.deepEqual(checkLiterals('scripts/checks/table.ts', tokenize(table)), []);
  assert.deepEqual(checkLiterals('scripts/checks/run.test.ts', tokenize("run('text', 42);\n")), []);
  assert.deepEqual(checkLiterals('scripts/constants/limits.ts', tokenize('export function limit() { return 2; }\n')), [
    'scripts/constants/limits.ts:1: a constants module holds only imports and constants (R34)',
  ]);
  assert.deepEqual(checkLiterals('scripts/run.ts', tokenize("const module = await import('node:fs');\nimport 'node:test';\n")), []);
});

test('checks the copyright header', () => {
  assert.equal(checkHeader('a.ts', `${HEADER}\nexport {};\n`), undefined);
  assert.equal(checkHeader('a.ts', HEADER.replace('2026.', '2026-2027.')), undefined);
  assert.equal(checkHeader('a.ts', '// no header\n'), 'a.ts:1: the file does not start with the copyright header');
  assert.equal(checkHeader('a.ts', HEADER.replace('2026.', '2027-2026.')), 'a.ts:2: the copyright line needs a year or a rising year range');
  assert.equal(checkHeader('a.ts', '/*\n'), 'a.ts:2: the copyright line needs a year or a rising year range');
  assert.equal(
    checkHeader('a.ts', HEADER.replace('All rights reserved.', 'Some rights reserved.')),
    'a.ts:3: the copyright header differs from the approved text',
  );
});

test('limits a script to 500 lines', () => {
  const body = 'export {};\n';
  assert.deepEqual(checkScript('a.ts', HEADER + body.repeat(500 - 16)), []);
  assert.deepEqual(checkScript('a.ts', HEADER + body.repeat(501 - 16)), ['a.ts:501: more lines than the limit allows (R34) (501 > 500)']);
});
