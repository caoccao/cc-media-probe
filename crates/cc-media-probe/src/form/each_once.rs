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

//! Whether a list names each of its values once (`json-output.md` sections 4.3, 4.4 and 7.2): the candidates of a
//! detection and of an error record by format, the limits and parts of the fixed-size record, and the names of the
//! members that a map keeps.

use std::collections::HashSet;
use std::hash::Hash;

/// Whether no two items have the same key, in time linear in the number of items, as the review of the third slice's
/// first part asked, where every item had been compared with every earlier one.
pub(crate) fn each_once<Item, Key: Hash + Eq + ?Sized>(items: &[Item], key: impl Fn(&Item) -> &Key) -> bool {
  let mut seen = HashSet::with_capacity(items.len());
  items.iter().all(|item| seen.insert(key(item)))
}
