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

//! The shape of every enum that carries data (`api.md` section 1, rules 5 and 7, and section 12): variants with or
//! without a value, the catch-all `Unrecognized` that keeps the kind a newer version wrote, and `as_str()`, the
//! kind's serialized name.
//!
//! One macro writes them all, so the enums share one body (R36). With the `serde` feature, each is a map of its
//! `kind` and the member named after the kind that holds its value, and a variant without a value is its `kind`
//! alone. The map is read as its members come, and a kind of a newer version keeps its other members as written and
//! writes them back (`json-output.md` section 6.2).

/// Defines a non-exhaustive enum whose variants carry at most one value, each with the serialized name of its kind,
/// and its catch-all.
///
/// The input is the enum's attributes, its name, and for each variant its attributes, its name, the type of its
/// value in parentheses where it has one, and the constant that holds its kind's serialized name. The arms that begin
/// with `@` write one variant's part of the form.
macro_rules! data_enumeration {
  (@reads $kind:ident, $name:ident, $text:expr) => {};
  (@reads $kind:ident, $name:ident, $text:expr, $value:ty) => {
    if $kind == $text {
      return <$value as $crate::form::KindValue>::reads($kind, $name);
    }
  };
  (@gather $kind:ident, $gathered:ident, $name:ident, $member:ident, $enumeration:ident, $variant:ident, $text:expr) => {};
  (
    @gather $kind:ident, $gathered:ident, $name:ident, $member:ident, $enumeration:ident, $variant:ident, $text:expr,
    $value:ty
  ) => {
    if $kind == $text {
      let value = <$value as $crate::form::KindValue>::read($kind, $name, $member)?;
      *$gathered = Some($enumeration::$variant(value));
      return Ok(());
    }
  };
  (@unit $enumeration:ident, $variant:ident, $text:expr) => {
    Some(($text, $enumeration::$variant))
  };
  (@unit $enumeration:ident, $variant:ident, $text:expr, $value:ty) => {
    None
  };
  (@write $self:ident, $map:ident, $enumeration:ident, $variant:ident, $text:expr) => {};
  (@write $self:ident, $map:ident, $enumeration:ident, $variant:ident, $text:expr, $value:ty) => {
    if let $enumeration::$variant(value) = $self {
      $crate::form::KindValue::write(value, $text, &mut $map)?;
    }
  };
  (
    $(#[$attribute:meta])*
    $name:ident {
      $($(#[$variant_attribute:meta])* $variant:ident $(($value:ty))? => $kind:expr,)*
    }
  ) => {
    $(#[$attribute])*
    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    #[non_exhaustive]
    pub enum $name {
      $($(#[$variant_attribute])* $variant $(($value))?,)*
      /// A kind that a newer version wrote, which keeps its name. The library never produces it while probing.
      Unrecognized($crate::model::UnrecognizedKind<$name>),
    }

    impl $name {
      /// The kind's serialized name, which is stable, or the name that the catch-all keeps.
      #[must_use]
      pub fn as_str(&self) -> &str {
        match self {
          $($name::$variant { .. } => $kind,)*
          $name::Unrecognized(kind) => kind.as_str(),
        }
      }
    }

    #[cfg(feature = "serde")]
    impl $crate::form::KindRead for $name {
      const KINDS: &'static [&'static str] = &[$($kind),*];

      fn reads(kind: &str, name: &str) -> bool {
        $($crate::data_enumeration::data_enumeration!(@reads kind, name, $kind $(, $value)?);)*
        false
      }

      fn gather<'input, Members: serde::de::MapAccess<'input>>(
        kind: &'static str,
        gathered: &mut Option<$name>,
        name: &str,
        member: $crate::form::MemberValue<'_, Members>,
      ) -> Result<(), Members::Error> {
        $($crate::data_enumeration::data_enumeration!(
          @gather kind, gathered, name, member, $name, $variant, $kind $(, $value)?
        );)*
        Ok(())
      }

      fn finish(kind: &'static str, gathered: Option<$name>) -> Option<$name> {
        gathered.or_else(|| {
          let units: &[Option<(&str, $name)>] = &[
            $($crate::data_enumeration::data_enumeration!(@unit $name, $variant, $kind $(, $value)?),)*
          ];
          units
            .iter()
            .flatten()
            .find(|(text, _)| *text == kind)
            .map(|(_, value)| value.clone())
        })
      }

      fn unrecognized(kind: String, members: $crate::form::KeptMembers) -> $name {
        $name::Unrecognized($crate::model::UnrecognizedKind::from_kept(kind, members))
      }
    }

    #[cfg(feature = "serde")]
    impl serde::Serialize for $name {
      fn serialize<Output: serde::Serializer>(&self, serializer: Output) -> Result<Output::Ok, Output::Error> {
        use serde::ser::SerializeMap;
        if let $name::Unrecognized(kind) = self {
          return kind.write_kept(serializer);
        }
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry($crate::form::KIND_MEMBER, self.as_str())?;
        $($crate::data_enumeration::data_enumeration!(@write self, map, $name, $variant, $kind $(, $value)?);)*
        map.end()
      }
    }

    #[cfg(feature = "serde")]
    impl<'input> $crate::form::ReadAlone<'input> for $name {
      fn read_members<Input: serde::Deserializer<'input>>(deserializer: Input) -> Result<$name, Input::Error> {
        $crate::form::read_by_kind(deserializer)
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

pub(crate) use data_enumeration;
