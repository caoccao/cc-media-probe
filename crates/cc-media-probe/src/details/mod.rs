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

//! The typed details (`api.md` section 11): `FormatDetails` and `CodecDetails`, the two open enums, and the shared
//! types through which every format and codec gives its conflicts, its observed lower bounds and its values beyond
//! the exact range. Each format and codec phase adds its variant and its submodule, such as `details::matroska`.

mod codec_details;
mod conflict;
mod constants;
#[cfg(feature = "serde")]
mod detail_value;
mod enumeration;
mod exact_value;
mod format_details;
#[cfg(feature = "serde")]
mod lowest_terms;
mod observed_bound;
#[cfg(feature = "serde")]
mod owner;
mod stored_value;
mod unrecognized;

pub use codec_details::CodecDetails;
pub use conflict::{Conflict, ConflictingValue};
pub(crate) use enumeration::SharedDetails;
pub use exact_value::{ExactInput, ExactValue};
pub use format_details::FormatDetails;
pub use observed_bound::{ObservedBound, ObservedValue};
pub use stored_value::StoredValue;
pub use unrecognized::UnrecognizedDetails;
