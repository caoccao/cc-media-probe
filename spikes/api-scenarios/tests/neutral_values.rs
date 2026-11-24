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
//!
//! The sixteenth review read two more values of ciborium before a version of 2.0, a tagged value and the integer
//! 2^64, and got a type error where the version error belongs. ciborium gives the first as an enum of one value and
//! the second as an integer of 128 bits; the deserializer here gives them through the same calls. The review also
//! read a detection of 2.0 nested in a document of 1.0, with an integer key after the detection's version: the
//! nested version error came only when the document's version came first.
//!
//! The seventeenth review wrapped a value of a known type in a tag, an identified detection's confidence of 60:
//! ciborium drops a tag when a type asks for a value of its own, so the detection read with its version first, and
//! a candidate read alone, while the held value failed with the version last. The deserializer here drops a tag on
//! such a request too. The review also put a tagged value and 2^64 before the version of a detection nested in a
//! document of 1.0, which gave the nested version error with the document's version first and malformed input with
//! its results first; the table of `json-output.md` section 2 asks for malformed input in both orders.

use cc_media_probe::completeness::DetectionStatus;
use cc_media_probe::document::{Document, EntryOutcome};
use cc_media_probe::model::{Candidate, Detection, TagValue};
use serde::de::value::{Error as ValueError, MapDeserializer, SeqDeserializer};
use serde::de::{DeserializeSeed, EnumAccess, Error, IntoDeserializer, VariantAccess, Visitor};
use serde::{Deserializer, forward_to_deserialize_any};

/// A value of a self-describing format whose map names may be integers.
#[derive(Clone)]
enum Tree {
  Text(&'static str),
  Integer(u64),
  Sequence(Vec<Tree>),
  Map(Vec<(Tree, Tree)>),
  /// A tagged value, which ciborium gives as an enum of one value.
  Tagged(Box<Tree>),
  /// An integer beyond 64 bits, which ciborium gives from a bignum.
  Wide(u128),
}

/// Requests of a value of a known type, which drop a tag around the value, as ciborium 0.2.2 does for every request
/// but `deserialize_any`.
macro_rules! untagged {
  ($($method:ident),* $(,)?) => {
    $(
      fn $method<Read: Visitor<'input>>(self, visitor: Read) -> Result<Read::Value, ValueError> {
        match self {
          Tree::Tagged(content) => content.$method(visitor),
          other => other.deserialize_any(visitor),
        }
      }
    )*
  };
}

impl<'input> Deserializer<'input> for Tree {
  type Error = ValueError;

  fn deserialize_any<Read: Visitor<'input>>(self, visitor: Read) -> Result<Read::Value, ValueError> {
    match self {
      Tree::Text(text) => visitor.visit_str(text),
      Tree::Integer(value) => visitor.visit_u64(value),
      Tree::Sequence(items) => visitor.visit_seq(SeqDeserializer::new(items.into_iter())),
      Tree::Map(members) => visitor.visit_map(MapDeserializer::new(members.into_iter())),
      Tree::Tagged(content) => visitor.visit_enum(Tag(*content)),
      Tree::Wide(value) => visitor.visit_u128(value),
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
    name: &'static str,
    variants: &'static [&'static str],
    visitor: Read,
  ) -> Result<Read::Value, ValueError> {
    match self {
      Tree::Text(text) => visitor.visit_enum(text.into_deserializer()),
      Tree::Tagged(content) => content.deserialize_enum(name, variants, visitor),
      other => other.deserialize_any(visitor),
    }
  }

  fn deserialize_struct<Read: Visitor<'input>>(
    self,
    name: &'static str,
    fields: &'static [&'static str],
    visitor: Read,
  ) -> Result<Read::Value, ValueError> {
    match self {
      Tree::Tagged(content) => content.deserialize_struct(name, fields, visitor),
      other => other.deserialize_any(visitor),
    }
  }

  untagged! {
    deserialize_bool, deserialize_i8, deserialize_i16, deserialize_i32, deserialize_i64, deserialize_u8,
    deserialize_u16, deserialize_u32, deserialize_u64, deserialize_f32, deserialize_f64, deserialize_char,
    deserialize_str, deserialize_string, deserialize_seq, deserialize_map, deserialize_identifier,
  }

  forward_to_deserialize_any! {
    <Read: Visitor<'input>>
    i128 u128 bytes byte_buf unit unit_struct tuple tuple_struct ignored_any
  }
}

/// The enum of one value by which ciborium gives a tagged value: a variant of a reserved name and the value.
struct Tag(Tree);

impl<'input> EnumAccess<'input> for Tag {
  type Error = ValueError;
  type Variant = Tag;

