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

// Tests of the package-contents comparison.

import assert from 'node:assert/strict';
import { test } from 'node:test';

import { compareFileLists } from './packages.ts';

test('reports unexpected and missing files', () => {
  assert.deepEqual(compareFileLists('list.txt', ['Cargo.toml', 'src/lib.rs'], ['Cargo.toml', 'src/lib.rs']), []);
  assert.deepEqual(compareFileLists('list.txt', ['Cargo.toml', 'README.md'], ['Cargo.toml', 'tests/golden.rs']), [
    'list.txt: file in the package but not in the expected list: tests/golden.rs',
    'list.txt: expected file missing from the package: README.md',
  ]);
});
