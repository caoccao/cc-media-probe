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

// The comparisons of plan section 5.2 that need no code: the deliverables against the tables of section 4, in the
// plan's own words; the keys of the deliverables and the coverage table, in both directions; and each source of a
// coverage row against the documents of the two manifests.

import { existsSync, readdirSync, readFileSync } from 'node:fs';
import { join } from 'node:path';

import { MARKDOWN_EXTENSION, TEXT_ENCODING } from '../../constants.ts';
import { EMPTY, PLAN_FILE } from '../constants.ts';
import { MANIFEST_FILE as DOCUMENTS_FILE, SPECIFICATIONS_DIRECTORY } from '../documents/constants.ts';
import { readDocuments } from '../documents/manifest.ts';
import { MANIFEST_FILE as RFCS_FILE, RFC_DIRECTORY } from '../rfc/constants.ts';
import { readManifest } from '../rfc/manifest.ts';
import {
  COVERAGE_FILE,
  DELIVERABLES_FILE,
  DESCRIPTIONS_DIRECTORY,
  DESCRIPTIONS_INDEX,
  DETECT,
  DETECT_ONLY_FAMILY,
  DETECTION_SECTION,
  FORMATS_SECTION,
  IDENTIFY,
  PAIR,
  PAIRS_SECTION,
  PROBE,
  RFC_SOURCE_PREFIX,
  SECTION_SEPARATOR,
  SIGNATURE_SEPARATOR,
} from './constants.ts';
import {
  MALFORMED_SOURCE,
  MISSING_DELIVERABLE,
  MISSING_TABLE,
  NO_COVERAGE,
  NO_DELIVERABLE,
  UNKNOWN_SOURCE,
  UNPLANNED_DELIVERABLE,
} from './messages.ts';
import { type PlannedTables, plannedTables } from './plan-tables.ts';
import { type Coverage, type Deliverable, readCoverage, readDeliverables } from './tables.ts';

/** The kinds whose deliverables the tables of section 4 list, and which are compared with them. */
const PLANNED_KINDS = new Set([DETECT, PROBE, PAIR, IDENTIFY]);

/** The findings of the deliverables and coverage tables in `root`. */
export function checkDeliverables(root: string): string[] {
  const directory = join(...SPECIFICATIONS_DIRECTORY);
  const deliverablesFile = join(directory, DELIVERABLES_FILE);
  const coverageFile = join(directory, COVERAGE_FILE);
  const missing = [deliverablesFile, coverageFile].filter((file) => !existsSync(join(root, file)));
  if (missing.length > 0) {
    return missing.map((file) => `${file}: ${MISSING_TABLE}`);
  }
  const deliverables = readDeliverables(deliverablesFile, readFileSync(join(root, deliverablesFile), TEXT_ENCODING));
  const coverage = readCoverage(coverageFile, readFileSync(join(root, coverageFile), TEXT_ENCODING));
  const plan = readFileSync(join(root, ...PLAN_FILE), TEXT_ENCODING);
  return [
    ...deliverables.findings,
    ...coverage.findings,
    ...compareWithPlan(deliverablesFile, deliverables.rows, plannedTables(plan)),
    ...compareKeys(deliverablesFile, coverageFile, deliverables.rows, coverage.rows),
    ...checkSources(coverageFile, coverage.rows, knownDocuments(root)),
  ];
}

/** Every promise of the tables of section 4 has a deliverable with its words, and every deliverable of those kinds
 * has its promise. A format of section 4.1 can have several detector entries, one per representation. */
