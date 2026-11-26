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

//! A value of the form read alone obeys the rules that it obeys inside a root record, as the thirteenth review of
//! `api.md` asked (sections 3.3 and 12): no null, no name given twice, and an object rather than a positional array.
//! Each type reads its value through `ReadAlone`, and its `Deserialize` goes through `read_alone`, which asks the
//! deserializer for the scope of the read (`super::scope`). Inside a read of the form, the deserializer is strict and
//! the value reads from it as it is. Outside one, the value reads from the deserializer wrapped in `Strict`, so each
//! map is checked once, also when the read starts inside another read's callback (fourteenth review). Every type of
//! the form reads so, an enum as well as a record, since a type that asked the format itself for its outermost value
//! gave ciborium a typed request, which drops a tag (eighteenth review).

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
pub(crate) fn read_alone<'input, Value: ReadAlone<'input>, Input: Deserializer<'input>>(
  deserializer: Input,
) -> Result<Value, Input::Error> {
  read_in_scope(deserializer, AloneRead::<Value>(PhantomData))
}

/// The read of one value in its scope.
struct AloneRead<Value>(PhantomData<Value>);

impl<'input, Value: ReadAlone<'input>> ScopedRead<'input> for AloneRead<Value> {
  type Value = Value;

  fn alone<Input: Deserializer<'input>>(self, deserializer: Input) -> Result<Value, Input::Error> {
    Value::read_members(Strict::value(deserializer, None))
  }

  fn within<Input: Deserializer<'input>>(self, deserializer: Input, _holder: Holder) -> Result<Value, Input::Error> {
    Value::read_members(deserializer)
  }
}

/// `ReadAlone` and `Deserialize` for types that read their value into a private form and check it with `TryFrom`,
/// such as a usage with its twins or a rational that must be reduced. The form's error is the message of the rule
/// that the value breaks.
macro_rules! checked_form {
  ($($value:ident from $form:ident),* $(,)?) => {
    $(
      impl<'input> $crate::form::ReadAlone<'input> for $value {
        fn read_members<Input: serde::Deserializer<'input>>(deserializer: Input) -> Result<$value, Input::Error> {
          let form = <$form as serde::Deserialize>::deserialize(deserializer)?;
          $value::try_from(form).map_err(<Input::Error as serde::de::Error>::custom)
        }
      }

      impl<'input> serde::Deserialize<'input> for $value {
        fn deserialize<Input: serde::Deserializer<'input>>(deserializer: Input) -> Result<$value, Input::Error> {
          $crate::form::read_alone(deserializer)
        }
      }
    )*
  };
}

pub(crate) use checked_form;

/// `Serialize`, `ReadAlone` and `Deserialize` for a newtype of `String` whose form is its text, such as an
/// identifier or a media type (`json-output.md` sections 5.3 and 5.4).
macro_rules! text_form {
  ($($value:ident),* $(,)?) => {
    $(
      impl serde::Serialize for $value {
        fn serialize<Output: serde::Serializer>(&self, serializer: Output) -> Result<Output::Ok, Output::Error> {
          serializer.serialize_str(&self.0)
        }
      }

      impl<'input> $crate::form::ReadAlone<'input> for $value {
        fn read_members<Input: serde::Deserializer<'input>>(deserializer: Input) -> Result<$value, Input::Error> {
          <String as serde::Deserialize>::deserialize(deserializer).map($value)
        }
      }

      impl<'input> serde::Deserialize<'input> for $value {
        fn deserialize<Input: serde::Deserializer<'input>>(deserializer: Input) -> Result<$value, Input::Error> {
          $crate::form::read_alone(deserializer)
        }
      }
    )*
  };
}

pub(crate) use text_form;
