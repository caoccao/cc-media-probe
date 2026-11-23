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

//! Tag sets, their targets and tags, `api.md` section 6.9. A tag set's tree of tags is one flat array in its
//! serialized form (`tree.rs`); the values are in `tag_value.rs`.

use std::fmt;
use std::hash::{Hash, Hasher};

use super::constants::{
  ALBUM_NAME, ARTIST_NAME, ATTACHMENT_NAME, CHAPTER_NAME, COMMENT_NAME, COPYRIGHT_NAME, DATE_NAME, DISC_NUMBER_NAME,
  EDITION_NAME, ENCODER_NAME, ENTITIES_NAME, GENRE_NAME, INCOMPLETE_NAME, PROGRAM_NAME, STREAM_NAME, TITLE_NAME,
  TRACK_NUMBER_NAME, WHOLE_FILE_NAME,
};
use super::language::Language;
use super::tag_value::TagValue;
use super::tree::{self, Branching};
use super::unrecognized_value::UnrecognizedValue;
use super::values::Identifier;

/// Tags with the entities they describe.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TagSet {
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
  target_level: Option<TargetLevel>,
  stream_targets: Vec<TargetReference>,
  edition_targets: Vec<TargetReference>,
  chapter_targets: Vec<TargetReference>,
  attachment_targets: Vec<TargetReference>,
  target_scope: TargetScope,
  #[cfg_attr(feature = "serde", serde(default))]
  superseded: bool,
  #[cfg_attr(feature = "serde", serde(with = "super::tree::flat"))]
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

  /// Whether the set describes the whole file, the entities its lists name, or a scope that the probe did not
  /// establish.
  pub fn target_scope(&self) -> &TargetScope {
    &self.target_scope
  }

  /// The top-level tags, in stored order.
  pub fn tags(&self) -> &[Tag] {
    &self.tags
  }

  /// Whether a later element replaced the one that holds this set, as a Tags element between the clusters of a live
  /// Matroska segment resets the earlier ones (RFC 9559 section 23.2). The set stays as history, and no lookup sees
  /// it (twelfth review).
  pub fn is_superseded(&self) -> bool {
    self.superseded
  }
}

/// What a tag set describes.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum TargetScope {
  /// The target structure was read in full and names no entity.
  WholeFile,
  /// The target structure was read in full, and the four lists hold every target it names.
  Entities,
  /// The probe stopped before the target structure was read in full, or did not keep every target. The lists hold
  /// the targets kept, and empty lists do not mean the whole file.
  Incomplete,
  /// A value that a newer version wrote.
  #[cfg_attr(feature = "serde", serde(with = "crate::model::unrecognized_text"))]
  Unrecognized(UnrecognizedValue<TargetScope>),
}

impl TargetScope {
  /// The serialized name, or the text of an unrecognized value.
  pub fn as_str(&self) -> &str {
    match self {
      TargetScope::WholeFile => WHOLE_FILE_NAME,
      TargetScope::Entities => ENTITIES_NAME,
      TargetScope::Incomplete => INCOMPLETE_NAME,
      TargetScope::Unrecognized(value) => value.as_str(),
    }
  }
}

/// A target level: a number and a type name.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TargetLevel {
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
  value: Option<u64>,
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
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
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
  identifier: Option<Identifier>,
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
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
  #[cfg_attr(feature = "serde", serde(with = "crate::model::unrecognized_text"))]
  Unrecognized(UnrecognizedValue<TargetReferenceKind>),
}

/// Where an entity is in the report.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EntityLocation {
  kind: EntityKind,
  index: usize,
  chapter_path: Vec<usize>,
}

impl EntityLocation {
  /// What the entity is.
  pub fn kind(&self) -> &EntityKind {
    &self.kind
  }

  /// The index of the stream, edition, attachment or program; for a chapter, the index of its edition.
  pub fn index(&self) -> usize {
    self.index
  }

  /// For a chapter, the child indexes from the edition's top level down; empty otherwise.
  pub fn chapter_path(&self) -> &[usize] {
    &self.chapter_path
  }
}

