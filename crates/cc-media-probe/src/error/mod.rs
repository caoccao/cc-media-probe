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

//! Errors (`api.md` section 10): the live `Error` that `probe_*` and `detect_*` return, re-exported at the crate
//! root with `ErrorKind`, and its serializable description `ErrorRecord`, with `CandidateCheck`, `CheckState` and
//! `Observations`. `Reason`, which an error and a candidate's check give, is the one of `completeness`, re-exported
//! here as `api.md` section 3.2 lists it.

mod candidate_check;
#[cfg(feature = "serde")]
mod candidates;
mod constants;
#[cfg(feature = "serde")]
mod detection_stops;
mod input_output_kind;
mod input_output_kinds;
mod kind;
#[cfg(feature = "serde")]
mod kind_rules;
mod live;
mod observations;
mod parts;
mod record;
#[cfg(feature = "serde")]
mod record_form;
#[cfg(feature = "serde")]
mod references;

pub use crate::completeness::Reason;
pub use candidate_check::{CandidateCheck, CheckState};
pub use kind::ErrorKind;
pub use live::Error;
pub use observations::Observations;
pub use record::ErrorRecord;
