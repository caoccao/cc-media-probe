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

//! What a crate that depends on the library cannot write with its public types (plan section 8.10): an exhaustive
//! match of an enum, and a struct literal. So a later version can add a variant, a field or a member without
//! breaking it (`api.md` section 14). Each public enum and struct has a doc test that must fail to compile and a
//! twin that must compile, as the maintainer chose for task 13 on 2026-11-27.
//!
//! The module is built only when rustdoc collects the doc tests and for the crate's own tests, which match each
//! enum's list exhaustively. A type that a later version adds joins the list of its module.

#[macro_use]
mod macros;

mod asynchronous;
mod completeness;
mod details;
mod diagnostics;
#[cfg(feature = "serde")]
mod document;
mod error;
mod model;
mod root;
mod source;
