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

//! `SourceEntry`, one entry of the source table (`api.md` section 6.15; plan section 7.7).

use super::{AttemptedIdentity, PortablePath, SourceIdentifier, SourceOutcome, SourceRole};
use crate::completeness::Reason;

/// One source that the probe requested, from `MediaReport::sources()`. Until companion lookup ships, the table
/// has one entry, the primary input.
///
/// ```
/// use cc_media_probe::MediaReport;
///
/// fn list_sources(report: &MediaReport) {
///     for entry in report.sources() {
///         let name = entry.path().map_or("(no path)", |path| path.display());
///         println!(
///             "source {}: {} {} {}, {} bytes read",
///             entry.identifier(),
///             entry.role().as_str(),
///             name,
///             entry.outcome().as_str(),
///             entry.bytes_read().unwrap_or(0),
///         );
///         if let Some(reason) = entry.reason() {
///             println!("  not opened: {}", reason.kind().as_str());
///         }
///         if let Some(identity) = entry.attempted_identity() {
///             println!("  requested {:?}", identity.requested_name());
///         }
///         if let Some(original) = entry.alias_of() {
///             println!("  the same source as {original}");
///         }
///     }
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SourceEntry {
  identifier: SourceIdentifier,
  role: SourceRole,
  attempted_identity: Option<AttemptedIdentity>,
  outcome: SourceOutcome,
  reason: Option<Reason>,
  bytes_read: Option<u64>,
  alias_of: Option<SourceIdentifier>,
  path: Option<PortablePath>,
}

impl SourceEntry {
  /// The source's number, where 0 is the primary input.
  #[must_use]
  pub fn identifier(&self) -> SourceIdentifier {
    self.identifier
  }

  /// The primary input or a companion's role.
  #[must_use]
  pub fn role(&self) -> &SourceRole {
    &self.role
  }

  /// For a companion that was requested, the validated identifier and the requested name.
  #[must_use]
  pub fn attempted_identity(&self) -> Option<&AttemptedIdentity> {
    self.attempted_identity.as_ref()
  }

  /// What became of the request.
  #[must_use]
  pub fn outcome(&self) -> &SourceOutcome {
    &self.outcome
  }

  /// Why a requested companion was not opened: `CompanionNotFound`, `CompanionDeclined` with the resolver's reason
  /// as its detail within the retained-data limit, or `CompanionLimited` with the limit named. Absent for the
  /// primary input, for an opened source and for a failed one, whose cause the error keeps (R19).
  #[must_use]
  pub fn reason(&self) -> Option<&Reason> {
    self.reason.as_ref()
  }

  /// The bytes read from the source, as the budget counted them.
  #[must_use]
  pub fn bytes_read(&self) -> Option<u64> {
    self.bytes_read
  }

  /// The source that this one is an alias of, when two requests led to one underlying source (plan section 7.7).
  #[must_use]
  pub fn alias_of(&self) -> Option<SourceIdentifier> {
    self.alias_of
  }

  /// The source's path, where it has one.
  #[must_use]
  pub fn path(&self) -> Option<&PortablePath> {
    self.path.as_ref()
  }
}

#[cfg(test)]
mod tests {
  use std::path::Path;

  use super::SourceEntry;
  use crate::completeness::{Reason, ReasonKind};
  use crate::source::{AttemptedIdentity, PortablePath, SourceIdentifier, SourceOutcome, SourceRole};

  impl SourceEntry {
    /// The entry of the primary input, opened, with the path `movie.mkv`.
    pub(crate) fn sample() -> SourceEntry {
      SourceEntry {
        identifier: SourceIdentifier::sample(0),
        role: SourceRole::Primary,
        attempted_identity: None,
        outcome: SourceOutcome::Opened,
        reason: None,
        bytes_read: Some(4_096),
        alias_of: None,
        path: Some(PortablePath::from_path(Path::new("movie.mkv"))),
      }
    }
  }

  #[test]
  fn describes_the_primary_input() {
    let entry = SourceEntry::sample();
    assert_eq!(entry.identifier(), SourceIdentifier::sample(0));
    assert_eq!(entry.role(), &SourceRole::Primary);
    assert_eq!(entry.attempted_identity(), None);
    assert_eq!(entry.outcome(), &SourceOutcome::Opened);
    assert_eq!(entry.reason(), None);
    assert_eq!(entry.bytes_read(), Some(4_096));
    assert_eq!(entry.alias_of(), None);
    assert_eq!(entry.path().map(PortablePath::display), Some("movie.mkv"));
  }

  #[test]
  fn describes_a_companion_that_was_not_found() {
    let entry = SourceEntry {
      identifier: SourceIdentifier::sample(1),
      attempted_identity: Some(AttemptedIdentity::sample(Some("movie"), Some("movie.sub"))),
      outcome: SourceOutcome::NotFound,
      reason: Some(Reason::sample(ReasonKind::CompanionNotFound)),
      bytes_read: None,
      alias_of: Some(SourceIdentifier::sample(0)),
      path: None,
      ..SourceEntry::sample()
    };
    assert_eq!(
      entry.attempted_identity().and_then(AttemptedIdentity::requested_name),
      Some("movie.sub")
    );
    assert_eq!(entry.reason().map(Reason::kind), Some(&ReasonKind::CompanionNotFound));
    assert_eq!(entry.alias_of(), Some(SourceIdentifier::sample(0)));
    assert_eq!(entry.path(), None);
  }
}
