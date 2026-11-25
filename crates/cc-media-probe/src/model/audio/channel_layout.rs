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

//! `ChannelLayout`, `ChannelPosition` and `Speaker`, where an audio stream's channels play (`api.md` section 6.6).

use super::Ambisonics;
use crate::Rational;
use crate::string_enumeration::string_enumeration;

/// Where an audio stream's channels play: one position per channel in stream order, and the ambisonic layout of
/// the channels that hold ambisonic components. A layout that a format states only by a standardized code without
/// an approved mapping stays in the details (F11).
///
/// ```
/// use cc_media_probe::model::ChannelLayout;
///
/// fn describe(layout: &ChannelLayout) -> Vec<String> {
///     let mut channels: Vec<String> = layout
///         .channels()
///         .iter()
///         .map(|channel| {
///             if channel.is_unassigned() {
///                 return "unassigned".to_owned();
///             }
///             if let Some(component) = channel.ambisonic_component() {
///                 return format!("ambisonic component {component}");
///             }
///             match (channel.speaker(), channel.azimuth(), channel.elevation()) {
///                 (Some(speaker), _, _) => speaker.as_str().to_owned(),
///                 (None, Some(azimuth), elevation) => format!("azimuth {azimuth}, elevation {elevation:?}"),
///                 (None, None, _) => "unknown".to_owned(),
///             }
///         })
///         .collect();
///     if let Some(ambisonics) = layout.ambisonics() {
///         channels.push(format!("ambisonics of order {}", ambisonics.order()));
///     }
///     channels
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ChannelLayout {
  channels: Vec<ChannelPosition>,
  ambisonics: Option<Ambisonics>,
}

impl ChannelLayout {
  /// One position per channel, in stream order.
  #[must_use]
  pub fn channels(&self) -> &[ChannelPosition] {
    &self.channels
  }

  /// The ambisonic layout of the channels that hold ambisonic components.
  #[must_use]
  pub fn ambisonics(&self) -> Option<&Ambisonics> {
    self.ambisonics.as_ref()
  }
}

/// Where one channel plays: a named speaker, coordinates in degrees, an ambisonic component, or no assigned
/// position. `ChannelLayout` shows an example.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ChannelPosition {
  speaker: Option<Speaker>,
  azimuth: Option<Rational>,
  elevation: Option<Rational>,
  ambisonic_component: Option<u64>,
  unassigned: bool,
}

impl ChannelPosition {
  /// The named position of the channel's speaker.
  #[must_use]
  pub fn speaker(&self) -> Option<&Speaker> {
    self.speaker.as_ref()
  }

  /// The azimuth, in signed degrees, where the format gives coordinates.
  #[must_use]
  pub fn azimuth(&self) -> Option<Rational> {
    self.azimuth
  }

  /// The elevation, in signed degrees, where the format gives coordinates.
  #[must_use]
  pub fn elevation(&self) -> Option<Rational> {
    self.elevation
  }

  /// The ambisonic component that the channel holds.
  #[must_use]
  pub fn ambisonic_component(&self) -> Option<u64> {
    self.ambisonic_component
  }

  /// Whether the channel has no assigned position.
  #[must_use]
  pub fn is_unassigned(&self) -> bool {
    self.unassigned
  }
}

string_enumeration! {
  /// A named speaker position, an open set such as front left and low frequency. The phases that fill channel
  /// layouts add the positions; until then a position is the catch-all. `ChannelLayout` shows an example.
  Speaker {}
}

#[cfg(test)]
mod tests {
  use super::{ChannelLayout, ChannelPosition, Speaker};
  use crate::Rational;
  use crate::model::{Ambisonics, UnrecognizedValue};

  impl ChannelPosition {
    /// The position of a named speaker.
    pub(crate) fn named(name: &str) -> ChannelPosition {
      ChannelPosition {
        speaker: Some(Speaker::Unrecognized(UnrecognizedValue::sample(name))),
        azimuth: None,
        elevation: None,
        ambisonic_component: None,
        unassigned: false,
      }
    }
  }

  impl ChannelLayout {
    /// A stereo pair beside four ambisonic channels of the first order.
    pub(crate) fn sample() -> ChannelLayout {
      let ambisonic = (0..4).map(|component| ChannelPosition {
        ambisonic_component: Some(component),
        ..ChannelPosition::unassigned()
      });
      ChannelLayout {
        channels: [
          ChannelPosition::named("front_left"),
          ChannelPosition::named("front_right"),
        ]
        .into_iter()
        .chain(ambisonic)
        .collect(),
        ambisonics: Some(Ambisonics::sample()),
      }
    }
  }

  impl ChannelPosition {
    /// A channel without an assigned position.
    fn unassigned() -> ChannelPosition {
      ChannelPosition {
        speaker: None,
        azimuth: None,
        elevation: None,
        ambisonic_component: None,
        unassigned: true,
      }
    }
  }

  #[test]
  fn keeps_each_channel_in_stream_order() {
    let layout = ChannelLayout::sample();
    assert_eq!(layout.channels().len(), 6);
    assert_eq!(layout.channels()[0].speaker().map(Speaker::as_str), Some("front_left"));
    assert!(!layout.channels()[0].is_unassigned());
    assert_eq!(layout.channels()[3].ambisonic_component(), Some(1));
    assert!(layout.ambisonics().is_some());
    let placed = ChannelPosition {
      azimuth: Rational::new(-30, 1),
      elevation: Rational::new(0, 1),
      ..ChannelPosition::unassigned()
    };
    assert_eq!(placed.azimuth(), Rational::new(-30, 1));
    assert_eq!(placed.elevation(), Rational::new(0, 1));
    assert!(placed.is_unassigned());
    assert_eq!(placed.speaker(), None);
  }
}
