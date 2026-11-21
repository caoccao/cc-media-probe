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

//! Labels of the rules that findings belong to.

/// The copyright header of every Rust file (G1.2).
pub const HEADER_LABEL: &str = "header";
/// Module roots only declare (R30).
pub const MODULE_ROOT_LABEL: &str = "module-root";
/// Directory modules use `mod.rs` (R30).
pub const MODULE_FILE_LABEL: &str = "module-file";
/// At most 500 lines before the embedded tests (R31).
pub const LINE_LIMIT_LABEL: &str = "line-limit";
/// No `#[path]` and no `include!` of Rust source (R31).
pub const INCLUSION_LABEL: &str = "inclusion";
/// Names are whole words (R32).
pub const NAMES_LABEL: &str = "names";
/// Literals live in constants modules (R33).
pub const CONSTANTS_LABEL: &str = "constants";
/// A source file that cannot be analyzed.
pub const SYNTAX_LABEL: &str = "syntax";
/// Line coverage above 90% (R28).
pub const COVERAGE_LABEL: &str = "coverage";
/// Every target belongs to its expected package (plan section 7.1).
pub const TARGETS_LABEL: &str = "targets";
/// The pure-Rust inventory (D4, first layer).
pub const INVENTORY_LABEL: &str = "inventory";
/// The outputs of build scripts (D4, third layer).
pub const BUILD_OUTPUT_LABEL: &str = "build-output";
