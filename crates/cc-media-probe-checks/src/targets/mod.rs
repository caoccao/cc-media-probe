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

//! The target-ownership check (plan section 7.1): every Cargo target of the
//! workspace belongs to its expected package, and the separate workspaces
//! under `spikes/` and `readers/` stay outside the root workspace.

mod cargo;
mod check;
pub mod constants;
mod exclusion;
mod expected;
mod metadata;

pub use cargo::run_cargo_metadata;
pub use check::compare_targets;
pub use exclusion::check_exclusion;
pub use expected::{TargetEntry, parse_expected_targets};
pub use metadata::workspace_targets;
