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

//! Fixtures as tests reach them (plan section 7.1): read by name from the fixtures' folder, which is found from the
//! kit's own manifest folder, with the seeds they give and the places of their expectations.

mod constants;
mod load;
mod location;

pub use constants::PARTS;
pub use load::{Fixture, FixtureError, Seed};
pub use location::{Expectation, Layout, expectation_path, fixture_path, folder, layout, layout_of, names};
