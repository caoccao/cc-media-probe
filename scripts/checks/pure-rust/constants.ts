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

// Commands, tools and messages of the pure-Rust verification (D4).

/** The checks-crate command of the first layer. */
export const INVENTORY_COMMAND = 'inventory';
/** The checks-crate command of the third layer. */
export const BUILD_OUTPUT_COMMAND = 'build-output';
/** The checks-crate command of the fourth layer. */
export const DYNAMIC_LIBRARIES_COMMAND = 'dynamic-libraries';
/** Builds the published crates. */
export const BUILD_ARGUMENTS = ['build', '--quiet', '--locked', '--package', 'cc-media-probe', '--package', 'cc-media-probe-cli'];
/** The feature selections that are built: none, the defaults, all. */
export const FEATURE_SELECTIONS = [['--no-default-features'], [], ['--all-features']];
/** The option that names the target directory. */
export const TARGET_DIRECTORY_OPTION = '--target-dir';
/** The target directory of these builds, relative to the repository root, by component. */
export const TARGET_DIRECTORY = ['target', 'pure-rust'];
/** A program inside the target directory that does not exist. Every native-compiler variable names it, so that any
 * use of a native compiler fails. */
export const MISSING_COMPILER_NAME = 'no-native-compiler';
/** Makes Cargo describe every artifact it builds, with the executable's path, as one JSON message per line. */
export const MESSAGE_FORMAT_ARGUMENT = '--message-format=json';
/** The reason of the message that describes a built artifact. */
export const COMPILER_ARTIFACT_REASON = 'compiler-artifact';
/** The binary whose dynamic libraries are inspected. */
export const BINARY_NAME = 'cc-media-probe';
/** Node.js platform names and the names of the allow-list. */
export const PLATFORM_NAMES = new Map([
  ['darwin', 'macos'],
  ['linux', 'linux'],
  ['win32', 'windows'],
]);
/** The allow-list name of Windows. */
export const WINDOWS = 'windows';
/** The allow-list name of macOS. */
export const MACOS = 'macos';
/** Lists the libraries of a macOS binary. */
export const OTOOL = 'otool';
/** The option of `otool` that lists libraries. */
export const OTOOL_LIBRARIES_OPTION = '-L';
/** Lists the dynamic section of a Linux binary. */
export const READELF = 'readelf';
/** The option of `readelf` that shows the dynamic section. */
export const READELF_DYNAMIC_OPTION = '--dynamic';
/** Lists the imports of a Windows binary, where binutils is installed. */
export const OBJDUMP = 'objdump';
/** The option of `objdump` that shows the headers with their imports. */
export const OBJDUMP_HEADERS_OPTION = '-p';
/** Finds Visual Studio tools: its path below the 32-bit program files directory, by component. */
export const VSWHERE_COMPONENTS = ['Microsoft Visual Studio', 'Installer', 'vswhere.exe'];
/** The variable with the 32-bit program files directory on Windows. */
export const PROGRAM_FILES_VARIABLE = 'ProgramFiles(x86)';
/** The arguments of `vswhere` before the pattern of the program to find. */
export const VSWHERE_ARGUMENTS = ['-latest', '-find'];
/** The pattern of `dumpbin` for `vswhere`, by component. */
export const DUMPBIN_COMPONENTS = ['**', 'Hostx64', 'x64', 'dumpbin.exe'];
/** The arguments of `dumpbin` that list dependencies. */
export const DUMPBIN_ARGUMENTS = ['-nologo', '-dependents'];
/** A library line of `otool -L`: its path, then the version in parentheses. */
export const OTOOL_LIBRARY_PATTERN = /^\s+(\S+)\s+\(/u;
/** A needed library of `readelf --dynamic`. */
export const READELF_NEEDED_PATTERN = /\(NEEDED\).*\[(.+)\]/u;
/** An imported DLL of `objdump -p`. */
export const OBJDUMP_DLL_PATTERN = /DLL Name:\s*(\S+)/u;
/** An imported DLL of `dumpbin -dependents`. */
export const DUMPBIN_DLL_PATTERN = /^\s+(\S+\.dll)\s*$/iu;
/** The layers, as they are announced. */
export const INVENTORY_LAYER = '--> layer 1: inventory';
/** The third layer. */
export const BUILD_LAYER = '--> layer 3: build inspection';
/** The fourth layer. */
export const RESULT_LAYER = '--> layer 4: result inspection';
/** A platform without an allow-list. */
export const UNSUPPORTED_PLATFORM = 'no allow-list of system libraries for this platform';
/** The build reported no executable for the binary. */
export const BINARY_NOT_FOUND = 'the build reported no executable for the binary';
/** The platform tool failed or is missing. */
export const LISTING_FAILED = 'could not list the dynamic libraries of the binary';
