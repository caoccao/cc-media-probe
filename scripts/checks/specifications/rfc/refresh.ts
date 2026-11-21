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

// The RFC set compared with the RFC Editor's current records (plan section 5.3): the metadata of each RFC, and the
// list of every erratum. A problem needs a change to the tables or to Appendix A; a note needs only a review.

import { CELL_SEPARATOR, EMPTY, LIST_SEPARATOR, SPACE, WHITESPACE_PATTERN } from '../constants.ts';
import {
  DOCUMENT_FIELD,
  DOCUMENT_PREFIX,
  ERRATUM_DOCUMENT_FIELD,
  ERRATUM_FIELD,
  ERRATUM_LABEL,
  ERRATUM_SECTION_FIELD,
  ERRATUM_STATUS_FIELD,
  ERRATUM_TYPE_FIELD,
  HELD,
  OBSOLETED_BY_FIELD,
  RECORD_STATUSES,
  RECORD_TYPES,
  RFC_LABEL,
  STATUS_FIELD,
  TITLE_FIELD,
  UPDATED_BY_FIELD,
  VERIFIED,
} from './constants.ts';
import { listField, textField } from '../json-fields.ts';
import type { Report } from '../report.ts';
import type { ErratumRow } from './errata.ts';
import type { RfcEntry } from './manifest.ts';
import {
  CHANGED_STATUS,
  CHANGED_TITLE,
  ERRATUM_CHANGED,
  MISSING_ERRATUM,
  OBSOLETED,
  UNKNOWN_ERRATUM,
  UNLISTED_HELD_ERRATUM,
  UNLISTED_UPDATE,
} from './messages.ts';

/** The RFC Editor's metadata of one RFC; the status is in capitals, as the RFC Editor writes it. */
export interface RfcMetadata {
  readonly number: number;
  readonly title: string;
  readonly status: string;
  readonly obsoletedBy: readonly number[];
  readonly updatedBy: readonly number[];
}

/** One erratum of the RFC Editor's list, with its status and type as the RFC Editor writes them. */
export interface ErratumRecord {
  readonly rfc: number;
  readonly erratum: number;
  readonly status: string;
  readonly type: string;
  readonly section: string;
}

/** The metadata in a parsed JSON document, or `undefined` when it does not name an RFC. */
export function parseMetadata(document: unknown): RfcMetadata | undefined {
  const number = rfcNumber(textField(document, DOCUMENT_FIELD));
  if (number === undefined) {
    return undefined;
  }
  return {
    number,
    title: textField(document, TITLE_FIELD),
    status: textField(document, STATUS_FIELD),
    obsoletedBy: numbersField(document, OBSOLETED_BY_FIELD),
    updatedBy: numbersField(document, UPDATED_BY_FIELD),
  };
}

/** The errata of a parsed JSON list; records that do not name an RFC and a number are left out. */
export function parseErrata(document: unknown): ErratumRecord[] {
  if (!Array.isArray(document)) {
    return [];
  }
  return document.flatMap((record: unknown) => {
    const rfc = rfcNumber(textField(record, ERRATUM_DOCUMENT_FIELD));
    const erratum = Number(textField(record, ERRATUM_FIELD));
    if (rfc === undefined || !Number.isSafeInteger(erratum) || erratum < 1) {
      return [];
    }
    const status = textField(record, ERRATUM_STATUS_FIELD);
    return [{ rfc, erratum, status, type: textField(record, ERRATUM_TYPE_FIELD), section: sectionOf(record) }];
  });
}

