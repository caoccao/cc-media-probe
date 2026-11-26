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

//! `Protection`, what a stream declares protected and where (`api.md` section 6.3; F22).

use super::constants::UNKNOWN_STATE;
use super::{ProtectionDeclaration, SampleDescriptionProtection, SampleDescriptionState};

/// What a stream declares protected, and where (F22).
///
/// Absence is proof: `Stream::protection()` is absent only when every structure of the stream that can declare
/// protection was examined and none does. A probe that stopped before such a structure, or could not keep a
/// declaration, gives a `Protection` that says so, even when it holds no declaration.
///
/// ```
/// use cc_media_probe::Stream;
///
/// fn protection(stream: &Stream) -> String {
///     let Some(protection) = stream.protection() else {
///         return "clear".to_owned();
///     };
///     let schemes: Vec<&str> = protection
///         .declarations()
///         .iter()
///         .filter_map(|declaration| declaration.scheme())
///         .map(|scheme| scheme.value())
///         .collect();
///     let configuration = match protection.is_configuration_clear() {
///         Some(true) => "the configuration is clear",
///         Some(false) => "the configuration is protected",
///         None => "the configuration may be protected",
///     };
///     let examined = if protection.is_fully_examined() { "" } else { " (not fully examined)" };
///     if !protection.has_sample_descriptions() {
///         return format!("{schemes:?}; {configuration}{examined}");
///     }
///     let descriptions = protection.sample_description_count().unwrap_or(0);
///     let first = protection.sample_description_state(1).map(|state| state.as_str());
///     format!("{schemes:?}; {configuration}{examined}; {descriptions} descriptions, the first {first:?}")
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Protection {
  declarations: Vec<ProtectionDeclaration>,
  fully_examined: bool,
  has_sample_descriptions: bool,
  sample_description_count: Option<u64>,
  sample_descriptions: Vec<SampleDescriptionProtection>,
  configuration_clear: Option<bool>,
}

impl Protection {
  /// The declarations, in stored order. Empty only when `is_fully_examined()` is false.
  ///
  /// A description in the state `Declared` can have no declaration here that names its index: the retained-data
  /// limit can drop a declaration and keep the description's state, and `is_fully_examined()` is then false.
  #[must_use]
  pub fn declarations(&self) -> &[ProtectionDeclaration] {
    &self.declarations
  }

  /// Whether every structure that can declare protection was examined and every declaration kept.
  #[must_use]
  pub fn is_fully_examined(&self) -> bool {
    self.fully_examined
  }

  /// Whether the stream's format has sample descriptions, such as the sample entries of an ISO track: true in the
  /// ISO family and false for a Matroska track. The format decides it, so it is known even when the probe read
  /// nothing of the stream's protection. Without sample descriptions, the count is absent, the list is empty, and
  /// no declaration has the scope `SampleDescription`.
  #[must_use]
  pub fn has_sample_descriptions(&self) -> bool {
    self.has_sample_descriptions
  }

  /// The number of sample descriptions that the file declares, such as the `entry_count` of an ISO `stsd` box.
  /// Absent for a format without sample descriptions, and when the count was not read.
  #[must_use]
  pub fn sample_description_count(&self) -> Option<u64> {
    self.sample_description_count
  }

  /// The descriptions whose state was established and kept, in strictly increasing order of their indexes, each
  /// from 1 and within the declared count when that is known. The list is sparse: a description that it does not
  /// hold is unknown.
  #[must_use]
  pub fn sample_descriptions(&self) -> &[SampleDescriptionProtection] {
    &self.sample_descriptions
  }

  /// The state of the description with the index `index`, as the file numbers it:
  ///
  /// - the state of its entry, where `sample_descriptions()` holds that index;
  /// - `Unknown` for any other index from 1 to the declared count, and for any index from 1 on while the count of
  ///   a stream with sample descriptions is unknown: the list holds no state entry for such an index, whether the
  ///   description was not read or its state, clear or declared, was not kept;
  /// - `None` for an index outside the declared count, and for every index where `has_sample_descriptions()` is
  ///   false, whether or not the stream was fully examined.
  ///
  /// The list is in index order, so the lookup is a binary search.
  #[must_use]
  pub fn sample_description_state(&self, index: u64) -> Option<&SampleDescriptionState> {
    let position = self
      .sample_descriptions
      .binary_search_by_key(&index, SampleDescriptionProtection::index)
      .ok();
    if let Some(entry) = position.and_then(|position| self.sample_descriptions.get(position)) {
      return Some(entry.state());
    }
    let within = match self.sample_description_count {
      Some(count) => (1..=count).contains(&index),
      None => index >= 1 && self.has_sample_descriptions,
    };
    within.then_some(&UNKNOWN_STATE)
  }

  /// Whether the configuration that the header status describes is clear: `Some(false)` when a kept declaration
  /// covers it, `Some(true)` when every structure that could cover it was examined and none does, and `None`
  /// otherwise.
  #[must_use]
  pub fn is_configuration_clear(&self) -> Option<bool> {
    self.configuration_clear
  }
}

#[cfg(test)]
mod tests {
  use super::Protection;
  use crate::model::{ProtectionDeclaration, SampleDescriptionProtection, SampleDescriptionState};

