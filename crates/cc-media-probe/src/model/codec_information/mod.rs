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

//! What a stream says about its codec (`api.md` section 6.4): `CodecInformation`, its candidates, its raw
//! identifiers and its profile, level and tier.

#[cfg(feature = "serde")]
mod agreement;
mod candidate;
mod constants;
mod information;
mod named_code;
mod raw_identifier;

pub use candidate::CodecCandidate;
pub use information::CodecInformation;
pub use named_code::{Level, Profile, Tier};
pub use raw_identifier::{CodecNamespace, RawCodecIdentifier};
