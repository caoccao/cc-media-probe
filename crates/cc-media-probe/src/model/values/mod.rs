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

//! The value types of the normalized fields (`api.md` section 6.1).

mod constants;
mod estimate;
mod floating_point;
mod guid;
mod identifier;
mod media_type;
mod provenance;
mod rational;
mod seconds;
mod size;
mod time_range;
mod utc_time;

pub use estimate::{Estimate, EstimateMethod};
pub use floating_point::{FloatingPointValue, FloatingPointWidth};
pub use guid::Guid;
pub use identifier::Identifier;
pub use media_type::MediaType;
pub use provenance::Provenance;
pub use rational::Rational;
pub use seconds::Seconds;
pub use size::Size;
pub use time_range::TimeRange;
pub use utc_time::UtcTime;
