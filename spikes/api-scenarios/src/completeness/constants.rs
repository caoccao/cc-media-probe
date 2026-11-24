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

//! The constants of completeness: the declared order of the limits and of the parts, which the fixed-size record's
//! arrays keep (`api.md` sections 4.5 and 5.3; fourteenth review). The stub has a subset of each.

#[cfg(feature = "serde")]
use super::statuses::Part;
#[cfg(feature = "serde")]
use crate::limits::Limit;

/// The limits that this version knows, in the order of `api.md` section 4.5.
#[cfg(feature = "serde")]
pub(super) const LIMIT_ORDER: [Limit; 3] = [Limit::BytesRead, Limit::WorkingMemory, Limit::Streams];

/// The parts that this version knows, in the order of `api.md` section 5.3.
#[cfg(feature = "serde")]
pub(super) const PART_ORDER: [Part; 5] = [
  Part::Detection,
  Part::Discovery,
  Part::Headers,
  Part::Tags,
  Part::Chapters,
];
