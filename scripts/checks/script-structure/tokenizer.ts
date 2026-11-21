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

// A tokenizer for the repository's TypeScript: enough to find literals, template parts and top-level statements.
// It skips comments and whitespace; every other character is part of a token.

import {
  BACKSLASH,
  BACKTICK,
  BLOCK_COMMENT_END,
  CLOSING_BRACE,
  CLOSING_BRACKET,
  DECIMAL_POINT,
  DIGIT_PATTERN,
  DOLLAR_SIGN,
  EXPONENT_PATTERN,
  FLAGS_PATTERN,
  IDENTIFIER_PART_PATTERN,
  IDENTIFIER_START_PATTERN,
  IDENTIFIER_TOKEN,
  LINE_FEED,
  NUMBER_PART_PATTERN,
  NUMBER_TOKEN,
  OPENING_BRACE,
  OPENING_BRACKET,
  OPERAND_ENDINGS,
  PUNCTUATION_TOKEN,
  QUOTES,
  REGULAR_EXPRESSION_KEYWORDS,
  REGULAR_EXPRESSION_TOKEN,
  SIGNS,
  SLASH,
  ASTERISK,
  STRING_TOKEN,
  TEMPLATE_TOKEN,
  WHITESPACE_PATTERN,
} from './constants.ts';

/** One token and the line it starts on. */
export interface Token {
  readonly kind: string;
  readonly text: string;
  readonly line: number;
}

/** Reads `source` into tokens. */
export function tokenize(source: string): Token[] {
  const tokens: Token[] = [];
  const substitutionDepths: number[] = [];
  let braceDepth = 0;
  let position = 0;
  let line = 1;
  const push = (kind: string, end: number): void => {
    const text = source.slice(position, end);
    tokens.push({ kind, text, line });
    line += text.split(LINE_FEED).length - 1;
    position = end;
  };
  while (position < source.length) {
    const character = source.charAt(position);
    const next = source.charAt(position + 1);
    if (WHITESPACE_PATTERN.test(character)) {
      line += character === LINE_FEED ? 1 : 0;
      position += 1;
    } else if (character === SLASH && next === SLASH) {
      const end = source.indexOf(LINE_FEED, position);
      position = end < 0 ? source.length : end;
    } else if (character === SLASH && next === ASTERISK) {
      const end = source.indexOf(BLOCK_COMMENT_END, position + 1 + 1);
      const stop = end < 0 ? source.length : end + BLOCK_COMMENT_END.length;
      line += source.slice(position, stop).split(LINE_FEED).length - 1;
      position = stop;
    } else if (character === BACKTICK || (character === CLOSING_BRACE && substitutionDepths.at(-1) === braceDepth)) {
      if (character === CLOSING_BRACE) {
        substitutionDepths.pop();
      }
      const { end, opensSubstitution } = templatePartEnd(source, position + 1);
      if (opensSubstitution) {
        substitutionDepths.push(braceDepth);
      }
      push(TEMPLATE_TOKEN, end);
    } else if (IDENTIFIER_START_PATTERN.test(character)) {
      push(IDENTIFIER_TOKEN, scanWhile(source, position + 1, (part) => IDENTIFIER_PART_PATTERN.test(part)));
    } else if (DIGIT_PATTERN.test(character) || (character === DECIMAL_POINT && DIGIT_PATTERN.test(next))) {
      push(NUMBER_TOKEN, numberEnd(source, position));
    } else if (QUOTES.includes(character)) {
      push(STRING_TOKEN, quotedEnd(source, position));
    } else if (character === SLASH && startsRegularExpression(tokens.at(-1))) {
      push(REGULAR_EXPRESSION_TOKEN, regularExpressionEnd(source, position));
    } else {
      braceDepth += character === OPENING_BRACE ? 1 : 0;
      braceDepth -= character === CLOSING_BRACE ? 1 : 0;
      push(PUNCTUATION_TOKEN, position + 1);
    }
  }
  return tokens;
}

function scanWhile(source: string, start: number, accepts: (character: string) => boolean): number {
  let end = start;
  while (end < source.length && accepts(source.charAt(end))) {
    end += 1;
  }
  return end;
}

/** The end of a template part that starts after a backtick or a closing brace, and whether it opens `${`. */
function templatePartEnd(source: string, start: number): { end: number; opensSubstitution: boolean } {
  let end = start;
  while (end < source.length) {
    const character = source.charAt(end);
    if (character === BACKSLASH) {
      end += 1 + 1;
    } else if (character === BACKTICK) {
      return { end: end + 1, opensSubstitution: false };
    } else if (character === DOLLAR_SIGN && source.charAt(end + 1) === OPENING_BRACE) {
      return { end: end + 1 + 1, opensSubstitution: true };
    } else {
      end += 1;
    }
  }
  return { end: source.length, opensSubstitution: false };
}

function numberEnd(source: string, start: number): number {
  let end = start + 1;
  while (end < source.length) {
    const character = source.charAt(end);
    const continuesExponent = SIGNS.includes(character) && EXPONENT_PATTERN.test(source.slice(start, end));
    if (!NUMBER_PART_PATTERN.test(character) && !continuesExponent) {
      break;
    }
    end += 1;
  }
  return end;
}

function quotedEnd(source: string, start: number): number {
  const quote = source.charAt(start);
  let end = start + 1;
  while (end < source.length && source.charAt(end) !== quote && source.charAt(end) !== LINE_FEED) {
    end += source.charAt(end) === BACKSLASH ? 1 + 1 : 1;
  }
  return Math.min(end + 1, source.length);
}

/** Whether a slash after `previous` starts a regular expression rather than dividing. */
function startsRegularExpression(previous: Token | undefined): boolean {
  if (previous === undefined) {
    return true;
  }
  if (previous.kind === PUNCTUATION_TOKEN) {
    return !OPERAND_ENDINGS.includes(previous.text);
  }
  return previous.kind === IDENTIFIER_TOKEN && REGULAR_EXPRESSION_KEYWORDS.includes(previous.text);
}

function regularExpressionEnd(source: string, start: number): number {
  let end = start + 1;
  let inClass = false;
  while (end < source.length && source.charAt(end) !== LINE_FEED) {
    const character = source.charAt(end);
    if (character === BACKSLASH) {
      end += 1;
    } else if (character === OPENING_BRACKET) {
      inClass = true;
    } else if (character === CLOSING_BRACKET) {
      inClass = false;
    } else if (character === SLASH && !inClass) {
      return scanWhile(source, end + 1, (flag) => FLAGS_PATTERN.test(flag));
    }
    end += 1;
  }
  return end;
}
