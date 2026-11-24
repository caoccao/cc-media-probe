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

//! `Strict` itself. Since the seventeenth review, it asks the wrapped deserializer for every value with
//! `deserialize_any`, whatever the type requested, and tells its visitor what the type asked for. A format answers a
//! typed request in ways of its own: ciborium drops a tag around a value that a type asks for, and serde_json gives a
//! number of arbitrary precision as its private map only to `deserialize_any`. The neutral form that holds the
//! members before the version reads them with `deserialize_any` too, so a value reaches its type through the same
//! calls whether it stood before the version or after it. An option is present, since the form writes no null, and a
//! newtype is its value. A value that the type ignores is walked all the same, so that null, a repeated name and a
//! value that JSON does not carry are refused inside it as anywhere else; the type then gets the unit that it
//! ignores.
//!
//! `Strict` also carries the scope of the read, as the fourteenth review asked: the version of the root record whose
//! body it reads, if any. A record of the form asks for it with a newtype request of a reserved name, which only
//! `Strict` answers, with an enum whose identifier is a private static and whose variant carries the version and
//! `Strict` itself (nineteenth review); every format gives the plain newtype, and Serde's value deserializers give
//! the value itself. So the scope belongs to the deserializer that reads, and an independent read on the same thread,
//! such as one inside a reader's callback, starts with none (`super::super::scope`).

use serde::de::{Deserializer, IgnoredAny, Visitor};

use super::scope_access::ScopeAnswer;
use super::visitor::{Note, Request, StrictVisitor};
use crate::constants::READ_SCOPE_NAME;
use crate::document::SchemaVersion;

/// The version of the root record whose body a strict read is reading, if any.
pub(crate) type Holder = Option<SchemaVersion>;

/// A deserializer read strictly: no null, no member named twice, no record as an array and no value that JSON does
/// not carry, at any depth.
pub(crate) struct Strict<'slot, Inner> {
  inner: Inner,
  note: Note<'slot>,
  holder: Holder,
}

impl<'slot, Inner> Strict<'slot, Inner> {
  /// The deserializer of a value, inside the body of a root record of that version, or of none.
  pub(crate) fn value(inner: Inner, holder: Holder) -> Strict<'slot, Inner> {
    Strict::noting(inner, Note::Nothing, holder)
  }

  /// The deserializer of a value whose visitor notes what `note` asks for: a member's name, read with no holder, or
  /// whether the value of a map's member named like serde_json's private map is a number that is not finite.
  pub(super) fn noting(inner: Inner, note: Note<'slot>, holder: Holder) -> Strict<'slot, Inner> {
    Strict { inner, note, holder }
  }

  /// Asks the wrapped deserializer for the value as the format holds it, for a visitor of that request.
  fn read<'input, Read: Visitor<'input>>(self, request: Request, visitor: Read) -> Result<Read::Value, Inner::Error>
  where
    Inner: Deserializer<'input>,
  {
    self
      .inner
      .deserialize_any(StrictVisitor::new(visitor, request, self.note, self.holder))
  }
}

/// Hands a request without arguments of its own to the wrapped deserializer as `deserialize_any`.
macro_rules! read_as {
  ($request:ident: $($method:ident),* $(,)?) => {
    $(
      fn $method<Read: Visitor<'input>>(self, visitor: Read) -> Result<Read::Value, Inner::Error> {
        self.read(Request::$request, visitor)
      }
    )*
  };
}

impl<'input, Inner: Deserializer<'input>> Deserializer<'input> for Strict<'_, Inner> {
  type Error = Inner::Error;

  read_as! { Any: deserialize_any }

  read_as! {
    Typed: deserialize_bool, deserialize_i8, deserialize_i16, deserialize_i32, deserialize_i64, deserialize_i128,
    deserialize_u8, deserialize_u16, deserialize_u32, deserialize_u64, deserialize_u128, deserialize_f32,
    deserialize_f64, deserialize_char, deserialize_str, deserialize_string, deserialize_bytes, deserialize_byte_buf,
    deserialize_unit, deserialize_seq, deserialize_map, deserialize_identifier,
  }

  /// A member that is present holds a value, since the form writes no null; a null fails where the value is read.
  fn deserialize_option<Read: Visitor<'input>>(self, visitor: Read) -> Result<Read::Value, Inner::Error> {
    visitor.visit_some(self)
  }

  fn deserialize_ignored_any<Read: Visitor<'input>>(self, visitor: Read) -> Result<Read::Value, Inner::Error> {
    self.read(Request::Any, IgnoredAny)?;
    visitor.visit_unit()
  }

  fn deserialize_unit_struct<Read: Visitor<'input>>(
    self,
    _name: &'static str,
    visitor: Read,
  ) -> Result<Read::Value, Inner::Error> {
    self.read(Request::Typed, visitor)
  }

  /// The request of the reserved name asks for the scope, which `Strict` gives as the enum that only it gives, with
  /// itself; any other newtype is its value.
  fn deserialize_newtype_struct<Read: Visitor<'input>>(
    self,
    name: &'static str,
    visitor: Read,
  ) -> Result<Read::Value, Inner::Error> {
    if name == READ_SCOPE_NAME {
      let holder = self.holder;
      return visitor.visit_enum(ScopeAnswer::new(holder, Strict::value(self.inner, holder)));
    }
    visitor.visit_newtype_struct(self)
  }

  fn deserialize_tuple<Read: Visitor<'input>>(
    self,
    _length: usize,
    visitor: Read,
  ) -> Result<Read::Value, Inner::Error> {
    self.read(Request::Typed, visitor)
  }

  fn deserialize_tuple_struct<Read: Visitor<'input>>(
    self,
    _name: &'static str,
    _length: usize,
    visitor: Read,
  ) -> Result<Read::Value, Inner::Error> {
    self.read(Request::Typed, visitor)
  }

  fn deserialize_struct<Read: Visitor<'input>>(
    self,
    _name: &'static str,
    _fields: &'static [&'static str],
    visitor: Read,
  ) -> Result<Read::Value, Inner::Error> {
    self.read(Request::Record, visitor)
  }

  fn deserialize_enum<Read: Visitor<'input>>(
    self,
    _name: &'static str,
    _variants: &'static [&'static str],
    visitor: Read,
  ) -> Result<Read::Value, Inner::Error> {
    self.read(Request::Enum, visitor)
  }

  fn is_human_readable(&self) -> bool {
    self.inner.is_human_readable()
  }
}
