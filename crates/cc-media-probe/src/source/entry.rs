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

  /// Whether the request was made and the companion was not opened: not found, declined or refused by a limit
  /// (`api.md` section 13), which gives the companions a reason with its source.
  #[cfg(feature = "serde")]
  pub(crate) fn was_refused(&self) -> bool {
    super::constants::REFUSED_OUTCOMES.contains(&self.outcome)
  }
}

/// A source entry as written: its bytes read as a number or beyond 2^53 - 1 its twin, and its path as the display
/// string with the flag and the raw form of a lossy path beside it (`json-output.md` sections 5.8 and 7.14).
#[cfg(feature = "serde")]
#[derive(serde::Serialize, serde::Deserialize)]
struct SourceEntryForm {
  identifier: SourceIdentifier,
  role: SourceRole,
  #[serde(skip_serializing_if = "Option::is_none")]
  attempted_identity: Option<AttemptedIdentity>,
  outcome: SourceOutcome,
  #[serde(skip_serializing_if = "Option::is_none")]
  reason: Option<Reason>,
  #[serde(skip_serializing_if = "Option::is_none")]
  bytes_read: Option<u64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  bytes_read_exact: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  alias_of: Option<SourceIdentifier>,
  #[serde(skip_serializing_if = "Option::is_none")]
  path: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  path_lossy: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  path_raw: Option<super::RawPath>,
}

#[cfg(feature = "serde")]
impl serde::Serialize for SourceEntry {
  fn serialize<Output: serde::Serializer>(&self, serializer: Output) -> Result<Output::Ok, Output::Error> {
    let (bytes_read, bytes_read_exact) = crate::form::unsigned_members(self.bytes_read);
    let (path, path_lossy, path_raw) = match self.path.as_ref().map(PortablePath::members) {
      Some((display, lossy, raw)) => (Some(display.to_owned()), lossy, raw.cloned()),
      None => (None, None, None),
    };
    let form = SourceEntryForm {
      identifier: self.identifier,
      role: self.role.clone(),
      attempted_identity: self.attempted_identity.clone(),
      outcome: self.outcome.clone(),
      reason: self.reason.clone(),
      bytes_read,
      bytes_read_exact,
      alias_of: self.alias_of,
      path,
      path_lossy,
      path_raw,
    };
    serde::Serialize::serialize(&form, serializer)
  }
}

#[cfg(feature = "serde")]
impl TryFrom<SourceEntryForm> for SourceEntry {
  type Error = &'static str;

  fn try_from(form: SourceEntryForm) -> Result<SourceEntry, &'static str> {
    let path = match (form.path, form.path_lossy, form.path_raw) {
      (Some(display), lossy, raw) => Some(PortablePath::from_members(display, lossy, raw)?),
      (None, None, None) => None,
      (None, _, _) => return Err(super::constants::PATH_MEMBERS_MESSAGE),
    };
    let entry = SourceEntry {
      identifier: form.identifier,
      role: form.role,
      attempted_identity: form.attempted_identity,
      outcome: form.outcome,
      reason: form.reason,
      bytes_read: crate::form::unsigned_value(form.bytes_read, form.bytes_read_exact)?,
      alias_of: form.alias_of,
      path,
    };
    check_entry(&entry)?;
    Ok(entry)
  }
}

/// Whether an entry's identifier, role, outcome and reason describe one request, which needs the entry alone, as the
/// review of the third slice's first part found (`api.md` section 6.15): the identifier 0 is the primary input's;
/// an alias was opened and names a source of a lower identifier, since requests take their identifiers in order,
/// which the fourth review of that part read unchecked on an entry alone that was not found and named source 7; the
/// primary input and an opened or failed source have no reason; and a companion that was not opened has the reason
/// of its outcome, `companion_not_found`, `companion_declined` or `companion_limited`, or a reason of a newer kind. An
/// outcome of a newer version is not checked.
#[cfg(feature = "serde")]
fn check_entry(entry: &SourceEntry) -> Result<(), &'static str> {
  use super::constants::{SOURCE_IDENTITY_MESSAGE, SOURCE_REASON_MESSAGE};
  use crate::completeness::ReasonKind;
  let primary = matches!(entry.role, SourceRole::Primary);
  let alias_fits = entry.alias_of.is_none_or(|first| {
    first < entry.identifier && matches!(entry.outcome, SourceOutcome::Opened | SourceOutcome::Unrecognized(_))
  });
  if primary != (entry.identifier.value() == 0) || !alias_fits {
    return Err(SOURCE_IDENTITY_MESSAGE);
  }
  let expected = match entry.outcome {
    SourceOutcome::Opened | SourceOutcome::Failed => None,
    SourceOutcome::NotFound => Some(ReasonKind::CompanionNotFound),
    SourceOutcome::Declined => Some(ReasonKind::CompanionDeclined),
    SourceOutcome::LimitReached => Some(ReasonKind::CompanionLimited),
    SourceOutcome::Unrecognized(_) => return Ok(()),
  };
  let fits = match (expected, entry.reason.as_ref().map(Reason::kind)) {
    (None, None) => true,
    (Some(expected), Some(kind)) => !primary && (*kind == expected || matches!(kind, ReasonKind::Unrecognized(_))),
    _ => false,
  };
  if fits { Ok(()) } else { Err(SOURCE_REASON_MESSAGE) }
}

#[cfg(feature = "serde")]
crate::form::checked_form!(SourceEntry from SourceEntryForm);

#[cfg(test)]
mod tests {
  use std::path::Path;

  use super::SourceEntry;
  use crate::completeness::{Reason, ReasonKind};
  use crate::model::UnrecognizedValue;
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

    /// The entry of the primary input, which failed after 4096 bytes, as the partial data of a source failure names
    /// it (`api.md` section 10.1).
    pub(crate) fn failed_primary() -> SourceEntry {
      SourceEntry {
        outcome: SourceOutcome::Failed,
        ..SourceEntry::sample()
      }
    }

    /// The entry of a requested companion, source 1, the `.sub` file of a VobSub index, which failed at byte 2048.
    /// Its role is one of the release that ships lookup, which this version reads as a role of a newer version.
    pub(crate) fn failed_companion() -> SourceEntry {
      SourceEntry {
        identifier: SourceIdentifier::sample(1),
        role: SourceRole::Unrecognized(UnrecognizedValue::sample("vob_sub_data")),
        attempted_identity: Some(AttemptedIdentity::sample(Some("movie"), Some("movie.sub"))),
        outcome: SourceOutcome::Failed,
        bytes_read: Some(2_048),
        path: None,
        ..SourceEntry::sample()
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
