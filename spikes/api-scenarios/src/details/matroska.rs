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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MatroskaDetails {
  document_type: String,
  timestamp_scale: StoredValue<u64>,
  muxing_application: Option<String>,
  writing_application: Option<String>,
  tracks: Vec<MatroskaTrackDetails>,
  conflicts: Vec<Conflict>,
}

impl MatroskaDetails {
  /// `DocType`: `matroska` or `webm`.
  pub fn document_type(&self) -> &str {
    &self.document_type
  }

  /// `TimestampScale`, default 1,000,000.
  pub fn timestamp_scale(&self) -> &StoredValue<u64> {
    &self.timestamp_scale
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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MatroskaTrackDetails {
  stream_index: usize,
  track_number: u64,
  codec_identifier: String,
  flag_default: StoredValue<bool>,
  codec_private_data: Option<Vec<u8>>,
}

impl MatroskaTrackDetails {
  /// The index of the stream the track became.
  pub fn stream_index(&self) -> usize {
    self.stream_index
  }

  /// `TrackNumber`.
  pub fn track_number(&self) -> u64 {
    self.track_number
  }

  /// `CodecID`.
  pub fn codec_identifier(&self) -> &str {
    &self.codec_identifier
  }

  /// `FlagDefault`, default 1.
  pub fn flag_default(&self) -> &StoredValue<bool> {
    &self.flag_default
  }

  /// `CodecPrivate`.
  pub fn codec_private_data(&self) -> Option<&[u8]> {
    self.codec_private_data.as_deref()
  }
}
