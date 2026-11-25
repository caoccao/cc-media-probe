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
//! One macro writes them all, so the enums share one body (R36).

/// Defines a non-exhaustive enum whose variants carry at most one value, each with the serialized name of its kind,
/// and its catch-all.
///
/// The input is the enum's attributes, its name, and for each variant its attributes, its name, the type of its
/// value in parentheses where it has one, and the constant that holds its kind's serialized name.
macro_rules! data_enumeration {
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
  };
}

pub(crate) use data_enumeration;
