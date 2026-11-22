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

//! The usage summary, `api.md` section 6.14, with two of its counters.

/// What the budget accounted for.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Usage {
  bytes_read: u64,
  peak_working_memory: u64,
}

impl Usage {
  /// The bytes read from every source.
  pub fn bytes_read(&self) -> u64 {
    self.bytes_read
  }

  /// The highest working memory in use at one time.
  pub fn peak_working_memory(&self) -> u64 {
    self.peak_working_memory
  }
}
