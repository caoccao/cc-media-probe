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

// Extensions, patterns and messages of the file hygiene check.

/** Extensions of scripts in languages other than TypeScript, which R34 rules out. */
export const OTHER_SCRIPT_EXTENSIONS = [
  '.awk', '.bash', '.bat', '.cjs', '.cmd', '.fish', '.js', '.mjs', '.pl', '.ps1', '.py', '.rb', '.sed', '.sh', '.zsh',
];
/** Verbatim third-party texts, exempt from hygiene. */
export const VERBATIM_PATTERNS = [/^LICENSE$/u, /^crates\/[^/]+\/LICENSE$/u, /^docs\/rfc\/rfc\d+\.txt$/u];
/** Tab-separated data, where tabs are the separator. */
export const TAB_SEPARATED_EXTENSION = '.tsv';
/** The decoder label of UTF-8. */
export const UTF8_LABEL = 'utf-8';
/** A carriage return. */
export const CARRIAGE_RETURN = '\r';
/** A line feed. */
export const LINE_FEED = '\n';
/** A tab. */
export const TAB = '\t';
/** Tabs at the end of a row of tab-separated data, which close its empty last cells. */
export const TRAILING_TABS_PATTERN = /\t+$/u;
/** Nothing. */
export const EMPTY = '';
/** Whitespace at the end of a line. */
export const TRAILING_WHITESPACE_PATTERN = /[ \t\f\v]+\r?$/u;
/** A script in another language. */
export const OTHER_SCRIPT_LANGUAGE = 'scripts are TypeScript run by Node.js (R34); rewrite this one in TypeScript';
/** A binary file. */
export const BINARY_FILE = 'binary file; nothing binary is committed here';
/** Text that is not UTF-8. */
export const NOT_UTF8 = 'not valid UTF-8';
/** A carriage return. */
export const CARRIAGE_RETURN_FOUND = 'carriage return; use LF line endings';
/** A missing final newline. */
export const NO_FINAL_NEWLINE = 'no newline at the end';
/** Trailing whitespace. */
export const TRAILING_WHITESPACE = 'trailing whitespace';
/** A tab character. */
export const TAB_FOUND = 'tab character';
