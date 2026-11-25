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

//! `AttemptedIdentity`, the identity that a request for a companion asked for (`api.md` sections 6.15 and 13).

/// The identity that a request for a companion asked for: the validated identifier and the requested name, each as
/// text that the report owns.
///
/// ```
/// use cc_media_probe::source::SourceEntry;
///
/// fn requested(entry: &SourceEntry) -> Option<&str> {
///     entry.attempted_identity()?.requested_name()
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AttemptedIdentity {
  identifier: Option<String>,
  requested_name: Option<String>,
}

impl AttemptedIdentity {
  /// The validated identifier of the companion, such as the stem `movie`.
  ///
  /// ```
  /// use cc_media_probe::source::AttemptedIdentity;
  ///
  /// fn stem(identity: &AttemptedIdentity) -> &str {
  ///     identity.identifier().unwrap_or("")
  /// }
  /// ```
  #[must_use]
  pub fn identifier(&self) -> Option<&str> {
    self.identifier.as_deref()
  }

  /// The name that the request carried, such as `movie.sub`.
  ///
  /// ```
  /// use cc_media_probe::source::AttemptedIdentity;
  ///
  /// fn name(identity: &AttemptedIdentity) -> &str {
  ///     identity.requested_name().unwrap_or("")
  /// }
  /// ```
  #[must_use]
  pub fn requested_name(&self) -> Option<&str> {
    self.requested_name.as_deref()
  }
}

#[cfg(test)]
mod tests {
  use super::AttemptedIdentity;

  impl AttemptedIdentity {
    /// The identity of a request for `requested_name` under `identifier`.
    pub(crate) fn sample(identifier: Option<&str>, requested_name: Option<&str>) -> AttemptedIdentity {
      AttemptedIdentity {
        identifier: identifier.map(str::to_owned),
        requested_name: requested_name.map(str::to_owned),
      }
    }
  }

  #[test]
  fn keeps_the_identifier_and_the_name() {
    let identity = AttemptedIdentity::sample(Some("movie"), Some("movie.sub"));
    assert_eq!(identity.identifier(), Some("movie"));
    assert_eq!(identity.requested_name(), Some("movie.sub"));
    assert_eq!(AttemptedIdentity::sample(None, None).identifier(), None);
  }
}
