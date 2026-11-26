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

//! The source table, the types that name a source, and the trait of a caller's own blocking source (`api.md`
//! sections 4.4, 6.3 and 6.15): `PositionalSource`, `SourceEntry`, `SourceIdentifier`, `SourceOffset`,
//! `PortablePath`, `RawPath` and `RawPathEncoding`.

mod attempted_identity;
mod constants;
mod entry;
mod identifier;
mod offset;
mod outcome;
mod portable_path;
mod positional;
mod raw_path;
#[cfg(feature = "serde")]
mod request_reasons;
mod role;
#[cfg(feature = "serde")]
mod table;

pub use attempted_identity::AttemptedIdentity;
#[cfg(feature = "serde")]
pub(crate) use constants::{BYTES_READ_MESSAGE, REQUEST_REASON_MESSAGE, UNKNOWN_SOURCE_MESSAGE};
pub use entry::SourceEntry;
pub use identifier::SourceIdentifier;
pub use offset::SourceOffset;
pub use outcome::SourceOutcome;
pub use portable_path::PortablePath;
pub use positional::PositionalSource;
#[cfg(test)]
pub(crate) use positional::tests::Ranges;
pub use raw_path::{RawPath, RawPathEncoding};
#[cfg(feature = "serde")]
pub(crate) use request_reasons::{request_reasons_not_kept, requests_agree};
pub use role::SourceRole;
#[cfg(feature = "serde")]
pub(crate) use table::{bytes_read_within, reasons_find_counters, table_identifiers};
