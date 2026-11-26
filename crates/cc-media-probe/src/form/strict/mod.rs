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

//! `Strict`, a deserializer that wraps another and refuses, at any depth, a member that holds null and a map that
//! names a member twice (plan section 8.8; RFC 7493 section 2.3). The form never writes null, so a member is present
//! when its name is, and null never stands for an absent value. It also refuses a record written as an array, a
//! value that JSON does not carry, and the name of `serde_json`'s private map in an object of a known type. It asks
//! for every value with `deserialize_any` and reads it as the type asked for, so a value held before the version and
//! one read after it meet the same rules through the same calls (`api.md` section 12).

mod access;
mod answer;
mod deserializer;
mod visitor;

pub(crate) use access::StrictMap;
pub(crate) use deserializer::{Holder, Strict};
