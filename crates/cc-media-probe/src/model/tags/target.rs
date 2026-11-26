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

//! The targets of a tag set: `TargetLevel`, `TargetReference`, `TargetReferenceKind`, `IdentifierNamespace` and
//! `TargetScope` (`api.md` section 6.9).

use super::constants::{
  ENTITIES_NAME, INCOMPLETE_NAME, MATROSKA_UNIQUE_IDENTIFIER_NAME, RESOLVED_NAME, UNMATCHED_NAME, WHOLE_FILE_NAME,
  WILDCARD_NAME,
};
use crate::form_record::form_record;
use crate::model::{EntityLocation, Identifier};
use crate::string_enumeration::string_enumeration;

form_record! {
  /// The level of the entity that a tag set describes, such as Matroska's `TargetTypeValue` 50 with `ALBUM`. A key
  /// never says which level a tag describes; its tag set does.
  ///
  /// ```
  /// use cc_media_probe::model::TagSet;
  ///
  /// fn level(set: &TagSet) -> String {
  ///     set.target_level().map_or_else(String::new, |level| {
  ///         format!("{:?} {}", level.value(), level.type_name().unwrap_or(""))
  ///     })
  /// }
  /// ```
  #[derive(Clone, Debug, PartialEq, Eq, Hash)]
  pub struct TargetLevel {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    value: Option<u64>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    type_name: Option<String>,
  }
}

impl TargetLevel {
  /// The level as a number.
  #[must_use]
  pub fn value(&self) -> Option<u64> {
    self.value
  }

  /// The level's name as written.
  #[must_use]
  pub fn type_name(&self) -> Option<&str> {
    self.type_name.as_deref()
  }
}

form_record! {
  /// One target of a tag set, as the file writes it: an identifier that resolves to an entity of the report, one
  /// that matches none, or a wildcard for every entity of its kind.
  ///
  /// ```
  /// use cc_media_probe::model::{TargetReference, TargetReferenceKind};
  ///
  /// fn describe(target: &TargetReference) -> String {
  ///     match target.kind() {
  ///         TargetReferenceKind::Resolved => {
  ///             format!("entity {:?}", target.location().map(|location| location.index()))
  ///         }
  ///         TargetReferenceKind::Unmatched => format!(
  ///             "identifier {:?} of {:?}, not in this report",
  ///             target.identifier().map(|identifier| identifier.as_str()),
  ///             target.namespace().map(|namespace| namespace.as_str()),
  ///         ),
  ///         TargetReferenceKind::Wildcard => "every entity of its kind".to_owned(),
  ///         other => other.as_str().to_owned(),
  ///     }
  /// }
  /// ```
  #[derive(Clone, Debug, PartialEq, Eq, Hash)]
  pub struct TargetReference {
    kind: TargetReferenceKind,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    identifier: Option<Identifier>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    namespace: Option<IdentifierNamespace>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    location: Option<EntityLocation>,
  }
  check = TargetReference::check_form;
}

impl TargetReference {
  /// Whether the reference resolved, matched nothing, or names every entity of its kind.
  #[must_use]
  pub fn kind(&self) -> &TargetReferenceKind {
    &self.kind
  }

  /// The identifier as written.
  #[must_use]
  pub fn identifier(&self) -> Option<&Identifier> {
    self.identifier.as_ref()
  }

  /// The namespace of the identifier.
  #[must_use]
  pub fn namespace(&self) -> Option<&IdentifierNamespace> {
    self.namespace.as_ref()
  }

  /// The entity that a resolved reference names.
  #[must_use]
  pub fn location(&self) -> Option<&EntityLocation> {
    self.location.as_ref()
  }

  /// Whether the reference's kind, identifier and location agree, which needs the reference alone (`api.md` section
  /// 6.9), as the review of the third slice's first part found `01` read as a Matroska identifier and a resolved
  /// target without its entity: a resolved target has a location and no other has one; and an identifier of the
  /// namespace `matroska_unique_identifier` is present, a 64-bit integer in canonical decimal, and `0` exactly for
  /// the wildcard. A kind or a namespace of a newer version is not checked. As the second review of that part asked,
  /// which read `01` beside a kind of a newer version, the namespace's spelling holds whatever the kind, while only a
  /// kind of this version is held to the wildcard, whose meaning a newer kind does not share.
  #[cfg(feature = "serde")]
  fn check_form(&self) -> Result<(), &'static str> {
    let wildcard = match self.kind {
      TargetReferenceKind::Resolved if self.location.is_some() => Some(false),
      TargetReferenceKind::Unmatched if self.location.is_none() => Some(false),
      TargetReferenceKind::Wildcard if self.location.is_none() => Some(true),
      TargetReferenceKind::Unrecognized(_) => None,
      _ => return Err(super::constants::TARGET_MESSAGE),
    };
    let identified = match self.namespace {
      Some(IdentifierNamespace::MatroskaUniqueIdentifier) => self
        .identifier
        .as_ref()
        .and_then(|identifier| crate::form::canonical::<u64>(identifier.as_str()))
        .is_some_and(|value| wildcard.is_none_or(|wildcard| (value == 0) == wildcard)),
      _ => true,
    };
    if identified {
      Ok(())
    } else {
      Err(super::constants::TARGET_MESSAGE)
    }
  }
}

