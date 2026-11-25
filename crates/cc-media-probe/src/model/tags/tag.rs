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

//! `Tag`, one tag with the tags nested below it (`api.md` section 6.9; field guide section 3.8).

use std::fmt;
use std::hash::{Hash, Hasher};

use super::{TagKey, TagName, TagValue};
use crate::model::Language;
use crate::model::tree::{Branching, copy, equal, format, hash, release};

/// One tag, with the tags nested below it. A node may have children and no value. A nested tag becomes an
/// attribute of its parent, so a key below the top level does not mean what it means there: the lookups of
/// `MediaReport` see the top level only.
///
/// No operation on the tree recurses on its depth: dropping, cloning, comparing, hashing and `Debug` walk it with
/// an explicit stack, and a caller who walks it does the same.
///
/// ```
/// use cc_media_probe::model::{Tag, TagSet};
///
/// fn dump(set: &TagSet) {
///     let mut pending: Vec<(usize, &Tag)> = set.tags().iter().rev().map(|tag| (0, tag)).collect();
///     while let Some((depth, tag)) = pending.pop() {
///         let value = tag.value().map(|value| value.as_str()).unwrap_or("no value");
///         println!(
///             "{:indent$}{} ({:?}) = {value} [{} languages, {} countries, default {:?}, type {:?}]",
///             "",
///             tag.name().as_written(),
///             tag.key().map(|key| key.as_str()),
///             tag.languages().len(),
///             tag.countries().len(),
///             tag.is_default(),
///             tag.native_type(),
///             indent = depth * 2,
///         );
///         pending.extend(tag.children().iter().rev().map(|child| (depth + 1, child)));
///     }
/// }
/// ```
pub struct Tag {
  node: TagNode,
  branches: Vec<Tag>,
}

/// A tag's own members, without the tags below it.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct TagNode {
  name: TagName,
  key: Option<TagKey>,
  languages: Vec<Language>,
  countries: Vec<String>,
  default: Option<bool>,
  value: Option<TagValue>,
  native_type: Option<String>,
}

impl Tag {
  /// The name as the file writes it.
  #[must_use]
  pub fn name(&self) -> &TagName {
    &self.node.name
  }

  /// The normalized key (F4); absent for a name that has none.
  #[must_use]
  pub fn key(&self) -> Option<&TagKey> {
    self.node.key.as_ref()
  }

  /// The languages that the tag applies to; empty when the file states none.
  #[must_use]
  pub fn languages(&self) -> &[Language] {
    &self.node.languages
  }

  /// The countries that the tag applies to, as ISO 3166 codes as written; empty when the file states none.
  #[must_use]
  pub fn countries(&self) -> &[String] {
    &self.node.countries
  }

  /// Whether the tag is the default among tags of the same name in other languages.
  #[must_use]
  pub fn is_default(&self) -> Option<bool> {
    self.node.default
  }

  /// The value (F25); absent for a node that holds only children, and for a tag that a damaged or truncated
  /// structure left without one.
  #[must_use]
  pub fn value(&self) -> Option<&TagValue> {
    self.node.value.as_ref()
  }

  /// The value's type name as written.
  #[must_use]
  pub fn native_type(&self) -> Option<&str> {
    self.node.native_type.as_deref()
  }

  /// The tags nested below this one, in stored order.
  #[must_use]
  pub fn children(&self) -> &[Tag] {
    &self.branches
  }
}

impl Branching for Tag {
  type Node = TagNode;

  fn node(&self) -> &TagNode {
    &self.node
  }

  fn branches(&self) -> &[Tag] {
    &self.branches
  }

  fn branches_mut(&mut self) -> &mut Vec<Tag> {
    &mut self.branches
  }

  fn from_parts(node: TagNode, branches: Vec<Tag>) -> Tag {
    Tag { node, branches }
  }
}

impl Clone for Tag {
  fn clone(&self) -> Tag {
    copy(self)
  }
}

impl PartialEq for Tag {
  fn eq(&self, other: &Tag) -> bool {
    equal(self, other)
  }
}

impl Eq for Tag {}

impl Hash for Tag {
  fn hash<State: Hasher>(&self, state: &mut State) {
    hash(self, state);
  }
}

impl fmt::Debug for Tag {
  /// Writes the nodes of the subtree in depth-first order, each with the position of its parent.
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    format(self, formatter)
  }
}

impl Drop for Tag {
  fn drop(&mut self) {
    release(self);
  }
}

#[cfg(test)]
mod tests {
  use super::{Tag, TagNode};
  use crate::model::tree::Branching;
  use crate::model::{Language, LanguageCodeSystem, TagKey, TagName, TagValue};

  impl Tag {
    /// A tag of this name, key and text, with these children.
    pub(crate) fn sample(name: &str, key: Option<TagKey>, text: Option<&str>, children: Vec<Tag>) -> Tag {
      Tag::from_parts(
        TagNode {
          name: TagName::sample(name),
          key,
          languages: vec![Language::sample("und", None, LanguageCodeSystem::Bcp47)],
          countries: Vec::new(),
          default: Some(true),
          value: text.map(|text| TagValue::Text(text.to_owned())),
          native_type: Some("utf8".to_owned()),
        },
        children,
      )
    }
  }

  #[test]
  fn gives_each_field_and_its_children() {
    let original = Tag::sample(
      "ORIGINAL",
      None,
      None,
      vec![Tag::sample("TITLE", Some(TagKey::Title), Some("Old"), Vec::new())],
    );
    assert_eq!(original.name().as_written(), "ORIGINAL");
    assert_eq!(original.key(), None);
    assert_eq!(original.languages().len(), 1);
    assert_eq!(original.countries(), &[] as &[String]);
    assert_eq!(original.is_default(), Some(true));
    assert_eq!(original.value(), None);
    assert_eq!(original.native_type(), Some("utf8"));
    let title = &original.children()[0];
    assert_eq!(title.key(), Some(&TagKey::Title));
    assert_eq!(title.value(), Some(&TagValue::Text("Old".to_owned())));
  }

  #[test]
  fn clones_compares_and_shows_its_subtree() {
    let tag = Tag::sample(
      "ORIGINAL",
      None,
      None,
      vec![Tag::sample("TITLE", Some(TagKey::Title), Some("Old"), Vec::new())],
    );
    assert_eq!(tag.clone(), tag);
    assert_ne!(Tag::sample("ORIGINAL", None, None, Vec::new()), tag);
    assert!(format!("{tag:?}").starts_with("[(None, TagNode"));
    let set: std::collections::HashSet<Tag> = [tag.clone(), tag].into_iter().collect();
    assert_eq!(set.len(), 1);
  }
}
