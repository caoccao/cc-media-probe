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

//! The generated test files (plan section 9.1, `docs/guide/test-files.md`): their paths below `target/test-files/`
//! at the repository's root, or below another folder, with an error that names the generator when a file is
//! missing, and the paths of the recipes that describe them, whose `expected` members hold the values that their
//! tests expect.

mod constants;
mod error;
mod path;

pub use error::GeneratedError;
pub use path::{folder, path, path_in, recipe_path};
