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

// The dynamic libraries of a linked binary, as the platform's own tool lists them (D4, fourth layer).

import { join } from 'node:path';

import { captureCommand } from '../command.ts';
import { LINE_BREAK_PATTERN } from '../constants.ts';
import {
  DUMPBIN_ARGUMENTS,
  DUMPBIN_COMPONENTS,
  DUMPBIN_DLL_PATTERN,
  MACOS,
  OBJDUMP,
  OBJDUMP_DLL_PATTERN,
  OBJDUMP_HEADERS_OPTION,
  OTOOL,
  OTOOL_LIBRARIES_OPTION,
  OTOOL_LIBRARY_PATTERN,
  PROGRAM_FILES_VARIABLE,
  READELF,
  READELF_DYNAMIC_OPTION,
  READELF_NEEDED_PATTERN,
  VSWHERE_ARGUMENTS,
  VSWHERE_COMPONENTS,
  WINDOWS,
} from './constants.ts';

/** The first group of `pattern` on every line of `output` that matches it. */
export function matchingLines(output: string, pattern: RegExp): string[] {
  return output
    .split(LINE_BREAK_PATTERN)
    .map((line) => pattern.exec(line)?.[1])
    .filter((library) => library !== undefined);
}

/** The libraries that `binary` loads on `platform`, or `undefined` when the tool fails. */
export function listDynamicLibraries(platform: string, binary: string, directory: string): string[] | undefined {
  if (platform === MACOS) {
    const output = captureCommand(OTOOL, [OTOOL_LIBRARIES_OPTION, binary], directory);
    return output === undefined ? undefined : matchingLines(output, OTOOL_LIBRARY_PATTERN);
  }
  if (platform === WINDOWS) {
    return listWindowsLibraries(binary, directory);
  }
  const output = captureCommand(READELF, [READELF_DYNAMIC_OPTION, binary], directory);
  return output === undefined ? undefined : matchingLines(output, READELF_NEEDED_PATTERN);
}

function listWindowsLibraries(binary: string, directory: string): string[] | undefined {
  const objdumpOutput = captureCommand(OBJDUMP, [OBJDUMP_HEADERS_OPTION, binary], directory);
  if (objdumpOutput !== undefined) {
    return matchingLines(objdumpOutput, OBJDUMP_DLL_PATTERN);
  }
  const programFiles = process.env[PROGRAM_FILES_VARIABLE];
  if (programFiles === undefined) {
    return undefined;
  }
  const vswhere = join(programFiles, ...VSWHERE_COMPONENTS);
  const dumpbin = captureCommand(vswhere, [...VSWHERE_ARGUMENTS, join(...DUMPBIN_COMPONENTS)], directory)
    ?.split(LINE_BREAK_PATTERN)
    .find((line) => line.length > 0);
  const dumpbinOutput = dumpbin === undefined ? undefined : captureCommand(dumpbin, [...DUMPBIN_ARGUMENTS, binary], directory);
  return dumpbinOutput === undefined ? undefined : matchingLines(dumpbinOutput, DUMPBIN_DLL_PATTERN);
}
