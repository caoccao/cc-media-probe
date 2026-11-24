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

//! The scope of a read of the form, as the fourteenth review asked: whether a read is under way, and the version of
//! the root record whose body is being decoded. Serde's derive passes nothing from a record to the records it holds,
//! so a record asks the deserializer that it was given, with a newtype request of a reserved name. The library's
//! strict deserializer answers with the version it carries and with itself, as a sequence of two; every format gives
//! the plain newtype, which says that no read of the form is under way. The scope therefore belongs to the
//! deserializer, never to the thread: an independent read, even one started inside a reader's callback while
//! another read waits for its bytes, has a scope of its own.

use std::fmt;
use std::marker::PhantomData;

use serde::Deserializer;
use serde::de::{DeserializeSeed, Error, SeqAccess, Visitor};

use super::strict::Holder;
use crate::constants::{READ_SCOPE_EXPECTING, READ_SCOPE_MESSAGE, READ_SCOPE_NAME};
use crate::document::SchemaVersion;

/// A read that depends on its scope.
pub(super) trait ScopedRead<'input>: Sized {
  /// What it reads.
  type Value;

  /// Outside a read of the form: the deserializer as the format gave it.
  fn alone<Input: Deserializer<'input>>(self, deserializer: Input) -> Result<Self::Value, Input::Error>;

  /// Inside a read of the form: a strict deserializer, within the body of a root record of that version, or of none.
  fn within<Input: Deserializer<'input>>(
    self,
    deserializer: Input,
    holder: Holder,
  ) -> Result<Self::Value, Input::Error>;
}

/// Asks the deserializer for its scope, and reads in it.
pub(super) fn read_in_scope<'input, Input: Deserializer<'input>, Read: ScopedRead<'input>>(
  deserializer: Input,
  read: Read,
) -> Result<Read::Value, Input::Error> {
  deserializer.deserialize_newtype_struct(READ_SCOPE_NAME, ScopeVisitor(read, PhantomData))
}

/// Takes either answer to the request for the scope.
struct ScopeVisitor<'input, Read>(Read, PhantomData<&'input ()>);

impl<'input, Read: ScopedRead<'input>> Visitor<'input> for ScopeVisitor<'input, Read> {
  type Value = Read::Value;

  fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(READ_SCOPE_EXPECTING)
  }

  /// A format's answer: no read of the form is under way.
  fn visit_newtype_struct<Input: Deserializer<'input>>(self, deserializer: Input) -> Result<Read::Value, Input::Error> {
    self.0.alone(deserializer)
  }

  /// The strict deserializer's answer: the version, then the deserializer itself.
  fn visit_seq<Answer: SeqAccess<'input>>(self, mut answer: Answer) -> Result<Read::Value, Answer::Error> {
    let text: String = answer
      .next_element()?
      .ok_or_else(|| Answer::Error::custom(READ_SCOPE_MESSAGE))?;
    let holder = SchemaVersion::try_from(text).ok();
    answer
      .next_element_seed(WithinSeed { read: self.0, holder })?
      .ok_or_else(|| Answer::Error::custom(READ_SCOPE_MESSAGE))
  }
}

/// Reads within the scope that the strict deserializer gave.
struct WithinSeed<Read> {
  read: Read,
  holder: Holder,
}

impl<'input, Read: ScopedRead<'input>> DeserializeSeed<'input> for WithinSeed<Read> {
  type Value = Read::Value;

  fn deserialize<Input: Deserializer<'input>>(self, deserializer: Input) -> Result<Read::Value, Input::Error> {
    self.read.within(deserializer, self.holder)
  }
}
