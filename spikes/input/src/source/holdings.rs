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

//! `Holdings`, what a source's buffers tell the walker besides its reads.

use crate::budget::Reclaim;

/// What a source holds: whether the byte before a position was delivered, and the optional memory that a buffer
/// the walk requires may take back. The walker passes it as a trait object, so the state machine's phases reach
/// their source without owning it.
pub(crate) trait Holdings: Reclaim {
  /// Whether the byte just before `position` was delivered and is still known, so a read at `position` that gave
  /// nothing found the end exactly there. The length is `position` exactly when that byte was delivered, or
  /// `position` is 0 (plan section 7.5, "The end of the input is observed").
  fn delivered_before(&self, position: u64) -> bool;
}
