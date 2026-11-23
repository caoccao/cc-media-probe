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
//! is read and its major checked, the held members are decoded from that form, and the members after the version are
//! decoded as they come. Both ways refuse a member that holds null and a map that names a member twice, at any depth:
//! the held ones through the kept form, which has no null, and the later ones through `Strict`, which wraps the
//! reader's own deserializer. The library's own output writes the version first, so it is read at its typed size,
//! and a reader's error keeps its position in the input.

mod held;
mod root;
mod strict;

pub(crate) use root::Rooted;
