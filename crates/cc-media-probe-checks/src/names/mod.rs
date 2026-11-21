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

//! The name check of R32: no single-letter names anywhere, and no word of a
//! defined name on the list of known abbreviations.

mod abbreviations;
mod check;
mod constants;
mod definitions;
mod words;

pub use abbreviations::{ABBREVIATIONS, FIXED_NAMES};
pub use check::check_names;
pub use definitions::defined_names;
pub use words::split_words;
