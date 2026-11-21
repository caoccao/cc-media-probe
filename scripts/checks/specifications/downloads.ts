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

// Downloads with the fetch function of Node.js, paced per host and backing off when a host shows signs of blocking,
// and the checksums of what is stored (plan section 5.2).

import { createHash } from 'node:crypto';

import {
  BACKING_OFF,
  BLOCKED,
  CHECKSUM_ALGORITHM,
  CHECKSUM_ENCODING,
  CONTENT_TYPE_HEADER,
  DATE_LENGTH,
  DOWNLOAD_TIMEOUT,
  EMPTY,
  FOLLOW_REDIRECTS,
  HTTP_OK,
  MAXIMUM_RETRIES,
  MEDIA_TYPE_PARAMETERS,
  MILLISECONDS_PER_SECOND,
  REFUSE_REDIRECTS,
  RETRY_AFTER_HEADER,
  SECONDS_UNIT,
  UNEXPECTED_STATUS,
  USER_AGENT,
  USER_AGENT_HEADER,
} from './constants.ts';
import { backOff, isRetried, waitForTurn } from './throttle.ts';

/** One downloaded resource: its media type, without parameters, and its bytes. */
export interface Download {
  readonly mediaType: string;
  readonly bytes: Buffer;
}

/** How a download treats redirects: an archive capture must not be replaced by another one. */
export interface DownloadOptions {
  readonly redirects: boolean;
}

/** Downloads one resource; rejects when the response is not a success. Tests pass their own. */
export type Downloader = (url: string, options?: DownloadOptions) => Promise<Download>;

/** Downloads `url`, following redirects unless `options` refuse them. A status that signals blocking or a passing
 * server failure, or a failed connection, grows the host's interval and is retried a few times; any other status
 * than 200, a refused redirect and the timeout reject. */
export async function download(url: string, options: DownloadOptions = { redirects: true }): Promise<Download> {
  const host = new URL(url).host;
  for (let attempt = 0; ; attempt += 1) {
    await waitForTurn(host);
    let response: Response;
    try {
      response = await fetch(url, {
        headers: { [USER_AGENT_HEADER]: USER_AGENT },
        redirect: options.redirects ? FOLLOW_REDIRECTS : REFUSE_REDIRECTS,
        signal: AbortSignal.timeout(DOWNLOAD_TIMEOUT),
      });
    } catch (error) {
      if (attempt >= MAXIMUM_RETRIES) {
        throw error;
      }
      report(host, backOff(host, null));
      continue;
    }
    if (isRetried(response.status)) {
      await response.body?.cancel();
      if (attempt >= MAXIMUM_RETRIES) {
        throw new Error(`${BLOCKED} ${response.status}: ${url}`);
      }
      report(host, backOff(host, response.headers.get(RETRY_AFTER_HEADER)));
      continue;
    }
    if (response.status !== HTTP_OK) {
      throw new Error(`${UNEXPECTED_STATUS} ${response.status}: ${url}`);
    }
    const bytes = Buffer.from(await response.arrayBuffer());
    return { mediaType: mediaType(response.headers.get(CONTENT_TYPE_HEADER) ?? EMPTY), bytes };
  }
}

/** Tells that `host` is now asked only every `interval` milliseconds. */
function report(host: string, interval: number): void {
  console.error(`${BACKING_OFF} ${host}: ${interval / MILLISECONDS_PER_SECOND}${SECONDS_UNIT}`);
}

/** The media type of a `Content-Type` value, in lower case and without its parameters. */
export function mediaType(contentType: string): string {
  return (contentType.split(MEDIA_TYPE_PARAMETERS)[0] ?? EMPTY).trim().toLowerCase();
}

/** The SHA-256 checksum of `bytes`, in lower-case hexadecimal. */
export function checksum(bytes: Uint8Array): string {
  return createHash(CHECKSUM_ALGORITHM).update(bytes).digest(CHECKSUM_ENCODING);
}

/** Today's date in UTC, as the tables write it. */
export function today(): string {
  return new Date().toISOString().slice(0, DATE_LENGTH);
}
