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

//! Tag sets, their targets and tags, `api.md` section 6.9.

use std::fmt;

use super::constants::{
  ALBUM_NAME, ARTIST_NAME, COMMENT_NAME, COPYRIGHT_NAME, DATE_NAME, DISC_NUMBER_NAME, ENCODER_NAME, GENRE_NAME,
  TITLE_NAME, TRACK_NUMBER_NAME,
};
use super::language::Language;
use super::values::{Identifier, UnrecognizedValue};

/// Tags with the entities they describe.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TagSet {
  target_level: Option<TargetLevel>,
  stream_targets: Vec<TargetReference>,
  edition_targets: Vec<TargetReference>,
  chapter_targets: Vec<TargetReference>,
  attachment_targets: Vec<TargetReference>,
  tags: Vec<Tag>,
}

impl TagSet {
  /// The target level.
  pub fn target_level(&self) -> Option<&TargetLevel> {
    self.target_level.as_ref()
  }

  /// The streams the set describes.
  pub fn stream_targets(&self) -> &[TargetReference] {
    &self.stream_targets
  }

  /// The editions the set describes.
  pub fn edition_targets(&self) -> &[TargetReference] {
    &self.edition_targets
  }

  /// The chapters the set describes.
  pub fn chapter_targets(&self) -> &[TargetReference] {
    &self.chapter_targets
  }

  /// The attachments the set describes.
  pub fn attachment_targets(&self) -> &[TargetReference] {
    &self.attachment_targets
  }

  /// Whether the set describes the whole file: all four target lists are empty.
  pub fn targets_whole_file(&self) -> bool {
    self.stream_targets.is_empty()
      && self.edition_targets.is_empty()
      && self.chapter_targets.is_empty()
      && self.attachment_targets.is_empty()
  }

  /// The top-level tags, in stored order.
  pub fn tags(&self) -> &[Tag] {
    &self.tags
  }
}

/// A target level: a number and a type name.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TargetLevel {
  value: Option<u64>,
  type_name: Option<String>,
}

impl TargetLevel {
  /// The level's number.
  pub fn value(&self) -> Option<u64> {
    self.value
  }

  /// The level's type name, such as `ALBUM`.
  pub fn type_name(&self) -> Option<&str> {
    self.type_name.as_deref()
  }
}

/// One reference of a target list.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TargetReference {
  kind: TargetReferenceKind,
  identifier: Option<Identifier>,
  location: Option<EntityLocation>,
}

impl TargetReference {
  /// Whether the reference resolved, matched nothing, or is a wildcard.
  pub fn kind(&self) -> &TargetReferenceKind {
    &self.kind
  }

  /// The identifier as written.
  pub fn identifier(&self) -> Option<&Identifier> {
    self.identifier.as_ref()
  }

  /// Where the entity is in the report, for a resolved reference.
  pub fn location(&self) -> Option<&EntityLocation> {
    self.location.as_ref()
  }
}

/// What a target reference is.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum TargetReferenceKind {
  /// It matches an entity of the report.
  Resolved,
  /// It matches no entity, and is kept as written.
  Unmatched,
  /// Every entity of its kind.
  Wildcard,
  /// A value that a newer version wrote.
  Unrecognized(UnrecognizedValue),
}

/// Where an entity is in the report.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EntityLocation {
  index: usize,
  chapter_path: Vec<usize>,
}

impl EntityLocation {
  /// The index of the stream, edition or attachment; for a chapter, the index of its edition.
  pub fn index(&self) -> usize {
    self.index
  }

  /// For a chapter, the child indexes from the edition's top level down; empty otherwise.
  pub fn chapter_path(&self) -> &[usize] {
    &self.chapter_path
  }
}

/// One tag.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Tag {
  name: TagName,
  key: Option<TagKey>,
  languages: Vec<Language>,
  value: Option<TagValue>,
  children: Vec<Tag>,
}

impl Tag {
  /// The name as written.
  pub fn name(&self) -> &TagName {
    &self.name
  }

