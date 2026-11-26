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

//! The elements of a sequence, collected in their order, for a value that the reader keeps or holds whole.

use serde::Deserialize;
use serde::de::SeqAccess;

/// The elements of the sequence, each read as a value of that type.
pub(crate) fn collect_items<'input, Items: SeqAccess<'input>, Value: Deserialize<'input>>(
  mut items: Items,
) -> Result<Vec<Value>, Items::Error> {
  let mut values = Vec::new();
  while let Some(value) = items.next_element()? {
    values.push(value);
  }
  Ok(values)
}
