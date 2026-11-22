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

//! Identifiers in the typed details, serialized as decimal strings at every coded width (`api.md` section 12). The
//! rule for identifiers comes before the rule that a narrower integer of the details is a number.

use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serializer};

/// Writes each identifier as a decimal string.
pub(super) fn serialize<Output: Serializer>(
  identifiers: &[u32],
  serializer: Output,
) -> Result<Output::Ok, Output::Error> {
  serializer.collect_seq(identifiers.iter().map(u32::to_string))
}

/// Reads each identifier from a decimal string.
pub(super) fn deserialize<'input, Input: Deserializer<'input>>(deserializer: Input) -> Result<Vec<u32>, Input::Error> {
  Vec::<String>::deserialize(deserializer)?
    .iter()
    .map(|text| text.parse().map_err(Input::Error::custom))
    .collect()
}