  /// The normalized key, where the library knows the name.
  pub fn key(&self) -> Option<&TagKey> {
    self.key.as_ref()
  }

  /// The languages the value applies to.
  pub fn languages(&self) -> &[Language] {
    &self.languages
  }

  /// The value; absent for a node that only has children.
  pub fn value(&self) -> Option<&TagValue> {
    self.value.as_ref()
  }

  /// The nested tags.
  pub fn children(&self) -> &[Tag] {
    &self.children
  }
}

/// A tag's name as written.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TagName {
  as_written: String,
  namespace: Option<String>,
  bytes: Option<Vec<u8>>,
}

impl TagName {
  /// The name decoded by the rules of its format or namespace, with U+FFFD for a byte they cannot decode.
  pub fn as_written(&self) -> &str {
    &self.as_written
  }

  /// The namespace, for a format whose keys have one.
  pub fn namespace(&self) -> Option<&str> {
    self.namespace.as_deref()
  }

  /// The key exactly as stored, for a format whose keys are bytes that a namespace interprets.
  pub fn bytes(&self) -> Option<&[u8]> {
    self.bytes.as_deref()
  }
}

/// A normalized tag key (F4).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum TagKey {
  /// The title.
  Title,
  /// The artist.
  Artist,
  /// The album.
  Album,
  /// The date of the content's creation.
  Date,
  /// The genre.
  Genre,
  /// A comment.
  Comment,
  /// The copyright.
  Copyright,
  /// The encoder.
  Encoder,
  /// The track number.
  TrackNumber,
  /// The disc number.
  DiscNumber,
  /// A value that a newer version wrote.
  Unrecognized(UnrecognizedValue),
}

impl TagKey {
  /// The serialized name, or the text of an unrecognized value.
  pub fn as_str(&self) -> &str {
    match self {
      TagKey::Title => TITLE_NAME,
      TagKey::Artist => ARTIST_NAME,
      TagKey::Album => ALBUM_NAME,
      TagKey::Date => DATE_NAME,
      TagKey::Genre => GENRE_NAME,
      TagKey::Comment => COMMENT_NAME,
      TagKey::Copyright => COPYRIGHT_NAME,
      TagKey::Encoder => ENCODER_NAME,
      TagKey::TrackNumber => TRACK_NUMBER_NAME,
      TagKey::DiscNumber => DISC_NUMBER_NAME,
      TagKey::Unrecognized(value) => value.as_str(),
    }
  }
}

/// A tag's value, in the format's own type (F25).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum TagValue {
  /// Text.
  Text(String),
  /// Binary data.
  Binary(BinaryValue),
  /// An integer.
  Integer(IntegerValue),
  /// A Boolean.
  Boolean(bool),
  /// A value type that a newer version wrote.
  Unrecognized(UnrecognizedValue),
}

/// A binary tag value.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BinaryValue {
  size: u64,
  data: Option<Vec<u8>>,
}

impl BinaryValue {
  /// The size of the value, in bytes.
  pub fn size(&self) -> u64 {
    self.size
  }

  /// The bytes, when they were kept within the retained-data limit.
  pub fn data(&self) -> Option<&[u8]> {
    self.data.as_deref()
  }
}

/// Any 64-bit signed or unsigned integer.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IntegerValue {
  magnitude: u64,
  negative: bool,
}

impl IntegerValue {
  /// The value as `i64`, when it fits.
  pub fn to_i64(&self) -> Option<i64> {
    let magnitude = i64::try_from(self.magnitude).ok()?;
    Some(if self.negative {
      magnitude.wrapping_neg()
    } else {
      magnitude
    })
  }

  /// The value as `u64`, when it is not negative.
  pub fn to_u64(&self) -> Option<u64> {
    (!self.negative).then_some(self.magnitude)
  }
}

impl fmt::Display for IntegerValue {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    if self.negative {
      write!(formatter, "-{}", self.magnitude)
    } else {
      write!(formatter, "{}", self.magnitude)
    }
  }
}
