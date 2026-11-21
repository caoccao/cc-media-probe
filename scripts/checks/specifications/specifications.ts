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

// The offline checks of the specification tables (plan section 5.2), run by `node scripts/check.ts`.

import { checkDeliverables } from './deliverables/check.ts';
import { checkDocuments } from './documents/check.ts';
import { checkRfcs } from './rfc/check.ts';

/** The findings of every specification table in `root`, whose files Git lists as `files`. */
export function checkSpecifications(root: string, files: readonly string[]): string[] {
  return [...checkRfcs(root, files), ...checkDocuments(root), ...checkDeliverables(root)];
}
