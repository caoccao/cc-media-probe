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

// The errata table (plan section 5.3): every verified erratum of every listed RFC, and every erratum held for
// document update that affects parsing, each classified by its relevance to this project. Rows are in ascending
// order of RFC and erratum.

import { EMPTY, LIST_SEPARATOR } from '../constants.ts';
import { parseTable } from '../tab-separated.ts';
import {
  ERRATA_COLUMNS,
  ERRATUM_TYPES,
  HELD,
  NO_RELEVANCE,
  NUMBER_PATTERN,
  RELEVANCES,
  VERIFIED,
} from './constants.ts';
import {
  COMPONENTS_RULE,
  DUPLICATE_ERRATUM,
  EMPTY_SUMMARY,
  MALFORMED_NUMBER,
  NOT_IN_VOCABULARY,
  OUT_OF_ORDER,
  UNLISTED_RFC,
} from './messages.ts';

/** One row of the errata table. */
export interface ErratumRow {
  readonly rfc: number;
  readonly erratum: number;
  readonly status: string;
  readonly type: string;
  readonly section: string;
  readonly relevance: string;
  readonly components: string;
  readonly summary: string;
}

/** The rows of the errata table `text`, checked against the RFCs of the manifest; findings name `file`. */
export function readErrata(
  file: string,
  text: string,
  listed: ReadonlySet<number>,
): { rows: ErratumRow[]; findings: string[] } {
  const table = parseTable(file, text, ERRATA_COLUMNS);
  const findings = [...table.findings];
  const rows: ErratumRow[] = [];
  const seen = new Set<number>();
  for (const row of table.rows) {
    const [
      rfc = EMPTY,
      erratum = EMPTY,
      status = EMPTY,
      type = EMPTY,
      section = EMPTY,
      relevance = EMPTY,
      components = EMPTY,
      summary = EMPTY,
    ] = row.cells;
    const location = `${file}:${row.line}`;
    if (!NUMBER_PATTERN.test(rfc) || !NUMBER_PATTERN.test(erratum)) {
      findings.push(`${location}: ${MALFORMED_NUMBER}`);
      continue;
    }
    const parsed: ErratumRow = {
      rfc: Number(rfc),
      erratum: Number(erratum),
      status,
      type,
      section,
      relevance,
      components,
      summary,
    };
    findings.push(...rowFindings(location, parsed, listed, seen));
    const previous = rows.at(-1);
    if (previous !== undefined && compareRows(previous, parsed) >= 0) {
      findings.push(`${location}: ${OUT_OF_ORDER}`);
    }
    seen.add(parsed.erratum);
    rows.push(parsed);
  }
  return { rows, findings };
}

/** Orders rows by RFC, then by erratum. */
export function compareRows(first: ErratumRow, second: ErratumRow): number {
  return first.rfc - second.rfc || first.erratum - second.erratum;
}

function rowFindings(
  location: string,
  row: ErratumRow,
  listed: ReadonlySet<number>,
  seen: ReadonlySet<number>,
): string[] {
  const findings: string[] = [];
  if (!listed.has(row.rfc)) {
    findings.push(`${location}: ${UNLISTED_RFC}: ${row.rfc}`);
  }
  if (seen.has(row.erratum)) {
    findings.push(`${location}: ${DUPLICATE_ERRATUM}: ${row.erratum}`);
  }
  const vocabularies: [string, readonly string[]][] = [
    [row.status, [VERIFIED, HELD]],
    [row.type, ERRATUM_TYPES],
    [row.relevance, RELEVANCES],
  ];
  for (const [value, vocabulary] of vocabularies) {
    if (!vocabulary.includes(value)) {
      findings.push(`${location}: ${value} ${NOT_IN_VOCABULARY} ${vocabulary.join(LIST_SEPARATOR)}`);
    }
  }
  if ((row.relevance === NO_RELEVANCE) !== (row.components === EMPTY)) {
    findings.push(`${location}: ${COMPONENTS_RULE}`);
  }
  if (row.summary === EMPTY) {
    findings.push(`${location}: ${EMPTY_SUMMARY}`);
  }
  return findings;
}
