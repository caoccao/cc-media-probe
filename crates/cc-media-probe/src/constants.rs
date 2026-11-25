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

//! Constants that several top-level modules of the library share (R33).

/// The largest magnitude that a normalized quantity can have, 2^53 - 1, so that every JSON reader holds it as an
/// exact number (R24; `api.md` section 7).
pub const LARGEST_EXACT: u64 = 9_007_199_254_740_991;
