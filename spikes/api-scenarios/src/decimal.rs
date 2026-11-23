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

//! A count that is always a decimal string, `api.md` section 12, as the omitted counts of the fixed-size record are,
//! so that its JSON type never depends on its value, and a source identifier, which is a string like every
//! identifier. A field uses it with `#[serde(with = "crate::decimal")]`.

use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serializer};

use crate::constants::DECIMAL_MESSAGE;

/// Writes the count as its decimal text.
pub(crate) fn serialize<Output: Serializer>(count: &u64, serializer: Output) -> Result<Output::Ok, Output::Error> {
  serializer.collect_str(count)
}

/// Reads the count from canonical decimal text: digits with no sign and no leading zero.
pub(crate) fn deserialize<'input, Input: Deserializer<'input>>(deserializer: Input) -> Result<u64, Input::Error> {
  let text = String::deserialize(deserializer)?;
  match text.parse::<u64>() {
    Ok(count) if count.to_string() == text => Ok(count),
    _ => Err(Input::Error::custom(DECIMAL_MESSAGE)),
  }
}
