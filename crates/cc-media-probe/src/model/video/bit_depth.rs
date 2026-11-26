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

//! `BitDepth`, `ComponentDepth` and `Component`, the precision of a video's samples (`api.md` section 6.5).

use super::constants::{ALPHA_NAME, BLUE_NAME, CHROMA_NAME, GREEN_NAME, LUMA_NAME, RED_NAME};
use crate::form_record::form_record;
use crate::string_enumeration::string_enumeration;

form_record! {
  /// The precision of a video's samples: the precision that every component shares, where the file states one, and
  /// each component's own.
  ///
  /// ```
  /// use cc_media_probe::model::{BitDepth, Component};
  ///
  /// fn luma_bits(depth: &BitDepth) -> Option<u64> {
  ///     depth.common().or_else(|| {
  ///         depth
  ///             .components()
  ///             .iter()
  ///             .find(|component| matches!(component.component(), Component::Luma))
  ///             .map(|component| component.bits())
  ///     })
  /// }
  /// ```
  #[derive(Clone, Debug, PartialEq, Eq, Hash)]
  pub struct BitDepth {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    common: Option<u64>,
    components: Vec<ComponentDepth>,
  }
  check = BitDepth::check_form;
}

impl BitDepth {
  /// The precision that every component shares, in bits.
  #[must_use]
  pub fn common(&self) -> Option<u64> {
    self.common
  }

  /// Each component's own precision, as the file states it.
  #[must_use]
  pub fn components(&self) -> &[ComponentDepth] {
    &self.components
  }

  /// Whether every component has the common precision where one is stated, since the common precision is the one
  /// that all present components share (field guide section 3.4), as the fourth review of the third slice's first
  /// part asked, which read a common precision of 8 beside a luma of 10 bits.
  #[cfg(feature = "serde")]
  fn check_form(&self) -> Result<(), &'static str> {
    let shared = self
      .common
      .is_none_or(|common| self.components.iter().all(|component| component.bits == common));
    if shared {
      Ok(())
    } else {
      Err(super::constants::BIT_DEPTH_MESSAGE)
    }
  }
}

form_record! {
  /// The precision of one component of a video's samples. `BitDepth` shows an example.
  #[derive(Clone, Debug, PartialEq, Eq, Hash)]
  pub struct ComponentDepth {
    component: Component,
    bits: u64,
  }
}

impl ComponentDepth {
  /// The component.
  #[must_use]
  pub fn component(&self) -> &Component {
    &self.component
  }

  /// Its precision, in bits.
  #[must_use]
  pub fn bits(&self) -> u64 {
    self.bits
  }
}

string_enumeration! {
  /// A component of a video's samples.
  ///
  /// ```
  /// use cc_media_probe::model::Component;
  ///
  /// assert_eq!(Component::Alpha.as_str(), "alpha");
  /// ```
  Component {
    /// Luma.
    Luma => LUMA_NAME,
    /// Chroma.
    Chroma => CHROMA_NAME,
    /// Red.
    Red => RED_NAME,
    /// Green.
    Green => GREEN_NAME,
    /// Blue.
    Blue => BLUE_NAME,
    /// Alpha.
    Alpha => ALPHA_NAME,
  }
}

#[cfg(test)]
mod tests {
  use super::{BitDepth, Component, ComponentDepth};
  use crate::model::UnrecognizedValue;

  impl BitDepth {
    /// A depth of this common precision, with these components.
    pub(crate) fn sample(common: Option<u64>, components: &[(Component, u64)]) -> BitDepth {
      BitDepth {
        common,
        components: components
          .iter()
          .map(|(component, bits)| ComponentDepth {
            component: component.clone(),
            bits: *bits,
          })
          .collect(),
      }
    }
  }

  #[test]
  fn keeps_the_common_and_the_component_depths() {
    let depth = BitDepth::sample(Some(10), &[(Component::Luma, 10), (Component::Chroma, 8)]);
    assert_eq!(depth.common(), Some(10));
    assert_eq!(depth.components().len(), 2);
    assert_eq!(depth.components()[1].component(), &Component::Chroma);
    assert_eq!(depth.components()[1].bits(), 8);
  }

  #[test]
  fn names_each_component() {
    for (component, name) in [
      (Component::Luma, "luma"),
      (Component::Chroma, "chroma"),
      (Component::Red, "red"),
      (Component::Green, "green"),
      (Component::Blue, "blue"),
      (Component::Alpha, "alpha"),
      (Component::Unrecognized(UnrecognizedValue::sample("depth")), "depth"),
    ] {
      assert_eq!(component.as_str(), name);
    }
  }
}
