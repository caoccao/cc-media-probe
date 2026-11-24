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
//! reader keeps the members that stand before the kind as written, so the order of the members never matters and the
//! kind alone decides how they are read; once the kind is known, its members are read as they come (nineteenth
//! review). A kind that it does not know keeps its name and its other members, unread, and writes them back
//! unchanged.

use std::fmt;

#[cfg(feature = "serde")]
use serde::de::MapAccess;
#[cfg(feature = "serde")]
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
use crate::kept::{KeptMembers, KindRead, MemberValue, read_by_kind, write_kept};
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

/// Read strictly, alone as inside a read, as the eighteenth review asked: read alone, a tag value took a member of
/// the name of serde_json's private map beside its kind, and dropped it.
#[cfg(feature = "serde")]
impl<'input> Deserialize<'input> for TagValue {
  fn deserialize<Input: Deserializer<'input>>(deserializer: Input) -> Result<TagValue, Input::Error> {
    crate::neutral::read_alone(deserializer)
  }
}

/// Reads the members as they come, once the kind is known; a member before the kind is kept until then (nineteenth
/// review). A known kind holds the member named after it, and no member of another known kind; other members are
/// passed by, as any unknown member is. A kind that this version does not know keeps its other members unread.
#[cfg(feature = "serde")]
impl<'input> crate::neutral::ReadAlone<'input> for TagValue {
  fn read_members<Input: Deserializer<'input>>(deserializer: Input) -> Result<TagValue, Input::Error> {
    read_by_kind(deserializer)
  }
}

/// What a kind of tag value that this version knows gathers from its members.
#[cfg(feature = "serde")]
pub(crate) enum TagGathered {
  Text(Option<String>),
  Binary(Option<BinaryValue>),
  Integer { number: Option<i64>, exact: Option<String> },
  Boolean(Option<bool>),
}

#[cfg(feature = "serde")]
impl KindRead for TagValue {
  type Gathered = TagGathered;

  const MEMBERS: &'static [&'static str] = &TAG_VALUE_MEMBERS;
  const MESSAGE: &'static str = TAG_VALUE_MESSAGE;

  fn start(kind: &str) -> Option<TagGathered> {
    match kind {
      TEXT_KIND => Some(TagGathered::Text(None)),
      BINARY_KIND => Some(TagGathered::Binary(None)),
      INTEGER_KIND => Some(TagGathered::Integer {
        number: None,
        exact: None,
      }),
      BOOLEAN_KIND => Some(TagGathered::Boolean(None)),
      _ => None,
    }
  }

  fn reads(gathered: &TagGathered, name: &str) -> bool {
    match gathered {
      TagGathered::Text(_) => name == TEXT_KIND,
      TagGathered::Binary(_) => name == BINARY_KIND,
      TagGathered::Integer { .. } => name == INTEGER_KIND || name == INTEGER_EXACT_MEMBER,
      TagGathered::Boolean(_) => name == BOOLEAN_KIND,
    }
  }

  fn gather<'input, Members: MapAccess<'input>>(
    gathered: &mut TagGathered,
    name: &str,
    value: MemberValue<'_, Members>,
  ) -> Result<(), Members::Error> {
    match gathered {
      TagGathered::Text(text) => *text = Some(value.read()?),
      TagGathered::Binary(binary) => *binary = Some(value.read()?),
      TagGathered::Integer { number, .. } if name == INTEGER_KIND => *number = Some(value.read()?),
      TagGathered::Integer { exact, .. } => *exact = Some(value.read()?),
      TagGathered::Boolean(boolean) => *boolean = Some(value.read()?),
    }
    Ok(())
  }

  fn finish<Failure: serde::de::Error>(gathered: TagGathered) -> Result<TagValue, Failure> {
    let malformed = || Failure::custom(TAG_VALUE_MESSAGE);
    Ok(match gathered {
      TagGathered::Text(text) => TagValue::Text(text.ok_or_else(malformed)?),
      TagGathered::Binary(binary) => TagValue::Binary(binary.ok_or_else(malformed)?),
      TagGathered::Integer { number, exact } => {
        let (magnitude, negative) = signed_value(number, exact).map_err(Failure::custom)?;
        TagValue::Integer(IntegerValue { magnitude, negative })
      }
      TagGathered::Boolean(boolean) => TagValue::Boolean(boolean.ok_or_else(malformed)?),
    })
  }

  fn unrecognized(kind: String, members: KeptMembers) -> TagValue {
    TagValue::Unrecognized(UnrecognizedKind::new(kind, members))
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
crate::neutral::checked_strict_traits!(BinaryValue from BinaryValueData);

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
