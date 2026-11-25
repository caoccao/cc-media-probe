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

//! `ImmersiveAudio` and `ImmersiveKind`, the immersive signaling of an audio stream (`api.md` section 6.6).

use super::constants::{HEIGHT_CHANNELS_NAME, INTERMEDIATE_SPATIAL_FORMAT_NAME, OBJECTS_NAME};
use crate::string_enumeration::string_enumeration;

/// That an audio stream signals immersive audio, and of which kinds. The list is empty when the source states the
/// signaling without its kind, as AC-4's indicator does.
///
/// ```
/// use cc_media_probe::model::{ImmersiveAudio, ImmersiveKind};
///
/// fn has_objects(immersive: &ImmersiveAudio) -> bool {
///     immersive.kinds().iter().any(|kind| matches!(kind, ImmersiveKind::Objects))
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ImmersiveAudio {
  kinds: Vec<ImmersiveKind>,
}

impl ImmersiveAudio {
  /// The kinds of immersive audio, in stored order.
  #[must_use]
  pub fn kinds(&self) -> &[ImmersiveKind] {
    &self.kinds
  }
}

string_enumeration! {
  /// A kind of immersive audio. `ImmersiveAudio` shows an example.
  ///
  /// ```
  /// use cc_media_probe::model::ImmersiveKind;
  ///
  /// assert_eq!(ImmersiveKind::HeightChannels.as_str(), "height_channels");
  /// ```
  ImmersiveKind {
    /// Audio objects.
    Objects => OBJECTS_NAME,
    /// An intermediate spatial format.
    IntermediateSpatialFormat => INTERMEDIATE_SPATIAL_FORMAT_NAME,
    /// Channels above the listener.
    HeightChannels => HEIGHT_CHANNELS_NAME,
  }
}

#[cfg(test)]
mod tests {
  use super::{ImmersiveAudio, ImmersiveKind};
  use crate::model::UnrecognizedValue;

  impl ImmersiveAudio {
    /// Signaling of objects.
    pub(crate) fn sample() -> ImmersiveAudio {
      ImmersiveAudio {
        kinds: vec![ImmersiveKind::Objects],
      }
    }
  }

  #[test]
  fn keeps_the_kinds() {
    assert_eq!(ImmersiveAudio::sample().kinds(), &[ImmersiveKind::Objects]);
    for (kind, name) in [
      (ImmersiveKind::Objects, "objects"),
      (ImmersiveKind::IntermediateSpatialFormat, "intermediate_spatial_format"),
      (ImmersiveKind::HeightChannels, "height_channels"),
      (
        ImmersiveKind::Unrecognized(UnrecognizedValue::sample("binaural")),
        "binaural",
      ),
    ] {
      assert_eq!(kind.as_str(), name);
    }
  }
}
