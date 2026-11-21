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

// The RFC manifest (plan section 5.2): one row per RFC of Appendix A, in ascending order, with the address of its
// plain-text copy, the SHA-256 of the stored file and the day it was retrieved. Number, title, status, address and
// role follow from Appendix A; the checksum and the date are recorded by the fetch script.

import { CHECKSUM_PATTERN, DATE_PATTERN, EMPTY } from '../constants.ts';
import { formatTable, parseTable } from '../tab-separated.ts';
import type { AppendixRfc } from './appendix.ts';
import {
  MANIFEST_COLUMNS,
  METADATA_ADDRESS_EXTENSION,
  NUMBER_PATTERN,
  RFC_FILE_EXTENSION,
  RFC_LABEL,
  RFC_FILE_PREFIX,
  TEXT_ADDRESS_PREFIX,
} from './constants.ts';
import {
  DIFFERS,
  MALFORMED_CHECKSUM,
  MALFORMED_DATE,
  MALFORMED_NUMBER,
  MISSING_FROM_APPENDIX,
  MISSING_FROM_MANIFEST,
  OUT_OF_ORDER,
} from './messages.ts';

/** One row of the manifest. An RFC that was never stored has an empty checksum and date. */
export interface RfcEntry {
  readonly number: number;
  readonly title: string;
  readonly status: string;
  readonly address: string;
  readonly checksum: string;
  readonly retrieved: string;
  readonly role: string;
}

/** The file name of an RFC's text in `docs/rfc/`. */
export function rfcFileName(number: number): string {
  return `${RFC_FILE_PREFIX}${number}${RFC_FILE_EXTENSION}`;
}

/** The address of the RFC Editor's plain-text copy. */
export function textAddress(number: number): string {
  return `${TEXT_ADDRESS_PREFIX}${number}${RFC_FILE_EXTENSION}`;
}

/** The address of the RFC Editor's metadata of one RFC. */
export function metadataAddress(number: number): string {
  return `${TEXT_ADDRESS_PREFIX}${number}${METADATA_ADDRESS_EXTENSION}`;
}

/** The entries of the manifest `text`, and findings for malformed rows; findings name `file`. */
export function readManifest(file: string, text: string): { entries: RfcEntry[]; findings: string[] } {
  const table = parseTable(file, text, MANIFEST_COLUMNS);
  const findings = [...table.findings];
  const entries: RfcEntry[] = [];
  for (const row of table.rows) {
    const [
      number = EMPTY,
      title = EMPTY,
      status = EMPTY,
      address = EMPTY,
      checksum = EMPTY,
      retrieved = EMPTY,
      role = EMPTY,
    ] = row.cells;
    const location = `${file}:${row.line}`;
    if (!NUMBER_PATTERN.test(number)) {
      findings.push(`${location}: ${MALFORMED_NUMBER}: ${number}`);
      continue;
    }
    if (checksum !== EMPTY && !CHECKSUM_PATTERN.test(checksum)) {
      findings.push(`${location}: ${MALFORMED_CHECKSUM}`);
    }
    if ((checksum === EMPTY) !== (retrieved === EMPTY) || (retrieved !== EMPTY && !DATE_PATTERN.test(retrieved))) {
      findings.push(`${location}: ${MALFORMED_DATE}: ${retrieved}`);
    }
    const previous = entries.at(-1);
    if (previous !== undefined && previous.number >= Number(number)) {
      findings.push(`${location}: ${OUT_OF_ORDER}`);
    }
    entries.push({ number: Number(number), title, status, address, checksum, retrieved, role });
  }
  return { entries, findings };
}

/** The cells of one entry, in column order. */
export function entryCells(entry: RfcEntry): string[] {
  const { number, title, status, address, checksum, retrieved, role } = entry;
  return [String(number), title, status, address, checksum, retrieved, role];
}

/** The manifest's text for `entries`, in ascending order. */
export function formatManifest(entries: readonly RfcEntry[]): string {
  const sorted = [...entries].sort((first, second) => first.number - second.number);
  return formatTable(MANIFEST_COLUMNS, sorted.map(entryCells));
}

/** The entries that Appendix A implies, in ascending order, keeping the recorded checksum and date of `existing`. */
export function manifestFromAppendix(appendix: readonly AppendixRfc[], existing: readonly RfcEntry[]): RfcEntry[] {
  const recorded = new Map(existing.map((entry) => [entry.number, entry]));
  return [...appendix]
    .sort((first, second) => first.number - second.number)
    .map(({ number, title, status, role }) => ({
      number,
      title,
      status,
      address: textAddress(number),
      checksum: recorded.get(number)?.checksum ?? EMPTY,
      retrieved: recorded.get(number)?.retrieved ?? EMPTY,
      role,
    }));
}

/** The differences between the manifest's entries and Appendix A; findings name `file`. */
export function compareWithAppendix(
  file: string,
  entries: readonly RfcEntry[],
  appendix: readonly AppendixRfc[],
): string[] {
  const findings: string[] = [];
  const actual = new Map(entries.map((entry) => [entry.number, entry]));
  const expected = manifestFromAppendix(appendix, entries);
  const expectedNumbers = new Set(expected.map((entry) => entry.number));
  for (const entry of expected) {
    const present = actual.get(entry.number);
    if (present === undefined) {
      findings.push(`${file}: ${RFC_LABEL}${entry.number}: ${MISSING_FROM_MANIFEST}`);
      continue;
    }
    const presentCells = entryCells(present);
    entryCells(entry).forEach((cell, index) => {
      if (presentCells[index] !== cell) {
        findings.push(`${file}: ${RFC_LABEL}${entry.number}: ${MANIFEST_COLUMNS[index]} ${DIFFERS}: ${cell}`);
      }
    });
  }
  for (const entry of entries) {
    if (!expectedNumbers.has(entry.number)) {
      findings.push(`${file}: ${RFC_LABEL}${entry.number}: ${MISSING_FROM_APPENDIX}`);
    }
  }
  return findings;
}