/// The kind of entity that a location names.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum EntityKind {
  /// A stream.
  Stream,
  /// A chapter edition.
  Edition,
  /// A chapter.
  Chapter,
  /// An attachment.
  Attachment,
  /// A program.
  Program,
  /// A value that a newer version wrote.
  #[cfg_attr(feature = "serde", serde(with = "crate::model::unrecognized_text"))]
  Unrecognized(UnrecognizedValue<EntityKind>),
}

impl EntityKind {
  /// The serialized name, or the text of an unrecognized value.
  pub fn as_str(&self) -> &str {
    match self {
      EntityKind::Stream => STREAM_NAME,
      EntityKind::Edition => EDITION_NAME,
      EntityKind::Chapter => CHAPTER_NAME,
      EntityKind::Attachment => ATTACHMENT_NAME,
      EntityKind::Program => PROGRAM_NAME,
      EntityKind::Unrecognized(value) => value.as_str(),
    }
  }
}

/// One tag, with the tags nested below it. No operation on a tag recurses on the depth of its tree (`tree.rs`).
pub struct Tag {
  node: TagNode,
  children: Vec<Tag>,
}

/// A tag's own members: everything but the tags below it.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub(crate) struct TagNode {
  name: TagName,
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
  key: Option<TagKey>,
  languages: Vec<Language>,
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
  value: Option<TagValue>,
}

impl Tag {
  /// The name as written.
  pub fn name(&self) -> &TagName {
    &self.node.name
  }

  /// The normalized key, where the library knows the name.
  pub fn key(&self) -> Option<&TagKey> {
    self.node.key.as_ref()
  }

  /// The languages the value applies to.
  pub fn languages(&self) -> &[Language] {
    &self.node.languages
  }

  /// The value; absent for a node that only has children.
  pub fn value(&self) -> Option<&TagValue> {
    self.node.value.as_ref()
  }

  /// The nested tags.
  pub fn children(&self) -> &[Tag] {
    &self.children
  }
}

impl Branching for Tag {
  type Node = TagNode;

  fn node(&self) -> &TagNode {
    &self.node
  }

  fn branches(&self) -> &[Tag] {
    &self.children
  }

  fn branches_mut(&mut self) -> &mut Vec<Tag> {
    &mut self.children
  }

  fn from_parts(node: TagNode, branches: Vec<Tag>) -> Tag {
    Tag {
      node,
      children: branches,
    }
  }
}

impl Drop for Tag {
  fn drop(&mut self) {
    tree::release(self);
  }
}

impl Clone for Tag {
  fn clone(&self) -> Tag {
    tree::copy(self)
  }
}

impl PartialEq for Tag {
  fn eq(&self, other: &Tag) -> bool {
    tree::equal(self, other)
  }
}

impl Eq for Tag {}

impl Hash for Tag {
  fn hash<State: Hasher>(&self, state: &mut State) {
    tree::hash(self, state);
  }
}

impl fmt::Debug for Tag {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    tree::format(self, formatter)
  }
}

/// A standalone tag is the flat form of its subtree, the tag itself first (`api.md` section 12).
#[cfg(feature = "serde")]
impl serde::Serialize for Tag {
  fn serialize<Output: serde::Serializer>(&self, serializer: Output) -> Result<Output::Ok, Output::Error> {
    tree::flat::serialize_subtree(self, serializer)
  }
}

#[cfg(feature = "serde")]
impl<'input> serde::Deserialize<'input> for Tag {
  fn deserialize<Input: serde::Deserializer<'input>>(deserializer: Input) -> Result<Tag, Input::Error> {
    tree::flat::deserialize_subtree(deserializer)
  }
}

/// A tag's name as written.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TagName {
  as_written: String,
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
  namespace: Option<String>,
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
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
  #[cfg_attr(feature = "serde", serde(with = "crate::model::unrecognized_text"))]
  Unrecognized(UnrecognizedValue<TagKey>),
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
