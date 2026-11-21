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

// Tab-separated tables (plan section 5.2): a header row that names the columns, then one row per record. A cell
// holds no tab and no line break, and every row ends with a line feed.

import { CELL_SEPARATOR, EMPTY, HEADER_DIFFERS, ROW_END, WRONG_CELL_COUNT } from './constants.ts';

/** One row of a table: its cells in column order, and its line number. */
export interface TableRow {
  readonly line: number;
  readonly cells: readonly string[];
}

/** A parsed table: the rows that have one cell per column, and findings for the rest. */
export interface ParsedTable {
  readonly rows: readonly TableRow[];
  readonly findings: readonly string[];
}

/** Parses `text`, whose header row must name exactly `columns`; findings name `file` and the line. */
export function parseTable(file: string, text: string, columns: readonly string[]): ParsedTable {
  const lines = text.split(ROW_END);
  if (lines.at(-1) === EMPTY) {
    lines.pop();
  }
  const [header, ...body] = lines;
  const expectedHeader = columns.join(CELL_SEPARATOR);
  if (header !== expectedHeader) {
    return { rows: [], findings: [`${file}:${1}: ${HEADER_DIFFERS} ${expectedHeader}`] };
  }
  const rows: TableRow[] = [];
  const findings: string[] = [];
  body.forEach((line, offset) => {
    const lineNumber = offset + 1 + 1;
    const cells = line.split(CELL_SEPARATOR);
    if (cells.length === columns.length) {
      rows.push({ line: lineNumber, cells });
    } else {
      findings.push(`${file}:${lineNumber}: ${WRONG_CELL_COUNT}`);
    }
  });
  return { rows, findings };
}

/** Formats a table: the header row, then each row's cells in column order. */
export function formatTable(columns: readonly string[], rows: readonly (readonly string[])[]): string {
  return [columns, ...rows].map((cells) => cells.join(CELL_SEPARATOR) + ROW_END).join(EMPTY);
}

/** Orders text by its UTF-16 code units, the same on every platform and in every locale. */
export function compareText(first: string, second: string): number {
  if (first === second) {
    return 0;
  }
  return first < second ? -1 : 1;
}

/** The cell at `index`, or nothing when the row is shorter. */
export function cellAt(row: TableRow, index: number): string {
  return row.cells[index] ?? EMPTY;
}
