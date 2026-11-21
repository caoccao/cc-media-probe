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

// Tests of reading the declared `rust-version`.

import assert from 'node:assert/strict';
import { test } from 'node:test';

import { declaredRustVersion } from './declared.ts';

function metadata(...packages: readonly unknown[]): string {
  return JSON.stringify({ packages, workspace_members: [] });
}

const LIBRARY = { name: 'cc-media-probe', rust_version: '1.100' };
const COMMAND_LINE = { name: 'cc-media-probe-cli', rust_version: '1.100' };

test('reads the release that both published packages declare', () => {
  const support = { name: 'cc-media-probe-checks', rust_version: null };
  assert.deepEqual(declaredRustVersion(metadata(support, LIBRARY, COMMAND_LINE)), { version: '1.100', findings: [] });
});

test('fails when a published package declares another release or none', () => {
  assert.deepEqual(declaredRustVersion(metadata(LIBRARY, { ...COMMAND_LINE, rust_version: '1.99' })), {
    version: undefined,
    findings: ['the published packages declare different rust-version values'],
  });
  assert.deepEqual(declaredRustVersion(metadata(LIBRARY, { ...COMMAND_LINE, rust_version: null })), {
    version: undefined,
    findings: ['cc-media-probe-cli: declares no rust-version'],
  });
  assert.deepEqual(declaredRustVersion(metadata(null, COMMAND_LINE)), {
    version: undefined,
    findings: ['cc-media-probe: is not in the workspace'],
  });
});

test('fails on output without a list of packages', () => {
  const unreadable = { version: undefined, findings: ['cargo metadata printed no list of packages'] };
  assert.deepEqual(declaredRustVersion('not JSON'), unreadable);
  assert.deepEqual(declaredRustVersion('null'), unreadable);
  assert.deepEqual(declaredRustVersion('{"packages":{}}'), unreadable);
});
