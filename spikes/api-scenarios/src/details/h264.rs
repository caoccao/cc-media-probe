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

//! The H.264 details of `api.md` section 11.5, with the accessors the scenarios read.

use super::shared::Conflict;

/// What the H.264 parameter sets and configuration record state.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct H264Details {
  sequence_parameter_sets: Vec<H264SequenceParameterSetDetails>,
  primary_sequence_parameter_set: Option<usize>,
  conflicts: Vec<Conflict>,
}

impl H264Details {
  /// Each sequence parameter set read, in the order read.
  pub fn sequence_parameter_sets(&self) -> &[H264SequenceParameterSetDetails] {
    &self.sequence_parameter_sets
  }

  /// The set that supplied the normalized fields.
  pub fn primary_sequence_parameter_set(&self) -> Option<&H264SequenceParameterSetDetails> {
    self
      .primary_sequence_parameter_set
      .and_then(|index| self.sequence_parameter_sets.get(index))
  }

  /// The raw values of every conflict.
  pub fn conflicts(&self) -> &[Conflict] {
    &self.conflicts
  }
}

/// One sequence parameter set. A set cut short keeps the fields read before the cut.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct H264SequenceParameterSetDetails {
  profile_indication: Option<u8>,
  constraint_set_flags: Option<u8>,
  level_indication: Option<u8>,
}

impl H264SequenceParameterSetDetails {
  /// `profile_idc`.
  pub fn profile_indication(&self) -> Option<u8> {
    self.profile_indication
  }

  /// `constraint_set0_flag` to `constraint_set5_flag`, bit 7 first.
  pub fn constraint_set_flags(&self) -> Option<u8> {
    self.constraint_set_flags
  }

  /// `level_idc`.
  pub fn level_indication(&self) -> Option<u8> {
    self.level_indication
  }
}