  impl Protection {
    /// The protection of an ISO track whose first of two descriptions is protected and whose second is clear.
    pub(crate) fn sample() -> Protection {
      Protection {
        declarations: vec![ProtectionDeclaration::sample(1)],
        fully_examined: true,
        has_sample_descriptions: true,
        sample_description_count: Some(2),
        sample_descriptions: vec![
          SampleDescriptionProtection::sample(1, SampleDescriptionState::Declared),
          SampleDescriptionProtection::sample(2, SampleDescriptionState::Clear),
        ],
        configuration_clear: Some(true),
      }
    }

    /// The protection of a Matroska track whose blocks are encrypted and whose private data is clear: its format
    /// has no sample descriptions.
    pub(crate) fn matroska_sample() -> Protection {
      Protection {
        declarations: vec![ProtectionDeclaration::packets_sample()],
        fully_examined: true,
        has_sample_descriptions: false,
        sample_description_count: None,
        sample_descriptions: Vec::new(),
        configuration_clear: Some(true),
      }
    }
  }

  #[test]
  fn has_no_description_in_a_matroska_track() {
    let protection = Protection::matroska_sample();
    assert!(!protection.has_sample_descriptions());
    assert_eq!(protection.sample_description_count(), None);
    assert_eq!(protection.sample_descriptions(), []);
    assert!(
      protection
        .declarations()
        .iter()
        .all(|declaration| declaration.sample_description_index().is_none())
    );
    assert_eq!(protection.sample_description_state(1), None);
  }

  #[test]
  fn gives_the_state_of_each_description() {
    let protection = Protection::sample();
    assert_eq!(protection.declarations().len(), 1);
    assert!(protection.is_fully_examined());
    assert!(protection.has_sample_descriptions());
    assert_eq!(protection.sample_description_count(), Some(2));
    assert_eq!(protection.sample_descriptions().len(), 2);
    assert_eq!(protection.is_configuration_clear(), Some(true));
    assert_eq!(
      protection.sample_description_state(1),
      Some(&SampleDescriptionState::Declared)
    );
    assert_eq!(
      protection.sample_description_state(2),
      Some(&SampleDescriptionState::Clear)
    );
    assert_eq!(protection.sample_description_state(3), None);
    assert_eq!(protection.sample_description_state(0), None);
  }

  #[test]
  fn finds_each_entry_of_a_long_list() {
    let states = [SampleDescriptionState::Clear, SampleDescriptionState::Declared];
    let long = Protection {
      sample_description_count: Some(1_000),
      sample_descriptions: (1..=1_000)
        .filter(|index| index % 3 != 0)
        .map(|index| SampleDescriptionProtection::sample(index, states[usize::from(index % 2 == 0)].clone()))
        .collect(),
      ..Protection::sample()
    };
    for index in 1..=1_000 {
      let expected = match (index % 3, index % 2) {
        (0, _) => SampleDescriptionState::Unknown,
        (_, 0) => SampleDescriptionState::Declared,
        _ => SampleDescriptionState::Clear,
      };
      assert_eq!(long.sample_description_state(index), Some(&expected));
    }
    assert_eq!(long.sample_description_state(1_001), None);
  }

  #[test]
  fn leaves_a_description_unknown_that_was_not_kept() {
    let sparse = Protection {
      sample_descriptions: vec![SampleDescriptionProtection::sample(2, SampleDescriptionState::Clear)],
      fully_examined: false,
      configuration_clear: None,
      ..Protection::sample()
    };
    assert_eq!(
      sparse.sample_description_state(1),
      Some(&SampleDescriptionState::Unknown)
    );
    let uncounted = Protection {
      sample_description_count: None,
      ..sparse.clone()
    };
    assert_eq!(
      uncounted.sample_description_state(9),
      Some(&SampleDescriptionState::Unknown)
    );
    assert_eq!(uncounted.sample_description_state(0), None);
    let without_descriptions = Protection {
      declarations: Vec::new(),
      fully_examined: true,
      has_sample_descriptions: false,
      sample_description_count: None,
      sample_descriptions: Vec::new(),
      configuration_clear: Some(true),
    };
    assert!(!without_descriptions.has_sample_descriptions());
    assert_eq!(without_descriptions.sample_description_state(1), None);
  }

  #[test]
  fn keeps_a_declared_state_whose_declaration_was_dropped() {
    let dropped = Protection {
      declarations: Vec::new(),
      fully_examined: false,
      configuration_clear: None,
      ..Protection::sample()
    };
    assert_eq!(dropped.declarations(), []);
    assert!(!dropped.is_fully_examined());
    assert_eq!(
      dropped.sample_description_state(1),
      Some(&SampleDescriptionState::Declared)
    );
    assert_eq!(
      dropped.sample_description_state(2),
      Some(&SampleDescriptionState::Clear)
    );
  }

  #[test]
  fn knows_a_format_without_descriptions_before_examining_it() {
    let cut_short = Protection {
      declarations: Vec::new(),
      fully_examined: false,
      has_sample_descriptions: false,
      sample_description_count: None,
      sample_descriptions: Vec::new(),
      configuration_clear: None,
    };
    assert_eq!(cut_short.sample_description_state(1), None);
    assert_eq!(cut_short.sample_description_state(u64::MAX), None);
  }
}
