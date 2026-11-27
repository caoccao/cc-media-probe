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

//! The async entry points of D15 (plan section 8.7): a caller's own async source, and one module per runtime
//! feature with a seekable and a forward-only entry point. Both runtimes' modules always exist here, so the spike
//! runs with both features of the library enabled.

pub mod futures;
mod source;
pub mod tokio;

pub(crate) use source::lookup;
pub use source::probe_source;
