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

// The Internet-Drafts of Appendix A.7 (plan section 5.3): each is pinned to a revision, and the manifest fetches
// exactly that revision's archive address.

import { EMPTY, ROW_END } from '../constants.ts';
import {
  DRAFT_ADDRESS_GROUP,
  DRAFT_NAME_GROUP,
  DRAFT_REVISION_GROUP,
  DRAFT_ROW_PATTERN,
  DRAFTS_HEADING,
  FETCH,
  NEXT_SECTION_HEADING,
} from './constants.ts';
import type { DocumentEntry } from './manifest.ts';
import { MISSING_DRAFT } from './messages.ts';

/** One pinned draft of Appendix A.7. */
export interface PinnedDraft {
  readonly name: string;
  readonly revision: string;
  readonly address: string;
}

/** The drafts that Appendix A.7 of `plan` pins. */
export function pinnedDrafts(plan: string): PinnedDraft[] {
  const lines = plan.split(ROW_END);
  const start = lines.findIndex((line) => line.startsWith(DRAFTS_HEADING));
  if (start < 0) {
    return [];
  }
  const length = lines.slice(start + 1).findIndex((line) => line.startsWith(NEXT_SECTION_HEADING));
  const section = lines.slice(start + 1, length < 0 ? lines.length : start + 1 + length);
  return section.flatMap((line) => {
    const match = DRAFT_ROW_PATTERN.exec(line);
    if (match === null) {
      return [];
    }
    return [
      {
        name: match[DRAFT_NAME_GROUP] ?? EMPTY,
        revision: match[DRAFT_REVISION_GROUP] ?? EMPTY,
        address: match[DRAFT_ADDRESS_GROUP] ?? EMPTY,
      },
    ];
  });
}

/** Each pinned draft has a row that fetches its archive address; findings name `file`. */
export function compareDrafts(
  file: string,
  drafts: readonly PinnedDraft[],
  entries: readonly DocumentEntry[],
): string[] {
  const fetched = new Set(entries.filter((entry) => entry.retrieval === FETCH).map((entry) => entry.artifact));
  return drafts
    .filter((draft) => !fetched.has(draft.address))
    .map((draft) => `${file}: ${MISSING_DRAFT} ${draft.name}-${draft.revision}`);
}
