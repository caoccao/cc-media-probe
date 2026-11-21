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

// Running programs: with shared output, or with captured output.

import { spawnSync } from 'node:child_process';

import { CAPTURED_STREAMS, INHERITED_STREAMS, TEXT_ENCODING } from './constants.ts';

/** Runs `program` in `directory` with this process's output; returns whether it succeeded. */
export function runCommand(
  program: string,
  commandArguments: readonly string[],
  directory: string,
  environment: NodeJS.ProcessEnv = process.env,
): boolean {
  const result = spawnSync(program, commandArguments, { cwd: directory, env: environment, stdio: INHERITED_STREAMS });
  return result.status === 0;
}

/** Runs `program` in `directory` and returns its standard output, or `undefined` when it fails. */
export function captureCommand(
  program: string,
  commandArguments: readonly string[],
  directory: string,
  environment: NodeJS.ProcessEnv = process.env,
): string | undefined {
  const result = spawnSync(program, commandArguments, {
    cwd: directory,
    env: environment,
    encoding: TEXT_ENCODING,
    stdio: CAPTURED_STREAMS,
    maxBuffer: Number.MAX_SAFE_INTEGER,
  });
  return result.status === 0 ? result.stdout : undefined;
}
