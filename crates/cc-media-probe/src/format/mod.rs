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

//! `Format` and `SupportLevel`, the identity of a container format (`api.md` section 6.4). The crate root
//! re-exports `Format`, and `model` re-exports `SupportLevel`.

mod constants;
mod identity;
mod support_level;

pub use identity::Format;
pub use support_level::SupportLevel;
