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

// The RFCs of Appendix A of the plan: every table row whose first cell links an RFC number to its information page.
// The drafts of A.7 and the errata of A.8 are written differently and are not read.

import { EMPTY, ROW_END } from '../constants.ts';
import {
  APPENDIX_HEADING,
  APPENDIX_LINK_GROUP,
  APPENDIX_NUMBER_GROUP,
  APPENDIX_ROLE_GROUP,
  APPENDIX_ROW_PATTERN,
  APPENDIX_STATUS_GROUP,
  APPENDIX_TITLE_GROUP,
  LEVEL_TWO_HEADING,
  STATUS_NAMES,
} from './constants.ts';
import { DUPLICATE_RFC, LINK_MISMATCH, NO_APPENDIX, UNKNOWN_STATUS } from './messages.ts';

/** One RFC as Appendix A lists it, with its status written out. */
export interface AppendixRfc {
  readonly number: number;
  readonly title: string;
  readonly status: string;
  readonly role: string;
}

/** The RFCs of Appendix A, and findings for malformed rows; findings name `file`. */
export function appendixRfcs(file: string, plan: string): { rfcs: AppendixRfc[]; findings: string[] } {
  const lines = plan.split(ROW_END);
  const start = lines.findIndex((line) => line.startsWith(APPENDIX_HEADING));
  if (start < 0) {
    return { rfcs: [], findings: [`${file}: ${NO_APPENDIX}`] };
  }
  const length = lines.slice(start + 1).findIndex((line) => line.startsWith(LEVEL_TWO_HEADING));
  const end = length < 0 ? lines.length : start + 1 + length;
  const rfcs: AppendixRfc[] = [];
  const findings: string[] = [];
  const seen = new Set<number>();
  for (let index = start; index < end; index += 1) {
    const match = APPENDIX_ROW_PATTERN.exec(lines[index] ?? EMPTY);
    if (match === null) {
      continue;
    }
    const location = `${file}:${index + 1}`;
    const number = Number(match[APPENDIX_NUMBER_GROUP]);
    const status = STATUS_NAMES.get(match[APPENDIX_STATUS_GROUP] ?? EMPTY);
    if (number !== Number(match[APPENDIX_LINK_GROUP])) {
      findings.push(`${location}: ${LINK_MISMATCH}`);
    } else if (status === undefined) {
      findings.push(`${location}: ${UNKNOWN_STATUS}`);
    } else if (seen.has(number)) {
      findings.push(`${location}: ${DUPLICATE_RFC}: ${number}`);
    } else {
      seen.add(number);
      const title = match[APPENDIX_TITLE_GROUP] ?? EMPTY;
      rfcs.push({ number, title, status, role: match[APPENDIX_ROLE_GROUP] ?? EMPTY });
    }
  }
  return { rfcs, findings };
}
