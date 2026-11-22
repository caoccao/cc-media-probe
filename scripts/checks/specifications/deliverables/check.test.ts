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

// Tests of the comparisons of the deliverables with section 4, with the coverage table and with the manifests.

import assert from 'node:assert/strict';
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { test } from 'node:test';

import { checkApproval, checkDeliverables, checkSources, compareKeys, compareWithPlan } from './check.ts';
import { PLAN } from './plan-tables.test.ts';
import { plannedTables } from './plan-tables.ts';
import type { Coverage, Deliverable } from './tables.ts';

/** A deliverable from its cells. */
function deliverable(...cells: [string, string, string, string, string, string, string]): Deliverable {
  const [key, kind, section, subject, item, carriage, phase] = cells;
  return { key, kind, section, subject, item, carriage, phase };
}

const CONFIGURATION = 'Decoder configuration record';
const DELIVERED: Deliverable[] = [
  deliverable(
    'detect.matroska',
    'detect', '4.4', 'Matroska', 'Matroska (`mkv`, `mka`)', '', '',
  ),
  deliverable(
    'detect.microdvd',
    'detect', '4.4', 'Detect-only', 'MicroDVD†', '', '',
  ),
  deliverable(
    'detect.webm',
    'detect', '4.4', 'Matroska', 'WebM (`webm`)', '', '',
  ),
  deliverable(
    'identify.vobbuttons.matroska',
    'identify', '4.3', 'VobButtons', '', 'Matroska', '5',
  ),
  deliverable(
    'pair.h264.configuration-record.flv-sequence-header',
    'pair', '4.3', 'H.264/AVC', CONFIGURATION, 'FLV sequence header', '13',
  ),
  deliverable(
    'pair.h264.configuration-record.matroska-private-data',
    'pair', '4.3', 'H.264/AVC', CONFIGURATION, 'Matroska private data', '5',
  ),
  deliverable(
    'pair.theora.identification-header.matroska-laced-private-data',
    'pair', '4.3', 'Theora', 'Identification header', 'Matroska laced private data', 'retrofit in 9',
  ),
  deliverable(
    'pair.theora.identification-header.ogg-packets',
    'pair', '4.3', 'Theora', 'Identification header', 'Ogg packets', '9',
  ),
  deliverable(
    'probe.matroska',
    'probe', '4.1', 'Matroska', 'Matroska (`mkv`, `mka`)', '', '5',
  ),
  deliverable(
    'probe.webm',
    'probe', '4.1', 'Matroska', 'WebM (`webm`)', '', '5',
  ),
  deliverable(
    'tags.matroska',
    'tags', '8.5', 'Matroska tags', '', '', '5',
  ),
];

function covered(key: string, sources: string[] = ['rfc9559: 5']): Coverage {
  return { key, status: 'active', release: '0.1.0', gate: '', sources, fixture: '', confirmed: '' };
}

test('matches the deliverables with the plan in both directions', () => {
  const tables = plannedTables(PLAN);
  assert.deepEqual(compareWithPlan('d.tsv', DELIVERED, tables), []);
  const changed = DELIVERED.filter((row) => row.key !== 'probe.webm').map((row) =>
    row.key === 'detect.microdvd' ? { ...row, item: 'MicroDVD' } : row,
  );
  assert.deepEqual(compareWithPlan('d.tsv', changed, tables), [
    'd.tsv: detect.microdvd: no row of section 4 has the words of this deliverable',
    'd.tsv: section 4 lists this, but no deliverable has its words: probe | 4.1 | Matroska | WebM (`webm`) |  | 5',
    'd.tsv: section 4 lists this, but no deliverable has its words: detect | 4.4 | Detect-only | MicroDVD† |  | ',
  ]);
});

test('matches the keys of both tables, and the sources with the documents', () => {
  const coverage = DELIVERED.filter((row) => row.key !== 'tags.matroska').map((row) => covered(row.key));
  assert.deepEqual(compareKeys('d.tsv', 'c.tsv', DELIVERED, [...coverage, covered('probe.extra')]), [
    'd.tsv: tags.matroska: deliverable without a coverage row',
    'c.tsv: probe.extra: coverage row without a deliverable',
  ]);
  const documents = new Set(['rfc9559', 'webm-container-guidelines']);
  const rows = [
    covered('probe.a', ['rfc9559: 5', 'webm-container-guidelines: Guidelines']),
    covered('probe.b', ['rfc1: 2', 'rfc9559', 'rfc9559: ']),
  ];
  assert.deepEqual(checkSources('c.tsv', rows, documents), [
    'c.tsv: probe.b: source names no document of the manifests: rfc1',
    'c.tsv: probe.b: a source is a document identifier, then a colon and the section: rfc9559',
    'c.tsv: probe.b: a source is a document identifier, then a colon and the section: rfc9559: ',
  ]);
});

