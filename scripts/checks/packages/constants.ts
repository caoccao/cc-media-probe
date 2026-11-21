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

// Commands, files and messages of the package-contents check.

/** Lists the files of a package without packaging it; uncommitted changes are allowed, since the check runs before
 * a commit. */
export const PACKAGE_LIST_ARGUMENTS = ['package', '--quiet', '--list', '--allow-dirty', '--locked', '--package'];
/** The directory of the expected file lists, by component. */
export const EXPECTED_LIST_DIRECTORY = ['crates', 'cc-media-probe-checks', 'data'];
/** The file name of an expected list: this prefix, the package, the extension. */
export const EXPECTED_LIST_PREFIX = 'package-';
/** The extension of an expected list. */
export const EXPECTED_LIST_EXTENSION = '.txt';
/** Files that a crate directory holds as byte-identical copies of the root files, by component. */
export const COPIED_FILES = [
  { copy: ['crates', 'cc-media-probe', 'README.md'], original: ['README.md'] },
  { copy: ['crates', 'cc-media-probe', 'LICENSE'], original: ['LICENSE'] },
  { copy: ['crates', 'cc-media-probe-cli', 'LICENSE'], original: ['LICENSE'] },
];
/** `cargo package --list` failed. */
export const LISTING_FAILED = 'cargo package --list failed';
/** A file in the package that is not expected. */
export const UNEXPECTED_FILE = 'file in the package but not in the expected list';
/** An expected file that is not in the package. */
export const MISSING_FILE = 'expected file missing from the package';
/** A copy that differs from its original. */
export const NOT_A_COPY = 'not a byte-identical copy of';
