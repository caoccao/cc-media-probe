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

//! `ExternalReference`, `ReferenceKind` and `ReferenceRole`, a reference to media outside the input (`api.md`
//! section 6.12).

use super::ReferenceOwner;
use super::constants::{
  DATA_REFERENCE_NAME, EXPECTED_COMPANION_NAME, LINKED_PICTURE_NAME, LINKED_SEGMENT_NAME, NEXT_SEGMENT_NAME,
  PLAYLIST_ENTRY_NAME, PREVIOUS_SEGMENT_NAME, TRACK_DATA_NAME,
};
use crate::model::Identifier;
use crate::string_enumeration::string_enumeration;

/// A reference to media outside the input, as written and never dereferenced, from
/// `MediaReport::external_references()`. A reference that the retained-data limit cannot hold is not listed; the
/// record counts it, and the part that found it is incomplete (`api.md` section 6.12).
///
/// ```
/// use cc_media_probe::MediaReport;
///
/// fn references(report: &MediaReport) {
///     for reference in report.external_references() {
///         println!(
///             "{} {:?} at {:?}, role {:?}, owned by {:?}",
///             reference.kind().as_str(),
///             reference.identifier().map(|identifier| identifier.as_str()),
///             reference.location(),
///             reference.role().map(|role| role.as_str()),
///             reference.owner().map(|owner| owner.location().index()),
///         );
///     }
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ExternalReference {
  kind: ReferenceKind,
  identifier: Option<Identifier>,
  location: Option<String>,
  role: Option<ReferenceRole>,
  owner: Option<ReferenceOwner>,
}

impl ExternalReference {
  /// What the reference points to.
  #[must_use]
  pub fn kind(&self) -> &ReferenceKind {
    &self.kind
  }

  /// The identifier of the referenced media, such as a segment's UUID or a companion's validated identifier.
  #[must_use]
  pub fn identifier(&self) -> Option<&Identifier> {
    self.identifier.as_ref()
  }

  /// Where the media lies, as written and never dereferenced; for an expected companion, the name that a request
  /// to a resolver would carry.
  #[must_use]
  pub fn location(&self) -> Option<&str> {
    self.location.as_deref()
  }

  /// What the referenced media is to this input.
  #[must_use]
  pub fn role(&self) -> Option<&ReferenceRole> {
    self.role.as_ref()
  }

  /// The entity that holds the reference; absent for a reference of the whole file.
  #[must_use]
  pub fn owner(&self) -> Option<&ReferenceOwner> {
    self.owner.as_ref()
  }
}

string_enumeration! {
  /// What an external reference points to. `ExternalReference` shows an example.
  ///
  /// ```
  /// use cc_media_probe::model::ReferenceKind;
  ///
  /// assert_eq!(ReferenceKind::ExpectedCompanion.as_str(), "expected_companion");
  /// ```
  ReferenceKind {
    /// Another segment that this one links to.
    LinkedSegment => LINKED_SEGMENT_NAME,
    /// Media data stored outside the file, as an ISO data reference.
    DataReference => DATA_REFERENCE_NAME,
    /// A picture linked by URI.
    LinkedPicture => LINKED_PICTURE_NAME,
    /// A companion file that the format expects, such as a VobSub data file.
    ExpectedCompanion => EXPECTED_COMPANION_NAME,
    /// An entry of a playlist.
    PlaylistEntry => PLAYLIST_ENTRY_NAME,
  }
}

string_enumeration! {
  /// What referenced media is to the input. `ExternalReference` shows an example.
  ///
  /// ```
  /// use cc_media_probe::model::ReferenceRole;
  ///
  /// assert_eq!(ReferenceRole::NextSegment.as_str(), "next_segment");
  /// ```
  ReferenceRole {
    /// The segment before this one.
    PreviousSegment => PREVIOUS_SEGMENT_NAME,
    /// The segment after this one.
    NextSegment => NEXT_SEGMENT_NAME,
    /// The data of a track.
    TrackData => TRACK_DATA_NAME,
  }
}

#[cfg(test)]
mod tests {
  use super::{ExternalReference, ReferenceKind, ReferenceRole};
  use crate::model::{Identifier, ReferenceOwner, UnrecognizedValue};

  impl ExternalReference {
    /// A reference to the next segment.
    pub(crate) fn sample() -> ExternalReference {
      ExternalReference {
        kind: ReferenceKind::LinkedSegment,
        identifier: Some(Identifier::sample("00112233445566778899aabbccddeeff")),
        location: Some("part2.mkv".to_owned()),
        role: Some(ReferenceRole::NextSegment),
        owner: None,
      }
    }
  }

  #[test]
  fn gives_each_field() {
    let reference = ExternalReference {
      owner: Some(ReferenceOwner::sample(0, 1)),
      ..ExternalReference::sample()
    };
    assert_eq!(reference.kind(), &ReferenceKind::LinkedSegment);
    assert!(reference.identifier().is_some());
    assert_eq!(reference.location(), Some("part2.mkv"));
    assert_eq!(reference.role(), Some(&ReferenceRole::NextSegment));
    assert!(reference.owner().is_some());
  }

  #[test]
  fn names_each_kind_and_role() {
    for (kind, name) in [
      (ReferenceKind::LinkedSegment, "linked_segment"),
      (ReferenceKind::DataReference, "data_reference"),
      (ReferenceKind::LinkedPicture, "linked_picture"),
      (ReferenceKind::ExpectedCompanion, "expected_companion"),
      (ReferenceKind::PlaylistEntry, "playlist_entry"),
      (
        ReferenceKind::Unrecognized(UnrecognizedValue::sample("archive_member")),
        "archive_member",
      ),
    ] {
      assert_eq!(kind.as_str(), name);
    }
    for (role, name) in [
      (ReferenceRole::PreviousSegment, "previous_segment"),
      (ReferenceRole::NextSegment, "next_segment"),
      (ReferenceRole::TrackData, "track_data"),
    ] {
      assert_eq!(role.as_str(), name);
    }
  }
}
