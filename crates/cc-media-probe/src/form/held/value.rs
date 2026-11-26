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

//! A member held before the version is known. It takes any value that a self-describing reader gives, null
//! included, and a map keeps every member, a repeated name included, so that nothing can fail to decode before the
//! version error (plan section 8.8). Once the version is checked, a held value is read through `Strict`, in the
//! order written, as a later member is read from the input: a null or a repeated name fails where it stands, so the
//! errors of a record do not depend on where its version stood (thirteenth review). Its depth is bounded where the
//! reader's is, as `serde_json`'s recursion limit bounds it.
//!
//! A value that JSON does not carry is held too, as the fifteenth review asked, so that it fails after the version
//! error rather than before it: a byte string, and a name that is not a string, such as an integer key of a CBOR
//! map, which ciborium reads. Since the sixteenth review, also a tagged value, which ciborium gives as an enum of one
//! value, and an integer beyond 64 bits, which it gives from a bignum. An enum is read as the newtype that ciborium
//! gives, and a failure of that read comes first, a shape that the reader refuses included, as the failure of any
//! read does (third and fourth reviews of the third slice's first part). Once the version is checked, each fails as
//! malformed input where it stands. A map keeps a name that is not a string in its place among the others, so a
//! nested root record held whole still reads its own version before it reaches such a name (sixteenth review).
//!
//! The private map of `serde_json`'s `arbitrary_precision` feature is held as written, as the maintainer chose at the
//! sixteenth review: whether it stands for a number is decided where it is read, by the type that reads it, exactly
//! as for a member after the version (`json-output.md` section 2).

use std::fmt;

use serde::de::{EnumAccess, Error, MapAccess, SeqAccess, VariantAccess, Visitor};
use serde::{Deserialize, Deserializer};

use crate::form::constants::HELD_VALUE_EXPECTING;
use crate::form::numbers::{Narrowed, narrow_signed, narrow_unsigned};

/// A value held as read.
pub(crate) enum HeldValue {
  /// Null, refused once the version is checked.
  Null,
  /// A Boolean.
  Boolean(bool),
  /// An integer that is not negative.
  Unsigned(u64),
  /// A negative integer.
  Signed(i64),
  /// A floating-point number.
  FloatingPoint(f64),
  /// A string.
  Text(String),
  /// A sequence.
  Sequence(Vec<HeldValue>),
  /// A map, its members in the order written. A name is held as text, or as a value that JSON does not carry.
  Map(Vec<(HeldValue, HeldValue)>),
  /// A value that JSON does not carry, refused once the version is checked: a byte string, a tagged value, an
  /// integer beyond 64 bits, or a member name that is not a string.
  NotJson,
}

impl<'input> Deserialize<'input> for HeldValue {
  fn deserialize<Input: Deserializer<'input>>(deserializer: Input) -> Result<HeldValue, Input::Error> {
    deserializer.deserialize_any(HeldValueVisitor)
  }
}

/// Reads any value, null included.
struct HeldValueVisitor;

impl<'input> Visitor<'input> for HeldValueVisitor {
  type Value = HeldValue;

  fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(HELD_VALUE_EXPECTING)
  }

  fn visit_bool<Failure>(self, value: bool) -> Result<HeldValue, Failure> {
    Ok(HeldValue::Boolean(value))
  }

  fn visit_i64<Failure: Error>(self, value: i64) -> Result<HeldValue, Failure> {
    Ok(held_integer(narrow_signed(i128::from(value))))
  }

  fn visit_u64<Failure>(self, value: u64) -> Result<HeldValue, Failure> {
    Ok(HeldValue::Unsigned(value))
  }

  /// An integer given as 128 bits: within 64 bits, the integer, and beyond them, such as a CBOR bignum, a value that
  /// JSON does not carry (sixteenth and nineteenth reviews).
  fn visit_i128<Failure>(self, value: i128) -> Result<HeldValue, Failure> {
    Ok(held_integer(narrow_signed(value)))
  }

  fn visit_u128<Failure>(self, value: u128) -> Result<HeldValue, Failure> {
    Ok(held_integer(narrow_unsigned(value)))
  }

  fn visit_f64<Failure>(self, value: f64) -> Result<HeldValue, Failure> {
    Ok(HeldValue::FloatingPoint(value))
  }

  fn visit_str<Failure>(self, value: &str) -> Result<HeldValue, Failure> {
    Ok(HeldValue::Text(value.to_owned()))
  }

  fn visit_string<Failure>(self, value: String) -> Result<HeldValue, Failure> {
    Ok(HeldValue::Text(value))
  }

  fn visit_bytes<Failure>(self, _value: &[u8]) -> Result<HeldValue, Failure> {
    Ok(HeldValue::NotJson)
  }

  fn visit_none<Failure>(self) -> Result<HeldValue, Failure> {
    Ok(HeldValue::Null)
  }

  fn visit_unit<Failure>(self) -> Result<HeldValue, Failure> {
    Ok(HeldValue::Null)
  }

  fn visit_some<Inner: Deserializer<'input>>(self, deserializer: Inner) -> Result<HeldValue, Inner::Error> {
    HeldValue::deserialize(deserializer)
  }

  fn visit_newtype_struct<Inner: Deserializer<'input>>(self, deserializer: Inner) -> Result<HeldValue, Inner::Error> {
    HeldValue::deserialize(deserializer)
  }

  /// A tagged value, which ciborium gives as an enum whose one variant holds the value: it is read whole, so the
  /// reader stays in place, and held as a value that JSON does not carry (sixteenth review). Whatever its content, an
  /// enum is a value that JSON does not carry, so it fails once the version is checked, after the version error.
  ///
  /// Every failure of the read is the reader's and comes first, as the maintainer chose at the fourth review of the
  /// third slice's first part (`json-output.md` section 2). That review read a missing colon in `{"name"}`, which
  /// Serde's `MapAccessDeserializer` gives as an enum: the reader failed before it handed the content over, which the
  /// third review's rule had held, while a failure inside the content stood. A reader that refuses the newtype shape
  /// fails before the hand-over too, and Serde gives no way to tell the two apart, so an enum of another shape, such
  /// as a unit variant of Serde's `EnumAccessDeserializer`, fails as the reader's refusal, also before the version.
  fn visit_enum<Access: EnumAccess<'input>>(self, access: Access) -> Result<HeldValue, Access::Error> {
    let (_variant, content): (HeldValue, _) = access.variant()?;
    content.newtype_variant::<HeldValue>()?;
    Ok(HeldValue::NotJson)
  }

  fn visit_seq<Items: SeqAccess<'input>>(self, items: Items) -> Result<HeldValue, Items::Error> {
    crate::form::items::collect_items(items).map(HeldValue::Sequence)
  }

  /// A map, whose names are read as any value, so that a name that is not a string is held as well, in its place.
  fn visit_map<Members: MapAccess<'input>>(self, mut members: Members) -> Result<HeldValue, Members::Error> {
    let mut held = Vec::new();
    while let Some((name, value)) = members.next_entry::<HeldValue, HeldValue>()? {
      let name = match name {
        HeldValue::Text(name) => HeldValue::Text(name),
        _ => HeldValue::NotJson,
      };
      held.push((name, value));
    }
    Ok(HeldValue::Map(held))
  }
}

/// An integer within 64 bits, or, beyond them, a value that JSON does not carry.
fn held_integer(narrowed: Option<Narrowed>) -> HeldValue {
  match narrowed {
    Some(Narrowed::Unsigned(value)) => HeldValue::Unsigned(value),
    Some(Narrowed::Signed(value)) => HeldValue::Signed(value),
    None => HeldValue::NotJson,
  }
}

#[cfg(test)]
mod tests {
  use serde::Deserialize;
  use serde::de::IntoDeserializer;
  use serde::de::value::{BytesDeserializer, EnumAccessDeserializer, Error, StrDeserializer, UnitDeserializer};

  use super::HeldValue;

  fn held<'input, Input: serde::Deserializer<'input, Error = Error>>(input: Input) -> Option<HeldValue> {
    HeldValue::deserialize(input).ok()
  }

  #[test]
  fn holds_every_value_that_a_self_describing_reader_gives() {
    assert!(matches!(held(true.into_deserializer()), Some(HeldValue::Boolean(true))));
    assert!(matches!(
      held((-1_i64).into_deserializer()),
      Some(HeldValue::Signed(-1))
    ));
    assert!(matches!(held(1_i64.into_deserializer()), Some(HeldValue::Unsigned(1))));
    assert!(matches!(
      held((-1_i128).into_deserializer()),
      Some(HeldValue::Signed(-1))
    ));
    assert!(matches!(held(1_u128.into_deserializer()), Some(HeldValue::Unsigned(1))));
    assert!(matches!(
      held((1_u128 << 64).into_deserializer()),
      Some(HeldValue::NotJson)
    ));
    assert!(matches!(
      held(1.5_f64.into_deserializer()),
      Some(HeldValue::FloatingPoint(_))
    ));
    assert!(matches!(held(StrDeserializer::new("x")), Some(HeldValue::Text(_))));
    assert!(matches!(held(BytesDeserializer::new(b"x")), Some(HeldValue::NotJson)));
    assert!(matches!(held(UnitDeserializer::new()), Some(HeldValue::Null)));
    // A unit variant refuses the newtype that an enum is read as, and the refusal is the reader's (fourth review).
    assert!(held(EnumAccessDeserializer::new(StrDeserializer::new("x"))).is_none());
    let some = serde_json::Value::from(Some(1_u8));
    assert!(matches!(
      HeldValue::deserialize(some).ok(),
      Some(HeldValue::Unsigned(1))
    ));
    let none = serde_json::Value::from(None::<u8>);
    assert!(matches!(HeldValue::deserialize(none).ok(), Some(HeldValue::Null)));
  }
}
