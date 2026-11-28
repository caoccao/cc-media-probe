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

//! JSON pointers (RFC 6901, section 3): a member's name is escaped, `~` as `~0` and `/` as `~1`, so that every
//! place in a record has exactly one pointer.

/// The pointer of the member `name` of the value at `pointer`.
pub fn member(pointer: &str, name: &str) -> String {
  format!("{pointer}/{}", name.replace('~', "~0").replace('/', "~1"))
}

/// The pointer of the entry `index` of the array at `pointer`.
pub fn entry(pointer: &str, index: usize) -> String {
  format!("{pointer}/{index}")
}
