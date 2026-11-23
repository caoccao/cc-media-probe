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

//! The Matroska details of `api.md` section 11.4, with the accessors the scenarios read.

use super::shared::{Conflict, StoredValue};

/// What a Matroska segment states beyond the normalized model.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(
  feature = "serde",
  derive(serde::Serialize, serde::Deserialize),
  serde(remote = "Self")
)]
pub struct MatroskaDetails {
  document_type: String,
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
  document_type_version: Option<StoredValue<u64>>,
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
  document_type_read_version: Option<StoredValue<u64>>,
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
  timestamp_scale: Option<StoredValue<u64>>,
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
  muxing_application: Option<String>,
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
  writing_application: Option<String>,
  tracks: Vec<MatroskaTrackDetails>,
  conflicts: Vec<Conflict>,
}

#[cfg(feature = "serde")]
crate::neutral::record_traits!(MatroskaDetails);

impl MatroskaDetails {
  /// `DocType`: `matroska` or `webm`.
  pub fn document_type(&self) -> &str {
    &self.document_type
  }

  /// `DocTypeVersion`, default 1; absent only when the input ended inside the EBML header.
  pub fn document_type_version(&self) -> Option<&StoredValue<u64>> {
    self.document_type_version.as_ref()
  }

  /// `DocTypeReadVersion`, default 1; absent only when the input ended inside the EBML header.
  pub fn document_type_read_version(&self) -> Option<&StoredValue<u64>> {
    self.document_type_read_version.as_ref()
  }

  /// `TimestampScale`, default 1,000,000; absent when the segment information was cut short before it was read.
  pub fn timestamp_scale(&self) -> Option<&StoredValue<u64>> {
    self.timestamp_scale.as_ref()
  }

  /// `MuxingApp`.
  pub fn muxing_application(&self) -> Option<&str> {
    self.muxing_application.as_deref()
  }

  /// `WritingApp`.
  pub fn writing_application(&self) -> Option<&str> {
    self.writing_application.as_deref()
  }

  /// One entry per reported track.
  pub fn tracks(&self) -> &[MatroskaTrackDetails] {
    &self.tracks
  }

  /// The track of the stream with that index.
  pub fn track(&self, stream_index: usize) -> Option<&MatroskaTrackDetails> {
    self.tracks.iter().find(|track| track.stream_index == stream_index)
  }

  /// The raw values of every conflict.
  pub fn conflicts(&self) -> &[Conflict] {
    &self.conflicts
  }
}

/// What a Matroska track states beyond the normalized model.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(
  feature = "serde",
  derive(serde::Serialize, serde::Deserialize),
  serde(remote = "Self")
)]
pub struct MatroskaTrackDetails {
  stream_index: usize,
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
  track_number: Option<u64>,
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
  codec_identifier: Option<String>,
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
  flag_default: Option<StoredValue<bool>>,
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
  codec_private_data: Option<Vec<u8>>,
}

#[cfg(feature = "serde")]
crate::neutral::record_traits!(MatroskaTrackDetails);

impl MatroskaTrackDetails {
  /// The index of the stream the track became.
  pub fn stream_index(&self) -> usize {
    self.stream_index
  }

  /// `TrackNumber` (RFC 9559 5.1.4.1.1). Mandatory, yet absent when the track entry was cut short before it: EBML
  /// fixes no order among a master's children, so a cut can fall before any of them.
  pub fn track_number(&self) -> Option<u64> {
    self.track_number
  }

  /// `CodecID` (RFC 9559 5.1.4.1.21); absent, like `TrackNumber`, when the track entry was cut short before it.
  pub fn codec_identifier(&self) -> Option<&str> {
    self.codec_identifier.as_deref()
  }

  /// `FlagDefault`, default 1; absent when the track entry was cut short before the flag was read, since an
  /// omission is then not established.
  pub fn flag_default(&self) -> Option<&StoredValue<bool>> {
    self.flag_default.as_ref()
  }

  /// `CodecPrivate`.
  pub fn codec_private_data(&self) -> Option<&[u8]> {
    self.codec_private_data.as_deref()
  }
}
