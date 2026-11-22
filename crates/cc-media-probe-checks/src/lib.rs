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

//! Repository checks for `cc-media-probe`.
//!
//! Every check here uses the standard library only, so it runs before any
//! dependency is approved. `scripts/check.ts` and CI run them through the
//! `cc-media-probe-checks` binary:
//!
//! - `structure`: the code rules R30 to R33 and the copyright header (D23).
//! - `coverage`: line coverage per published crate and per library area
//!   (R28), read from the LCOV report of `cargo llvm-cov`.
//! - `targets`: every Cargo target belongs to its expected package, and the
//!   excluded workspaces stay outside the root workspace (plan section 7.1).
//! - `inventory`: the first layer of the pure-Rust verification (D4).
//! - `build-output`: the third layer, over the outputs of build scripts.
//! - `dynamic-libraries`: the fourth layer, over the libraries of the linked
//!   command-line binary.

pub mod command_line;
pub mod constants;
pub mod coverage;
mod cursor;
pub mod findings;
pub mod inventory;
pub mod json;
pub mod lexer;
pub mod literals;
pub mod names;
pub mod repository;
pub mod rules;
pub mod source_error;
pub mod structure;
pub mod syntax;
pub mod targets;
