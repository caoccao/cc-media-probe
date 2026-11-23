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

//! The neutral form of plan section 8.8, task 5: reading a root record's version before anything else is decoded,
//! wherever the member stands. JSON does not fix the order of an object's members, and tools reorder them, so the
//! reader does not rely on `schema_version` coming first.
//!
//! As the maintainer chose on 2026-11-23, only the members written before the version are held: each in a neutral
//! form that takes any value, null included, so that no member can fail before the version error. Once the version
//! is read and checked, the held members and then the members after the version are decoded in the order written,
//! all of them through `Strict`, which wraps the reader's own deserializer and refuses, at any depth, a member that
//! holds null, a map that names a member twice and a record written as an array. The library's own output writes the
//! version first, so it is read at its typed size, and a reader's error keeps its position in the input.
//!
//! Since the thirteenth review, a read keeps a scope: a nested root record compares its version with that of the
//! record that holds it before it decodes its body, and a record read alone is read strictly too.

mod alone;
mod held;
mod held_reader;
mod root;
mod scope;
mod strict;

pub(crate) use alone::{ReadAlone, checked_record_traits, read_alone, record_traits};
pub(crate) use root::Rooted;
