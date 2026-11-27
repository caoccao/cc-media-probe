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

//! `widen` and `narrow`, which convert between `usize` and `u64` without a cast, saturating where a value does not
//! fit.

/// A size as a position or a count of bytes.
pub(crate) fn widen(value: usize) -> u64 {
  u64::try_from(value).unwrap_or(u64::MAX)
}

/// A position or a count of bytes as a size, saturated to `usize::MAX` where it does not fit.
pub(crate) fn narrow(value: u64) -> usize {
  usize::try_from(value).unwrap_or(usize::MAX)
}
