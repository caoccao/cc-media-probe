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

//! `Prober` and its options (plan section 7.2; `api.md` sections 4.2 to 4.5): the builder, its blocking methods,
//! the limits and the names of their counters, and the two option enums. The crate root re-exports `Prober`,
//! `Limits` and `Limit`, and `model` the option enums.

mod attachment_handling;
mod blocking;
mod builder;
mod constants;
mod limit;
mod limits;
mod procedure;
mod section_request;

pub use attachment_handling::AttachmentHandling;
pub use builder::Prober;
pub use limit::Limit;
pub use limits::Limits;
pub use section_request::SectionRequest;
