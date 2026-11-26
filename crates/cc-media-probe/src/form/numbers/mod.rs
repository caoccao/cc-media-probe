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

//! The numbers of the form (`json-output.md` section 5.5): integers that a reader hands over as 128 bits, the text of
//! a number that `serde_json`'s `arbitrary_precision` hands over as a map, the twins of large quantities, and the
//! integers that are always decimal strings.

mod decimal;
mod text;
mod twin;
mod width;

pub(crate) use decimal::{canonical, read_decimal, write_decimal};
pub(crate) use text::{NumberText, is_not_finite, parse, unexpected};
pub(crate) use twin::{required_unsigned, signed_members, signed_value, unsigned_members, unsigned_value};
pub(crate) use width::{Narrowed, narrow_signed, narrow_unsigned};
