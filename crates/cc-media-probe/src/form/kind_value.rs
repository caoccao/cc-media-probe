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

//! The value of a kind of an enum that carries data, written as the member named after the kind (`json-output.md`
//! section 6.2). Most values have a form of their own and stand under that one member. Two do not: a tag's integer,
//! which `IntegerValue` writes only through the tag value that holds it, as the maintainer chose at the thirteenth
//! review of `api.md`, as `integer` or beyond 2^53 - 1 as the twin `integer_exact`, from -2^63 to 2^64 - 1; and an
//! observed count, which is always a decimal string, as a 64-bit integer of the typed details is (`api.md` section
//! 12).

use serde::de::{Error, MapAccess};
use serde::ser::SerializeMap;

use super::constants::{DECIMAL_MESSAGE, EXACT_SUFFIX};
use super::kept::MemberValue;
use super::numbers::{canonical, signed_members, signed_value};
use crate::model::{BinaryValue, ChromaSubsampling, EncodingName, FloatingPointValue, Guid, IntegerValue, Seconds};

/// The value of a kind, read from and written to the members beside `kind`.
pub(crate) trait KindValue: Sized {
  /// Whether a member of that name, beside a kind of that name, holds the value.
  fn reads(kind: &str, name: &str) -> bool {
    name == kind
  }

  /// Reads the value from the member of that name.
  fn read<'input, Members: MapAccess<'input>>(
    kind: &str,
    name: &str,
    value: MemberValue<'_, Members>,
  ) -> Result<Self, Members::Error>;

  /// Writes the members that hold the value, beside the kind of that name.
  fn write<Map: SerializeMap>(&self, kind: &str, map: &mut Map) -> Result<(), Map::Error>;
}

/// The values that have a form of their own, written as the one member named after the kind.
macro_rules! plain_kind_values {
  ($($value:ty),* $(,)?) => {
    $(
      impl KindValue for $value {
        fn read<'input, Members: MapAccess<'input>>(
          _kind: &str,
          _name: &str,
          value: MemberValue<'_, Members>,
        ) -> Result<$value, Members::Error> {
          value.read::<$value>()
        }

        fn write<Map: SerializeMap>(&self, kind: &str, map: &mut Map) -> Result<(), Map::Error> {
          map.serialize_entry(kind, self)
        }
      }
    )*
  };
}

plain_kind_values!(
  String,
  bool,
  BinaryValue,
  FloatingPointValue,
  Guid,
  Seconds,
  ChromaSubsampling,
  EncodingName
);

/// A twin's member name: the kind's with the suffix `_exact`.
fn exact_name(kind: &str) -> String {
  format!("{kind}{EXACT_SUFFIX}")
}

impl KindValue for IntegerValue {
  /// The integer is `integer`, or beyond 2^53 - 1 in magnitude `integer_exact` (`json-output.md` section 5.5).
  fn reads(kind: &str, name: &str) -> bool {
    name == kind || name == exact_name(kind)
  }

  fn read<'input, Members: MapAccess<'input>>(
    kind: &str,
    name: &str,
    value: MemberValue<'_, Members>,
  ) -> Result<IntegerValue, Members::Error> {
    let (number, twin) = if name == kind {
      (Some(value.read::<i64>()?), None)
    } else {
      (None, Some(value.read::<String>()?))
    };
    signed_value(number, twin)
      .map(IntegerValue::new)
      .map_err(Members::Error::custom)
  }

  fn write<Map: SerializeMap>(&self, kind: &str, map: &mut Map) -> Result<(), Map::Error> {
    match signed_members(self.value()) {
      (Some(number), _) => map.serialize_entry(kind, &number),
      (None, twin) => map.serialize_entry(&exact_name(kind), &twin),
    }
  }
}

impl KindValue for u64 {
  /// An observed count, always a decimal string.
  fn read<'input, Members: MapAccess<'input>>(
    _kind: &str,
    _name: &str,
    value: MemberValue<'_, Members>,
  ) -> Result<u64, Members::Error> {
    canonical(&value.read::<String>()?).ok_or_else(|| Members::Error::custom(DECIMAL_MESSAGE))
  }

  fn write<Map: SerializeMap>(&self, kind: &str, map: &mut Map) -> Result<(), Map::Error> {
    map.serialize_entry(kind, &self.to_string())
  }
}
