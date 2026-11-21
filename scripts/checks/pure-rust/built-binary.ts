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

// The path of a built binary, as Cargo reports it, so that no path or extension of the target directory is assumed.

import { LINE_BREAK_PATTERN } from '../constants.ts';
import { COMPILER_ARTIFACT_REASON } from './constants.ts';

/** The part of a Cargo build message that names a built artifact. */
interface ArtifactMessage {
  readonly reason?: string;
  readonly target?: { readonly name?: string };
  readonly executable?: string | null;
}

function parseMessage(line: string): ArtifactMessage | undefined {
  try {
    return JSON.parse(line) as ArtifactMessage;
  } catch {
    return undefined;
  }
}

/** The executable of the binary `name` among Cargo's JSON build messages, if one was reported. */
export function executableFromMessages(messages: string, name: string): string | undefined {
  for (const line of messages.split(LINE_BREAK_PATTERN)) {
    const message = parseMessage(line);
    const executable = message?.executable;
    const isNamedArtifact = message?.reason === COMPILER_ARTIFACT_REASON && message.target?.name === name;
    if (isNamedArtifact && executable !== undefined && executable !== null) {
      return executable;
    }
  }
  return undefined;
}
