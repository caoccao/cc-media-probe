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

//! The scope of a read of the form: whether a read is under way, and the version of the root record whose body it
//! reads, which a value learns from the deserializer that it is given, never from the thread (`api.md` section 12,
//! after its fourteenth and nineteenth reviews).

mod identifier;
mod read;

pub(crate) use read::{ScopedRead, read_in_scope};
