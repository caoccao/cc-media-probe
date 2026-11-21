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

// Constants that several checks share (R33): programs, arguments and messages.

/** The Cargo executable. */
export const CARGO = 'cargo';
/** The Git executable. */
export const GIT = 'git';
/** Runs a command of the `cc-media-probe-checks` binary. */
export const CHECKS_ARGUMENTS = ['run', '--quiet', '--locked', '--package', 'cc-media-probe-checks', '--'];
/** Lists tracked files and new files that are not ignored, separated by NUL characters. */
export const LIST_FILES_ARGUMENTS = ['ls-files', '--cached', '--others', '--exclude-standard', '-z'];
/** Separates the file names of `git ls-files -z`. */
export const NUL_CHARACTER = '\0';
/** Splits text into lines. */
export const LINE_BREAK_PATTERN = /\r?\n/u;
/** The encoding of every text file. */
export const TEXT_ENCODING = 'utf8';
/** The parent directory in a path. */
export const PARENT_DIRECTORY = '..';
/** Separates a file from its line in findings. */
export const LOCATION_SEPARATOR = ':';
/** Child process streams: no input, captured output, shown errors. */
export const CAPTURED_STREAMS = ['ignore', 'pipe', 'inherit'];
/** Child process streams that are shared with this process. */
export const INHERITED_STREAMS = 'inherit';
/** The suffix of the scripts' test files. */
export const TEST_FILE_SUFFIX = '.test.ts';
/** The extension of TypeScript files. */
export const TYPESCRIPT_EXTENSION = '.ts';
/** The extension of Markdown files. */
export const MARKDOWN_EXTENSION = '.md';
/** The published packages (R6). */
export const PUBLISHED_PACKAGES = ['cc-media-probe', 'cc-media-probe-cli'];
/** A Git command that failed. */
export const GIT_FAILED = 'git ls-files failed; the checks need a Git working tree';
