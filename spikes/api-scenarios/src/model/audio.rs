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

//! `AudioInformation`, `api.md` section 6.6, with the fields the scenarios read.

use super::values::{Provenance, Rational};

/// What an audio stream states.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AudioInformation {
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
  sample_rate: Option<Rational>,
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
  channel_count: Option<u64>,
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
  channel_count_provenance: Option<Provenance>,
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
  bit_depth: Option<u64>,
}

impl AudioInformation {
  /// The sample rate, in hertz.
  pub fn sample_rate(&self) -> Option<Rational> {
    self.sample_rate
  }

  /// The decoder's output channel count.
  pub fn channel_count(&self) -> Option<u64> {
    self.channel_count
  }

  /// Where the channel count came from.
  pub fn channel_count_provenance(&self) -> Option<&Provenance> {
    self.channel_count_provenance.as_ref()
  }

  /// Bits per sample.
  pub fn bit_depth(&self) -> Option<u64> {
    self.bit_depth
  }
}
