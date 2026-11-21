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

//! Where names are defined. Each module recognizes one kind of definition
//! site from the tokens; together they find the names that the code itself
//! chooses, so that names fixed by a dependency are not judged.

mod closures;
mod collector;
mod fields;
mod generics;
mod items;
mod matches;
mod parameters;
mod patterns;

pub use collector::defined_names;
