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

//! The shape of the two open enums of the typed details (`api.md` section 11.1): one variant per format or codec,
//! the catch-all `Unrecognized`, the kind's stable name through `as_str()` (`api.md` section 1, rule 7), and the
//! three lists that every detail gives the same way (`api.md` section 11.3).
//!
//! One macro writes both enums, so their methods share one body (R36).

use super::{Conflict, ExactValue, ObservedBound};

/// The three lists that every typed detail keeps beside its own fields, which the enums give without matching on
/// a variant (`api.md` section 11.3).
pub(crate) trait SharedDetails {
  /// The raw values of each conflict between sources.
  fn conflicts(&self) -> &[Conflict];

  /// The observed lower bounds of totals that were not established for the whole.
  fn observed_lower_bounds(&self) -> &[ObservedBound];

  /// The values beyond the exact range.
  fn exact_values(&self) -> &[ExactValue];
}

/// Defines an open enum of typed details: its variants, each holding the detail type of a format or codec that
/// implements `SharedDetails` and naming the constant that holds its kind's serialized name, its catch-all, the
/// kind's name through `as_str()`, and the three lists.
macro_rules! details_enumeration {
  (
    $(#[$attribute:meta])*
    $name:ident {
      $($(#[$variant_attribute:meta])* $variant:ident($details:ty) => $kind:expr,)*
    }
  ) => {
    $(#[$attribute])*
    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    #[non_exhaustive]
    pub enum $name {
      $($(#[$variant_attribute])* $variant($details),)*
      /// The details of a kind that a newer version wrote, which keep the kind's name.
      Unrecognized($crate::details::UnrecognizedDetails<$name>),
    }

    impl $name {
      /// The kind's serialized name, which is stable, as the lowercase format or codec name (`api.md` section 2),
      /// or the name that the catch-all keeps.
      #[must_use]
      pub fn as_str(&self) -> &str {
        match self {
          $($name::$variant(_) => $kind,)*
          $name::Unrecognized(details) => details.kind(),
        }
      }

      /// The raw values of each conflict between sources (`api.md` section 11.3).
      #[must_use]
      pub fn conflicts(&self) -> &[$crate::details::Conflict] {
        $crate::details::SharedDetails::conflicts(self.shared())
      }

      /// The observed lower bounds of totals that were not established for the whole.
      #[must_use]
      pub fn observed_lower_bounds(&self) -> &[$crate::details::ObservedBound] {
        $crate::details::SharedDetails::observed_lower_bounds(self.shared())
      }

      /// The values beyond the exact range, as decimal strings.
      #[must_use]
      pub fn exact_values(&self) -> &[$crate::details::ExactValue] {
        $crate::details::SharedDetails::exact_values(self.shared())
      }

      /// The variant's details, through the lists that every detail keeps.
      fn shared(&self) -> &dyn $crate::details::SharedDetails {
        match self {
          $($name::$variant(details) => details,)*
          $name::Unrecognized(details) => details,
        }
      }
    }
  };
}

pub(crate) use details_enumeration;
