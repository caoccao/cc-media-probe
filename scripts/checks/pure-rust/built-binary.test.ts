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

// Tests of reading the executable from Cargo's build messages.

import assert from 'node:assert/strict';
import { join } from 'node:path';
import { test } from 'node:test';

import { executableFromMessages } from './built-binary.ts';

test('finds the executable of the named binary', () => {
  const executable = join('target', 'pure-rust', 'debug', 'cc-media-probe');
  const messages = [
    JSON.stringify({ reason: 'compiler-artifact', target: { name: 'cc_media_probe' }, executable: null }),
    'not json',
    JSON.stringify({ reason: 'compiler-artifact', target: { name: 'cc-media-probe' }, executable }),
    JSON.stringify({ reason: 'build-finished', success: true }),
  ].join('\n');
  assert.equal(executableFromMessages(messages, 'cc-media-probe'), executable);
  assert.equal(executableFromMessages(messages, 'missing'), undefined);
});
