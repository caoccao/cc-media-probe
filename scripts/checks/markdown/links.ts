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

// Every relative link and heading anchor in the repository's Markdown files resolves. External links are checked
// separately, against the specification manifests.

import { existsSync, readFileSync } from 'node:fs';
import { join, posix } from 'node:path';

import { MARKDOWN_EXTENSION, TEXT_ENCODING } from '../constants.ts';
import { collectAnchors, linesOutsideCode } from './anchors.ts';
import {
  ANCHOR_SEPARATOR,
  ANGLE_BRACKETS_PATTERN,
  CODE_SPAN_PATTERN,
  EMPTY,
  INLINE_LINK_PATTERN,
  MISSING_FILE,
  MISSING_HEADING,
  REFERENCE_DEFINITION_PATTERN,
  SCHEME_PATTERN,
  TITLE_PATTERN,
} from './constants.ts';

/** A link target and the line it is on. */
export interface LinkReference {
  readonly line: number;
  readonly target: string;
}

/** The targets of every inline link and reference definition outside code. */
export function linkReferences(text: string): LinkReference[] {
  const references: LinkReference[] = [];
  linesOutsideCode(text).forEach((line, offset) => {
    const withoutCode = line.replace(CODE_SPAN_PATTERN, EMPTY);
    for (const match of withoutCode.matchAll(INLINE_LINK_PATTERN)) {
      references.push({ line: offset + 1, target: match[1] ?? EMPTY });
    }
    const definition = REFERENCE_DEFINITION_PATTERN.exec(withoutCode)?.[1];
    if (definition !== undefined) {
      references.push({ line: offset + 1, target: definition });
    }
  });
  return references;
}

/** The findings for every Markdown file among `files`. */
export function checkMarkdownLinks(root: string, files: readonly string[]): string[] {
  const anchors = new Map<string, Set<string>>();
  const anchorsOf = (file: string): Set<string> => {
    let found = anchors.get(file);
    if (found === undefined) {
      found = collectAnchors(readFileSync(join(root, file), TEXT_ENCODING));
      anchors.set(file, found);
    }
    return found;
  };
  const findings: string[] = [];
  for (const file of files.filter((candidate) => candidate.endsWith(MARKDOWN_EXTENSION))) {
    for (const reference of linkReferences(readFileSync(join(root, file), TEXT_ENCODING))) {
      const problem = checkTarget(root, file, reference.target, anchorsOf);
      if (problem !== undefined) {
        findings.push(`${file}:${reference.line}: ${problem}: ${reference.target}`);
      }
    }
  }
  return findings;
}

function checkTarget(
  root: string,
  file: string,
  rawTarget: string,
  anchorsOf: (file: string) => Set<string>,
): string | undefined {
  const target = rawTarget.replace(ANGLE_BRACKETS_PATTERN, EMPTY).replace(TITLE_PATTERN, EMPTY);
  if (target === EMPTY || SCHEME_PATTERN.test(target)) {
    return undefined;
  }
  const separator = target.indexOf(ANCHOR_SEPARATOR);
  const path = separator < 0 ? target : target.slice(0, separator);
  const anchor = separator < 0 ? EMPTY : decodeAnchor(target.slice(separator + 1));
  const resolved = path === EMPTY ? file : posix.normalize(posix.join(posix.dirname(file), path));
  if (!existsSync(join(root, resolved))) {
    return MISSING_FILE;
  }
  if (anchor !== EMPTY && resolved.endsWith(MARKDOWN_EXTENSION) && !anchorsOf(resolved).has(anchor)) {
    return MISSING_HEADING;
  }
  return undefined;
}

function decodeAnchor(anchor: string): string {
  try {
    return decodeURIComponent(anchor);
  } catch {
    return anchor;
  }
}
