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

//! The H.264 details of `api.md` section 11.5, with the accessors the scenarios read. Since the twelfth review they
//! keep the picture parameter sets, the links that the first picture's activation read, and where each set came from.

use super::shared::Conflict;
use crate::model::UnrecognizedValue;

/// What the H.264 parameter sets and configuration record state.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct H264Details {
  sequence_parameter_sets: Vec<H264SequenceParameterSetDetails>,
  #[cfg_attr(feature = "serde", serde(default))]
  picture_parameter_sets: Vec<H264PictureParameterSetDetails>,
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
  primary_sequence_parameter_set: Option<usize>,
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
  activation: Option<H264ActivationDetails>,
  conflicts: Vec<Conflict>,
}

impl H264Details {
  /// Each sequence parameter set read, in the order read.
  pub fn sequence_parameter_sets(&self) -> &[H264SequenceParameterSetDetails] {
    &self.sequence_parameter_sets
  }

  /// Each picture parameter set read, in the order read, a set cut short with the fields read before the cut.
  pub fn picture_parameter_sets(&self) -> &[H264PictureParameterSetDetails] {
    &self.picture_parameter_sets
  }

  /// The set that supplied the normalized fields.
  pub fn primary_sequence_parameter_set(&self) -> Option<&H264SequenceParameterSetDetails> {
    self
      .primary_sequence_parameter_set
      .and_then(|index| self.sequence_parameter_sets.get(index))
  }

  /// The identifiers that the first picture's activation named, as read; absent when no picture was read.
  pub fn activation(&self) -> Option<&H264ActivationDetails> {
    self.activation.as_ref()
  }

  /// The raw values of every conflict.
  pub fn conflicts(&self) -> &[Conflict] {
    &self.conflicts
  }
}

/// Where a parameter set was read.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum H264ParameterSetOrigin {
  /// The configuration record, such as an `avcC` box or a Matroska `CodecPrivate`.
  ConfigurationRecord,
  /// A sample of the stream itself, as `avc3` allows.
  Sample,
  /// A sample of the parameter-set track that the track links through `avcp`.
  ParameterSetTrack,
  /// A value that a newer version wrote.
  #[cfg_attr(feature = "serde", serde(with = "crate::model::unrecognized_text"))]
  Unrecognized(UnrecognizedValue<H264ParameterSetOrigin>),
}

/// One sequence parameter set. A set cut short keeps the fields read before the cut.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct H264SequenceParameterSetDetails {
  origin: H264ParameterSetOrigin,
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
  profile_indication: Option<u8>,
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
  constraint_set_flags: Option<u8>,
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
  level_indication: Option<u8>,
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
  sequence_parameter_set_identifier: Option<u32>,
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
  picture_width_in_macroblocks_minus_one: Option<u32>,
}

impl H264SequenceParameterSetDetails {
  /// Where the set was read.
  pub fn origin(&self) -> &H264ParameterSetOrigin {
    &self.origin
  }

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

  /// `seq_parameter_set_id`.
  pub fn sequence_parameter_set_identifier(&self) -> Option<u32> {
    self.sequence_parameter_set_identifier
  }

  /// `pic_width_in_mbs_minus1`, which follows `seq_parameter_set_id` (ITU-T H.264 7.3.2.1.1), so a primary set can
  /// end before it (tenth review).
  pub fn picture_width_in_macroblocks_minus_one(&self) -> Option<u32> {
    self.picture_width_in_macroblocks_minus_one
  }
}

/// One picture parameter set. A set cut short keeps the fields read before the cut: `pic_parameter_set_id` comes
/// first and `seq_parameter_set_id` second (ITU-T H.264 7.3.2.2).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct H264PictureParameterSetDetails {
  origin: H264ParameterSetOrigin,
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
  picture_parameter_set_identifier: Option<u32>,
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
  sequence_parameter_set_identifier: Option<u32>,
}

impl H264PictureParameterSetDetails {
  /// Where the set was read.
  pub fn origin(&self) -> &H264ParameterSetOrigin {
    &self.origin
  }

  /// `pic_parameter_set_id`.
  pub fn picture_parameter_set_identifier(&self) -> Option<u32> {
    self.picture_parameter_set_identifier
  }

  /// `seq_parameter_set_id`, the sequence parameter set that this set names.
  pub fn sequence_parameter_set_identifier(&self) -> Option<u32> {
    self.sequence_parameter_set_identifier
  }
}

/// The identifiers that the first picture's activation named, each as read (ITU-T H.264 7.4.1.2.1).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct H264ActivationDetails {
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
  picture_parameter_set_identifier: Option<u32>,
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
  buffering_period_sequence_parameter_set_identifier: Option<u32>,
}

impl H264ActivationDetails {
  /// The first slice header's `pic_parameter_set_id` (7.3.3).
  pub fn picture_parameter_set_identifier(&self) -> Option<u32> {
    self.picture_parameter_set_identifier
  }

  /// The `seq_parameter_set_id` of the access unit's buffering-period SEI message (D.1.2).
  pub fn buffering_period_sequence_parameter_set_identifier(&self) -> Option<u32> {
    self.buffering_period_sequence_parameter_set_identifier
  }
}
