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

//! Tags with their identity and targets (`api.md` section 6.9; plan section 8.5).

mod binary_value;
mod constants;
mod integer_value;
mod native_scope;
mod tag;
mod tag_key;
mod tag_name;
mod tag_set;
mod tag_value;
mod target;

pub use binary_value::BinaryValue;
pub use integer_value::IntegerValue;
pub use native_scope::NativeScope;
pub use tag::Tag;
pub use tag_key::TagKey;
pub use tag_name::TagName;
pub use tag_set::TagSet;
pub use tag_value::TagValue;
pub use target::{IdentifierNamespace, TargetLevel, TargetReference, TargetReferenceKind, TargetScope};