string_enumeration! {
  /// What a target reference names. `TargetReference` shows an example.
  ///
  /// ```
  /// use cc_media_probe::model::TargetReferenceKind;
  ///
  /// assert_eq!(TargetReferenceKind::Wildcard.as_str(), "wildcard");
  /// ```
  TargetReferenceKind {
    /// An identifier that matches an entity of the report; `location()` gives the entity.
    Resolved => RESOLVED_NAME,
    /// An identifier that matches no entity of the report, kept as written. The entity can still be in the file:
    /// it is unresolved in this report, not proven absent from the file.
    Unmatched => UNMATCHED_NAME,
    /// Every entity of the kind, which Matroska writes as the identifier zero.
    Wildcard => WILDCARD_NAME,
  }
}

string_enumeration! {
  /// The namespace of a target's identifier. A phase that adds targets of its own adds its namespace.
  ///
  /// ```
  /// use cc_media_probe::model::IdentifierNamespace;
  ///
  /// assert_eq!(IdentifierNamespace::MatroskaUniqueIdentifier.as_str(), "matroska_unique_identifier");
  /// ```
  IdentifierNamespace {
    /// The `TagTrackUID`, `TagEditionUID`, `TagChapterUID` or `TagAttachmentUID` of a Matroska tag target, a
    /// 64-bit integer in canonical decimal, where zero is the wildcard (RFC 9559 sections 5.1.8.1.1.3 to
    /// 5.1.8.1.1.6).
    MatroskaUniqueIdentifier => MATROSKA_UNIQUE_IDENTIFIER_NAME,
  }
}

string_enumeration! {
  /// What the target lists of a tag set mean. A tag set can be cut short before its target structure was read,
  /// and then empty lists must not read as the whole file.
  ///
  /// ```
  /// use cc_media_probe::model::{TagSet, TargetScope};
  ///
  /// fn describes_the_whole_file(set: &TagSet) -> bool {
  ///     matches!(set.target_scope(), TargetScope::WholeFile)
  /// }
  ///
  /// assert_eq!(TargetScope::WholeFile.as_str(), "whole_file");
  /// ```
  TargetScope {
    /// The target structure was read in full and names no entity.
    WholeFile => WHOLE_FILE_NAME,
    /// The target structure was read in full, and the lists hold every target it names.
    Entities => ENTITIES_NAME,
    /// The probe stopped before the target structure was read in full, or could not keep every target. The lists
    /// hold the targets kept; the tags section is then incomplete.
    Incomplete => INCOMPLETE_NAME,
  }
}

#[cfg(test)]
mod tests {
  use super::{IdentifierNamespace, TargetLevel, TargetReference, TargetReferenceKind, TargetScope};
  use crate::model::{EntityKind, EntityLocation, Identifier, UnrecognizedValue};

  impl TargetLevel {
    /// The level 50 of an album or a movie.
    pub(crate) fn sample() -> TargetLevel {
      TargetLevel {
        value: Some(50),
        type_name: Some("ALBUM".to_owned()),
      }
    }
  }

  impl TargetReference {
    /// A reference that resolves to the stream at this index.
    pub(crate) fn stream(index: usize) -> TargetReference {
      TargetReference {
        kind: TargetReferenceKind::Resolved,
        identifier: Some(Identifier::sample("12345")),
        namespace: Some(IdentifierNamespace::MatroskaUniqueIdentifier),
        location: Some(EntityLocation::sample(EntityKind::Stream, index, &[])),
      }
    }

    /// A wildcard reference.
    pub(crate) fn wildcard() -> TargetReference {
      TargetReference {
        kind: TargetReferenceKind::Wildcard,
        identifier: Some(Identifier::sample("0")),
        location: None,
        ..TargetReference::stream(0)
      }
    }
  }

  #[test]
  fn keeps_the_level_and_the_references() {
    let level = TargetLevel::sample();
    assert_eq!((level.value(), level.type_name()), (Some(50), Some("ALBUM")));
    let resolved = TargetReference::stream(1);
    assert_eq!(resolved.kind(), &TargetReferenceKind::Resolved);
    assert_eq!(resolved.identifier().map(Identifier::as_str), Some("12345"));
    assert_eq!(
      resolved.namespace(),
      Some(&IdentifierNamespace::MatroskaUniqueIdentifier)
    );
    assert_eq!(resolved.location().map(EntityLocation::index), Some(1));
    assert_eq!(TargetReference::wildcard().location(), None);
  }

  #[test]
  fn names_each_value() {
    for (kind, name) in [
      (TargetReferenceKind::Resolved, "resolved"),
      (TargetReferenceKind::Unmatched, "unmatched"),
      (TargetReferenceKind::Wildcard, "wildcard"),
    ] {
      assert_eq!(kind.as_str(), name);
    }
    for (scope, name) in [
      (TargetScope::WholeFile, "whole_file"),
      (TargetScope::Entities, "entities"),
      (TargetScope::Incomplete, "incomplete"),
      (
        TargetScope::Unrecognized(UnrecognizedValue::sample("partial")),
        "partial",
      ),
    ] {
      assert_eq!(scope.as_str(), name);
    }
    assert_eq!(
      IdentifierNamespace::MatroskaUniqueIdentifier.as_str(),
      "matroska_unique_identifier"
    );
  }
}
