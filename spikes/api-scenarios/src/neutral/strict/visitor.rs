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

//! The visitor of `Strict`: it is given every value as the format holds it, since `Strict` asks for every value with
//! `deserialize_any`, and reads it as the type asked for, so that a value read from the input and one held before the
//! version reach the type through the same calls (seventeenth review). It wraps what holds further values, refuses
//! null, whether the reader gives it as an absent option or as a unit, and refuses every value that JSON does not
//! carry: a non-finite number, a byte string, an integer beyond 64 bits, and a tagged value, which ciborium gives
//! as an enum, since no format gives an enum to `deserialize_any` otherwise. An enum that the type asked for is a
//! variant's name or an object of one member (`super::variant`). When it reads a member's name, it also notes the
//! name, so that the map can refuse one given twice; a name is a string, as every name of JSON is, so a name of
//! another type, such as an integer key of CBOR, fails as a value that JSON does not carry (fifteenth review). When it
//! reads the value of a member named like serde_json's private map, it notes whether the value is the text of a number
//! that is not finite, while the value streams to its type, so a map of that name is never held to find out
//! (eighteenth review).

use std::fmt;

use serde::de::value::{BorrowedStrDeserializer, StrDeserializer, StringDeserializer};
use serde::de::{Deserializer, EnumAccess, Error, MapAccess, SeqAccess, Visitor};

use super::access::{StrictMap, StrictSequence};
use super::deserializer::Holder;
use super::variant::read_variant;
use crate::constants::{NOT_JSON_VALUE_MESSAGE, NULL_MEMBER_MESSAGE, POSITIONAL_RECORD_MESSAGE};
use crate::integer_width::{Narrowed, narrow_signed, narrow_unsigned};
use crate::number_text;

/// What a visitor notes about the value that it reads.
pub(super) enum Note<'slot> {
  /// Nothing: an ordinary value.
  Nothing,
  /// A member's name, which is a string: its text.
  Name(&'slot mut Option<String>),
  /// The value of the one member of a map named like serde_json's private map: whether it is the text of a number
  /// that is not finite, which the map then stands for.
  NotFinite(&'slot mut bool),
}

/// What the type asked for, which decides how the value that the format gives is read.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Request {
  /// No type: a value kept, held or ignored as written, such as a catch-all's.
  Any,
  /// A value of a known type other than a record or an enum.
  Typed,
  /// A record: an object of named members, which a positional array never is (thirteenth review).
  Record,
  /// An enum: a variant's name, or an object of one member named after the variant.
  Enum,
}

/// A visitor wrapped so that the values below it are read strictly too.
pub(super) struct StrictVisitor<'slot, Read> {
  read: Read,
  request: Request,
  note: Note<'slot>,
  holder: Holder,
}

impl<'slot, Read> StrictVisitor<'slot, Read> {
  /// The visitor of a value of that request, which notes what `note` asks for.
  pub(super) fn new(read: Read, request: Request, note: Note<'slot>, holder: Holder) -> StrictVisitor<'slot, Read> {
    StrictVisitor {
      read,
      request,
      note,
      holder,
    }
  }

  /// A name that is not a string: no name of JSON is one (fifteenth review).
  fn refuse_name<Failure: Error>(&self) -> Result<(), Failure> {
    match self.note {
      Note::Name(_) => not_json(),
      _ => Ok(()),
    }
  }

  /// An integer within 64 bits as JSON's reader gives it, or, beyond them, a value that JSON does not carry.
  fn visit_narrowed<'input, Failure: Error>(self, narrowed: Option<Narrowed>) -> Result<Read::Value, Failure>
  where
    Read: Visitor<'input>,
  {
    self.refuse_name()?;
    match narrowed {
      Some(Narrowed::Unsigned(value)) => self.read.visit_u64(value),
      Some(Narrowed::Signed(value)) => self.read.visit_i64(value),
      None => not_json(),
    }
  }

  /// Notes a member's name, or whether the private map's value is the text of a number that is not finite.
  fn note(&mut self, text: &str) {
    match &mut self.note {
      Note::Name(slot) => **slot = Some(text.to_owned()),
      Note::NotFinite(slot) => **slot = number_text::is_not_finite(text),
      Note::Nothing => {}
    }
  }
}

/// A value that JSON does not carry.
fn not_json<Value, Failure: Error>() -> Result<Value, Failure> {
  Err(Failure::custom(NOT_JSON_VALUE_MESSAGE))
}

/// Hands a value that holds no further values to the wrapped visitor as it came.
macro_rules! forward_values {
  ($($method:ident: $value:ty),* $(,)?) => {
    $(
      fn $method<Failure: Error>(self, value: $value) -> Result<Read::Value, Failure> {
        self.refuse_name()?;
        self.read.$method(value)
      }
    )*
  };
}

/// Refuses a value that JSON does not carry.
macro_rules! refuse_values {
  ($($method:ident: $value:ty),* $(,)?) => {
    $(
      fn $method<Failure: Error>(self, _value: $value) -> Result<Read::Value, Failure> {
        not_json()
      }
    )*
  };
}

/// Hands a string to the wrapped visitor, or, for an enum, the variant that it names.
macro_rules! forward_text {
  ($($method:ident: $value:ty => $variant:ident),* $(,)?) => {
    $(
      fn $method<Failure: Error>(mut self, value: $value) -> Result<Read::Value, Failure> {
        self.note(&value);
        match self.request {
          Request::Enum => self.read.visit_enum($variant::new(value)),
          _ => self.read.$method(value),
        }
      }
    )*
  };
}

impl<'input, Read: Visitor<'input>> Visitor<'input> for StrictVisitor<'_, Read> {
  type Value = Read::Value;

  fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    self.read.expecting(formatter)
  }

  forward_values! {
    visit_bool: bool, visit_i8: i8, visit_i16: i16, visit_i32: i32, visit_i64: i64, visit_u8: u8, visit_u16: u16,
    visit_u32: u32, visit_u64: u64, visit_char: char,
  }

  // A byte string: the form writes bytes as hexadecimal text (fourteenth and sixteenth reviews).
  refuse_values! {
    visit_bytes: &[u8], visit_borrowed_bytes: &'input [u8], visit_byte_buf: Vec<u8>,
  }

  /// An integer given as 128 bits: within 64 bits, the integer that JSON's reader gives, and beyond them, such as a
  /// CBOR bignum, a value that JSON does not carry (sixteenth and nineteenth reviews).
  fn visit_i128<Failure: Error>(self, value: i128) -> Result<Read::Value, Failure> {
    self.visit_narrowed(narrow_signed(value))
  }

  fn visit_u128<Failure: Error>(self, value: u128) -> Result<Read::Value, Failure> {
    self.visit_narrowed(narrow_unsigned(value))
  }

  forward_text! {
    visit_str: &str => StrDeserializer, visit_borrowed_str: &'input str => BorrowedStrDeserializer,
    visit_string: String => StringDeserializer,
  }

  /// A finite number passes; NaN and the infinities have no JSON form (fourteenth review).
  fn visit_f32<Failure: Error>(self, value: f32) -> Result<Read::Value, Failure> {
    if !value.is_finite() {
      return not_json();
    }
    self.refuse_name()?;
    self.read.visit_f32(value)
  }

  fn visit_f64<Failure: Error>(self, value: f64) -> Result<Read::Value, Failure> {
    if !value.is_finite() {
      return not_json();
    }
    self.refuse_name()?;
    self.read.visit_f64(value)
  }

  fn visit_none<Failure: Error>(self) -> Result<Read::Value, Failure> {
    Err(Failure::custom(NULL_MEMBER_MESSAGE))
  }

  fn visit_unit<Failure: Error>(self) -> Result<Read::Value, Failure> {
    Err(Failure::custom(NULL_MEMBER_MESSAGE))
  }

  /// A value that the format gives wrapped is read as the value itself, for the same request.
  fn visit_some<Inner: Deserializer<'input>>(self, deserializer: Inner) -> Result<Read::Value, Inner::Error> {
    deserializer.deserialize_any(self)
  }

  fn visit_newtype_struct<Inner: Deserializer<'input>>(self, deserializer: Inner) -> Result<Read::Value, Inner::Error> {
    deserializer.deserialize_any(self)
  }

  fn visit_seq<Items: SeqAccess<'input>>(self, items: Items) -> Result<Read::Value, Items::Error> {
    self.refuse_name()?;
    if self.request == Request::Record {
      return Err(Items::Error::custom(POSITIONAL_RECORD_MESSAGE));
    }
    self.read.visit_seq(StrictSequence::new(items, self.holder))
  }

  /// A map of a known type refuses the name of serde_json's private map, and one of no type the private map of a
  /// number that is not finite (`super::access`).
  fn visit_map<Members: MapAccess<'input>>(self, members: Members) -> Result<Read::Value, Members::Error> {
    self.refuse_name()?;
    let typed = self.request != Request::Any;
    let map = StrictMap::new(members, self.holder, typed);
    match self.request {
      Request::Enum => read_variant(self.read, map),
      _ => self.read.visit_map(map),
    }
  }

  /// A tagged value of ciborium, which JSON does not carry (sixteenth and seventeenth reviews).
  fn visit_enum<Access: EnumAccess<'input>>(self, _access: Access) -> Result<Read::Value, Access::Error> {
    not_json()
  }
}
