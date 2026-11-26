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

//! `Ambisonics`, `AmbisonicChannelOrdering` and `AmbisonicNormalization`, an ambisonic layout (`api.md` section
//! 6.6).

use crate::form_record::form_record;
use crate::string_enumeration::string_enumeration;

form_record! {
  /// The ambisonic layout of an audio stream's channels: its order, and the ordering and normalization of its
  /// components.
  ///
  /// ```
  /// use cc_media_probe::model::Ambisonics;
  ///
  /// fn describe(ambisonics: &Ambisonics) -> String {
  ///     format!(
  ///         "order {}, {} ordering, {} normalization",
  ///         ambisonics.order(),
  ///         ambisonics.channel_ordering().as_str(),
  ///         ambisonics.normalization().as_str(),
  ///     )
  /// }
  /// ```
  #[derive(Clone, Debug, PartialEq, Eq, Hash)]
  pub struct Ambisonics {
    order: u64,
    channel_ordering: AmbisonicChannelOrdering,
    normalization: AmbisonicNormalization,
  }
}

impl Ambisonics {
  /// The ambisonic order.
  #[must_use]
  pub fn order(&self) -> u64 {
    self.order
  }

  /// The ordering of the components among the channels.
  #[must_use]
  pub fn channel_ordering(&self) -> &AmbisonicChannelOrdering {
    &self.channel_ordering
  }

  /// The normalization of the components.
  #[must_use]
  pub fn normalization(&self) -> &AmbisonicNormalization {
    &self.normalization
  }
}

string_enumeration! {
  /// The ordering of ambisonic components among the channels. The phases that fill ambisonic layouts add the
  /// orderings; until then an ordering is the catch-all. `Ambisonics` shows an example.
  AmbisonicChannelOrdering {}
}

string_enumeration! {
  /// The normalization of ambisonic components. The phases that fill ambisonic layouts add the normalizations;
  /// until then a normalization is the catch-all. `Ambisonics` shows an example.
  AmbisonicNormalization {}
}

#[cfg(test)]
mod tests {
  use super::{AmbisonicChannelOrdering, AmbisonicNormalization, Ambisonics};
  use crate::model::UnrecognizedValue;

  impl Ambisonics {
    /// A layout of the first order.
    pub(crate) fn sample() -> Ambisonics {
      Ambisonics {
        order: 1,
        channel_ordering: AmbisonicChannelOrdering::Unrecognized(UnrecognizedValue::sample("acn")),
        normalization: AmbisonicNormalization::Unrecognized(UnrecognizedValue::sample("sn3d")),
      }
    }
  }

  #[test]
  fn keeps_the_order_and_the_conventions() {
    let ambisonics = Ambisonics::sample();
    assert_eq!(ambisonics.order(), 1);
    assert_eq!(ambisonics.channel_ordering().as_str(), "acn");
    assert_eq!(ambisonics.normalization().as_str(), "sn3d");
  }
}
