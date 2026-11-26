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

//! What a probe established, what it did not, and why (`api.md` sections 5.3 to 5.5; R23): `Completeness`, the
//! header status of a stream, the statuses, parts and reasons, and the fixed-size `LimitRecord`.

#[cfg(feature = "serde")]
mod agreement;
mod constants;
mod feature;
mod header_state;
mod header_status;
mod part;
mod part_reason;
mod reason;
mod reason_kind;
mod record;
mod statuses;
mod summary;

#[cfg(feature = "serde")]
pub(crate) use agreement::{Shortfall, omitted_at_least, stated_reasons_not_kept};
pub use feature::Feature;
pub use header_state::{HeaderState, Identification};
pub use header_status::HeaderStatus;
pub use part::Part;
pub use part_reason::PartReason;
pub use reason::Reason;
pub use reason_kind::ReasonKind;
pub use record::LimitRecord;
pub use statuses::{CompanionStatus, DetectionStatus, DiscoveryStatus, HeadersStatus, SectionStatus, TimingStatus};
pub use summary::Completeness;
