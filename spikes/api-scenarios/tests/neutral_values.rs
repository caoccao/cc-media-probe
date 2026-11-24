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

//! The fifteenth review's case of `json-output.md` section 2: a map whose names JSON cannot carry, such as the
//! integer keys of a CBOR map, is held before the version like any other value, so the version error still comes
//! first. The review read a document of version 2.0 through ciborium 0.2.2 and got "expected string" for such a map
//! before the version, which ciborium itself can read. Once the version is checked, such a map fails as a value that
//! JSON does not carry, wherever it stands, also in a member that no type reads. The spike keeps no CBOR reader, so a
//! small self-describing deserializer of its own gives the integer keys here, through the same Serde calls.

use cc_media_probe::document::Document;
use serde::de::value::{Error as ValueError, MapDeserializer, SeqDeserializer};
use serde::de::{IntoDeserializer, Visitor};
use serde::{Deserializer, forward_to_deserialize_any};

/// A value of a self-describing format whose map names may be integers.
#[derive(Clone)]
enum Tree {
  Text(&'static str),
  Integer(u64),
  Sequence(Vec<Tree>),
  Map(Vec<(Tree, Tree)>),
}

impl<'input> Deserializer<'input> for Tree {
  type Error = ValueError;

  fn deserialize_any<Read: Visitor<'input>>(self, visitor: Read) -> Result<Read::Value, ValueError> {
    match self {
      Tree::Text(text) => visitor.visit_str(text),
      Tree::Integer(value) => visitor.visit_u64(value),
      Tree::Sequence(items) => visitor.visit_seq(SeqDeserializer::new(items.into_iter())),
      Tree::Map(members) => visitor.visit_map(MapDeserializer::new(members.into_iter())),
    }
  }

  fn deserialize_option<Read: Visitor<'input>>(self, visitor: Read) -> Result<Read::Value, ValueError> {
    visitor.visit_some(self)
  }

  fn deserialize_newtype_struct<Read: Visitor<'input>>(
    self,
    _name: &'static str,
    visitor: Read,
  ) -> Result<Read::Value, ValueError> {
    visitor.visit_newtype_struct(self)
  }

  fn deserialize_enum<Read: Visitor<'input>>(
    self,
    _name: &'static str,
    _variants: &'static [&'static str],
    visitor: Read,
  ) -> Result<Read::Value, ValueError> {
    match self {
      Tree::Text(text) => visitor.visit_enum(text.into_deserializer()),
      other => other.deserialize_any(visitor),
    }
  }

  forward_to_deserialize_any! {
    <Read: Visitor<'input>>
    bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string bytes byte_buf unit unit_struct seq tuple
    tuple_struct map struct identifier ignored_any
  }
}

impl<'input> IntoDeserializer<'input, ValueError> for Tree {
  type Deserializer = Tree;

  fn into_deserializer(self) -> Tree {
    self
  }
}

fn text(name: &'static str, value: Tree) -> (Tree, Tree) {
  (Tree::Text(name), value)
}

/// A map whose one name is the integer 1.
fn integer_keyed() -> Tree {
  Tree::Map(vec![(Tree::Integer(1), Tree::Text("x"))])
}

/// A document of that version, with the members given before and after the version.
fn document(version: &'static str, before: Vec<(Tree, Tree)>, after: Vec<(Tree, Tree)>) -> Tree {
  let generator = Tree::Map(vec![text("name", Tree::Text("g")), text("version", Tree::Text("1"))]);
  let mut members = before;
  members.push(text("schema_version", Tree::Text(version)));
  members.push(text("generator", generator));
  members.push(text("results", Tree::Sequence(Vec::new())));
  members.extend(after);
  Tree::Map(members)
}

fn read(tree: Tree) -> Result<Document, String> {
  serde::Deserialize::deserialize(tree).map_err(|error: ValueError| error.to_string())
}

const MAJOR: &str = "a root record has a schema major version that this version does not read: 2.0";
const NESTED: &str = "a nested root record has a schema version other than that of the record that holds it";
const NOT_JSON: &str = "a value is one that JSON carries as read";

#[test]
fn a_document_of_this_format_reads() {
  assert!(read(document("1.0", Vec::new(), Vec::new())).is_ok());
}

#[test]
fn an_integer_keyed_map_before_the_version_is_held_and_the_version_error_comes_first() {
  // The review's case: a future member holding such a map, before a version of 2.0.
  let message = read(document("2.0", vec![text("future", integer_keyed())], Vec::new())).unwrap_err();
  assert!(message.starts_with(MAJOR), "{message}");
  // An integer name of the root record's own is held the same way.
  let message = read(document("2.0", vec![(Tree::Integer(7), Tree::Text("x"))], Vec::new())).unwrap_err();
  assert!(message.starts_with(MAJOR), "{message}");
}

#[test]
fn an_integer_keyed_map_fails_once_the_version_is_known() {
  for (before, after) in [
    (vec![text("future", integer_keyed())], Vec::new()),
    (Vec::new(), vec![text("future", integer_keyed())]),
    (vec![(Tree::Integer(7), Tree::Text("x"))], Vec::new()),
    (Vec::new(), vec![(Tree::Integer(7), Tree::Text("x"))]),
  ] {
    let message = read(document("1.0", before, after)).unwrap_err();
    assert!(message.starts_with(NOT_JSON), "{message}");
  }
}

#[test]
fn a_nested_record_refuses_such_a_map_before_its_own_version() {
  // The record that holds it reads strictly, so a value that JSON does not carry is malformed input before the
  // nested version, as a null is (`json-output.md` section 2), and after it the nested version error comes first.
  let error = |before: Vec<(Tree, Tree)>, version: &'static str| {
    let mut members = before;
    members.extend([
      text("schema_version", Tree::Text(version)),
      text("kind", Tree::Text("format_not_determined")),
      text("message", Tree::Integer(42)),
    ]);
    let entry = Tree::Map(vec![
      text("index", Tree::Integer(0)),
      text("input", Tree::Text("a")),
      text("outcome", Tree::Text("error")),
      text("error", Tree::Map(members)),
    ]);
    let mut document = document("1.0", Vec::new(), Vec::new());
    if let Tree::Map(members) = &mut document {
      members.retain(|(name, _)| !matches!(name, Tree::Text("results")));
      members.push(text("results", Tree::Sequence(vec![entry])));
    }
    read(document).unwrap_err()
  };
  let message = error(vec![text("future", integer_keyed())], "1.1");
  assert!(message.starts_with(NOT_JSON), "{message}");
  let message = error(Vec::new(), "1.1");
  assert!(message.starts_with(NESTED), "{message}");
}
