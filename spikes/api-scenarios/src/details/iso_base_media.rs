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

//! The ISO family's details, as far as the observations of a media segment keep them (plan section 8.3). Phase 6
//! designs the type; the stub holds one field, an identifier of 32 bits.

/// What the ISO family states beyond the normalized model.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IsoBaseMediaDetails {
  #[cfg_attr(feature = "serde", serde(with = "super::identifier_text"))]
  fragment_track_identifiers: Vec<u32>,
}

impl IsoBaseMediaDetails {
  /// The `track_ID` of each track fragment header read, in the order read, at its coded width (ISO/IEC 14496-12,
  /// `tfhd`). Serialized, each is a decimal string, since an identifier is a string at every width.
  pub fn fragment_track_identifiers(&self) -> &[u32] {
    &self.fragment_track_identifiers
  }
}
