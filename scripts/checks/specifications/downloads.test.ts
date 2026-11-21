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

// Tests of the media types, checksums and dates of downloads. The download itself needs the network and is
// exercised by the fetch scripts.

import assert from 'node:assert/strict';
import { test } from 'node:test';

import { checksum, mediaType, today } from './downloads.ts';

test('reduces a content type to its media type', () => {
  assert.equal(mediaType('text/plain;charset=utf-8'), 'text/plain');
  assert.equal(mediaType('Application/JSON ; charset=UTF-8'), 'application/json');
  assert.equal(mediaType(''), '');
});

test('computes SHA-256 in lower-case hexadecimal', () => {
  assert.equal(checksum(Buffer.from('abc')), 'ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad');
  assert.equal(checksum(new Uint8Array()), 'e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855');
});

test('writes today as a calendar date', () => {
  assert.match(today(), /^\d{4}-\d{2}-\d{2}$/u);
});
