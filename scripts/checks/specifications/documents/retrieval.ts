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

// Storing one document (plan section 5.2). A stored file is verified against the manifest. A missing one is
// downloaded by its retrieval: the artifact of a `fetch` row, the immutable reference of a `snapshot` row (or the
// living page once, when it has none), the walked pages of a `tree` row. Its media type is checked before anything
// is written, and a recorded checksum must be met exactly; a manual row is only reported until its file is placed.

import { existsSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { dirname, join } from 'node:path';

import { EMPTY } from '../constants.ts';
import { checksum, type Download, type Downloader } from '../downloads.ts';
import type { Report } from '../report.ts';
import { ARCHIVE_PREFIX, LINK, MANUAL, TREE } from './constants.ts';
import { type DocumentEntry, sourceAddress, storedPath } from './manifest.ts';
import {
  CHANGED_DOCUMENT,
  CHECKSUM_MISMATCH,
  DOWNLOAD_FAILED,
  PLACE_MANUALLY,
  UNRECORDED_FILE,
  WRONG_MEDIA_TYPE,
} from './messages.ts';
import { collectTree } from './tree.ts';

/** Stores or verifies the document of `entry` in `directory`; returns the entry as it is to be recorded. */
export async function storeDocument(
  directory: string,
  entry: DocumentEntry,
  downloader: Downloader,
  date: string,
  report: Report,
): Promise<DocumentEntry> {
  if (entry.retrieval === LINK) {
    return entry;
  }
  const path = join(directory, ...storedPath(entry));
  if (existsSync(path)) {
    const bytes = readFileSync(path);
    if (entry.checksum !== EMPTY) {
      if (checksum(bytes) !== entry.checksum) {
        report.problems.push(`${entry.identifier}: ${CHECKSUM_MISMATCH}`);
      }
      return entry;
    }
    if (entry.retrieval !== MANUAL) {
      report.problems.push(`${entry.identifier}: ${UNRECORDED_FILE}`);
      return entry;
    }
    return { ...entry, checksum: checksum(bytes), retrieved: date };
  }
  if (entry.retrieval === MANUAL) {
    report.notes.push(`${entry.identifier}: ${PLACE_MANUALLY} ${path}`);
    return entry;
  }
  let downloaded: Download;
  try {
    downloaded = await obtain(entry, downloader);
  } catch (error) {
    report.problems.push(`${entry.identifier}: ${DOWNLOAD_FAILED}: ${String(error)}`);
    return entry;
  }
  if (downloaded.mediaType !== entry.contentType) {
    report.problems.push(`${entry.identifier}: ${WRONG_MEDIA_TYPE}: ${downloaded.mediaType}`);
    return entry;
  }
  const actual = checksum(downloaded.bytes);
  if (entry.checksum !== EMPTY && actual !== entry.checksum) {
    report.problems.push(`${entry.identifier}: ${CHANGED_DOCUMENT}`);
    return entry;
  }
  mkdirSync(dirname(path), { recursive: true });
  writeFileSync(path, downloaded.bytes);
  return entry.checksum === EMPTY ? { ...entry, checksum: actual, retrieved: date } : entry;
}

/** Downloads what `entry` stores. An archive capture is never followed to another capture. */
async function obtain(entry: DocumentEntry, downloader: Downloader): Promise<Download> {
  if (entry.retrieval === TREE) {
    return { mediaType: entry.contentType, bytes: await collectTree(entry.artifact, downloader) };
  }
  const address = sourceAddress(entry);
  return downloader(address, { redirects: !address.startsWith(ARCHIVE_PREFIX) });
}
