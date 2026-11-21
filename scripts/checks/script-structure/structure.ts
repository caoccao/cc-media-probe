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

// The structure check of the TypeScript scripts (R34): the copyright header, the line limit and the literal rule.

import { readFileSync } from 'node:fs';
import { join } from 'node:path';

import { LINE_BREAK_PATTERN, TEXT_ENCODING, TYPESCRIPT_EXTENSION } from '../constants.ts';
import {
  COPYRIGHT_LINE,
  COPYRIGHT_PATTERN,
  DIFFERENT_HEADER_LINE,
  EMPTY,
  FIRST_YEAR_GROUP,
  HEADER_BODY,
  HEADER_BODY_FIRST_LINE,
  HEADER_OPENING,
  HEADER_OPENING_LINE,
  LAST_YEAR_GROUP,
  LINE_FEED,
  MALFORMED_COPYRIGHT,
  MAXIMUM_LINES,
  MISSING_HEADER,
  TOO_MANY_LINES,
} from './constants.ts';
import { checkLiterals } from './literals.ts';
import { tokenize } from './tokenizer.ts';

/** The header finding of one file, if any. */
export function checkHeader(file: string, source: string): string | undefined {
  const lines = source.split(LINE_BREAK_PATTERN);
  if (lines[0] !== HEADER_OPENING) {
    return `${file}:${HEADER_OPENING_LINE}: ${MISSING_HEADER}`;
  }
  const years = COPYRIGHT_PATTERN.exec(lines[COPYRIGHT_LINE - 1] ?? EMPTY);
  const first = Number(years?.[FIRST_YEAR_GROUP]);
  const lastText = years?.[LAST_YEAR_GROUP];
  const last = lastText === undefined ? undefined : Number(lastText);
  if (years === null || (last !== undefined && last <= first)) {
    return `${file}:${COPYRIGHT_LINE}: ${MALFORMED_COPYRIGHT}`;
  }
  const offset = HEADER_BODY.findIndex((expected, index) => lines[HEADER_BODY_FIRST_LINE - 1 + index] !== expected);
  return offset < 0 ? undefined : `${file}:${HEADER_BODY_FIRST_LINE + offset}: ${DIFFERENT_HEADER_LINE}`;
}

/** The findings of one script and its source. */
export function checkScript(file: string, source: string): string[] {
  const findings: string[] = [];
  const header = checkHeader(file, source);
  if (header !== undefined) {
    findings.push(header);
  }
  const lineCount = source.split(LINE_BREAK_PATTERN).length - (source.endsWith(LINE_FEED) ? 1 : 0);
  if (lineCount > MAXIMUM_LINES) {
    findings.push(`${file}:${MAXIMUM_LINES + 1}: ${TOO_MANY_LINES} (${lineCount} > ${MAXIMUM_LINES})`);
  }
  findings.push(...checkLiterals(file, tokenize(source)));
  return findings;
}

/** The findings of every TypeScript file among `files`. */
export function checkScriptStructure(root: string, files: readonly string[]): string[] {
  return files
    .filter((file) => file.endsWith(TYPESCRIPT_EXTENSION))
    .flatMap((file) => checkScript(file, readFileSync(join(root, file), TEXT_ENCODING)));
}
