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

//! `SourceIdentifier`, the number of a source in the source table (`api.md` section 6.15).

use std::fmt;

/// The number of a source in the source table, where 0 is the primary input. It is 64 bits wide, so every request
/// that the companion-request limit allows has an identifier of its own.
///
/// `Display` writes the number in canonical decimal, such as `1`, and JSON holds it as that text; `Debug` writes
/// `SourceIdentifier(1)`. `Ord` compares by number.
///
/// ```
/// use cc_media_probe::diagnostics::Diagnostic;
///
/// fn from_a_companion(diagnostic: &Diagnostic) -> Option<String> {
///     let source = diagnostic.source_identifier();
///     (source.value() != 0).then(|| format!("source {source}"))
/// }
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SourceIdentifier(u64);

impl SourceIdentifier {
  /// The number, where 0 is the primary input.
  ///
  /// ```
  /// use cc_media_probe::source::SourceEntry;
  ///
  /// fn is_primary(entry: &SourceEntry) -> bool {
  ///     entry.identifier().value() == 0
  /// }
  /// ```
  #[must_use]
  pub fn value(&self) -> u64 {
    self.0
  }
}

impl fmt::Display for SourceIdentifier {
  /// Writes the number in canonical decimal.
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(formatter, "{}", self.0)
  }
}

#[cfg(test)]
mod tests {
  use super::SourceIdentifier;

  impl SourceIdentifier {
    /// The identifier of the source with this number.
    pub(crate) fn sample(value: u64) -> SourceIdentifier {
      SourceIdentifier(value)
    }
  }

  #[test]
  fn writes_its_number() {
    let identifier = SourceIdentifier::sample(1);
    assert_eq!(identifier.value(), 1);
    assert_eq!(identifier.to_string(), "1");
    assert_eq!(format!("{identifier:?}"), "SourceIdentifier(1)");
    assert!(SourceIdentifier::sample(0) < identifier);
    assert_eq!(SourceIdentifier::sample(u64::MAX).to_string(), "18446744073709551615");
  }
}