/** Compares one manifest entry with its metadata; RFCs that update it are noted unless the manifest lists them. */
export function compareMetadata(entry: RfcEntry, metadata: RfcMetadata, listed: ReadonlySet<number>): Report {
  const label = `${RFC_LABEL}${entry.number}`;
  const problems: string[] = [];
  if (metadata.title !== entry.title) {
    problems.push(`${label}: ${CHANGED_TITLE}: ${metadata.title}`);
  }
  if (metadata.status !== entry.status.toUpperCase()) {
    problems.push(`${label}: ${CHANGED_STATUS}: ${metadata.status}`);
  }
  if (metadata.obsoletedBy.length > 0) {
    problems.push(`${label} ${OBSOLETED} ${metadata.obsoletedBy.map(rfcLabel).join(LIST_SEPARATOR)}`);
  }
  const unlisted = metadata.updatedBy.filter((number) => !listed.has(number));
  const updates = unlisted.map(rfcLabel).join(LIST_SEPARATOR);
  const notes = unlisted.length > 0 ? [`${label} ${UNLISTED_UPDATE} ${updates}`] : [];
  return { problems, notes };
}

/** Compares the errata table with the RFC Editor's list, for the RFCs of the manifest. */
export function compareErrata(
  listed: ReadonlySet<number>,
  rows: readonly ErratumRow[],
  records: readonly ErratumRecord[],
): Report {
  const problems: string[] = [];
  const notes: string[] = [];
  const tabled = new Map(rows.map((row) => [row.erratum, row]));
  const known = new Map(records.map((record) => [record.erratum, record]));
  const relevant = records.filter((candidate) => listed.has(candidate.rfc));
  relevant.sort((first, second) => first.rfc - second.rfc || first.erratum - second.erratum);
  for (const record of relevant) {
    const status = RECORD_STATUSES.get(record.status);
    if (tabled.has(record.erratum) || (status !== VERIFIED && status !== HELD)) {
      continue;
    }
    const label = `${RFC_LABEL}${record.rfc}, ${ERRATUM_LABEL}${record.erratum}`;
    if (status === VERIFIED) {
      problems.push(`${label}: ${MISSING_ERRATUM}: ${templateRow(record)}`);
    } else {
      notes.push(`${label}: ${UNLISTED_HELD_ERRATUM}`);
    }
  }
  for (const row of rows) {
    const record = known.get(row.erratum);
    const label = `${RFC_LABEL}${row.rfc}, ${ERRATUM_LABEL}${row.erratum}`;
    if (record?.rfc !== row.rfc) {
      problems.push(`${label}: ${UNKNOWN_ERRATUM}`);
      continue;
    }
    const given = [RECORD_STATUSES.get(record.status), RECORD_TYPES.get(record.type), record.section];
    const held = [row.status, row.type, row.section];
    if (given.some((value, index) => value !== held[index])) {
      problems.push(`${label}: ${ERRATUM_CHANGED} ${held.join(LIST_SEPARATOR)}: ${given.join(LIST_SEPARATOR)}`);
    }
  }
  return { problems, notes };
}

/** A table row for a missing erratum, with its relevance, components and summary still to be written. */
export function templateRow(record: ErratumRecord): string {
  const status = RECORD_STATUSES.get(record.status) ?? EMPTY;
  const type = RECORD_TYPES.get(record.type) ?? EMPTY;
  return [String(record.rfc), String(record.erratum), status, type, record.section, EMPTY, EMPTY, EMPTY].join(
    CELL_SEPARATOR,
  );
}

function rfcLabel(number: number): string {
  return `${RFC_LABEL}${number}`;
}

/** The number of an RFC that the RFC Editor names as `RFCnnnn`. */
function rfcNumber(name: string): number | undefined {
  if (!name.startsWith(DOCUMENT_PREFIX)) {
    return undefined;
  }
  const number = Number(name.slice(DOCUMENT_PREFIX.length));
  return Number.isSafeInteger(number) && number > 0 ? number : undefined;
}

/** An erratum's section, on one line: the RFC Editor writes some sections across lines. */
function sectionOf(record: unknown): string {
  return textField(record, ERRATUM_SECTION_FIELD).split(WHITESPACE_PATTERN).filter(Boolean).join(SPACE);
}

function numbersField(document: unknown, name: string): number[] {
  return listField(document, name).flatMap((item) => {
    const number = rfcNumber(String(item));
    return number === undefined ? [] : [number];
  });
}