  fn variant_seed<Seed: DeserializeSeed<'input>>(self, seed: Seed) -> Result<(Seed::Value, Tag), ValueError> {
    Ok((seed.deserialize(Tree::Text("@@TAGGED@@"))?, self))
  }
}

impl<'input> VariantAccess<'input> for Tag {
  type Error = ValueError;

  fn unit_variant(self) -> Result<(), ValueError> {
    Err(ValueError::custom("expected tag"))
  }

  fn newtype_variant_seed<Seed: DeserializeSeed<'input>>(self, seed: Seed) -> Result<Seed::Value, ValueError> {
    seed.deserialize(self.0)
  }

  fn tuple_variant<Read: Visitor<'input>>(self, _length: usize, _visitor: Read) -> Result<Read::Value, ValueError> {
    Err(ValueError::custom("expected tag"))
  }

  fn struct_variant<Read: Visitor<'input>>(
    self,
    _fields: &'static [&'static str],
    _visitor: Read,
  ) -> Result<Read::Value, ValueError> {
    Err(ValueError::custom("expected tag"))
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

/// The two values of the review: a tagged value and 2^64.
fn wider_than_json() -> [Tree; 2] {
  [Tree::Tagged(Box::new(Tree::Integer(0))), Tree::Wide(1 << 64)]
}

#[test]
fn a_tagged_value_and_an_integer_of_128_bits_are_held_before_the_version() {
  for value in wider_than_json() {
    let message = read(document("2.0", vec![text("future", value.clone())], Vec::new())).unwrap_err();
    assert!(message.starts_with(MAJOR), "{message}");
    // Deeper inside a held member too.
    let deep = Tree::Map(vec![text("values", Tree::Sequence(vec![Tree::Integer(1), value]))]);
    let message = read(document("2.0", vec![text("future", deep)], Vec::new())).unwrap_err();
    assert!(message.starts_with(MAJOR), "{message}");
  }
}

#[test]
fn a_tagged_value_and_an_integer_of_128_bits_fail_once_the_version_is_known() {
  // In a member that no type reads, before the version and after it: after it, Serde's own way of ignoring a value
  // took both, so the order of the members decided.
  for value in wider_than_json() {
    for (before, after) in [
      (vec![text("future", value.clone())], Vec::new()),
      (Vec::new(), vec![text("future", value)]),
    ] {
      let message = read(document("1.0", before, after)).unwrap_err();
      assert!(message.starts_with(NOT_JSON), "{message}");
    }
  }
}

#[test]
fn a_nested_record_reads_its_version_before_a_later_integer_key_in_either_order() {
  // The review's nested case: a detection of 2.0 with an integer key after its version, in a document of 1.0.
  let detection = Tree::Map(vec![
    text("schema_version", Tree::Text("2.0")),
    (Tree::Integer(1), Tree::Text("x")),
  ]);
  let entry = Tree::Map(vec![
    text("index", Tree::Integer(0)),
    text("input", Tree::Text("a")),
    text("outcome", Tree::Text("ok")),
    text("detection", detection),
  ]);
  let results = text("results", Tree::Sequence(vec![entry]));
  let version = text("schema_version", Tree::Text("1.0"));
  let generator = text(
    "generator",
    Tree::Map(vec![text("name", Tree::Text("g")), text("version", Tree::Text("1"))]),
  );
  for members in [
    vec![version.clone(), generator.clone(), results.clone()],
    vec![results, version, generator],
  ] {
    let message = read(Tree::Map(members)).unwrap_err();
    assert!(message.starts_with(NESTED), "{message}");
  }
}

/// An identified detection whose confidence is that value, with its version first or last.
fn identified(confidence: Tree, version_first: bool) -> Tree {
  let usage = Tree::Map(vec![
    text("bytes_read", Tree::Integer(0)),
    text("peak_working_memory", Tree::Integer(0)),
  ]);
  let mut members = vec![
    text("outcome", Tree::Text("identified")),
    text("format", Tree::Text("Matroska")),
    text("confidence", confidence),
    text("candidates", Tree::Sequence(Vec::new())),
    text("usage", usage),
  ];
  let version = text("schema_version", Tree::Text("1.0"));
  match version_first {
    true => members.insert(0, version),
    false => members.push(version),
  }
  Tree::Map(members)
}

#[test]
fn a_tag_around_a_value_of_a_known_type_fails_in_either_order() {
  // The seventeenth review's case: with the version first, the detection read and the tag was dropped.
  let tagged = || Tree::Tagged(Box::new(Tree::Integer(60)));
  for version_first in [true, false] {
    let read = <Detection as serde::Deserialize>::deserialize(identified(tagged(), version_first));
    let message = read.map(|_| ()).unwrap_err().to_string();
    assert!(message.starts_with(NOT_JSON), "{version_first}: {message}");
    assert!(<Detection as serde::Deserialize>::deserialize(identified(Tree::Integer(60), version_first)).is_ok());
  }
  // A candidate read alone, and a tag around a record.
  let candidate = Tree::Map(vec![text("format", Tree::Text("Mp4")), text("confidence", tagged())]);
  let message = <Candidate as serde::Deserialize>::deserialize(candidate)
    .map(|_| ())
    .unwrap_err()
    .to_string();
  assert!(message.starts_with(NOT_JSON), "{message}");
  let candidate = Tree::Map(vec![
    text("format", Tree::Text("Mp4")),
    text("confidence", Tree::Integer(60)),
  ]);
  let message = <Candidate as serde::Deserialize>::deserialize(Tree::Tagged(Box::new(candidate)))
    .map(|_| ())
    .unwrap_err()
    .to_string();
  assert!(message.starts_with(NOT_JSON), "{message}");
}

#[test]
fn a_nested_record_refuses_a_tag_or_a_wide_integer_before_its_version_in_either_parent_order() {
  // The seventeenth review's case: with the document's version first, the nested version error came instead.
  for value in wider_than_json() {
    let detection = Tree::Map(vec![text("future", value), text("schema_version", Tree::Text("2.0"))]);
    let entry = Tree::Map(vec![
      text("index", Tree::Integer(0)),
      text("input", Tree::Text("a")),
      text("outcome", Tree::Text("ok")),
      text("detection", detection),
    ]);
    let results = text("results", Tree::Sequence(vec![entry]));
    let version = text("schema_version", Tree::Text("1.0"));
    let generator = text(
      "generator",
      Tree::Map(vec![text("name", Tree::Text("g")), text("version", Tree::Text("1"))]),
    );
    for members in [
      vec![version.clone(), generator.clone(), results.clone()],
      vec![results, version, generator],
    ] {
      let message = read(Tree::Map(members)).unwrap_err();
      assert!(message.starts_with(NOT_JSON), "{message}");
    }
  }
}

#[test]
fn a_wide_integer_fails_as_a_value_that_json_does_not_carry_in_a_future_entry() {
  // A future entry keeps its members through Serde's buffer, which refused 2^64 with a message of its own.
  let entry = Tree::Map(vec![
    text("index", Tree::Integer(0)),
    text("input", Tree::Text("a")),
    text("outcome", Tree::Text("skipped")),
    text("ratio", Tree::Wide(1 << 64)),
  ]);
  let mut document = document("1.0", Vec::new(), Vec::new());
  if let Tree::Map(members) = &mut document {
    members.retain(|(name, _)| !matches!(name, Tree::Text("results")));
    members.push(text("results", Tree::Sequence(vec![entry])));
  }
  let message = read(document).unwrap_err();
  assert!(message.starts_with(NOT_JSON), "{message}");
}

#[test]
fn a_tag_around_a_root_record_or_a_value_read_alone_fails() {
  // The eighteenth review's cases: tag 42 around a whole detection, with its version first and last, and around a
  // standalone outcome `ok`, which the outermost request gave the format as a typed request, so the tag was dropped.
  let tagged = |tree: Tree| Tree::Tagged(Box::new(tree));
  for version_first in [true, false] {
    let read = <Detection as serde::Deserialize>::deserialize(tagged(identified(Tree::Integer(60), version_first)));
    let message = read.map(|_| ()).unwrap_err().to_string();
    assert!(message.starts_with(NOT_JSON), "{version_first}: {message}");
  }
  let message = read(tagged(document("1.0", Vec::new(), Vec::new()))).unwrap_err();
  assert!(message.starts_with(NOT_JSON), "{message}");
  // An enum written as its name, one that Serde's derive reads, and one that carries data.
  let outcome = <EntryOutcome as serde::Deserialize>::deserialize(tagged(Tree::Text("ok")));
  assert!(outcome.map(|_| ()).unwrap_err().to_string().starts_with(NOT_JSON));
  assert!(<EntryOutcome as serde::Deserialize>::deserialize(Tree::Text("ok")).is_ok());
  let status = <DetectionStatus as serde::Deserialize>::deserialize(tagged(Tree::Text("Complete")));
  assert!(status.map(|_| ()).unwrap_err().to_string().starts_with(NOT_JSON));
  assert!(<DetectionStatus as serde::Deserialize>::deserialize(Tree::Text("Complete")).is_ok());
  let text = Tree::Map(vec![
    text("kind", Tree::Text("text")),
    text("text", Tree::Text("Title")),
  ]);
  let value = <TagValue as serde::Deserialize>::deserialize(tagged(text.clone()));
  assert!(value.map(|_| ()).unwrap_err().to_string().starts_with(NOT_JSON));
  assert!(<TagValue as serde::Deserialize>::deserialize(text).is_ok());
}
