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

// What a fetch script reports: problems, which fail the run, and notes, which are printed for review.

import { NOTE_PREFIX } from './constants.ts';

/** Problems fail the run; notes are printed for review. */
export interface Report {
  readonly problems: string[];
  readonly notes: string[];
}

/** Adds `addition` to `report`. */
export function merge(report: Report, addition: Report): void {
  report.problems.push(...addition.problems);
  report.notes.push(...addition.notes);
}

/** Prints the notes and problems, then `success` when there is no problem; returns the exit status. */
export function finish(report: Report, success: string): number {
  for (const note of report.notes) {
    console.log(`${NOTE_PREFIX}${note}`);
  }
  for (const problem of report.problems) {
    console.error(problem);
  }
  if (report.problems.length > 0) {
    return 1;
  }
  console.log(success);
  return 0;
}