export function compareWithPlan(file: string, rows: readonly Deliverable[], tables: PlannedTables): string[] {
  const planned = new Set<string>();
  for (const { family, format, phase } of tables.formats) {
    planned.add(signature(DETECT, DETECTION_SECTION, family, format, EMPTY, EMPTY));
    if (family !== DETECT_ONLY_FAMILY) {
      planned.add(signature(PROBE, FORMATS_SECTION, family, format, EMPTY, phase));
    }
  }
  for (const { codec, representation, carriage, phase } of tables.pairs) {
    planned.add(signature(PAIR, PAIRS_SECTION, codec, representation, carriage, phase));
  }
  for (const { codec, carriage, phase } of tables.identified) {
    planned.add(signature(IDENTIFY, PAIRS_SECTION, codec, EMPTY, carriage, phase));
  }
  const delivered = new Set<string>();
  const findings: string[] = [];
  for (const row of rows.filter((candidate) => PLANNED_KINDS.has(candidate.kind))) {
    const words = signature(row.kind, row.section, row.subject, row.item, row.carriage, row.phase);
    delivered.add(words);
    if (!planned.has(words)) {
      findings.push(`${file}: ${row.key}: ${UNPLANNED_DELIVERABLE}`);
    }
  }
  for (const words of planned) {
    if (!delivered.has(words)) {
      findings.push(`${file}: ${MISSING_DELIVERABLE} ${words}`);
    }
  }
  return findings;
}

/** The keys of both tables are the same set. */
export function compareKeys(
  deliverablesFile: string,
  coverageFile: string,
  deliverables: readonly Deliverable[],
  coverage: readonly Coverage[],
): string[] {
  const delivered = new Set(deliverables.map((row) => row.key));
  const covered = new Set(coverage.map((row) => row.key));
  return [
    ...[...delivered].filter((key) => !covered.has(key)).map((key) => `${deliverablesFile}: ${key}: ${NO_COVERAGE}`),
    ...[...covered].filter((key) => !delivered.has(key)).map((key) => `${coverageFile}: ${key}: ${NO_DELIVERABLE}`),
  ];
}

/** Every source names a document of the RFC manifest (`rfcNNNN`), of the manifest of other documents, or a committed
 * description, and a section. */
export function checkSources(file: string, rows: readonly Coverage[], documents: ReadonlySet<string>): string[] {
  const findings: string[] = [];
  for (const row of rows) {
    for (const source of row.sources) {
      const position = source.indexOf(SECTION_SEPARATOR);
      const document = position < 0 ? source : source.slice(0, position);
      if (position < 0 || source.slice(position + SECTION_SEPARATOR.length) === EMPTY) {
        findings.push(`${file}: ${row.key}: ${MALFORMED_SOURCE} ${source}`);
      } else if (!documents.has(document)) {
        findings.push(`${file}: ${row.key}: ${UNKNOWN_SOURCE} ${document}`);
      }
    }
  }
  return findings;
}

/** The documents that a source may name: `rfcNNNN` for each RFC of the manifest, each identifier of the manifest of
 * other documents, and the file name of each committed description of a paid standard (`<identifier>.md`). */
function knownDocuments(root: string): Set<string> {
  const documents = new Set<string>();
  const rfcFile = join(...RFC_DIRECTORY, RFCS_FILE);
  if (existsSync(join(root, rfcFile))) {
    for (const entry of readManifest(rfcFile, readFileSync(join(root, rfcFile), TEXT_ENCODING)).entries) {
      documents.add(`${RFC_SOURCE_PREFIX}${entry.number}`);
    }
  }
  const documentsFile = join(...SPECIFICATIONS_DIRECTORY, DOCUMENTS_FILE);
  if (existsSync(join(root, documentsFile))) {
    for (const entry of readDocuments(documentsFile, readFileSync(join(root, documentsFile), TEXT_ENCODING)).entries) {
      documents.add(entry.identifier);
    }
  }
  const descriptions = join(root, ...DESCRIPTIONS_DIRECTORY);
  if (existsSync(descriptions)) {
    for (const name of readdirSync(descriptions)) {
      if (name.endsWith(MARKDOWN_EXTENSION) && name !== DESCRIPTIONS_INDEX) {
        documents.add(name);
      }
    }
  }
  return documents;
}

/** The words of a deliverable as the tables of section 4 give them. */
function signature(...words: string[]): string {
  return words.join(SIGNATURE_SEPARATOR);
}
