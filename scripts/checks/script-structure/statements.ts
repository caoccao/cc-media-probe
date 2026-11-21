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

// The top-level statements of a script, as far as the constants rule needs them: imports, constants and others.

import {
  CLOSING_DELIMITERS,
  CONSTANT_KEYWORD,
  EXPORT_KEYWORD,
  IDENTIFIER_TOKEN,
  IMPORT_KEYWORD,
  OPENING_DELIMITERS,
  PUNCTUATION_TOKEN,
  SEMICOLON,
} from './constants.ts';
import type { Token } from './tokenizer.ts';

/** A top-level statement: its first token and whether it is an import, a constant, or something else. */
export interface Statement {
  readonly first: Token;
  readonly isImport: boolean;
  readonly isConstant: boolean;
}

/** The top-level statements of a token sequence. A statement ends at a `;` outside every group, or at a closing
 * brace outside every group that the next token follows on a later line, as after a function declaration. */
export function topLevelStatements(tokens: readonly Token[]): Statement[] {
  const statements: Statement[] = [];
  let depth = 0;
  let expectsStart = true;
  tokens.forEach((token, index) => {
    if (expectsStart && depth === 0) {
      const next = tokens[index + 1];
      statements.push({
        first: token,
        isImport: isWord(token, IMPORT_KEYWORD),
        isConstant: isWord(token, CONSTANT_KEYWORD) || (isWord(token, EXPORT_KEYWORD) && isWord(next, CONSTANT_KEYWORD)),
      });
      expectsStart = false;
    }
    if (token.kind !== PUNCTUATION_TOKEN) {
      return;
    }
    if (OPENING_DELIMITERS.includes(token.text)) {
      depth += 1;
    } else if (CLOSING_DELIMITERS.includes(token.text)) {
      depth -= 1;
      const following = tokens[index + 1];
      expectsStart = depth === 0 && following !== undefined && following.line > token.line;
    } else if (token.text === SEMICOLON && depth === 0) {
      expectsStart = true;
    }
  });
  return statements;
}

function isWord(token: Token | undefined, word: string): boolean {
  return token?.kind === IDENTIFIER_TOKEN && token.text === word;
}
