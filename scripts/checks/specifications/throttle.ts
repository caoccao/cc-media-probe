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

// Pacing of requests, per host: one request at a time, at least the host's interval apart; some hosts ask for a
// longer interval than the default. When a host shows signs of blocking, its interval grows, by the factor or to
// what the host asks for in `Retry-After`, and stays grown for the rest of the run.

import { setTimeout as pause } from 'node:timers/promises';

import {
  BACK_OFF_FACTOR,
  DIGITS_PATTERN,
  HOST_INTERVALS,
  MAXIMUM_INTERVAL,
  MILLISECONDS_PER_SECOND,
  REQUEST_INTERVAL,
  RETRIED_STATUSES,
} from './constants.ts';

/** The interval of each host that backed off, in milliseconds. */
const intervals = new Map<string, number>();
/** When each host was last asked, in milliseconds since the epoch. */
const lastRequests = new Map<string, number>();

/** Waits until `host` was last asked at least its interval ago, and records the new request. */
export async function waitForTurn(host: string): Promise<void> {
  const wait = (lastRequests.get(host) ?? 0) + intervalOf(host) - Date.now();
  if (wait > 0) {
    await pause(wait);
  }
  lastRequests.set(host, Date.now());
}

/** Grows the interval of `host` after a sign of blocking; returns the new interval in milliseconds. */
export function backOff(host: string, retryAfter: string | null): number {
  const grown = nextInterval(intervalOf(host), requestedDelay(retryAfter, Date.now()));
  intervals.set(host, grown);
  return grown;
}

/** The current interval of `host`, in milliseconds: grown by backing off, or the host's own, or the default. */
export function intervalOf(host: string): number {
  return intervals.get(host) ?? HOST_INTERVALS.get(host) ?? REQUEST_INTERVAL;
}

/** Whether a response status is retried after backing off: a sign of blocking, or a passing server failure. */
export function isRetried(status: number): boolean {
  return RETRIED_STATUSES.includes(status);
}

/** The interval after a sign of blocking: the current one times the factor, or the delay that the host asked for
 * when that is longer, never more than the maximum. */
export function nextInterval(current: number, requested: number | undefined): number {
  return Math.min(MAXIMUM_INTERVAL, Math.max(current * BACK_OFF_FACTOR, requested ?? 0));
}

/** The delay that a `Retry-After` value asks for, in milliseconds: a number of seconds or an HTTP date. */
export function requestedDelay(retryAfter: string | null, now: number): number | undefined {
  if (retryAfter === null) {
    return undefined;
  }
  const value = retryAfter.trim();
  if (DIGITS_PATTERN.test(value)) {
    return Number(value) * MILLISECONDS_PER_SECOND;
  }
  const date = Date.parse(value);
  return Number.isNaN(date) ? undefined : Math.max(0, date - now);
}
