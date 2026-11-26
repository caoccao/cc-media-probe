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

//! `Projection` and `ProjectionKind`, how a video maps onto a sphere (`api.md` section 6.5).

use super::constants::{CUBEMAP_NAME, EQUIRECTANGULAR_NAME, MESH_NAME, RECTANGULAR_NAME};
use crate::Rational;
use crate::form_record::form_record;
use crate::string_enumeration::string_enumeration;

form_record! {
  /// How a video maps onto a sphere: the kind of projection and its pose, in degrees.
  ///
  /// ```
  /// use cc_media_probe::model::{Projection, ProjectionKind};
  ///
  /// fn describe(projection: &Projection) -> String {
  ///     let spherical = !matches!(projection.kind(), ProjectionKind::Rectangular);
  ///     format!(
  ///         "{} (spherical {spherical}), yaw {:?}, pitch {:?}, roll {:?}",
  ///         projection.kind().as_str(),
  ///         projection.yaw(),
  ///         projection.pitch(),
  ///         projection.roll(),
  ///     )
  /// }
  /// ```
  #[derive(Clone, Debug, PartialEq, Eq, Hash)]
  pub struct Projection {
    kind: ProjectionKind,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    yaw: Option<Rational>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pitch: Option<Rational>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    roll: Option<Rational>,
  }
}

impl Projection {
  /// The kind of projection.
  #[must_use]
  pub fn kind(&self) -> &ProjectionKind {
    &self.kind
  }

  /// The yaw, in degrees.
  #[must_use]
  pub fn yaw(&self) -> Option<Rational> {
    self.yaw
  }

  /// The pitch, in degrees.
  #[must_use]
  pub fn pitch(&self) -> Option<Rational> {
    self.pitch
  }

  /// The roll, in degrees.
  #[must_use]
  pub fn roll(&self) -> Option<Rational> {
    self.roll
  }
}

string_enumeration! {
  /// The kind of a video's projection. `Projection` shows an example.
  ///
  /// ```
  /// use cc_media_probe::model::ProjectionKind;
  ///
  /// assert_eq!(ProjectionKind::Equirectangular.as_str(), "equirectangular");
  /// ```
  ProjectionKind {
    /// A flat picture.
    Rectangular => RECTANGULAR_NAME,
    /// An equirectangular projection.
    Equirectangular => EQUIRECTANGULAR_NAME,
    /// A cube map.
    Cubemap => CUBEMAP_NAME,
    /// A mesh.
    Mesh => MESH_NAME,
  }
}

#[cfg(test)]
mod tests {
  use super::{Projection, ProjectionKind};
  use crate::Rational;
  use crate::model::UnrecognizedValue;

  impl Projection {
    /// An equirectangular projection turned by 90 degrees of yaw.
    pub(crate) fn sample() -> Projection {
      Projection {
        kind: ProjectionKind::Equirectangular,
        yaw: Rational::new(90, 1),
        pitch: Rational::new(0, 1),
        roll: None,
      }
    }
  }

  #[test]
  fn keeps_the_kind_and_the_pose() {
    let projection = Projection::sample();
    assert_eq!(projection.kind(), &ProjectionKind::Equirectangular);
    assert_eq!(projection.yaw(), Rational::new(90, 1));
    assert_eq!(projection.pitch(), Rational::new(0, 1));
    assert_eq!(projection.roll(), None);
    for (kind, name) in [
      (ProjectionKind::Rectangular, "rectangular"),
      (ProjectionKind::Equirectangular, "equirectangular"),
      (ProjectionKind::Cubemap, "cubemap"),
      (ProjectionKind::Mesh, "mesh"),
      (
        ProjectionKind::Unrecognized(UnrecognizedValue::sample("fisheye")),
        "fisheye",
      ),
    ] {
      assert_eq!(kind.as_str(), name);
    }
  }
}
