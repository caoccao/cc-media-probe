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

//! The strict reader of the serialized form (plan section 8.8; `api.md` section 12; `json-output.md` sections 2 and
//! 10), behind the `serde` feature. Every type of the form reads through it, alone as nested: a root record reads
//! its version before anything else is decoded, wherever the member stands, and every value refuses null, a member
//! named twice, a record written as an array and a value that JSON does not carry.

mod alone;
mod bytes;
mod constants;
mod each_once;
mod held;
mod items;
mod kept;
mod kind_value;
mod numbers;
mod root;
mod scope;
mod spelling;
mod strict;
#[cfg(test)]
mod tests;

pub(crate) use alone::{ReadAlone, checked_form, read_alone, text_form};
pub(crate) use bytes::{BorrowedBytes, read_optional_bytes, write_optional_bytes};
pub(crate) use constants::{KIND_MEMBER, MINUS_SIGN, PROVENANCE_MESSAGE, SCHEMA_VERSION_MEMBER, VERSION_FORM_MESSAGE};
pub(crate) use each_once::each_once;
pub(crate) use kept::{KeptMembers, KindRead, MemberValue, Outcome, WithKept, read_by_kind, write_kept};
pub(crate) use kind_value::KindValue;
pub(crate) use numbers::{canonical, read_decimal, required_unsigned, unsigned_members, unsigned_value, write_decimal};
pub(crate) use root::Rooted;
pub(crate) use spelling::{is_bytes, is_decimal, is_positive_decimal, is_whole_decimal};
