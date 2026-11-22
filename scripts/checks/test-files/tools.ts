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

// The tools that generate test files: found on the system path or in their usual install locations, with versions.

import { statSync } from 'node:fs';
import { join, parse } from 'node:path';

import { captureCommand, runCommand } from '../command.ts';
import {
  EXECUTABLE_SUFFIXES,
  LINE_PATTERN,
  ROOT_TOOL_FOLDERS,
  TOOL_MISSING,
  TOOLS,
  VARIABLE_TOOL_FOLDERS,
} from './constants.ts';

/** How the generator reaches its tools; the tests put a stand-in in its place. */
export interface ToolRunner {
  /** The first line of the version report of `program`, a name or a path, or `undefined` when it cannot be run. */
  version(program: string, versionArguments: readonly string[], directory: string): string | undefined;
  /** Runs `program`; returns whether it succeeded. */
  run(program: string, commandArguments: readonly string[], directory: string): boolean;
  /** Whether a file exists at `path`. */
  isFile(path: string): boolean;
}

/** A tool as the generator runs it: by name from the system path, or by its full path. */
export interface LocatedTool {
  readonly program: string;
  readonly version: string;
}

/** The real tools, started without a shell. */
export const SYSTEM_TOOLS: ToolRunner = {
  version(program: string, versionArguments: readonly string[], directory: string): string | undefined {
    const firstLine = captureCommand(program, versionArguments, directory)?.split(LINE_PATTERN)[0]?.trim();
    return firstLine === undefined || firstLine.length === 0 ? undefined : firstLine;
  },
  run(program: string, commandArguments: readonly string[], directory: string): boolean {
    return runCommand(program, commandArguments, directory);
  },
  isFile(path: string): boolean {
    return statSync(path, { throwIfNoEntry: false })?.isFile() === true;
  },
};

/** The usual install locations of `tool`, on any platform: the folders below the file-system root of `directory`
 * and below the folders that environment variables name, each with and without an executable suffix. Folders that do
 * not exist on a platform are simply not found, so the list needs no branch per platform. */
export function usualLocations(tool: string, directory: string, environment: NodeJS.ProcessEnv): string[] {
  const root = parse(directory).root;
  const folders = [
    ...ROOT_TOOL_FOLDERS.map((components) => join(root, ...components)),
    ...VARIABLE_TOOL_FOLDERS.flatMap(([variable, components]) => {
      const base = environment[variable];
      return base === undefined || base.length === 0 ? [] : [join(base, ...components)];
    }),
  ];
  return folders.flatMap((folder) => EXECUTABLE_SUFFIXES.map((suffix) => join(folder, tool + suffix)));
}

/** Finds each tool of `tools`: first by name on the system path, then in its usual locations. A tool found in
 * neither is a problem that stops the run. */
export function locateTools(
  tools: Iterable<string>,
  runner: ToolRunner,
  directory: string,
  environment: NodeJS.ProcessEnv = process.env,
): { located: Map<string, LocatedTool>; problems: string[] } {
  const located = new Map<string, LocatedTool>();
  const problems: string[] = [];
  for (const tool of [...new Set(tools)].sort()) {
    const versionArguments = TOOLS.get(tool) ?? [];
    const candidates = [tool, ...usualLocations(tool, directory, environment).filter((path) => runner.isFile(path))];
    const found = candidates
      .map((program) => ({ program, version: runner.version(program, versionArguments, directory) }))
      .find((candidate): candidate is LocatedTool => candidate.version !== undefined);
    if (found === undefined) {
      problems.push(`${tool} ${TOOL_MISSING}`);
    } else {
      located.set(tool, found);
    }
  }
  return { located, problems };
}
