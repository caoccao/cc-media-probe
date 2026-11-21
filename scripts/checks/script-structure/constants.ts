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

// Token kinds, patterns, the copyright header and messages of the script structure check.

/** An identifier or keyword. */
export const IDENTIFIER_TOKEN = 'identifier';
/** A number. */
export const NUMBER_TOKEN = 'number';
/** A quoted string. */
export const STRING_TOKEN = 'string';
/** One static part of a template literal, with its delimiters. */
export const TEMPLATE_TOKEN = 'template';
/** A regular expression literal. */
export const REGULAR_EXPRESSION_TOKEN = 'regular-expression';
/** A single punctuation character. */
export const PUNCTUATION_TOKEN = 'punctuation';

/** Starts an identifier. */
export const IDENTIFIER_START_PATTERN = /[\p{L}_$]/u;
/** Continues an identifier. */
export const IDENTIFIER_PART_PATTERN = /[\p{L}\p{N}_$]/u;
/** Whitespace between tokens. */
export const WHITESPACE_PATTERN = /\s/u;
/** A decimal digit. */
export const DIGIT_PATTERN = /[0-9]/u;
/** The decimal point, which can start a number such as `.5`. */
export const DECIMAL_POINT = '.';
/** Characters inside a number: digits, radix and exponent letters, separators, the BigInt suffix. */
export const NUMBER_PART_PATTERN = /[0-9A-Za-z_.]/u;
/** An exponent letter, which a sign may follow. */
export const EXPONENT_PATTERN = /[eE]$/u;
/** Characters with a meaning in the tokenizer. */
export const SLASH = '/';
/** The second character of a block comment's delimiters. */
export const ASTERISK = '*';
/** Ends a block comment. */
export const BLOCK_COMMENT_END = '*/';
/** A line feed. */
export const LINE_FEED = '\n';
/** Starts an escape. */
export const BACKSLASH = '\\';
/** Delimits a template literal. */
export const BACKTICK = '`';
/** Starts a template substitution with the following brace. */
export const DOLLAR_SIGN = '$';
/** Opens a block or substitution. */
export const OPENING_BRACE = '{';
/** Closes a block or substitution. */
export const CLOSING_BRACE = '}';
/** Quotation marks of strings. */
export const QUOTES = ['"', '\''];
/** Opens a character class in a regular expression. */
export const OPENING_BRACKET = '[';
/** Closes a character class. */
export const CLOSING_BRACKET = ']';
/** A sign after an exponent letter. */
export const SIGNS = ['+', '-'];
/** Punctuation after which a slash ends an operand, so it divides. */
export const OPERAND_ENDINGS = [')', ']', '}'];
/** Keywords after which a slash starts a regular expression. */
export const REGULAR_EXPRESSION_KEYWORDS = [
  'await', 'case', 'delete', 'do', 'else', 'in', 'instanceof', 'new', 'of', 'return', 'throw', 'typeof', 'void', 'yield',
];
/** Letters that may follow a regular expression. */
export const FLAGS_PATTERN = /[a-z]/u;

/** Opening delimiters. */
export const OPENING_DELIMITERS = ['(', '[', '{'];
/** Closing delimiters. */
export const CLOSING_DELIMITERS = [')', ']', '}'];
/** Ends a statement. */
export const SEMICOLON = ';';
/** Statements that a constants module may hold. */
export const IMPORT_KEYWORD = 'import';
/** A constant declaration. */
export const CONSTANT_KEYWORD = 'const';
/** An export. */
export const EXPORT_KEYWORD = 'export';
/** Introduces a module specifier. */
export const FROM_KEYWORD = 'from';
/** Opens a call's arguments. */
export const OPENING_PARENTHESIS = '(';

/** The file name of a constants module. */
export const CONSTANTS_FILE = 'constants.ts';
/** The directory name of a constants module split by group. */
export const CONSTANTS_DIRECTORY = 'constants';
/** The suffix of test files. */
export const TEST_FILE_SUFFIX = '.test.ts';
/** Template text outside substitutions: anything but a letter or digit is allowed. */
export const WORD_CHARACTER_PATTERN = /[\p{L}\p{N}]/u;
/** Escapes in template text, which stand for spacing rather than letters. */
export const SPACING_ESCAPE_PATTERN = /\\[nrt0\\`$]/gu;
/** The delimiters around a template part. */
export const TEMPLATE_DELIMITERS_PATTERN = /^[`}]|`$|\$\{$/gu;
/** Underscores in numbers, and the BigInt suffix. */
export const NUMBER_DECORATION_PATTERN = /_|n$/gu;
/** Nothing. */
export const EMPTY = '';

/** The most lines a script file may have (R31, R34). */
export const MAXIMUM_LINES = 500;
/** The first line of the copyright header. */
export const HEADER_OPENING = '/*';
/** The copyright line, with the year or year range in groups. */
export const COPYRIGHT_PATTERN = /^\* Copyright \(c\) (\d{4})(?:-(\d{4}))?\. caoccao\.com Sam Cao$/u;
/** The lines after the copyright line. */
export const HEADER_BODY = [
  '* All rights reserved.',
  '',
  '* Licensed under the Apache License, Version 2.0 (the "License");',
  '* you may not use this file except in compliance with the License.',
  '* You may obtain a copy of the License at',
  '',
  '* http://www.apache.org/licenses/LICENSE-2.0',
  '',
  '* Unless required by applicable law or agreed to in writing, software',
  '* distributed under the License is distributed on an "AS IS" BASIS,',
  '* WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.',
  '* See the License for the specific language governing permissions and',
  '* limitations under the License.',
  '*/',
];
/** The line number of the first body line. */
export const HEADER_BODY_FIRST_LINE = 3;
/** The line number of the header's opening. */
export const HEADER_OPENING_LINE = 1;
/** The group of the copyright pattern with the first year. */
export const FIRST_YEAR_GROUP = 1;
/** The group of the copyright pattern with the last year of a range. */
export const LAST_YEAR_GROUP = 2;
/** The line number of the copyright line. */
export const COPYRIGHT_LINE = 2;

/** A file without the header. */
export const MISSING_HEADER = 'the file does not start with the copyright header';
/** A malformed copyright line. */
export const MALFORMED_COPYRIGHT = 'the copyright line needs a year or a rising year range';
/** A header line that differs from the approved text. */
export const DIFFERENT_HEADER_LINE = 'the copyright header differs from the approved text';
/** A file over the line limit. */
export const TOO_MANY_LINES = 'more lines than the limit allows (R34)';
/** A literal outside a constants module. */
export const LITERAL_OUTSIDE_CONSTANTS = 'literal outside a constants module; name it as a constant (R34)';
/** A template with text outside its substitutions. */
export const TEMPLATE_WITH_WORDS = 'template literal holds text; move the text into a constant (R34)';
/** A statement other than an import or a constant in a constants module. */
export const NOT_A_CONSTANT = 'a constants module holds only imports and constants (R34)';
