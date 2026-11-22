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

// The record of what was generated: each file's identity and the tool version that made it.

import { existsSync, mkdirSync, readFileSync, renameSync, writeFileSync } from 'node:fs';
import { dirname } from 'node:path';

import { FILE_ENCODING, LINE_END, STATE_INDENTATION, STATE_PARTIAL_SUFFIX } from './constants.ts';
import { isRecord, isText } from './json-values.ts';

/** What the record keeps about one generated file. */
export interface GeneratedFile {
  readonly identity: string;
  readonly tool: string;
  readonly version: string;
}

/** Reads the record at `file`; a missing or unreadable record knows no file, so everything is generated again. */
export function readState(file: string): Map<string, GeneratedFile> {
  if (!existsSync(file)) {
    return new Map();
  }
  try {
    const document: unknown = JSON.parse(readFileSync(file, FILE_ENCODING));
    const entries = isRecord(document) ? Object.entries(document) : [];
    return new Map(entries.filter((entry): entry is [string, GeneratedFile] => isGeneratedFile(entry[1])));
  } catch {
    return new Map();
  }
}

/** Writes the record to `file`, sorted by path, replacing the old one only once the new one is complete. */
export function writeState(file: string, state: ReadonlyMap<string, GeneratedFile>): void {
  const sorted = Object.fromEntries([...state.entries()].sort(([first], [second]) => first.localeCompare(second)));
  const partial = `${file}${STATE_PARTIAL_SUFFIX}`;
  mkdirSync(dirname(file), { recursive: true });
  writeFileSync(partial, JSON.stringify(sorted, undefined, STATE_INDENTATION) + LINE_END, FILE_ENCODING);
  renameSync(partial, file);
}

/** Whether `value` has the shape of a record entry. */
function isGeneratedFile(value: unknown): value is GeneratedFile {
  if (!isRecord(value)) {
    return false;
  }
  const { identity, tool, version } = value;
  return isText(identity) && isText(tool) && isText(version);
}
