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

//! The shape of every enum whose values are strings (`api.md` section 1, rules 5 and 7): unit variants, the
//! catch-all `Unrecognized` that keeps the text a newer version wrote, and `as_str()`, the serialized name, with a
//! second text of each variant where an enum has one, such as the name for people.
//!
//! One macro writes them all, so the many enums of the model share one body (R36). With the `serde` feature, each is
//! its `snake_case` name, and a name that this version does not know reads into the catch-all, which writes it back
//! unchanged (`json-output.md` section 6.1).

/// Defines a non-exhaustive enum of unit variants, each with its serialized name, and its catch-all.
///
/// The input is the enum's attributes, its name, and for each variant its attributes, its name and the constant
/// that holds its serialized name. Its second form names a method in parentheses after the enum's name, with that
/// method's attributes and visibility, and gives each variant a second constant, which the method returns; for the
/// catch-all, the method returns the text as written.
macro_rules! string_enumeration {
  (
    $(#[$attribute:meta])*
    $name:ident ($(#[$second_attribute:meta])* $second_visibility:vis fn $second_method:ident) {
      $($(#[$variant_attribute:meta])* $variant:ident => $text:expr, $second_text:expr,)*
    }
  ) => {
    string_enumeration! {
      $(#[$attribute])*
      $name {
        $($(#[$variant_attribute])* $variant => $text,)*
      }
    }

    impl $name {
      $(#[$second_attribute])*
      #[must_use]
      $second_visibility fn $second_method(&self) -> &str {
        match self {
          $($name::$variant => $second_text,)*
          $name::Unrecognized(value) => value.as_str(),
        }
      }
    }
  };
  (
    $(#[$attribute:meta])*
    $name:ident {
      $($(#[$variant_attribute:meta])* $variant:ident => $text:expr,)*
    }
  ) => {
    $(#[$attribute])*
    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    #[non_exhaustive]
    pub enum $name {
      $($(#[$variant_attribute])* $variant,)*
      /// A value that a newer version wrote and this version does not know. It keeps the text as written, which
      /// `as_str()` returns. The library never produces it while probing.
      Unrecognized($crate::model::UnrecognizedValue<$name>),
    }

    impl $name {
      /// The serialized name, which is stable, or the text that the catch-all keeps.
      #[must_use]
      pub fn as_str(&self) -> &str {
        match self {
          $($name::$variant => $text,)*
          $name::Unrecognized(value) => value.as_str(),
        }
      }

      /// The value that a serialized name stands for, or the catch-all that keeps a name of a newer version.
      #[cfg(feature = "serde")]
      pub(crate) fn from_text(text: String) -> $name {
        $(
          if text == $text {
            return $name::$variant;
          }
        )*
        $name::Unrecognized($crate::model::UnrecognizedValue::from_text(text))
      }
    }

    #[cfg(feature = "serde")]
    impl serde::Serialize for $name {
      fn serialize<Output: serde::Serializer>(&self, serializer: Output) -> Result<Output::Ok, Output::Error> {
        serializer.serialize_str(self.as_str())
      }
    }

    #[cfg(feature = "serde")]
    impl<'input> $crate::form::ReadAlone<'input> for $name {
      fn read_members<Input: serde::Deserializer<'input>>(deserializer: Input) -> Result<$name, Input::Error> {
        <String as serde::Deserialize>::deserialize(deserializer).map($name::from_text)
      }
    }

    #[cfg(feature = "serde")]
    impl<'input> serde::Deserialize<'input> for $name {
      fn deserialize<Input: serde::Deserializer<'input>>(deserializer: Input) -> Result<$name, Input::Error> {
        $crate::form::read_alone(deserializer)
      }
    }
  };
}

pub(crate) use string_enumeration;
