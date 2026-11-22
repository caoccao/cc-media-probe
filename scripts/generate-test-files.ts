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

// Generates the test files that the recipes in `crates/cc-media-probe-testkit/test-files/` describe, with the
// ffmpeg and mkvmerge binaries found on the system path or in their usual install locations, into the git-ignored
// `target/test-files/`: `node scripts/generate-test-files.ts`. A file is generated once and kept while its recipe and
// tool version stay the same; files that no recipe produces are removed (plan section 9.1, docs/guide/test-files.md).

import { repositoryRoot } from './checks/repository-files.ts';
import { generateTestFiles } from './checks/test-files/generate.ts';

process.exitCode = generateTestFiles(repositoryRoot());
