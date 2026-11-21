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

// Heading anchors as GitHub derives them, and the lines outside code blocks.

import { LINE_BREAK_PATTERN } from '../constants.ts';
import { EMPTY, FENCE_PATTERN, HEADING_PATTERN, HYPHEN, NOT_ANCHOR_CHARACTERS, SPACE_PATTERN } from './constants.ts';

/** The lines of `text`, with the lines of fenced code blocks and their fences emptied. */
export function linesOutsideCode(text: string): string[] {
  let inFence = false;
  return text.split(LINE_BREAK_PATTERN).map((line) => {
    if (FENCE_PATTERN.test(line)) {
      inFence = !inFence;
      return EMPTY;
    }
    return inFence ? EMPTY : line;
  });
}

/** The anchor of a heading: lower case, without punctuation, with hyphens for spaces. */
export function headingAnchor(heading: string): string {
  return heading.toLowerCase().replace(NOT_ANCHOR_CHARACTERS, EMPTY).replace(SPACE_PATTERN, HYPHEN);
}

/** Every heading anchor of a document; repeated headings get `-1`, `-2` and so on. */
export function collectAnchors(text: string): Set<string> {
  const anchors = new Set<string>();
  const occurrences = new Map<string, number>();
  for (const line of linesOutsideCode(text)) {
    const heading = HEADING_PATTERN.exec(line)?.[1];
    if (heading === undefined) {
      continue;
    }
    const anchor = headingAnchor(heading);
    const seen = occurrences.get(anchor) ?? 0;
    occurrences.set(anchor, seen + 1);
    anchors.add(seen === 0 ? anchor : `${anchor}${HYPHEN}${seen}`);
  }
  return anchors;
}
