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

//! The repository-only entry points (plan section 7.1), compiled only under the repository marker: what integration
//! tests and benchmarks reach of the private walker. In the spike, the toy walk's outcome and its streams' header
//! states; the permanent list is in `docs/design/test-kit.md` section 7.

use crate::toy::walk;

/// The outcome of the toy walk over `bytes`: `report` or `truncated`, or detection's `unknown_format` or
/// `insufficient_data`.
#[must_use]
pub fn toy_outcome(bytes: &[u8]) -> &'static str {
  walk(bytes).outcome
}

/// Each declared stream's header state at the end of the toy walk over `bytes`, in order.
#[must_use]
pub fn toy_stream_outcomes(bytes: &[u8]) -> Vec<&'static str> {
  walk(bytes).streams
}
