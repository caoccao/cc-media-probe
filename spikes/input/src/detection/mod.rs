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

//! Detection of the toy format: the file header, then the code of the first element. On a forward-only input both
//! samples must lie within the pinned prefix, so the bytes that detection read stay available to the walker (plan
//! section 8.6); a sample beyond it gives insufficient data with the range unavailable.

mod decide;
mod detect;

pub(crate) use decide::{Detected, judge_first_element, judge_header};
pub(crate) use detect::detect;
