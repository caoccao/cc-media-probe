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

//! A JSON value.

/// A parsed JSON value. Numbers keep their text; members keep their order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum JsonValue {
  /// `null`.
  Null,
  /// `true` or `false`.
  Boolean(bool),
  /// A number, as written.
  Number(String),
  /// A string.
  Text(String),
  /// An array.
  Array(Vec<JsonValue>),
  /// An object.
  Object(Vec<(String, JsonValue)>),
}

impl JsonValue {
  /// The value of member `name`, if this is an object that has one.
  #[must_use]
  pub fn get(&self, name: &str) -> Option<&JsonValue> {
    match self {
      Self::Object(members) => members
        .iter()
        .find(|(member, _)| member == name)
        .map(|(_, value)| value),
      _ => None,
    }
  }

  /// The text, if this is a string.
  #[must_use]
  pub fn as_text(&self) -> Option<&str> {
    match self {
      Self::Text(text) => Some(text),
      _ => None,
    }
  }

  /// The elements, if this is an array; empty otherwise.
  #[must_use]
  pub fn elements(&self) -> &[JsonValue] {
    match self {
      Self::Array(elements) => elements,
      _ => &[],
    }
  }

  /// Whether this is `null`.
  #[must_use]
  pub fn is_null(&self) -> bool {
    matches!(self, Self::Null)
  }

  /// Moves the nested values of an array or object onto `pending`.
  fn move_children(&mut self, pending: &mut Vec<JsonValue>) {
    match self {
      Self::Array(elements) => pending.append(elements),
      Self::Object(members) => pending.extend(members.drain(..).map(|(_, value)| value)),
      _ => {}
    }
  }
}

/// Drops nested values one at a time with an explicit stack, so that a
/// deeply nested document cannot overflow the call stack.
impl Drop for JsonValue {
  fn drop(&mut self) {
    let mut pending = Vec::new();
    self.move_children(&mut pending);
    while let Some(mut value) = pending.pop() {
      value.move_children(&mut pending);
    }
  }
}

#[cfg(test)]
mod tests {
  use super::JsonValue;

  #[test]
  fn reads_members_texts_and_elements() {
    let value = JsonValue::Object(vec![
      ("name".to_owned(), JsonValue::Text("library".to_owned())),
      ("items".to_owned(), JsonValue::Array(vec![JsonValue::Null])),
    ]);
    assert_eq!(value.get("name").and_then(JsonValue::as_text), Some("library"));
    assert_eq!(
      value.get("items").map(JsonValue::elements).map(<[JsonValue]>::len),
      Some(1)
    );
    assert!(value.get("missing").is_none());
    assert!(JsonValue::Null.get("name").is_none());
    assert!(JsonValue::Null.as_text().is_none());
    assert_eq!(JsonValue::Null.elements().len(), 0);
    assert!(JsonValue::Null.is_null());
    assert!(!JsonValue::Boolean(true).is_null());
  }
}
