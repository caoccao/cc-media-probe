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

// Runs every repository check: `node scripts/check.ts`. Every change passes it before it is committed; CI runs it on
// Linux, macOS and Windows.

import { runAllChecks } from './checks/all-checks.ts';
import { repositoryRoot } from './checks/repository-files.ts';

process.exitCode = runAllChecks(repositoryRoot());
