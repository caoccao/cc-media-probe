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

//! The trees of the report, chapters and tags, and the operations on them that never recurse on their depth
//! (`api.md` section 3.3; plan section 7.6).

#[cfg(feature = "serde")]
mod constants;
#[cfg(feature = "serde")]
mod flat;
mod operations;

#[cfg(feature = "serde")]
pub(crate) use flat::{read_flat, read_subtree, write_flat, write_subtree};
#[cfg(feature = "serde")]
pub(crate) use operations::depth_first;
pub(crate) use operations::{Branching, copy, equal, format, hash, release};
