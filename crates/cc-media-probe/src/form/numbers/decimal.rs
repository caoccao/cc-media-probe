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

//! Integers that are always decimal strings (`api.md` section 12): the omitted counts of the fixed-size record, so
//! that their JSON type never depends on their value, and integer identifiers, which are strings at every coded
//! width (R24). A field names `write_decimal` and `read_decimal` in its `serialize_with` and `deserialize_with`.

use std::fmt::Display;
use std::str::FromStr;

use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serializer};

use crate::form::constants::DECIMAL_MESSAGE;

/// A number of its type from canonical decimal text: ASCII digits with no plus sign and no leading zero, and a minus
/// sign only for a negative number of a signed type, within the type's width. It is the text that the number writes,
/// so a number has one spelling (`json-output.md` section 5.4).
pub(crate) fn canonical<Number: FromStr + Display>(text: &str) -> Option<Number> {
  text.parse::<Number>().ok().filter(|number| number.to_string() == text)
}

/// Writes the integer as its decimal text.
pub(crate) fn write_decimal<Number: Display, Output: Serializer>(
  number: &Number,
  serializer: Output,
) -> Result<Output::Ok, Output::Error> {
  serializer.collect_str(number)
}

/// Reads the integer from canonical decimal text within its width.
pub(crate) fn read_decimal<'input, Number: FromStr + Display, Input: Deserializer<'input>>(
  deserializer: Input,
) -> Result<Number, Input::Error> {
  canonical(&String::deserialize(deserializer)?).ok_or_else(|| Input::Error::custom(DECIMAL_MESSAGE))
}
