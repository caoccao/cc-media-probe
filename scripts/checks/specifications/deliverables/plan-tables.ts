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

// What the tables of plan section 4 promise, in the plan's own words: each format of the table in section 4.1, each
// pair of a row and one of its carriages in the checklist of section 4.3, and each carriage of a codec that the
// checklist identifies only. The deliverables table is compared with this list.

import { EMPTY, ROW_END } from '../constants.ts';
import {
  BACKTICK,
  CARRIAGE_GROUP,
  CARRIAGE_SEPARATOR,
  CHECKLIST_HEADING,
  CLOSING_PARENTHESIS,
  DELIMITER_ROW_PATTERN,
  DETECTION_HEADING,
  FORMATS_HEADER,
  FORMATS_HEADING,
  IDENTIFIED_HEADER,
  ITEM_SEPARATOR,
  OPENING_PARENTHESIS,
  PAIRS_HEADER,
  PHASE_GROUP,
  PHASED_PATTERN,
  TABLE_CELL_SEPARATOR,
  TABLE_ROW_START,
} from './constants.ts';

/** A format of the table of section 4.1, with its family, phase and release. */
export interface PlannedFormat {
  readonly family: string;
  readonly format: string;
  readonly phase: string;
  readonly release: string;
}

/** A pair of the checklist, or a carriage of a codec that is identified only. */
export interface PlannedCarriage {
  readonly codec: string;
  readonly representation: string;
  readonly carriage: string;
  readonly phase: string;
}

/** Everything the tables of section 4 list. */
export interface PlannedTables {
  readonly formats: readonly PlannedFormat[];
  readonly pairs: readonly PlannedCarriage[];
  readonly identified: readonly PlannedCarriage[];
}

/** The tables of section 4 of `plan`. */
export function plannedTables(plan: string): PlannedTables {
  const lines = plan.split(ROW_END);
  const formats = tableRows(section(lines, FORMATS_HEADING, CHECKLIST_HEADING), FORMATS_HEADER).flatMap(
    ([family = EMPTY, cell = EMPTY, phase = EMPTY, release = EMPTY]) =>
      splitOutsideGroups(cell, ITEM_SEPARATOR).map((format) => ({ family, format, phase, release })),
  );
  const checklist = section(lines, CHECKLIST_HEADING, DETECTION_HEADING);
  const pairs = tableRows(checklist, PAIRS_HEADER).flatMap(([codec = EMPTY, representation = EMPTY, , cell = EMPTY]) =>
    splitOutsideGroups(cell, CARRIAGE_SEPARATOR).map((carriage) => ({ codec, representation, ...phased(carriage) })),
  );
  const identified = tableRows(checklist, IDENTIFIED_HEADER).flatMap(([codec = EMPTY, , cell = EMPTY]) =>
    splitOutsideGroups(cell, ITEM_SEPARATOR).map((carriage) => ({ codec, representation: EMPTY, ...phased(carriage) })),
  );
  return { formats, pairs, identified };
}

/** The lines from the heading that starts with `start` to the one that starts with `end`. */
function section(lines: readonly string[], start: string, end: string): string[] {
  const first = lines.findIndex((line) => line.startsWith(start));
  if (first < 0) {
    return [];
  }
  const length = lines.slice(first + 1).findIndex((line) => line.startsWith(end));
  return lines.slice(first + 1, length < 0 ? lines.length : first + 1 + length);
}

/** The cells of the rows of every table in `lines` whose header row is `header`. */
function tableRows(lines: readonly string[], header: string): string[][] {
  const rows: string[][] = [];
  let inside = false;
  for (const line of lines) {
    if (line === header) {
      inside = true;
    } else if (!line.startsWith(TABLE_ROW_START)) {
      inside = false;
    } else if (inside && !DELIMITER_ROW_PATTERN.test(line)) {
      rows.push(line.split(TABLE_CELL_SEPARATOR).slice(1, -1).map((cell) => cell.trim()));
    }
  }
  return rows;
}

/** A carriage and the phase in parentheses at its end. */
function phased(text: string): { carriage: string; phase: string } {
  const match = PHASED_PATTERN.exec(text);
  if (match === null) {
    return { carriage: text, phase: EMPTY };
  }
  return { carriage: match[CARRIAGE_GROUP] ?? EMPTY, phase: match[PHASE_GROUP] ?? EMPTY };
}

/** Splits `text` at `separator`, except inside parentheses and code spans. */
export function splitOutsideGroups(text: string, separator: string): string[] {
  const parts: string[] = [];
  let depth = 0;
  let code = false;
  let start = 0;
  for (let index = 0; index < text.length; index += 1) {
    const character = text.charAt(index);
    if (character === BACKTICK) {
      code = !code;
    } else if (!code && character === OPENING_PARENTHESIS) {
      depth += 1;
    } else if (!code && character === CLOSING_PARENTHESIS) {
      depth -= 1;
    } else if (!code && depth === 0 && text.startsWith(separator, index)) {
      parts.push(text.slice(start, index));
      start = index + separator.length;
    }
  }
  parts.push(text.slice(start));
  return parts.map((part) => part.trim()).filter((part) => part !== EMPTY);
}
