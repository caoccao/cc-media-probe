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

// The deliverables and coverage tables (plan section 5.2): reading them and checking each row on its own. Keys are
// unique, start with their kind, and rows are in ascending order of key.

import { DATE_PATTERN, EMPTY, LIST_SEPARATOR } from '../constants.ts';
import { compareText, parseTable } from '../tab-separated.ts';
import {
  ACTIVE,
  COVERAGE_COLUMNS,
  COVERAGE_GATE_PATTERN,
  DELIVERABLE_COLUMNS,
  KEY_PATTERN,
  KINDS,
  RELEASE_PATTERN,
  SOURCE_SEPARATOR,
  STATUSES,
} from './constants.ts';
import {
  DUPLICATE_KEY,
  KIND_MISMATCH,
  MALFORMED_DATE,
  MALFORMED_GATE_OR_RELEASE,
  MALFORMED_KEY,
  NOT_IN_VOCABULARY,
  OUT_OF_ORDER,
  SOURCES_RULE,
} from './messages.ts';

/** One deliverable, in the words of the plan where a table of section 4 lists it. */
export interface Deliverable {
  readonly key: string;
  readonly kind: string;
  readonly section: string;
  readonly subject: string;
  readonly item: string;
  readonly carriage: string;
  readonly phase: string;
}

/** The coverage of one deliverable. */
export interface Coverage {
  readonly key: string;
  readonly status: string;
  readonly release: string;
  readonly gate: string;
  readonly sources: readonly string[];
  readonly fixture: string;
  readonly confirmed: string;
}

/** The deliverables of `text`, and findings for malformed rows; findings name `file`. */
export function readDeliverables(file: string, text: string): { rows: Deliverable[]; findings: string[] } {
  const table = parseTable(file, text, DELIVERABLE_COLUMNS);
  const findings = [...table.findings];
  const rows: Deliverable[] = [];
  for (const row of table.rows) {
    const [key = EMPTY, kind = EMPTY, section = EMPTY, subject = EMPTY, item = EMPTY, carriage = EMPTY, phase = EMPTY] =
      row.cells;
    const location = `${file}:${row.line}`;
    findings.push(...keyFindings(location, key, rows.at(-1)?.key));
    if (!KINDS.includes(kind)) {
      findings.push(`${location}: ${kind} ${NOT_IN_VOCABULARY} ${KINDS.join(LIST_SEPARATOR)}`);
    } else if (KEY_PATTERN.exec(key)?.[1] !== kind) {
      findings.push(`${location}: ${KIND_MISMATCH}`);
    }
    rows.push({ key, kind, section, subject, item, carriage, phase });
  }
  return { rows, findings: [...findings, ...duplicateFindings(file, rows.map((row) => row.key))] };
}

/** The coverage rows of `text`, and findings for malformed rows; findings name `file`. */
export function readCoverage(file: string, text: string): { rows: Coverage[]; findings: string[] } {
  const table = parseTable(file, text, COVERAGE_COLUMNS);
  const findings = [...table.findings];
  const rows: Coverage[] = [];
  for (const row of table.rows) {
    const [
      key = EMPTY,
      status = EMPTY,
      release = EMPTY,
      gate = EMPTY,
      sources = EMPTY,
      fixture = EMPTY,
      confirmed = EMPTY,
    ] = row.cells;
    const location = `${file}:${row.line}`;
    findings.push(...keyFindings(location, key, rows.at(-1)?.key));
    const parsed: Coverage = {
      key,
      status,
      release,
      gate,
      sources: sources === EMPTY ? [] : sources.split(SOURCE_SEPARATOR),
      fixture,
      confirmed,
    };
    findings.push(...coverageFindings(location, parsed));
    rows.push(parsed);
  }
  return { rows, findings: [...findings, ...duplicateFindings(file, rows.map((row) => row.key))] };
}

function keyFindings(location: string, key: string, previous: string | undefined): string[] {
  const findings: string[] = [];
  if (!KEY_PATTERN.test(key)) {
    findings.push(`${location}: ${MALFORMED_KEY}: ${key}`);
  }
  if (previous !== undefined && compareText(previous, key) > 0) {
    findings.push(`${location}: ${OUT_OF_ORDER}`);
  }
  return findings;
}

function duplicateFindings(file: string, keys: readonly string[]): string[] {
  const seen = new Set<string>();
  const findings: string[] = [];
  for (const key of keys) {
    if (seen.has(key)) {
      findings.push(`${file}: ${DUPLICATE_KEY}: ${key}`);
    }
    seen.add(key);
  }
  return findings;
}

/** An active row has a release and sources and no gate; a deferred or dropped row names its gate. */
function coverageFindings(location: string, row: Coverage): string[] {
  const findings: string[] = [];
  if (!STATUSES.includes(row.status)) {
    findings.push(`${location}: ${row.status} ${NOT_IN_VOCABULARY} ${STATUSES.join(LIST_SEPARATOR)}`);
  }
  const active = row.status === ACTIVE;
  const releaseValid = active ? RELEASE_PATTERN.test(row.release) : row.release === EMPTY;
  const gateValid = active ? row.gate === EMPTY : COVERAGE_GATE_PATTERN.test(row.gate);
  if (!releaseValid || !gateValid) {
    findings.push(`${location}: ${MALFORMED_GATE_OR_RELEASE}`);
  }
  if (active && row.sources.length === 0) {
    findings.push(`${location}: ${SOURCES_RULE}`);
  }
  if (row.confirmed !== EMPTY && !DATE_PATTERN.test(row.confirmed)) {
    findings.push(`${location}: ${MALFORMED_DATE}`);
  }
  return findings;
}
