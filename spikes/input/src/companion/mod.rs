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

//! Companion lookup as plan section 7.7 and `api.md` section 13 design it, reduced to what the spike exercises: a
//! request per identifier-derived name, registered before anything is opened; a blocking resolver and an async one;
//! and the built-in resolver's check-then-open sequence with a trusted root, containment after resolving links,
//! and the second check.

mod built_in;
mod constants;
mod identifier;
mod lookup;
mod resolver;

pub use built_in::BuiltInResolver;
pub(crate) use built_in::{file_numbers, resolved_identity};
pub(crate) use constants::FILE_NUMBERS_LENGTH;
pub(crate) use identifier::{companion_name, is_valid};
pub(crate) use lookup::{AsyncLookup, BlockingLookup, Companions, Resolved};
pub use resolver::{
  Answer, AsyncCompanionResolver, AsyncResolution, CompanionLookup, CompanionRequest, CompanionResolver, Resolution,
};
