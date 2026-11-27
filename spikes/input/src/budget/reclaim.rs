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

//! `Reclaim`, what gives back optional memory when a buffer that the probe requires does not fit.

use super::Budget;

/// The owner of optional memory: a block cache holds its blocks beyond the one used last only to save reads, so a
/// buffer that the probe requires may take them back (plan section 7.5).
pub(crate) trait Reclaim {
  /// Returns the optional memory with its charge, and says whether any was returned.
  fn reclaim(&mut self, budget: &mut Budget) -> bool;
}
