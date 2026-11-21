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

// Tests of reading the deliverables and coverage tables.

import assert from 'node:assert/strict';
import { test } from 'node:test';

import { readCoverage, readDeliverables } from './tables.ts';

const DELIVERABLES = 'key\tkind\tsection\tsubject\titem\tcarriage\tphase\n';
const COVERAGE = 'key\tstatus\trelease\tgate\tsources\tfixture\tconfirmed\n';

test('reads well-formed rows', () => {
  const row = 'probe.webm\tprobe\t4.1\tMatroska\tWebM (`webm`)\t\t5\n';
  const deliverables = readDeliverables('d.tsv', DELIVERABLES + row);
  const webm = { key: 'probe.webm', kind: 'probe', section: '4.1', subject: 'Matroska', item: 'WebM (`webm`)' };
  assert.deepEqual(deliverables, { rows: [{ ...webm, carriage: '', phase: '5' }], findings: [] });
  const coverage = readCoverage(
    'c.tsv',
    COVERAGE + 'probe.webm\tactive\t0.1.0\t\trfc9559: 5; webm-container-guidelines: Guidelines\t\t2026-11-21\n' +
      'probe.zz\tdeferred\t\tG17.n\t\t\t\n',
  );
  assert.deepEqual(coverage.findings, []);
  assert.deepEqual(coverage.rows[0]?.sources, ['rfc9559: 5', 'webm-container-guidelines: Guidelines']);
});

test('reports malformed keys, kinds, order, duplicates, statuses, gates, releases, sources and dates', () => {
  const deliverables = readDeliverables(
    'd.tsv',
    DELIVERABLES +
      'probe.webm\tdetect\t4.1\t\t\t\t\nProbe Webm\tprobe\t\t\t\t\t\n' +
      'probe.aa\tmagic\t\t\t\t\t\nprobe.aa\tprobe\t\t\t\t\t\n',
  );
  assert.deepEqual(deliverables.findings, [
    'd.tsv:2: the key does not start with its kind',
    'd.tsv:3: not a key of a kind and lower-case words joined by dots or hyphens: Probe Webm',
    'd.tsv:3: rows are not in ascending order of key',
    'd.tsv:3: the key does not start with its kind',
    'd.tsv:4: magic is not one of detect, probe, pair, identify, tags, registry, rule',
    'd.tsv: key listed twice: probe.aa',
  ]);
  const coverage = readCoverage(
    'c.tsv',
    COVERAGE + 'probe.a\tactive\t\tG1.5\t\t\tsoon\nprobe.b\tdeferred\t0.2.0\t\t\t\t\nprobe.c\tplanned\t\t\t\t\t\n',
  );
  assert.deepEqual(coverage.findings, [
    'c.tsv:2: an active row names its release and no gate; a deferred or dropped row names its gate and no release',
    'c.tsv:2: an active row names at least one source',
    'c.tsv:2: the confirmation date is not written as YYYY-MM-DD',
    'c.tsv:3: an active row names its release and no gate; a deferred or dropped row names its gate and no release',
    'c.tsv:4: planned is not one of active, deferred, dropped',
    'c.tsv:4: an active row names its release and no gate; a deferred or dropped row names its gate and no release',
  ]);
});
