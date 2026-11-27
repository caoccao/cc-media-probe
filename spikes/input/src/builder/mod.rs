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

//! Builders of toy files and of the toy codec's units, the draft of the test kit's builders of plan Phase 3, task 5.

mod constants;
mod toy_file;
mod units;

pub use toy_file::ToyFile;
pub use units::{parameters_body, parameters_unit, payload_unit, setup_body, setup_unit, unit};
