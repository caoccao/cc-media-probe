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

//! `Usage`, what a probe charged to its budget.

/// What a probe charged to its budget: the measurements that the spike's tests assert on (plan section 7.5).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Usage {
  /// The bytes that every source of the probe delivered, including the bytes read only to skip forward.
  pub bytes_read: u64,
  /// The largest working memory that the probe's buffers held at one moment.
  pub peak_working_memory: u64,
  /// The working memory still charged when the report was made, 0 when every buffer returned its charge.
  pub working_memory_at_end: u64,
  /// The elements visited and the bytes that the codec parser examined.
  pub work_units: u64,
}
