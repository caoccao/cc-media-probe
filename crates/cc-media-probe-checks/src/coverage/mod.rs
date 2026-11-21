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

//! The coverage check of D23: line coverage above 90% for each published
//! crate and each library area (R28), from the LCOV report of
//! `cargo llvm-cov`, without the lines of embedded test blocks.

mod areas;
mod check;
mod constants;
mod lcov;
mod tally;

pub use areas::{area_of, crate_of};
pub use check::{CoverageReport, check_coverage};
pub use lcov::{FileRecord, parse_lcov};
pub use tally::Tally;
