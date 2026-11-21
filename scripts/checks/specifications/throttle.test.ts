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

// Tests of the pacing of requests and of backing off.

import assert from 'node:assert/strict';
import { test } from 'node:test';

import { backOff, intervalOf, isRetried, nextInterval, requestedDelay, waitForTurn } from './throttle.ts';

test('retries signs of blocking and passing server failures, and nothing else', () => {
  assert.deepEqual([403, 429, 500, 502, 503, 504].map(isRetried), [true, true, true, true, true, true]);
  assert.deepEqual([200, 301, 404, 410, 501].map(isRetried), [false, false, false, false, false]);
});

test('doubles the interval, follows a longer requested delay and stops at the maximum', () => {
  assert.equal(nextInterval(2_000, undefined), 4_000);
  assert.equal(nextInterval(2_000, 30_000), 30_000);
  assert.equal(nextInterval(2_000, 1_000), 4_000);
  assert.equal(nextInterval(200_000, undefined), 300_000);
  assert.equal(nextInterval(2_000, 3_600_000), 300_000);
});

test('reads a Retry-After value in seconds or as a date', () => {
  const now = Date.parse('2026-11-21T10:00:00Z');
  assert.equal(requestedDelay(null, now), undefined);
  assert.equal(requestedDelay(' 120 ', now), 120_000);
  assert.equal(requestedDelay('Sat, 21 Nov 2026 10:01:00 GMT', now), 60_000);
  assert.equal(requestedDelay('Sat, 21 Nov 2026 09:00:00 GMT', now), 0);
  assert.equal(requestedDelay('soon', now), undefined);
});

test('keeps a grown interval for its host only', async () => {
  assert.equal(intervalOf('slow.example'), 2_000);
  assert.equal(backOff('slow.example', null), 4_000);
  assert.equal(backOff('slow.example', '10'), 10_000);
  assert.equal(intervalOf('slow.example'), 10_000);
  assert.equal(intervalOf('other.example'), 2_000);
  assert.equal(intervalOf('web.archive.org'), 6_000);
  const start = Date.now();
  await waitForTurn('first.example');
  assert.ok(Date.now() - start < 1_000);
});
