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

//! `SampleDescriptionProtection` and `SampleDescriptionState`, whether one sample description is protected
//! (`api.md` section 6.3).

use super::constants::{CLEAR_NAME, DECLARED_NAME, UNKNOWN_NAME};
use crate::form_record::form_record;
use crate::string_enumeration::string_enumeration;

form_record! {
  /// The established state of one sample description, as `Protection::sample_descriptions()` lists it.
  ///
  /// ```
  /// use cc_media_probe::model::{Protection, SampleDescriptionState};
  ///
  /// fn clear_descriptions(protection: &Protection) -> Vec<u64> {
  ///     protection
  ///         .sample_descriptions()
  ///         .iter()
  ///         .filter(|entry| matches!(entry.state(), SampleDescriptionState::Clear))
  ///         .map(|entry| entry.index())
  ///         .collect()
  /// }
  /// ```
  #[derive(Clone, Debug, PartialEq, Eq, Hash)]
  pub struct SampleDescriptionProtection {
    index: u64,
    state: SampleDescriptionState,
  }
  check = SampleDescriptionProtection::check_form;
}

impl SampleDescriptionProtection {
  /// The index of the description as the file numbers it, from 1 in the ISO family, and within the declared count
  /// when that is known.
  #[must_use]
  pub fn index(&self) -> u64 {
    self.index
  }

  /// The state of the description: `Clear` or `Declared`, or a state that a newer version wrote, but never
  /// `Unknown`, since the list holds no entry for an unknown description.
  #[must_use]
  pub fn state(&self) -> &SampleDescriptionState {
    &self.state
  }

  /// Whether the entry counts from 1 and holds a state that the list holds, never `unknown`, which needs the entry
  /// alone, so an entry read alone checks it as its protection does (review of the third slice's first part).
  #[cfg(feature = "serde")]
  fn check_form(&self) -> Result<(), &'static str> {
    if self.index >= 1 && !matches!(self.state, SampleDescriptionState::Unknown) {
      Ok(())
    } else {
      Err(super::constants::SAMPLE_DESCRIPTION_MESSAGE)
    }
  }
}

string_enumeration! {
  /// Whether one sample description is protected.
  ///
  /// ```
  /// use cc_media_probe::model::SampleDescriptionState;
  ///
  /// assert_eq!(SampleDescriptionState::Declared.as_str(), "declared");
  /// ```
  SampleDescriptionState {
    /// Examined, and it declares no protection.
    Clear => CLEAR_NAME,
    /// A declaration names its index. The retained-data limit can drop that declaration and keep this state, so
    /// `Protection::declarations()` can hold no declaration that names the index; `Protection::is_fully_examined()`
    /// is then false.
    Declared => DECLARED_NAME,
    /// No state entry was kept: the description was not read, or its state, clear or declared, was dropped.
    /// `Protection::sample_description_state()` gives it for an index in range that the sparse list holds no entry
    /// for; the list itself never holds it.
    Unknown => UNKNOWN_NAME,
  }
}

#[cfg(test)]
mod tests {
  use super::{SampleDescriptionProtection, SampleDescriptionState};
  use crate::model::UnrecognizedValue;

  impl SampleDescriptionProtection {
    /// The state of the description with this index.
    pub(crate) fn sample(index: u64, state: SampleDescriptionState) -> SampleDescriptionProtection {
      SampleDescriptionProtection { index, state }
    }
  }

  #[test]
  fn keeps_the_index_and_the_state() {
    let entry = SampleDescriptionProtection::sample(2, SampleDescriptionState::Clear);
    assert_eq!(entry.index(), 2);
    assert_eq!(entry.state(), &SampleDescriptionState::Clear);
    for (state, name) in [
      (SampleDescriptionState::Clear, "clear"),
      (SampleDescriptionState::Declared, "declared"),
      (SampleDescriptionState::Unknown, "unknown"),
      (
        SampleDescriptionState::Unrecognized(UnrecognizedValue::sample("partly")),
        "partly",
      ),
    ] {
      assert_eq!(state.as_str(), name);
    }
  }
}
