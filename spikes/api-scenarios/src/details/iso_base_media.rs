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

//! The ISO family's details, as far as the scenarios and the tests use them: the track identifiers that the
//! observations of a media segment keep (plan section 8.3), an identifier of 32 bits, and for each track the index of
//! the first sample's description once it is established (plan Phase 6, task 7). Phase 6 designs the type.

/// What the ISO family states beyond the normalized model.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IsoBaseMediaDetails {
  #[cfg_attr(feature = "serde", serde(with = "super::identifier_text"))]
  fragment_track_identifiers: Vec<u32>,
  #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Vec::is_empty"))]
  tracks: Vec<IsoBaseMediaTrackDetails>,
}

impl IsoBaseMediaDetails {
  /// The `track_ID` of each track fragment header read, in the order read, at its coded width (ISO/IEC 14496-12,
  /// `tfhd`). Serialized, each is a decimal string, since an identifier is a string at every width.
  pub fn fragment_track_identifiers(&self) -> &[u32] {
    &self.fragment_track_identifiers
  }

  /// One entry per reported track.
  pub fn tracks(&self) -> &[IsoBaseMediaTrackDetails] {
    &self.tracks
  }

  /// The track of the stream with that index.
  pub fn track(&self, stream_index: usize) -> Option<&IsoBaseMediaTrackDetails> {
    self.tracks.iter().find(|track| track.stream_index == stream_index)
  }
}

/// What an ISO track states beyond the normalized model.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IsoBaseMediaTrackDetails {
  stream_index: usize,
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
  first_sample_description_index: Option<u32>,
}

impl IsoBaseMediaTrackDetails {
  /// The index of the stream the track became.
  pub fn stream_index(&self) -> usize {
    self.stream_index
  }

  /// The `sample_description_index` that selects the description of the track's first sample in decoding order,
  /// from the sample-to-chunk table, the track-extends default or a track-fragment override (ISO/IEC 14496-12,
  /// `stsc`, `trex`, `tfhd`); absent while that selection is not established. It is kept also when the probe could
  /// not read or keep the description it selects, whose codec is then absent.
  pub fn first_sample_description_index(&self) -> Option<u32> {
    self.first_sample_description_index
  }
}