test('requires an approved source for each active row, and none for a deferred one', () => {
  const approved = new Set(['rfc9559', 'iso-iec-14496-15.md']);
  const rows = [
    covered('probe.a', ['candidate-description: 2', 'rfc9559: 5']),
    covered('probe.b', ['candidate-description: 2']),
    covered('probe.c', ['iso-iec-14496-15.md: 2']),
    { ...covered('probe.d', []), status: 'deferred', release: '', gate: 'G17.n' },
  ];
  assert.deepEqual(checkApproval('c.tsv', rows, approved), [
    'c.tsv: probe.b: an active row names at least one approved source',
  ]);
});

test('reports missing tables, and checks a repository whose tables agree', () => {
  const root = mkdtempSync(join(tmpdir(), 'deliverables-'));
  mkdirSync(join(root, 'docs', 'plans'), { recursive: true });
  mkdirSync(join(root, 'docs', 'specs'));
  writeFileSync(join(root, 'docs', 'plans', 'plan-init-project.md'), PLAN);
  assert.deepEqual(checkDeliverables(root), [
    `${join('docs', 'specs', 'deliverables.tsv')}: the table is missing`,
    `${join('docs', 'specs', 'coverage.tsv')}: the table is missing`,
  ]);
  const table = (header: string, rows: string[][]): string =>
    [header, ...rows.map((cells) => cells.join('\t'))].join('\n') + '\n';
  const deliverables = DELIVERED.map((row) => [
    row.key,
    row.kind,
    row.section,
    row.subject,
    row.item,
    row.carriage,
    row.phase,
  ]);
  const coverage = DELIVERED.map((row) => [row.key, 'deferred', '', 'G1.5', '', '', '']);
  const specifications = join(root, 'docs', 'specs');
  const deliverablesHeader = 'key\tkind\tsection\tsubject\titem\tcarriage\tphase';
  const coverageHeader = 'key\tstatus\trelease\tgate\tsources\tfixture\tconfirmed';
  writeFileSync(join(specifications, 'deliverables.tsv'), table(deliverablesHeader, deliverables));
  writeFileSync(join(specifications, 'coverage.tsv'), table(coverageHeader, coverage));
  assert.deepEqual(checkDeliverables(root), []);
  rmSync(root, { recursive: true });
});

test('accepts a committed description of a paid standard as a source, but not its index', () => {
  const root = mkdtempSync(join(tmpdir(), 'deliverables-'));
  const specifications = join(root, 'docs', 'specs');
  mkdirSync(join(root, 'docs', 'plans'), { recursive: true });
  mkdirSync(join(specifications, 'iso-iec'), { recursive: true });
  writeFileSync(join(root, 'docs', 'plans', 'plan-init-project.md'), PLAN);
  writeFileSync(join(specifications, 'iso-iec', 'iso-iec-14496-15.md'), '# Description\n');
  writeFileSync(join(specifications, 'iso-iec', 'README.md'), '# Index\n');
  const cells = (row: Deliverable): string =>
    [row.key, row.kind, row.section, row.subject, row.item, row.carriage, row.phase].join('\t');
  writeFileSync(
    join(specifications, 'deliverables.tsv'),
    ['key\tkind\tsection\tsubject\titem\tcarriage\tphase', ...DELIVERED.map(cells)].join('\n') + '\n',
  );
  const coverage = DELIVERED.map((row, index) =>
    [row.key, 'active', '0.1.0', '', index === 0 ? 'README.md: 1' : 'iso-iec-14496-15.md: 2', '', '2026-11-21'].join(
      '\t',
    ),
  );
  writeFileSync(
    join(specifications, 'coverage.tsv'),
    ['key\tstatus\trelease\tgate\tsources\tfixture\tconfirmed', ...coverage].join('\n') + '\n',
  );
  assert.deepEqual(checkDeliverables(root), [
    `${join('docs', 'specs', 'coverage.tsv')}: detect.matroska: source names no document of the manifests: README.md`,
    `${join('docs', 'specs', 'coverage.tsv')}: detect.matroska: an active row names at least one approved source`,
  ]);
  rmSync(root, { recursive: true });
});
