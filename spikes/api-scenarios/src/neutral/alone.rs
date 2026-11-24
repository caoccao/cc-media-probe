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

//! A record of the form read alone obeys the rules that it obeys inside a root record, as the thirteenth review asked
//! (`api.md` sections 3.3 and 12): no null, no name given twice, and an object rather than a positional array. Each
//! record type reads its members through `ReadAlone`, and its `Deserialize` goes through `read_alone`, which asks the
//! deserializer for the scope of the read (`super::scope`). Inside a read of the form, the deserializer is strict and
//! the record reads from it as it is. Outside one, the record reads from the deserializer wrapped in `Strict`, so each
//! map is checked once, also when the read starts inside another read's callback (fourteenth review).
//!
//! Since the eighteenth review every type of the form reads so, an enum as well as a record: an enum written as its
//! name, one that Serde's derive reads and one that carries data. A type that asked the format itself for its
//! outermost value gave ciborium a typed request, which drops a tag, so the review read tag 42 around a standalone
//! entry outcome `ok` as that outcome.
//!
//! A type whose members Serde's derive reads is written with `crate::form::strict_form!`, as the nineteenth review asked. The
//! stub had derived such a type with `serde(remote = "Self")`, which gives the type public inherent `serialize` and
//! `deserialize` functions beside the traits, so `Generator::deserialize` read the positional array
//! `["catalog", "1"]` that the trait refuses. The macro now derives `Serialize` on the type itself, and `Deserialize`
//! on a private mirror of the same definition inside a block, with `serde(remote)` naming the type, so the derive's
//! inherent `deserialize` belongs to the mirror, which nothing outside the block can name. The mirror is named
//! `Form` in every block, so a field that names the type, such as a catch-all's payload, names the type and not the
//! mirror. A record that checks its members after reading them, through a shadow type and `TryFrom`, gets its entry
//! from `checked_strict_traits!`; its shadow type is private as well.

use std::marker::PhantomData;

use serde::Deserializer;

use super::scope::{ScopedRead, read_in_scope};
use super::strict::{Holder, Strict};

/// A type's own reading of its value, from a deserializer that is already strict.
pub(crate) trait ReadAlone<'input>: Sized {
  /// Reads the value: a record's members, an enum's name or the members of an enum that carries data.
  fn read_members<Input: Deserializer<'input>>(deserializer: Input) -> Result<Self, Input::Error>;
}

/// Reads a value of the form, strictly whether it stands alone or inside a read already under way.
pub(crate) fn read_alone<'input, Record: ReadAlone<'input>, Input: Deserializer<'input>>(
  deserializer: Input,
) -> Result<Record, Input::Error> {
  read_in_scope(deserializer, AloneRead::<Record>(PhantomData))
}

/// The read of one record in its scope.
struct AloneRead<Record>(PhantomData<Record>);

impl<'input, Record: ReadAlone<'input>> ScopedRead<'input> for AloneRead<Record> {
  type Value = Record;

  fn alone<Input: Deserializer<'input>>(self, deserializer: Input) -> Result<Record, Input::Error> {
    Record::read_members(Strict::value(deserializer, None))
  }

  fn within<Input: Deserializer<'input>>(self, deserializer: Input, _holder: Holder) -> Result<Record, Input::Error> {
    Record::read_members(deserializer)
  }
}

/// `ReadAlone` and `Deserialize` for types that read their value into a shadow type and check it with `TryFrom`,
/// such as a rational that must be reduced, or an enum read from its name as a `String`; their `Serialize` stays
/// derived.
macro_rules! checked_strict_traits {
  ($($record:ident from $data:ident),* $(,)?) => {
    $(
      impl<'input> $crate::neutral::ReadAlone<'input> for $record {
        fn read_members<Input: serde::Deserializer<'input>>(deserializer: Input) -> Result<$record, Input::Error> {
          let data = <$data as serde::Deserialize>::deserialize(deserializer)?;
          $record::try_from(data).map_err(<Input::Error as serde::de::Error>::custom)
        }
      }

      impl<'input> serde::Deserialize<'input> for $record {
        fn deserialize<Input: serde::Deserializer<'input>>(deserializer: Input) -> Result<$record, Input::Error> {
          $crate::neutral::read_alone(deserializer)
        }
      }
    )*
  };
}

pub(crate) use checked_strict_traits;
