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

//! `StoredValue`, a value with a default, which says whether the file stored it (`api.md` section 11.2; F1).
//! Serialized, it is a record of `value`, in the form of its type, so that a 64-bit integer is a decimal string
//! whatever its value, and `stored` (`json-output.md` sections 5.5 and 8).

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[cfg(feature = "serde")]
use super::detail_value::DetailValue;

/// The value of an element that a specification gives a default, which says whether the file stored it or the
/// effective default applies (F1). The effective default holds only once the element's omission is established:
/// its parent was read in full without it.
///
/// ```
/// use cc_media_probe::details::StoredValue;
///
/// fn describe(version: &StoredValue<u64>) -> String {
///     if version.is_stored() {
///         format!("version {}", version.value())
///     } else {
///         format!("version {} by default", version.value())
///     }
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct StoredValue<Value> {
  value: Value,
  stored: bool,
}

impl<Value> StoredValue<Value> {
  /// The value: stored, or the effective default.
  #[must_use]
  pub fn value(&self) -> &Value {
    &self.value
  }

  /// Whether the file stored the value, rather than leaving it to its default.
  #[must_use]
  pub fn is_stored(&self) -> bool {
    self.stored
  }
}

/// A stored value as written: its value in the form of its type, and the flag.
#[cfg(feature = "serde")]
#[derive(Serialize, Deserialize)]
struct StoredForm<Form> {
  value: Form,
  stored: bool,
}

#[cfg(feature = "serde")]
impl<Value: DetailValue> Serialize for StoredValue<Value> {
  fn serialize<Output: Serializer>(&self, serializer: Output) -> Result<Output::Ok, Output::Error> {
    StoredForm {
      value: self.value.to_form(),
      stored: self.stored,
    }
    .serialize(serializer)
  }
}

#[cfg(feature = "serde")]
impl<'input, Value: DetailValue> crate::form::ReadAlone<'input> for StoredValue<Value> {
  fn read_members<Input: Deserializer<'input>>(deserializer: Input) -> Result<StoredValue<Value>, Input::Error> {
    let form = StoredForm::<Value::Form>::deserialize(deserializer)?;
    let value = Value::from_form(form.value)
      .ok_or_else(|| <Input::Error as serde::de::Error>::custom(super::constants::DETAIL_VALUE_MESSAGE))?;
    Ok(StoredValue {
      value,
      stored: form.stored,
    })
  }
}

#[cfg(feature = "serde")]
impl<'input, Value: DetailValue> Deserialize<'input> for StoredValue<Value> {
  fn deserialize<Input: Deserializer<'input>>(deserializer: Input) -> Result<StoredValue<Value>, Input::Error> {
    crate::form::read_alone(deserializer)
  }
}

#[cfg(test)]
mod tests {
  use super::StoredValue;

  #[test]
  fn tells_a_stored_value_from_a_default() {
    let stored = StoredValue {
      value: 4_u64,
      stored: true,
    };
    assert_eq!((stored.value(), stored.is_stored()), (&4, true));
    let default = StoredValue {
      value: 1_u64,
      stored: false,
    };
    assert_eq!((default.value(), default.is_stored()), (&1, false));
  }
}
