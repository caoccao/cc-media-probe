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

//! The tests of the public surface as a crate that depends on the library sees it: the traits that `api.md`
//! section 3.3 states, and the entry points that leave a reader or a source with its caller, as the Rust API
//! Guidelines checklist names them (`docs/design/api-guidelines-checklist.md`, section 15).

#[cfg(test)]
mod readers;
#[cfg(test)]
mod traits;
