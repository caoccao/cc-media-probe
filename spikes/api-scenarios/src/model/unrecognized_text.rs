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

//! The text of a catch-all, written and read by the enum that owns it, for `#[serde(with)]` on its variant. The
//! payload has no serialized form of its own, so no caller can read one for an enum other than the one it names.

use serde::{Deserialize, Deserializer, Serializer};

use super::unrecognized_value::UnrecognizedValue;

/// Writes the text as written.
pub(crate) fn serialize<Owner, Output: Serializer>(
  value: &UnrecognizedValue<Owner>,
  serializer: Output,
) -> Result<Output::Ok, Output::Error> {
  serializer.serialize_str(value.as_str())
}

/// Reads the text of a value that the enum `Owner` does not know.
pub(crate) fn deserialize<'input, Owner, Input: Deserializer<'input>>(
  deserializer: Input,
) -> Result<UnrecognizedValue<Owner>, Input::Error> {
  String::deserialize(deserializer).map(UnrecognizedValue::new)
}
