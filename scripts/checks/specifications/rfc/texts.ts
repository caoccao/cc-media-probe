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

// Storing one RFC text (plan section 5.2): a text that is present is verified against the manifest; a missing one is
// downloaded, checked for its media type and its own number, and stored. A new RFC gets its checksum and date
// recorded; a recorded one must come back with the same bytes.

import { existsSync, readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

import { TEXT_ENCODING } from '../../constants.ts';
import { EMPTY } from '../constants.ts';
import { checksum, type Download, type Downloader } from '../downloads.ts';
import { DIGIT_PATTERN, NUMBER_LINE_PREFIX, RFC_LABEL, TEXT_MEDIA_TYPE } from './constants.ts';
import { type RfcEntry, rfcFileName } from './manifest.ts';
import { CHECKSUM_MISMATCH, DOWNLOAD_FAILED, NOT_THE_RFC, UNRECORDED_TEXT, WRONG_MEDIA_TYPE } from './messages.ts';

/** Stores or verifies the text of `entry` in `directory`; returns the entry as it is to be recorded. */
export async function storeText(
  directory: string,
  entry: RfcEntry,
  downloader: Downloader,
  date: string,
  problems: string[],
): Promise<RfcEntry> {
  const path = join(directory, rfcFileName(entry.number));
  const label = `${RFC_LABEL}${entry.number}`;
  if (existsSync(path)) {
    if (entry.checksum === EMPTY) {
      problems.push(`${label}: ${UNRECORDED_TEXT}`);
    } else if (checksum(readFileSync(path)) !== entry.checksum) {
      problems.push(`${label}: ${CHECKSUM_MISMATCH}`);
    }
    return entry;
  }
  let downloaded: Download;
  try {
    downloaded = await downloader(entry.address);
  } catch (error) {
    problems.push(`${label}: ${DOWNLOAD_FAILED}: ${String(error)}`);
    return entry;
  }
  const problem = textProblem(entry, downloaded);
  if (problem !== undefined) {
    problems.push(`${label}: ${problem}`);
    return entry;
  }
  writeFileSync(path, downloaded.bytes);
  return entry.checksum === EMPTY ? { ...entry, checksum: checksum(downloaded.bytes), retrieved: date } : entry;
}

/** Why a downloaded text cannot be stored as the RFC of `entry`, or `undefined` when it can. */
export function textProblem(entry: RfcEntry, downloaded: Download): string | undefined {
  if (downloaded.mediaType !== TEXT_MEDIA_TYPE) {
    return `${WRONG_MEDIA_TYPE}: ${downloaded.mediaType}`;
  }
  if (!namesNumber(downloaded.bytes.toString(TEXT_ENCODING), entry.number)) {
    return NOT_THE_RFC;
  }
  if (entry.checksum !== EMPTY && checksum(downloaded.bytes) !== entry.checksum) {
    return CHECKSUM_MISMATCH;
  }
  return undefined;
}

/** Whether `text` holds the line that names RFC `number`, and not a longer number that starts with it. */
function namesNumber(text: string, number: number): boolean {
  const marker = `${NUMBER_LINE_PREFIX}${number}`;
  const position = text.indexOf(marker);
  return position >= 0 && !DIGIT_PATTERN.test(text.charAt(position + marker.length));
}
