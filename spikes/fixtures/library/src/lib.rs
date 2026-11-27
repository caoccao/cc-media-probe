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

//! The stand-in for the library in the fixture spike of plan Phase 3, task 3, for gate G3.2: a private toy walker
//! and codec parser, compiled only in repository builds and for fuzzing, the hidden `fuzzing_entry_points` that
//! fuzz targets and the seed check call, and the hidden `repository_entry_points` that integration tests call. A
//! build outside the repository, without cargo-fuzz, compiles none of them. See `../README.md`.

#![forbid(unsafe_code)]

#[cfg(any(fuzzing, cc_media_probe_repository))]
#[doc(hidden)]
pub mod fuzzing_entry_points;
#[cfg(cc_media_probe_repository)]
#[doc(hidden)]
pub mod repository_entry_points;
#[cfg(any(fuzzing, cc_media_probe_repository))]
mod toy;
