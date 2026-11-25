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

//! `ProtectionDeclaration` and `ProtectionScope`, one declaration of protection (`api.md` section 6.3).

use super::ProtectionScheme;
use super::constants::{
  CODEC_PRIVATE_DATA_NAME, ENCODING_SETTINGS_NAME, OTHER_NAME, PACKETS_NAME, SAMPLE_DESCRIPTION_NAME, UNREAD_NAME,
};
use crate::string_enumeration::string_enumeration;

/// One declaration of protection: a scheme, as written, and the scope it protects. A declaration establishes a
/// protection mechanism for its scope; it does not say that every byte of that scope is encrypted.
///
/// ```
/// use cc_media_probe::model::{ProtectionDeclaration, ProtectionScope};
///
/// fn describe(declaration: &ProtectionDeclaration) -> String {
///     let scheme = declaration.scheme().map_or("an unknown scheme", |scheme| scheme.value());
///     match declaration.scope() {
///         ProtectionScope::SampleDescription => {
///             format!("{scheme} for description {:?}", declaration.sample_description_index())
///         }
///         ProtectionScope::EncodingSettings => {
///             format!("{scheme} for the settings of encoding {:?}", declaration.encoding_order())
///         }
///         scope => format!("{scheme} for {}", scope.as_str()),
///     }
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ProtectionDeclaration {
  scheme: Option<ProtectionScheme>,
  scope: ProtectionScope,
  sample_description_index: Option<u64>,
  encoding_order: Option<u64>,
}

impl ProtectionDeclaration {
  /// The scheme as written with its namespace.
  #[must_use]
  pub fn scheme(&self) -> Option<&ProtectionScheme> {
    self.scheme.as_ref()
  }

  /// What the declaration protects.
  #[must_use]
  pub fn scope(&self) -> &ProtectionScope {
    &self.scope
  }

  /// The sample description, as the file numbers it, for the scope `SampleDescription`: from 1, and within the
  /// protection's declared count when that is known.
  #[must_use]
  pub fn sample_description_index(&self) -> Option<u64> {
    self.sample_description_index
  }

  /// The `ContentEncodingOrder` of the next encoding, whose settings the declaration protects, for the scope
  /// `EncodingSettings`; absent when the track has no next encoding.
  #[must_use]
  pub fn encoding_order(&self) -> Option<u64> {
    self.encoding_order
  }
}

string_enumeration! {
  /// What a protection declaration protects.
  ///
  /// ```
  /// use cc_media_probe::model::ProtectionScope;
  ///
  /// assert_eq!(ProtectionScope::CodecPrivateData.as_str(), "codec_private_data");
  /// ```
  ProtectionScope {
    /// Every packet of the stream.
    Packets => PACKETS_NAME,
    /// The codec's private data.
    CodecPrivateData => CODEC_PRIVATE_DATA_NAME,
    /// The packets that use one sample description.
    SampleDescription => SAMPLE_DESCRIPTION_NAME,
    /// The settings of another content encoding.
    EncodingSettings => ENCODING_SETTINGS_NAME,
    /// A scope that the file states and this version does not know.
    Other => OTHER_NAME,
    /// A scope that the probe could not read, because the input, a limit, the reach of a forward-only source or a
    /// source failure ended the structure before it.
    Unread => UNREAD_NAME,
  }
}

#[cfg(test)]
mod tests {
  use super::{ProtectionDeclaration, ProtectionScope};
  use crate::model::{ProtectionScheme, ProtectionSchemeNamespace, UnrecognizedValue};

  impl ProtectionDeclaration {
    /// A declaration of the ISO scheme `cenc` for one sample description.
    pub(crate) fn sample(sample_description_index: u64) -> ProtectionDeclaration {
      ProtectionDeclaration {
        scheme: Some(ProtectionScheme::sample(
          ProtectionSchemeNamespace::IsoSchemeType,
          "63656e63",
        )),
        scope: ProtectionScope::SampleDescription,
        sample_description_index: Some(sample_description_index),
        encoding_order: None,
      }
    }

    /// The declaration of a Matroska track whose blocks are encrypted with AES, `ContentEncAlgo` 5.
    pub(crate) fn packets_sample() -> ProtectionDeclaration {
      ProtectionDeclaration {
        scheme: Some(ProtectionScheme::sample(
          ProtectionSchemeNamespace::MatroskaContentEncryptionAlgorithm,
          "5",
        )),
        scope: ProtectionScope::Packets,
        sample_description_index: None,
        encoding_order: None,
      }
    }
  }

  #[test]
  fn keeps_the_scheme_and_its_scope() {
    let declaration = ProtectionDeclaration::sample(1);
    assert_eq!(declaration.scheme().map(ProtectionScheme::value), Some("63656e63"));
    assert_eq!(declaration.scope(), &ProtectionScope::SampleDescription);
    assert_eq!(declaration.sample_description_index(), Some(1));
    assert_eq!(declaration.encoding_order(), None);
  }

  #[test]
  fn names_each_scope() {
    for (scope, name) in [
      (ProtectionScope::Packets, "packets"),
      (ProtectionScope::CodecPrivateData, "codec_private_data"),
      (ProtectionScope::SampleDescription, "sample_description"),
      (ProtectionScope::EncodingSettings, "encoding_settings"),
      (ProtectionScope::Other, "other"),
      (ProtectionScope::Unread, "unread"),
      (
        ProtectionScope::Unrecognized(UnrecognizedValue::sample("range")),
        "range",
      ),
    ] {
      assert_eq!(scope.as_str(), name);
    }
  }
}
