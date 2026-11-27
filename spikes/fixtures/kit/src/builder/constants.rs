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

//! The values of the toy format that its builder writes (`spikes/input/README.md` section 2).

/// The signature, `CCToy` and a zero byte.
pub const SIGNATURE: &[u8] = b"CCToy\0";
/// The version that the builder writes.
pub const VERSION: u8 = 1;
/// The offset of the first element right after the header.
pub const FIRST_ELEMENT: u32 = 12;
/// The length of an element's code, four ASCII characters.
pub const CODE_LENGTH: usize = 4;
