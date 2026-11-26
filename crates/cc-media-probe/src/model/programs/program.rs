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

//! `Program` and `MapState`, a program of a multiplex (`api.md` section 6.11).

use super::constants::{COMPLETE_NAME, INCOMPLETE_NAME};
use crate::Seconds;
use crate::form_record::form_record;
use crate::model::{Identifier, Provenance};
use crate::string_enumeration::string_enumeration;

form_record! {
  /// A program of a multiplex, such as a service of a transport stream, from `MediaReport::programs()`. No format of
  /// `0.1.0` has programs.
  ///
  /// ```
  /// use cc_media_probe::MediaReport;
  ///
  /// fn programs(report: &MediaReport) {
  ///     for program in report.programs() {
  ///         println!(
  ///             "{:?} {:?} by {:?}: streams {:?}, clock {:?}, {:?} ({:?}), map {}",
  ///             program.program_identifier().map(|identifier| identifier.as_str()),
  ///             program.name(),
  ///             program.provider(),
  ///             program.streams(),
  ///             program.clock_reference_identifier().map(|identifier| identifier.as_str()),
  ///             program.duration(),
  ///             program.duration_provenance(),
  ///             program.map_state().as_str(),
  ///         );
  ///     }
  /// }
  /// ```
  #[derive(Clone, Debug, PartialEq, Eq, Hash)]
  pub struct Program {
    #[cfg_attr(feature = "serde", serde(rename = "program_identifier", skip_serializing_if = "Option::is_none"))]
    identifier: Option<Identifier>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    name: Option<String>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    provider: Option<String>,
    streams: Vec<usize>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    clock_reference_identifier: Option<Identifier>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    duration: Option<Seconds>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    duration_provenance: Option<Provenance>,
    map_state: MapState,
  }
  pairs = [
    duration with duration_provenance,
  ];
}

impl Program {
  /// The program's identifier, such as a program number.
  #[must_use]
  pub fn program_identifier(&self) -> Option<&Identifier> {
    self.identifier.as_ref()
  }

  /// The program's name.
  #[must_use]
  pub fn name(&self) -> Option<&str> {
    self.name.as_deref()
  }

  /// The program's provider.
  #[must_use]
  pub fn provider(&self) -> Option<&str> {
    self.provider.as_deref()
  }

  /// The indexes of the program's streams.
  #[must_use]
  pub fn streams(&self) -> &[usize] {
    &self.streams
  }

  /// The identifier of the stream that carries the program's clock reference.
  #[must_use]
  pub fn clock_reference_identifier(&self) -> Option<&Identifier> {
    self.clock_reference_identifier.as_ref()
  }

  /// The program's duration.
  #[must_use]
  pub fn duration(&self) -> Option<Seconds> {
    self.duration
  }

  /// Where the duration came from.
  #[must_use]
  pub fn duration_provenance(&self) -> Option<&Provenance> {
    self.duration_provenance.as_ref()
  }

  /// Whether the program's map of streams was read in full.
  #[must_use]
  pub fn map_state(&self) -> &MapState {
    &self.map_state
  }
}

string_enumeration! {
  /// Whether a program's map of streams was read in full. `Program` shows an example.
  ///
  /// ```
  /// use cc_media_probe::model::MapState;
  ///
  /// assert_eq!(MapState::Incomplete.as_str(), "incomplete");
  /// ```
  MapState {
    /// The map was read in full.
    Complete => COMPLETE_NAME,
    /// The map was cut short.
    Incomplete => INCOMPLETE_NAME,
  }
}

#[cfg(test)]
mod tests {
  use super::{MapState, Program};
  use crate::Seconds;
  use crate::model::{Identifier, Provenance};

  #[test]
  fn gives_each_field() {
    let program = Program {
      identifier: Some(Identifier::sample("1")),
      name: Some("News".to_owned()),
      provider: Some("Broadcaster".to_owned()),
      streams: vec![0, 1],
      clock_reference_identifier: Some(Identifier::sample("256")),
      duration: Some(Seconds::sample(60, 1)),
      duration_provenance: Some(Provenance::Derived),
      map_state: MapState::Complete,
    };
    assert_eq!(program.program_identifier().map(Identifier::as_str), Some("1"));
    assert_eq!(program.name(), Some("News"));
    assert_eq!(program.provider(), Some("Broadcaster"));
    assert_eq!(program.streams(), &[0, 1]);
    assert_eq!(
      program.clock_reference_identifier().map(Identifier::as_str),
      Some("256")
    );
    assert_eq!(program.duration(), Some(Seconds::sample(60, 1)));
    assert_eq!(program.duration_provenance(), Some(&Provenance::Derived));
    assert_eq!(program.map_state(), &MapState::Complete);
    assert_eq!(MapState::Incomplete.as_str(), "incomplete");
  }
}
