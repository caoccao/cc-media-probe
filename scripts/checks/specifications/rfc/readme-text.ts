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

// The fixed text of the generated `docs/rfc/README.md`: what precedes the index, and the license notice after it.

/** The lines before the index. */
export const README_OPENING = [
  '# RFCs',
  '',
  'The RFCs that this project builds on, as Appendix A of the ' +
    '[development plan](../plans/plan-init-project.md#appendix-a--rfc-list) lists them. Each `rfcNNNN.txt` is the ' +
    'RFC Editor\'s plain-text copy, byte for byte. [manifest.tsv](manifest.tsv) records where each one came from, ' +
    'its SHA-256 and the day it was retrieved, and `node scripts/check.ts` verifies every file against it.',
  '',
  '[errata.tsv](errata.tsv) lists every verified erratum of these RFCs, and every erratum held for document update ' +
    'that affects parsing, each with its relevance to this project and the components it affects (plan section ' +
    '5.3). Code and fixtures that depend on a corrected rule cite the erratum.',
  '',
  'This file is generated from the manifest by `node scripts/fetch-rfcs.ts`; do not edit it by hand. The same ' +
    'script downloads a missing RFC, and reports what changed at the RFC Editor: an RFC that was obsoleted, a title ' +
    'or status that changed, an updating RFC that Appendix A does not list, and a new erratum.',
  '',
  '## Index',
  '',
  '| RFC | Title | Status | Role |',
  '|---|---|---|---|',
];

/** The lines after the index. */
export const README_CLOSING = [
  '',
  '## License notice',
  '',
  'The RFC texts in this directory are not covered by the project\'s Apache-2.0 license. No file here is modified, ' +
    'so each keeps its own copyright notice and legal text as published.',
  '',
  '- RFCs published from November 10, 2008 on are copyright the IETF Trust and the persons identified as their ' +
    'authors. They are distributed here under the ' +
    '[IETF Trust Legal Provisions](https://trustee.ietf.org/license-info), which permit anyone to copy, publish, ' +
    'display and distribute them in full and without modification: section 3.c.i for the IETF stream, and section ' +
    '8.c.iii for the Independent Submission, IAB and IRTF streams.',
  '- Earlier RFCs carry the copyright notice of the Internet Society or of the IETF Trust, which permits copying ' +
    'and distributing them unmodified with the notice kept, or state that their distribution is unlimited.',
  '- RFC 1950 and RFC 1951 carry their authors\' copyright, with permission to copy and distribute them for any ' +
    'purpose, provided that the notice is kept and any change is marked.',
];

/** Opens a table row of the index. */
export const ROW_OPENING = '| ';
/** Separates the cells of a row of the index. */
export const CELL_DIVIDER = ' | ';
/** Closes a table row of the index. */
export const ROW_CLOSING = ' |';
