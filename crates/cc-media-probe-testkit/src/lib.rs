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

//! The test kit of `cc-media-probe` (plan section 7.1). It uses the standard library only and never depends on the
//! library, so it gives bytes and standard-library readers only:
//! - [`syntax`]: the text format of hand-annotated hex fixtures and its assembler (`docs/design/test-kit.md`
//!   section 2);
//! - [`fixture`]: the fixtures read by name from `fixtures/<format>/<name>.hex`, the layout of that folder, and
//!   the places of each fixture's expectations (sections 2 and 4.1);
//! - [`builder`]: the byte and bit writers that specifications' syntax tables are written with, and the removal
//!   of start-code emulation from a NAL unit's payload (section 5);
//! - [`readers`]: a reader that counts what it is asked for, and one that cuts its reads at fixed bytes and fails
//!   on purpose (section 5);
//! - [`generated`]: the paths of the generated test files below `target/test-files/` and of their recipes (plan
//!   section 9.1, `docs/guide/test-files.md`).
//!
//! Later tasks add the container builders as their formats arrive, the toy format's builder (plan Phase 3, task
//! 5), and each fixture's seeds by name with the truncation and mutation harness (task 19).

pub mod builder;
pub mod fixture;
pub mod generated;
pub mod readers;
pub mod syntax;
