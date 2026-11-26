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

//! The shape of every record of the serialized form whose members Serde's derive writes and reads (`api.md`
//! sections 3.3 and 12): one definition gives the type and, with the `serde` feature, both of its traits.
//!
//! The type derives `Serialize`. Its `Deserialize` reads through the strict reader of `crate::form`, alone as
//! nested, and the members come from a private mirror of the same definition inside a block, which derives
//! `Deserialize` and becomes the type through `From`. So no inherent `deserialize` stands beside the trait, which
//! Serde's derive would give the type itself with `serde(remote = "Self")`, as the nineteenth review of `api.md`
//! found. The mirror is named `Form` in every block, so a member whose type names the record, as a tree's does,
//! names the record and not its mirror. One macro writes them all, so the records share one body (R36).
//!
//! The macro is defined in every build, since it also defines the type; what it adds for Serde stands behind the
//! `serde` feature.

/// Defines a record of the form: its attributes, its name with at most one type parameter, and its fields, each
/// with its attributes. After the definition, `pairs` names each field of a value with its provenance, which reading
/// requires together (`json-output.md` section 7.1), and a `check`, a function of the record that returns its
/// message as an error, runs on every value read. Both hold alone as inside a report (`api.md` section 3.3).
macro_rules! form_record {
  (
    $(#[$attribute:meta])*
    $visibility:vis struct $name:ident $(<$parameter:ident>)? {
      $($(#[$field_attribute:meta])* $field_visibility:vis $field:ident: $field_type:ty),* $(,)?
    }
    $(pairs = [$($value:ident with $provenance:ident),* $(,)?];)?
    $(check = $check:path;)?
  ) => {
    $(#[$attribute])*
    #[cfg_attr(feature = "serde", derive(serde::Serialize))]
    $visibility struct $name $(<$parameter>)? {
      $($(#[$field_attribute])* $field_visibility $field: $field_type,)*
    }

    #[cfg(feature = "serde")]
    const _: () = {
      #[derive(serde::Deserialize)]
      struct Form $(<$parameter>)? {
        $($(#[$field_attribute])* $field: $field_type,)*
      }

      impl $(<$parameter>)? From<Form $(<$parameter>)?> for $name $(<$parameter>)? {
        fn from(form: Form $(<$parameter>)?) -> Self {
          let Form { $($field),* } = form;
          $name { $($field),* }
        }
      }

      impl<'input $(, $parameter: serde::Deserialize<'input>)?> $crate::form::ReadAlone<'input>
        for $name $(<$parameter>)?
      {
        fn read_members<Input: serde::Deserializer<'input>>(deserializer: Input) -> Result<Self, Input::Error> {
          let value = Self::from(<Form $(<$parameter>)? as serde::Deserialize>::deserialize(deserializer)?);
          $($(
            if value.$value.is_some() != value.$provenance.is_some() {
              return Err(<Input::Error as serde::de::Error>::custom($crate::form::PROVENANCE_MESSAGE));
            }
          )*)?
          $($check(&value).map_err(<Input::Error as serde::de::Error>::custom)?;)?
          Ok(value)
        }
      }

      impl<'input $(, $parameter: serde::Deserialize<'input>)?> serde::Deserialize<'input> for $name $(<$parameter>)? {
        fn deserialize<Input: serde::Deserializer<'input>>(deserializer: Input) -> Result<Self, Input::Error> {
          $crate::form::read_alone(deserializer)
        }
      }
    };
  };
}

pub(crate) use form_record;
