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

//! The two open enums, `api.md` section 11.1, and the accessors they share, section 11.3.

use super::h264::H264Details;
use super::iso_base_media::IsoBaseMediaDetails;
use super::matroska::MatroskaDetails;
use super::shared::{Conflict, UnrecognizedDetails};

/// A format's details.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum FormatDetails {
  /// Matroska and WebM.
  Matroska(MatroskaDetails),
  /// The ISO base media family.
  IsoBaseMedia(IsoBaseMediaDetails),
  /// A kind that a newer version wrote.
  Unrecognized(UnrecognizedDetails),
}

impl FormatDetails {
  /// The raw values of every conflict, for any format.
  pub fn conflicts(&self) -> &[Conflict] {
    match self {
      FormatDetails::Matroska(matroska) => matroska.conflicts(),
      FormatDetails::IsoBaseMedia(_) | FormatDetails::Unrecognized(_) => &[],
    }
  }
}

/// A codec header's details.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum CodecDetails {
  /// H.264.
  H264(H264Details),
  /// A kind that a newer version wrote.
  Unrecognized(UnrecognizedDetails),
}

impl CodecDetails {
  /// The raw values of every conflict, for any codec.
  pub fn conflicts(&self) -> &[Conflict] {
    match self {
      CodecDetails::H264(h264) => h264.conflicts(),
      CodecDetails::Unrecognized(_) => &[],
    }
  }
}
