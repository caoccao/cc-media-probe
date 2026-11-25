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

//! The source table and the types that name a source (`api.md` sections 6.3 and 6.15): `SourceEntry`,
//! `SourceIdentifier`, `SourceOffset`, `PortablePath`, `RawPath` and `RawPathEncoding`.

mod attempted_identity;
mod constants;
mod entry;
mod identifier;
mod offset;
mod outcome;
mod portable_path;
mod raw_path;
mod role;

pub use attempted_identity::AttemptedIdentity;
pub use entry::SourceEntry;
pub use identifier::SourceIdentifier;
pub use offset::SourceOffset;
pub use outcome::SourceOutcome;
pub use portable_path::PortablePath;
pub use raw_path::{RawPath, RawPathEncoding};
pub use role::SourceRole;
