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

//! `positive`, which gives the constants of positive sizes their type without a panic.

use std::num::NonZeroUsize;

/// `value` as a positive size, or 1 for 0. The constants that use it are never 0, so the second case is never
/// taken; it exists because a constant cannot unwrap.
pub(crate) const fn positive(value: usize) -> NonZeroUsize {
  match NonZeroUsize::new(value) {
    Some(positive) => positive,
    None => NonZeroUsize::MIN,
  }
}
