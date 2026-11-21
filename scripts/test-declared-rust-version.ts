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

// Builds and tests the workspace with exactly the `rust-version` it declares (R12):
// `node scripts/test-declared-rust-version.ts`. CI runs it on Linux (plan section 9.8).

import { repositoryRoot } from './checks/repository-files.ts';
import { testDeclaredRustVersion } from './checks/rust-version/run-tests.ts';

process.exitCode = testDeclaredRustVersion(repositoryRoot()) ? 0 : 1;
