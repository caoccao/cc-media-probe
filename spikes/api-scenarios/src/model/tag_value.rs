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

//! A tag's value, `api.md` section 6.9 (F25), and its serialized form, section 12. An enum whose variants carry
//! data is a map of its `kind` and the member named after the kind, as in `{"kind": "text", "text": "Title"}`. A
//! reader that meets a kind it does not know keeps the kind's name and ignores the payload, as it ignores any
//! unknown member, so the order of the members never matters.

use std::fmt;

use super::constants::{BINARY_KIND, BOOLEAN_KIND, INTEGER_KIND, TEXT_KIND};
use super::values::UnrecognizedValue;
#[cfg(feature = "serde")]
use crate::constants::TAG_VALUE_MESSAGE;
#[cfg(feature = "serde")]
use crate::document::InvalidDocument;
#[cfg(feature = "serde")]
use crate::twin::{signed_members, signed_value};

/// A tag's value, in the format's own type (F25).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(into = "TagValueMembers", try_from = "TagValueMembers"))]
#[non_exhaustive]
pub enum TagValue {
  /// Text.
  Text(String),
  /// Binary data.
  Binary(BinaryValue),
  /// An integer.
  Integer(IntegerValue),
  /// A Boolean.
  Boolean(bool),
  /// A kind of value that a newer version wrote; it keeps the kind's name, and its payload is not read.
  Unrecognized(UnrecognizedValue),
}

impl TagValue {
  /// The serialized name of the kind, or the text of an unrecognized kind.
  pub fn as_str(&self) -> &str {
    match self {
      TagValue::Text(_) => TEXT_KIND,
      TagValue::Binary(_) => BINARY_KIND,
      TagValue::Integer(_) => INTEGER_KIND,
      TagValue::Boolean(_) => BOOLEAN_KIND,
      TagValue::Unrecognized(kind) => kind.as_str(),
    }
  }
}

/// A tag value's members: its kind, and the payload under the kind's name. An integer beyond the exact range has
/// the twin `integer_exact` in place of `integer`.
#[cfg(feature = "serde")]
#[derive(serde::Serialize, serde::Deserialize)]
struct TagValueMembers {
  kind: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  text: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  binary: Option<BinaryValue>,
  #[serde(skip_serializing_if = "Option::is_none")]
  integer: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  integer_exact: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  boolean: Option<bool>,
}

#[cfg(feature = "serde")]
impl From<TagValue> for TagValueMembers {
  fn from(value: TagValue) -> TagValueMembers {
    let mut members = TagValueMembers {
      kind: value.as_str().to_owned(),
      text: None,
      binary: None,
      integer: None,
      integer_exact: None,
      boolean: None,
    };
    match value {
      TagValue::Text(text) => members.text = Some(text),
      TagValue::Binary(binary) => members.binary = Some(binary),
      TagValue::Integer(integer) => {
        (members.integer, members.integer_exact) = signed_members(integer.magnitude, integer.negative);
      }
      TagValue::Boolean(boolean) => members.boolean = Some(boolean),
      TagValue::Unrecognized(_) => {}
    }
    members
  }
}

#[cfg(feature = "serde")]
impl TryFrom<TagValueMembers> for TagValue {
  type Error = InvalidDocument;

  /// A known kind holds the member named after it and no member of another known kind. An unknown kind keeps its
  /// name, and its members are ignored.
  fn try_from(members: TagValueMembers) -> Result<TagValue, InvalidDocument> {
    let TagValueMembers {
      kind,
      text,
      binary,
      integer,
      integer_exact,
      boolean,
    } = members;
    let has_integer = integer.is_some() || integer_exact.is_some();
    let present = [text.is_some(), binary.is_some(), has_integer, boolean.is_some()]
      .into_iter()
      .filter(|present| *present)
      .count();
    let value = match kind.as_str() {
      TEXT_KIND => text.map(TagValue::Text),
      BINARY_KIND => binary.map(TagValue::Binary),
      INTEGER_KIND if has_integer => {
        let (magnitude, negative) = signed_value(integer, integer_exact)?;
        Some(TagValue::Integer(IntegerValue { magnitude, negative }))
      }
      INTEGER_KIND => None,
      BOOLEAN_KIND => boolean.map(TagValue::Boolean),
      _ => return Ok(TagValue::Unrecognized(UnrecognizedValue::new(kind))),
    };
    match value {
      Some(value) if present == 1 => Ok(value),
      _ => Err(InvalidDocument::new(TAG_VALUE_MESSAGE)),
    }
  }
}

/// A binary tag value.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BinaryValue {
  size: u64,
  data: Option<Vec<u8>>,
}

impl BinaryValue {
  /// The size of the value, in bytes.
  pub fn size(&self) -> u64 {
    self.size
  }

  /// The bytes, when they were kept within the retained-data limit.
  pub fn data(&self) -> Option<&[u8]> {
    self.data.as_deref()
  }
}

/// Any 64-bit signed or unsigned integer.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IntegerValue {
  magnitude: u64,
  negative: bool,
}

impl IntegerValue {
  /// The value as `i64`, when it fits, `i64::MIN` included.
  pub fn to_i64(&self) -> Option<i64> {
    if self.negative {
      0_i64.checked_sub_unsigned(self.magnitude)
    } else {
      i64::try_from(self.magnitude).ok()
    }
  }

  /// The value as `u64`, when it is not negative.
  pub fn to_u64(&self) -> Option<u64> {
    (!self.negative).then_some(self.magnitude)
  }
}

impl fmt::Display for IntegerValue {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    if self.negative {
      write!(formatter, "-{}", self.magnitude)
    } else {
      write!(formatter, "{}", self.magnitude)
    }
  }
}
