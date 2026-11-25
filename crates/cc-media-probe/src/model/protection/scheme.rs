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

//! `ProtectionScheme` and `ProtectionSchemeNamespace`, a protection scheme as written (`api.md` section 6.3).

use super::constants::{ISO_SCHEME_TYPE_NAME, MATROSKA_CONTENT_ENCRYPTION_ALGORITHM_NAME};
use crate::string_enumeration::string_enumeration;

/// A protection scheme as the file writes it, with its namespace, spelled as a raw codec identifier is: four bytes
/// in lowercase hexadecimal, or a coded number in canonical decimal (`api.md` section 6.4).
///
/// ```
/// use cc_media_probe::model::{ProtectionScheme, ProtectionSchemeNamespace};
///
/// fn is_common_encryption(scheme: &ProtectionScheme) -> bool {
///     // "cenc" as the four bytes of an ISO scheme_type.
///     matches!(scheme.namespace(), ProtectionSchemeNamespace::IsoSchemeType) && scheme.value() == "63656e63"
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ProtectionScheme {
  namespace: ProtectionSchemeNamespace,
  value: String,
}

impl ProtectionScheme {
  /// The namespace of the scheme.
  #[must_use]
  pub fn namespace(&self) -> &ProtectionSchemeNamespace {
    &self.namespace
  }

  /// The scheme as written, in the spelling of its namespace.
  #[must_use]
  pub fn value(&self) -> &str {
    &self.value
  }
}

string_enumeration! {
  /// The namespace of a protection scheme. Phases 5 and 6 confirm these at their gates, and later phases add
  /// their namespaces.
  ///
  /// ```
  /// use cc_media_probe::model::ProtectionSchemeNamespace;
  ///
  /// assert_eq!(ProtectionSchemeNamespace::IsoSchemeType.as_str(), "iso_scheme_type");
  /// ```
  ProtectionSchemeNamespace {
    /// The `scheme_type` of an ISO `schm` box, four bytes in lowercase hexadecimal.
    IsoSchemeType => ISO_SCHEME_TYPE_NAME,
    /// A Matroska `ContentEncAlgo` (RFC 9559 section 5.1.4.1.31.9), in canonical decimal.
    MatroskaContentEncryptionAlgorithm => MATROSKA_CONTENT_ENCRYPTION_ALGORITHM_NAME,
  }
}

#[cfg(test)]
mod tests {
  use super::{ProtectionScheme, ProtectionSchemeNamespace};
  use crate::model::UnrecognizedValue;

  impl ProtectionScheme {
    /// The scheme `value` in `namespace`.
    pub(crate) fn sample(namespace: ProtectionSchemeNamespace, value: &str) -> ProtectionScheme {
      ProtectionScheme {
        namespace,
        value: value.to_owned(),
      }
    }
  }

  #[test]
  fn keeps_the_scheme_as_written() {
    let scheme = ProtectionScheme::sample(ProtectionSchemeNamespace::MatroskaContentEncryptionAlgorithm, "5");
    assert_eq!(
      scheme.namespace(),
      &ProtectionSchemeNamespace::MatroskaContentEncryptionAlgorithm
    );
    assert_eq!(scheme.value(), "5");
    assert_eq!(scheme.namespace().as_str(), "matroska_content_encryption_algorithm");
    assert_eq!(ProtectionSchemeNamespace::IsoSchemeType.as_str(), "iso_scheme_type");
    assert_eq!(
      ProtectionSchemeNamespace::Unrecognized(UnrecognizedValue::sample("asf")).as_str(),
      "asf"
    );
  }
}
