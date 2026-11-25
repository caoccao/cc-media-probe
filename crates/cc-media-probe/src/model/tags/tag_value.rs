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

//! `TagValue`, the value of a tag (`api.md` section 6.9; F25).

use super::constants::{BINARY_KIND, BOOLEAN_KIND, FLOATING_POINT_KIND, GUID_KIND, INTEGER_KIND, TEXT_KIND};
use super::{BinaryValue, IntegerValue};
use crate::data_enumeration::data_enumeration;
use crate::model::{FloatingPointValue, Guid};

data_enumeration! {
  /// The value of a tag (F25), in the format's own value type: an ASF rating stored as a number is not the text of
  /// its digits, and a Boolean is no integer. An enum that carries data: serialized, it is a map of its `kind` and
  /// the member named after it (`json-output.md` section 6.2).
  ///
  /// ```
  /// use cc_media_probe::model::TagValue;
  ///
  /// fn describe(value: &TagValue) -> String {
  ///     match value {
  ///         TagValue::Text(text) => text.clone(),
  ///         TagValue::Binary(binary) => format!("{} bytes", binary.size()),
  ///         TagValue::Integer(integer) => integer.to_string(),
  ///         TagValue::FloatingPoint(number) => number.to_f64().to_string(),
  ///         TagValue::Boolean(flag) => flag.to_string(),
  ///         TagValue::Guid(guid) => guid.to_string(),
  ///         other => format!("a value of the kind {}", other.as_str()),
  ///     }
  /// }
  /// ```
  TagValue {
    /// Text.
    Text(String) => TEXT_KIND,
    /// Bytes.
    Binary(BinaryValue) => BINARY_KIND,
    /// An integer of 64 bits, signed or unsigned.
    Integer(IntegerValue) => INTEGER_KIND,
    /// A floating-point number exactly as stored.
    FloatingPoint(FloatingPointValue) => FLOATING_POINT_KIND,
    /// A Boolean.
    Boolean(bool) => BOOLEAN_KIND,
    /// A GUID.
    Guid(Guid) => GUID_KIND,
  }
}

#[cfg(test)]
mod tests {
  use super::TagValue;
  use crate::model::{BinaryValue, FloatingPointValue, Guid, IntegerValue, UnrecognizedKind};

  #[test]
  fn names_each_kind() {
    for (value, name) in [
      (TagValue::Text("Title".to_owned()), "text"),
      (TagValue::Binary(BinaryValue::sample(&[1])), "binary"),
      (TagValue::Integer(IntegerValue::signed(-1)), "integer"),
      (
        TagValue::FloatingPoint(FloatingPointValue::double(3.5)),
        "floating_point",
      ),
      (TagValue::Boolean(true), "boolean"),
      (TagValue::Guid(Guid::sample([0; 16])), "guid"),
      (TagValue::Unrecognized(UnrecognizedKind::sample("rating")), "rating"),
    ] {
      assert_eq!(value.as_str(), name);
    }
  }
}
