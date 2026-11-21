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

//! The pure-Rust verification of D4, as far as a program can do it: the
//! inventory of the published crates' dependency graph (first layer), the
//! inspection of build-script outputs (third layer), and the comparison of
//! a binary's dynamic libraries with the allow-list (fourth layer). The
//! second layer, reading every build script, is a review.

mod build_output;
mod check;
pub mod constants;
mod graph;
mod lists;
mod system_libraries;

pub use build_output::check_build_outputs;
pub use check::check_inventory;
pub use graph::{CrateNode, published_graph};
pub use lists::{ParsedList, parse_allowed_libraries, parse_reviewed_scripts};
pub use system_libraries::check_dynamic_libraries;
