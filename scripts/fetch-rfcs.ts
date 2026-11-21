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

// Writes the RFC manifest from Appendix A of the plan, downloads and verifies every RFC text, regenerates the index,
// and reports what changed at the RFC Editor: `node scripts/fetch-rfcs.ts`. It needs the network; the offline check
// of the same files is part of `node scripts/check.ts`.

import { repositoryRoot } from './checks/repository-files.ts';
import { fetchRfcs } from './checks/specifications/rfc/fetch.ts';

process.exitCode = await fetchRfcs(repositoryRoot());
