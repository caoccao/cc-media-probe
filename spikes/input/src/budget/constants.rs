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

//! The flags of the record of exhausted counters.

/// The read allowance ran out.
pub const BYTES_READ_FLAG: u8 = 0b0000_0001;
/// The working memory ran out.
pub const WORKING_MEMORY_FLAG: u8 = 0b0000_0010;
/// A header needed more than the per-stream header limit.
pub const HEADER_BYTES_FLAG: u8 = 0b0000_0100;
/// The work units ran out.
pub const WORK_UNITS_FLAG: u8 = 0b0000_1000;
/// How much a full list multiplies its capacity by when it grows, so the charges of a growing list stay few.
pub const LIST_GROWTH_FACTOR: usize = 2;
/// The capacity of a list's first reservation, so a list of one item charges one.
pub const MINIMUM_LIST_CAPACITY: usize = 1;
