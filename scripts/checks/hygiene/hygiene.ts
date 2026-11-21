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

// File hygiene: TypeScript scripts only, UTF-8, LF line endings, a final newline, no trailing whitespace and no
// tabs outside tab-separated data. Verbatim third-party texts are exempt.

import { readFileSync } from 'node:fs';
import { extname, join } from 'node:path';

import {
  BINARY_FILE,
  CARRIAGE_RETURN,
  CARRIAGE_RETURN_FOUND,
  LINE_FEED,
  NO_FINAL_NEWLINE,
  NOT_UTF8,
  OTHER_SCRIPT_EXTENSIONS,
  OTHER_SCRIPT_LANGUAGE,
  TAB,
  TAB_FOUND,
  TAB_SEPARATED_EXTENSION,
  TRAILING_WHITESPACE,
  TRAILING_WHITESPACE_PATTERN,
  UTF8_LABEL,
  VERBATIM_PATTERNS,
} from './constants.ts';

/** The hygiene findings of every file. */
export function checkHygiene(root: string, files: readonly string[]): string[] {
  return files.flatMap((file) => checkFile(file, readFileSync(join(root, file))));
}

/** The hygiene findings of one file and its content. */
export function checkFile(file: string, content: Uint8Array): string[] {
  if (OTHER_SCRIPT_EXTENSIONS.includes(extname(file))) {
    return [`${file}: ${OTHER_SCRIPT_LANGUAGE}`];
  }
  if (VERBATIM_PATTERNS.some((pattern) => pattern.test(file))) {
    return [];
  }
  if (content.includes(0)) {
    return [`${file}: ${BINARY_FILE}`];
  }
  let text: string;
  try {
    text = new TextDecoder(UTF8_LABEL, { fatal: true }).decode(content);
  } catch {
    return [`${file}: ${NOT_UTF8}`];
  }
  const findings: string[] = [];
  if (text.includes(CARRIAGE_RETURN)) {
    findings.push(`${file}: ${CARRIAGE_RETURN_FOUND}`);
  }
  if (text.length > 0 && !text.endsWith(LINE_FEED)) {
    findings.push(`${file}: ${NO_FINAL_NEWLINE}`);
  }
  const allowsTabs = extname(file) === TAB_SEPARATED_EXTENSION;
  text.split(LINE_FEED).forEach((line, offset) => {
    const location = `${file}:${offset + 1}`;
    if (TRAILING_WHITESPACE_PATTERN.test(line)) {
      findings.push(`${location}: ${TRAILING_WHITESPACE}`);
    }
    if (!allowsTabs && line.includes(TAB)) {
      findings.push(`${location}: ${TAB_FOUND}`);
    }
  });
  return findings;
}
