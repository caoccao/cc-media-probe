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
//! reader keeps the members as written before it looks at the kind, so the order of the members never matters and
//! the kind alone decides how they are read. A kind that it does not know keeps its name and its other members,
//! unread, and writes them back unchanged.

use std::fmt;

#[cfg(feature = "serde")]
use serde::de::Error as _;
#[cfg(feature = "serde")]
use serde::ser::SerializeMap;
#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::constants::{BINARY_KIND, BOOLEAN_KIND, INTEGER_KIND, TEXT_KIND};
#[cfg(feature = "serde")]
use super::constants::{INTEGER_EXACT_MEMBER, TAG_VALUE_MEMBERS};
use super::unrecognized_kind::UnrecognizedKind;
#[cfg(feature = "serde")]
use crate::constants::{BINARY_DATA_MESSAGE, KIND_MEMBER, TAG_VALUE_MESSAGE, TWIN_MESSAGE};
#[cfg(feature = "serde")]
use crate::document::InvalidDocument;
#[cfg(feature = "serde")]
use crate::hexadecimal;
#[cfg(feature = "serde")]
use crate::kept::{read_kind, refuse_other_kinds, write_kept};
#[cfg(feature = "serde")]
use crate::twin::{signed_members, signed_value, unsigned_members, unsigned_value};

/// A tag's value, in the format's own type (F25).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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
  /// A kind of value that a newer version wrote. It keeps the kind's name and the value's other members, unread.
  Unrecognized(UnrecognizedKind<TagValue>),
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

/// Writes the kind, then the member named after it; a kind of a newer version writes its kept members back.
#[cfg(feature = "serde")]
impl Serialize for TagValue {
  fn serialize<Output: Serializer>(&self, serializer: Output) -> Result<Output::Ok, Output::Error> {
    if let TagValue::Unrecognized(unrecognized) = self {
      return write_kept(serializer, unrecognized.as_str(), unrecognized.members());
    }
    let mut map = serializer.serialize_map(None)?;
    map.serialize_entry(KIND_MEMBER, self.as_str())?;
    match self {
      TagValue::Text(text) => map.serialize_entry(TEXT_KIND, text)?,
      TagValue::Binary(binary) => map.serialize_entry(BINARY_KIND, binary)?,
      TagValue::Integer(integer) => {
        let (number, exact) = signed_members(integer.magnitude, integer.negative);
        if let Some(number) = number {
          map.serialize_entry(INTEGER_KIND, &number)?;
        }
        if let Some(exact) = exact {
          map.serialize_entry(INTEGER_EXACT_MEMBER, &exact)?;
        }
      }
      TagValue::Boolean(boolean) => map.serialize_entry(BOOLEAN_KIND, boolean)?,
      TagValue::Unrecognized(_) => {}
    }
    map.end()
  }
}

/// Keeps the members as written, then reads them as the kind says. A known kind holds the member named after it,
/// and once its own members are taken out, no member of another known kind may remain; other members are ignored,
/// as any unknown member is. A kind that this version does not know keeps its other members unread.
#[cfg(feature = "serde")]
impl<'input> Deserialize<'input> for TagValue {
  fn deserialize<Input: Deserializer<'input>>(deserializer: Input) -> Result<TagValue, Input::Error> {
    let (kind, mut members) = read_kind(deserializer, TAG_VALUE_MESSAGE)?;
    let malformed = || Input::Error::custom(TAG_VALUE_MESSAGE);
    let value = match kind.as_str() {
      TEXT_KIND => TagValue::Text(members.take::<_, Input::Error>(TEXT_KIND)?.ok_or_else(malformed)?),
      BINARY_KIND => TagValue::Binary(members.take::<_, Input::Error>(BINARY_KIND)?.ok_or_else(malformed)?),
      INTEGER_KIND => {
        let number = members.take::<_, Input::Error>(INTEGER_KIND)?;
        let exact = members.take::<_, Input::Error>(INTEGER_EXACT_MEMBER)?;
        let (magnitude, negative) = signed_value(number, exact).map_err(Input::Error::custom)?;
        TagValue::Integer(IntegerValue { magnitude, negative })
      }
      BOOLEAN_KIND => TagValue::Boolean(members.take::<_, Input::Error>(BOOLEAN_KIND)?.ok_or_else(malformed)?),
      _ => return Ok(TagValue::Unrecognized(UnrecognizedKind::new(kind, members))),
    };
    refuse_other_kinds::<Input::Error>(&members, &TAG_VALUE_MEMBERS, TAG_VALUE_MESSAGE)?;
    Ok(value)
  }
}

/// A binary tag value. Its size is a quantity that the value cannot omit, so beyond 2^53 - 1 it is the twin
/// `size_exact` (`api.md` section 12).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(into = "BinaryValueData"))]
pub struct BinaryValue {
  size: u64,
  data: Option<Vec<u8>>,
}

#[cfg(feature = "serde")]
crate::neutral::checked_record_traits!(BinaryValue from BinaryValueData);

/// A binary value's members as written: the size as its number or its twin, and the data, when it was kept, in
/// lowercase hexadecimal.
#[cfg(feature = "serde")]
#[derive(serde::Serialize, serde::Deserialize)]
struct BinaryValueData {
  #[serde(
    default,
    deserialize_with = "crate::presence::present",
    skip_serializing_if = "Option::is_none"
  )]
  size: Option<u64>,
  #[serde(
    default,
    deserialize_with = "crate::presence::present",
    skip_serializing_if = "Option::is_none"
  )]
  size_exact: Option<String>,
  #[serde(
    default,
    deserialize_with = "crate::presence::present",
    skip_serializing_if = "Option::is_none"
  )]
  data: Option<String>,
}

#[cfg(feature = "serde")]
impl TryFrom<BinaryValueData> for BinaryValue {
  type Error = InvalidDocument;

  /// Exactly one of the size's number and twin, each in its own range, and data, when present, of exactly that many
  /// bytes in lowercase hexadecimal (thirteenth review).
  fn try_from(read: BinaryValueData) -> Result<BinaryValue, InvalidDocument> {
    let size = unsigned_value(read.size, read.size_exact)?.ok_or(InvalidDocument::new(TWIN_MESSAGE))?;
    let data = match read.data {
      Some(text) => {
        let bytes = hexadecimal::decode(&text).ok_or(InvalidDocument::new(BINARY_DATA_MESSAGE))?;
        if u64::try_from(bytes.len()).ok() != Some(size) {
          return Err(InvalidDocument::new(BINARY_DATA_MESSAGE));
        }
        Some(bytes)
      }
      None => None,
    };
    Ok(BinaryValue { size, data })
  }
}

#[cfg(feature = "serde")]
impl From<BinaryValue> for BinaryValueData {
  fn from(value: BinaryValue) -> BinaryValueData {
    let (size, size_exact) = unsigned_members(Some(value.size));
    BinaryValueData {
      size,
      size_exact,
      data: value.data.as_deref().map(hexadecimal::encode),
    }
  }
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

/// Any 64-bit signed or unsigned integer. It has no serialized form of its own, as the maintainer chose at the
/// thirteenth review: the tag value that holds it writes it as `integer` or the twin `integer_exact` and checks its
/// bounds, from -2^63 to 2^64 - 1 (`api.md` section 3.3).
#[cfg_attr(
  feature = "serde",
  doc = "",
  doc = "With the `serde` feature, it still cannot be read alone:",
  doc = "",
  doc = "```compile_fail",
  doc = "let read: cc_media_probe::model::IntegerValue = serde_json::from_str(\"5\").unwrap();",
  doc = "```"
)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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
