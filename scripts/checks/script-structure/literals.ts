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

// The literal rule for scripts (R34, after R33): numbers other than 0 and 1, strings and regular expressions live in
// constants modules; a template literal outside one holds no text; a constants module holds only imports and
// constants. Test files are exempt, and so are module specifiers.

import { posix } from 'node:path';

import {
  CONSTANTS_DIRECTORY,
  CONSTANTS_FILE,
  EMPTY,
  FROM_KEYWORD,
  IDENTIFIER_TOKEN,
  IMPORT_KEYWORD,
  LITERAL_OUTSIDE_CONSTANTS,
  NOT_A_CONSTANT,
  NUMBER_DECORATION_PATTERN,
  NUMBER_TOKEN,
  OPENING_PARENTHESIS,
  REGULAR_EXPRESSION_TOKEN,
  SPACING_ESCAPE_PATTERN,
  STRING_TOKEN,
  TEMPLATE_DELIMITERS_PATTERN,
  TEMPLATE_TOKEN,
  TEMPLATE_WITH_WORDS,
  TEST_FILE_SUFFIX,
  WORD_CHARACTER_PATTERN,
} from './constants.ts';
import { topLevelStatements } from './statements.ts';
import type { Token } from './tokenizer.ts';

/** Whether `file` is named as a constants module. */
export function isNamedConstantsModule(file: string): boolean {
  return posix.basename(file) === CONSTANTS_FILE || posix.dirname(file).split(posix.sep).includes(CONSTANTS_DIRECTORY);
}

/** The literal findings of one script. */
export function checkLiterals(file: string, tokens: readonly Token[]): string[] {
  if (file.endsWith(TEST_FILE_SUFFIX)) {
    return [];
  }
  const statements = topLevelStatements(tokens);
  if (isNamedConstantsModule(file)) {
    return statements
      .filter((statement) => !statement.isImport && !statement.isConstant)
      .map((statement) => `${file}:${statement.first.line}: ${NOT_A_CONSTANT}`);
  }
  const holdsOnlyConstants =
    statements.some((statement) => statement.isConstant) &&
    statements.every((statement) => statement.isImport || statement.isConstant);
  if (holdsOnlyConstants) {
    return [];
  }
  const findings: string[] = [];
  tokens.forEach((token, index) => {
    const message = literalProblem(tokens, index, token);
    if (message !== undefined) {
      findings.push(`${file}:${token.line}: ${message}`);
    }
  });
  return findings;
}

function literalProblem(tokens: readonly Token[], index: number, token: Token): string | undefined {
  switch (token.kind) {
    case NUMBER_TOKEN: {
      const value = Number(token.text.replace(NUMBER_DECORATION_PATTERN, EMPTY));
      return value === 0 || value === 1 ? undefined : `${LITERAL_OUTSIDE_CONSTANTS}: ${token.text}`;
    }
    case STRING_TOKEN:
      return isModuleSpecifier(tokens, index) ? undefined : `${LITERAL_OUTSIDE_CONSTANTS}: ${token.text}`;
    case REGULAR_EXPRESSION_TOKEN:
      return `${LITERAL_OUTSIDE_CONSTANTS}: ${token.text}`;
    case TEMPLATE_TOKEN: {
      const text = token.text.replace(TEMPLATE_DELIMITERS_PATTERN, EMPTY).replace(SPACING_ESCAPE_PATTERN, EMPTY);
      return WORD_CHARACTER_PATTERN.test(text) ? TEMPLATE_WITH_WORDS : undefined;
    }
    default:
      return undefined;
  }
}

/** Whether the string at `index` names a module: after `from`, after `import`, or in `import(…)`. */
function isModuleSpecifier(tokens: readonly Token[], index: number): boolean {
  const previous = tokens[index - 1];
  const beforePrevious = tokens[index - 1 - 1];
  const isWord = (token: Token | undefined, word: string): boolean =>
    token?.kind === IDENTIFIER_TOKEN && token.text === word;
  return (
    isWord(previous, FROM_KEYWORD) ||
    isWord(previous, IMPORT_KEYWORD) ||
    (previous?.text === OPENING_PARENTHESIS && isWord(beforePrevious, IMPORT_KEYWORD))
  );
}
