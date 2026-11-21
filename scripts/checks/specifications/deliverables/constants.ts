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

// Files, columns, vocabularies and plan anchors of the deliverables and coverage tables (plan section 5.2).

/** The deliverables table, in `docs/specs/`. */
export const DELIVERABLES_FILE = 'deliverables.tsv';
/** The coverage table, in `docs/specs/`. */
export const COVERAGE_FILE = 'coverage.tsv';
/** The columns of the deliverables table. */
export const DELIVERABLE_COLUMNS = ['key', 'kind', 'section', 'subject', 'item', 'carriage', 'phase'];
/** The columns of the coverage table. */
export const COVERAGE_COLUMNS = ['key', 'status', 'release', 'gate', 'sources', 'fixture', 'confirmed'];

/** Detector entries (plan section 4.4). */
export const DETECT = 'detect';
/** Container probers (plan section 4.1). */
export const PROBE = 'probe';
/** Pairs of a codec representation and one carriage (plan section 4.3). */
export const PAIR = 'pair';
/** Codecs identified only, by design, in one carriage (plan section 4.3). */
export const IDENTIFY = 'identify';
/** The kinds that the tables of plan section 4 list, and the kinds kept by hand. */
export const KINDS = [DETECT, PROBE, PAIR, IDENTIFY, 'tags', 'registry', 'rule'];
/** A key: its kind, then lower-case words and numbers joined by dots or hyphens. */
export const KEY_PATTERN = /^([a-z]+)\.[a-z0-9]+(?:[.-][a-z0-9]+)*$/u;
/** The coverage statuses (plan section 5.2). */
export const ACTIVE = 'active';
/** Not scheduled yet; a gate decides. */
export const DEFERRED = 'deferred';
/** Removed by a gate decision. */
export const DROPPED = 'dropped';
/** Every status. */
export const STATUSES = [ACTIVE, DEFERRED, DROPPED];
/** A gate that decides a deferred or dropped row: a numbered gate, or one of the post-1.0 gates G17.n. */
export const COVERAGE_GATE_PATTERN = /^G(?:S|\d+)\.(?:\d+|n)$/u;
/** A release: a version, or a later minor release of the 1.x line. */
export const RELEASE_PATTERN = /^(?:\d+\.\d+\.\d+|1\.x)$/u;
/** Separates the sources of one coverage row. */
export const SOURCE_SEPARATOR = '; ';
/** Separates a source's document from its section. */
export const SECTION_SEPARATOR = ': ';
/** The committed descriptions of the paid standards, relative to the repository root, by component. */
export const DESCRIPTIONS_DIRECTORY = ['docs', 'specs', 'iso-iec'];
/** The index of the descriptions, which is not one. */
export const DESCRIPTIONS_INDEX = 'README.md';
/** Starts the document of an RFC source; the number follows. */
export const RFC_SOURCE_PREFIX = 'rfc';
/** The section of plan section 4 that lists each kind. */
export const FORMATS_SECTION = '4.1';
/** The checklist. */
export const PAIRS_SECTION = '4.3';
/** The detection inventory. */
export const DETECTION_SECTION = '4.4';
/** Joins the words of a deliverable in messages. */
export const SIGNATURE_SEPARATOR = ' | ';

/** Opens the table of formats (plan section 4.1). */
export const FORMATS_HEADING = '### 4.1 ';
/** Opens the checklist (plan section 4.3). */
export const CHECKLIST_HEADING = '### 4.3 ';
/** Opens the section after the checklist, which ends it. */
export const DETECTION_HEADING = '### 4.4 ';
/** The header row of the table of formats. */
export const FORMATS_HEADER = '| Family | Formats (typical extensions) | Phase | Release |';
/** The header row of a checklist table. */
export const PAIRS_HEADER = '| Codec | Representation | Parser phase | Carriages (phase in which the pair ships) |';
/** The header row of the table of codecs identified only. */
export const IDENTIFIED_HEADER = '| Codec | Why there is nothing to parse | Carried in (phase) |';
/** The family whose formats are detected and never probed. */
export const DETECT_ONLY_FAMILY = 'Detect-only';
/** Starts a table row. */
export const TABLE_ROW_START = '|';
/** Separates the cells of a table row. */
export const TABLE_CELL_SEPARATOR = '|';
/** The delimiter row under a table's header. */
export const DELIMITER_ROW_PATTERN = /^\|(?:\s*:?-+:?\s*\|)+$/u;
/** Separates the carriages of a checklist row. */
export const CARRIAGE_SEPARATOR = '; ';
/** Separates the formats of a row of the table of formats, and the carriers of an identified codec. */
export const ITEM_SEPARATOR = ', ';
/** A carriage with its phase in parentheses at the end. */
export const PHASED_PATTERN = /^(.*) \(([^()]+)\)$/u;
/** The capture groups of a phased carriage. */
export const CARRIAGE_GROUP = 1;
/** The phase. */
export const PHASE_GROUP = 2;
/** Opens a parenthesis, inside which no item is split. */
export const OPENING_PARENTHESIS = '(';
/** Closes a parenthesis. */
export const CLOSING_PARENTHESIS = ')';
/** Delimits a code span. */
export const BACKTICK = '`';
