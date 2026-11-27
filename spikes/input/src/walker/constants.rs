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

//! The constants of the walker.

/// The source identifier of the primary input (plan section 7.7).
pub const PRIMARY_SOURCE: u64 = 0;
/// The work units that visiting one element costs.
pub const ELEMENT_WORK: u64 = 1;
/// The bytes of an identifier's key in the identifier set, which hold the longest identifier, 16 bytes.
pub const KEY_LENGTH: usize = size_of::<u128>();
/// The slot of the identifier set that holds no key: no identifier is empty, so none has this key.
pub const EMPTY_SLOT: u128 = 0;
/// The slots of the identifier set's first table, a power of two.
pub const MINIMUM_SLOTS: usize = 8;
/// How much the identifier set's table grows by, which keeps its size a power of two.
pub const SLOT_GROWTH: usize = 2;
/// The slots that the identifier set keeps for each key at least, so it is never more than half full.
pub const SLOTS_PER_KEY: usize = 2;
