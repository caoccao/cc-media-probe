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

//! `Strict` itself: each request goes to the wrapped deserializer with the visitor wrapped. A value that would be
//! ignored is still walked, so that null and a repeated name are refused in a member that no type reads, too.
//!
//! `Strict` also carries the scope of the read, as the fourteenth review asked: the version of the root record whose
//! body it reads, if any. A record of the form asks for it with a newtype request of a reserved name, which only
//! `Strict` answers, with the version and itself; every other deserializer gives the plain newtype that Serde's
//! formats give. So the scope belongs to the deserializer that reads, and an independent read on the same thread,
//! such as one inside a reader's callback, starts with none (`super::super::scope`).

use serde::de::{Deserializer, Visitor};

use super::scope_access::ScopeAccess;
use super::visitor::{NameSlot, StrictVisitor};
use crate::constants::READ_SCOPE_NAME;
use crate::document::SchemaVersion;

/// The version of the root record whose body a strict read is reading, if any.
pub(crate) type Holder = Option<SchemaVersion>;

/// A deserializer read strictly: no null, no member named twice and no record as an array, at any depth.
pub(crate) struct Strict<'name, Inner> {
  inner: Inner,
  name: NameSlot<'name>,
  holder: Holder,
}

impl<'name, Inner> Strict<'name, Inner> {
  /// The deserializer of a value, inside the body of a root record of that version, or of none.
  pub(crate) fn value(inner: Inner, holder: Holder) -> Strict<'name, Inner> {
    Strict {
      inner,
      name: None,
      holder,
    }
  }

  /// The deserializer of a member's name, which it notes in `slot`.
  pub(super) fn name(inner: Inner, slot: &'name mut Option<String>) -> Strict<'name, Inner> {
    Strict {
      inner,
      name: Some(slot),
      holder: None,
    }
  }
}

/// Hands a request without arguments of its own to the wrapped deserializer, with the visitor wrapped.
macro_rules! forward_strictly {
  ($($method:ident),* $(,)?) => {
    $(
      fn $method<Read: Visitor<'input>>(self, visitor: Read) -> Result<Read::Value, Inner::Error> {
        self.inner.$method(StrictVisitor::new(visitor, self.name, self.holder))
      }
    )*
  };
}

impl<'input, Inner: Deserializer<'input>> Deserializer<'input> for Strict<'_, Inner> {
  type Error = Inner::Error;

  forward_strictly! {
    deserialize_any, deserialize_bool, deserialize_i8, deserialize_i16, deserialize_i32, deserialize_i64,
    deserialize_i128, deserialize_u8, deserialize_u16, deserialize_u32, deserialize_u64, deserialize_u128,
    deserialize_f32, deserialize_f64, deserialize_char, deserialize_str, deserialize_string, deserialize_bytes,
    deserialize_byte_buf, deserialize_option, deserialize_unit, deserialize_seq, deserialize_map,
    deserialize_identifier,
  }

  /// A value that the type ignores is walked all the same, so a null or a repeated name inside it is refused.
  fn deserialize_ignored_any<Read: Visitor<'input>>(self, visitor: Read) -> Result<Read::Value, Inner::Error> {
    self
      .inner
      .deserialize_any(StrictVisitor::new(visitor, self.name, self.holder))
  }

  fn deserialize_unit_struct<Read: Visitor<'input>>(
    self,
    name: &'static str,
    visitor: Read,
  ) -> Result<Read::Value, Inner::Error> {
    self
      .inner
      .deserialize_unit_struct(name, StrictVisitor::new(visitor, self.name, self.holder))
  }

  /// The request of the reserved name asks for the scope, which `Strict` gives with itself; any other newtype goes
  /// to the wrapped deserializer.
  fn deserialize_newtype_struct<Read: Visitor<'input>>(
    self,
    name: &'static str,
    visitor: Read,
  ) -> Result<Read::Value, Inner::Error> {
    if name == READ_SCOPE_NAME {
      let holder = self.holder;
      return visitor.visit_seq(ScopeAccess::new(holder, Strict::value(self.inner, holder)));
    }
    self
      .inner
      .deserialize_newtype_struct(name, StrictVisitor::new(visitor, self.name, self.holder))
  }

  fn deserialize_tuple<Read: Visitor<'input>>(self, length: usize, visitor: Read) -> Result<Read::Value, Inner::Error> {
    self
      .inner
      .deserialize_tuple(length, StrictVisitor::new(visitor, self.name, self.holder))
  }

  fn deserialize_tuple_struct<Read: Visitor<'input>>(
    self,
    name: &'static str,
    length: usize,
    visitor: Read,
  ) -> Result<Read::Value, Inner::Error> {
    self
      .inner
      .deserialize_tuple_struct(name, length, StrictVisitor::new(visitor, self.name, self.holder))
  }

  fn deserialize_struct<Read: Visitor<'input>>(
    self,
    name: &'static str,
    fields: &'static [&'static str],
    visitor: Read,
  ) -> Result<Read::Value, Inner::Error> {
    self
      .inner
      .deserialize_struct(name, fields, StrictVisitor::record(visitor, self.holder))
  }

  fn deserialize_enum<Read: Visitor<'input>>(
    self,
    name: &'static str,
    variants: &'static [&'static str],
    visitor: Read,
  ) -> Result<Read::Value, Inner::Error> {
    self
      .inner
      .deserialize_enum(name, variants, StrictVisitor::new(visitor, self.name, self.holder))
  }

  fn is_human_readable(&self) -> bool {
    self.inner.is_human_readable()
  }
}
