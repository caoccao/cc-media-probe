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

// The index of `docs/rfc/README.md`, generated from the manifest: number linked to the text, title, status, role,
// then the license notice (plan section 5.2).

import { EMPTY, ROW_END } from '../constants.ts';
import { type RfcEntry, rfcFileName } from './manifest.ts';
import { CELL_DIVIDER, README_CLOSING, README_OPENING, ROW_CLOSING, ROW_OPENING } from './readme-text.ts';

/** The README's text for `entries`, in ascending order. */
export function renderReadme(entries: readonly RfcEntry[]): string {
  const rows = [...entries]
    .sort((first, second) => first.number - second.number)
    .map(({ number, title, status, role }) => {
      const cells = [`[${number}](${rfcFileName(number)})`, title, status, role];
      return ROW_OPENING + cells.join(CELL_DIVIDER) + ROW_CLOSING;
    });
  return [...README_OPENING, ...rows, ...README_CLOSING].map((line) => line + ROW_END).join(EMPTY);
}
