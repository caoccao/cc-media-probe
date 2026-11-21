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

// Stores and verifies the non-RFC documents of `docs/specs/manifest.tsv` in the git-ignored `docs/specs/files/`:
// `node scripts/fetch-specs.ts`, or with identifiers to handle only those rows. It needs the network and asks each
// site at most once every few seconds; the offline check of the manifest is part of `node scripts/check.ts`.

import { repositoryRoot } from './checks/repository-files.ts';
import { fetchDocuments } from './checks/specifications/documents/fetch.ts';

process.exitCode = await fetchDocuments(repositoryRoot(), process.argv.slice(1 + 1));
