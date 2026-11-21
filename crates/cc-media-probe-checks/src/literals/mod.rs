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

//! The literal check of R33: every literal outside the exempt places lives in
//! a constants module, a constants module holds only constants, and a format
//! template holds no words.

mod check;
mod constants;
mod constants_module;
mod numbers;
mod places;
mod templates;

pub use check::check_literals;
pub use constants_module::{holds_only_constants, is_named_constants_module};
pub use numbers::is_zero_or_one;
pub use templates::template_has_words;
