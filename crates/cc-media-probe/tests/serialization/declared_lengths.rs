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

//! The lengths that a document declares to its serializer up front (`api.md` section 12; `json-output.md` section
//! 2). A format that writes the length of a map or a sequence before its contents, such as CBOR, writes the one
//! given, while JSON ignores it, so these tests observe the declared lengths with a serializer of their own, as the
//! review of the third slice's second part asked. A document declares three members for results and four with a
//! usage error, its results' length when it holds them and none when the writer streams them, and every map and
//! sequence inside it, an entry's kept members included, declares the count that it writes or none.

use std::cell::RefCell;
use std::fmt;

use cc_media_probe::document::{Document, DocumentEntry, Generator, UsageError};
use serde::ser::{self, Serialize};
use serde_json::json;

use crate::fixtures::{detection, document, entry, error_record, partial_report, report, stream, with, with_partial};

/// What the serializer was told about one struct, map or sequence, in the order they began: the struct's name, the
/// length declared up front, and the count of members or elements written.
#[derive(Debug, PartialEq)]
struct Declaration {
  name: Option<&'static str>,
  length: Option<usize>,
  written: usize,
}

/// A serializer that writes nothing and notes what it was told about each struct, map and sequence.
#[derive(Clone, Copy)]
struct Noting<'log> {
  log: &'log RefCell<Vec<Declaration>>,
}

/// A struct, map or sequence being written, whose count goes into its slot of the log at its end.
struct Compound<'log> {
  log: &'log RefCell<Vec<Declaration>>,
  slot: usize,
  written: usize,
}

#[derive(Debug)]
struct Refused(String);

impl fmt::Display for Refused {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.0)
  }
}

impl std::error::Error for Refused {}

impl ser::Error for Refused {
  fn custom<Message: fmt::Display>(message: Message) -> Refused {
    Refused(message.to_string())
  }
}

impl<'log> Noting<'log> {
  fn open(self, name: Option<&'static str>, length: Option<usize>) -> Compound<'log> {
    let mut log = self.log.borrow_mut();
    let slot = log.len();
    log.push(Declaration {
      name,
      length,
      written: 0,
    });
    Compound {
      log: self.log,
      slot,
      written: 0,
    }
  }
}

impl Compound<'_> {
  fn write<Item: ?Sized + Serialize>(&mut self, item: &Item) -> Result<(), Refused> {
    item.serialize(Noting { log: self.log })?;
    self.written = self.written.saturating_add(1);
    Ok(())
  }

  fn close(self) {
    self.log.borrow_mut()[self.slot].written = self.written;
  }
}

/// The serializer's methods for values that hold no member, which write nothing.
macro_rules! scalars {
  ($($method:ident: $type:ty),* $(,)?) => {
    $(fn $method(self, _value: $type) -> Result<(), Refused> {
      Ok(())
    })*
  };
}

impl<'log> ser::Serializer for Noting<'log> {
  type Ok = ();
  type Error = Refused;
  type SerializeSeq = Compound<'log>;
  type SerializeTuple = Compound<'log>;
  type SerializeTupleStruct = Compound<'log>;
  type SerializeTupleVariant = Compound<'log>;
  type SerializeMap = Compound<'log>;
  type SerializeStruct = Compound<'log>;
  type SerializeStructVariant = Compound<'log>;

  scalars! {
    serialize_bool: bool, serialize_i8: i8, serialize_i16: i16, serialize_i32: i32, serialize_i64: i64,
    serialize_i128: i128, serialize_u8: u8, serialize_u16: u16, serialize_u32: u32, serialize_u64: u64,
    serialize_u128: u128, serialize_f32: f32, serialize_f64: f64, serialize_char: char, serialize_str: &str,
    serialize_bytes: &[u8], serialize_unit_struct: &'static str,
  }

  fn serialize_none(self) -> Result<(), Refused> {
    Ok(())
  }

  fn serialize_some<Item: ?Sized + Serialize>(self, item: &Item) -> Result<(), Refused> {
    item.serialize(self)
  }

  fn serialize_unit(self) -> Result<(), Refused> {
    Ok(())
  }

  fn serialize_unit_variant(self, _name: &'static str, _index: u32, _variant: &'static str) -> Result<(), Refused> {
    Ok(())
  }

  fn serialize_newtype_struct<Item: ?Sized + Serialize>(self, _name: &'static str, item: &Item) -> Result<(), Refused> {
    item.serialize(self)
  }

  fn serialize_newtype_variant<Item: ?Sized + Serialize>(
    self,
    _name: &'static str,
    _index: u32,
    _variant: &'static str,
    item: &Item,
  ) -> Result<(), Refused> {
    item.serialize(self)
  }

  fn serialize_seq(self, length: Option<usize>) -> Result<Compound<'log>, Refused> {
    Ok(self.open(None, length))
  }

  fn serialize_tuple(self, length: usize) -> Result<Compound<'log>, Refused> {
    Ok(self.open(None, Some(length)))
  }

  fn serialize_tuple_struct(self, name: &'static str, length: usize) -> Result<Compound<'log>, Refused> {
    Ok(self.open(Some(name), Some(length)))
  }

  fn serialize_tuple_variant(
    self,
    name: &'static str,
    _index: u32,
    _variant: &'static str,
    length: usize,
  ) -> Result<Compound<'log>, Refused> {
    Ok(self.open(Some(name), Some(length)))
  }

  fn serialize_map(self, length: Option<usize>) -> Result<Compound<'log>, Refused> {
    Ok(self.open(None, length))
  }

  fn serialize_struct(self, name: &'static str, length: usize) -> Result<Compound<'log>, Refused> {
    Ok(self.open(Some(name), Some(length)))
  }

  fn serialize_struct_variant(
    self,
    name: &'static str,
    _index: u32,
    _variant: &'static str,
    length: usize,
  ) -> Result<Compound<'log>, Refused> {
    Ok(self.open(Some(name), Some(length)))
  }
}

/// The traits of the compounds whose elements have no name.
macro_rules! elements {
  ($($kind:ident :: $method:ident),* $(,)?) => {
    $(impl ser::$kind for Compound<'_> {
      type Ok = ();
      type Error = Refused;

      fn $method<Item: ?Sized + Serialize>(&mut self, item: &Item) -> Result<(), Refused> {
        self.write(item)
      }

      fn end(self) -> Result<(), Refused> {
        self.close();
        Ok(())
      }
    })*
  };
}

elements! {
  SerializeSeq::serialize_element, SerializeTuple::serialize_element, SerializeTupleStruct::serialize_field,
  SerializeTupleVariant::serialize_field,
}

/// The traits of the compounds whose members have a name. A skipped member counts as none written.
macro_rules! members {
  ($($kind:ident),* $(,)?) => {
    $(impl ser::$kind for Compound<'_> {
      type Ok = ();
      type Error = Refused;

      fn serialize_field<Item: ?Sized + Serialize>(&mut self, _name: &'static str, item: &Item) -> Result<(), Refused> {
        self.write(item)
      }

      fn end(self) -> Result<(), Refused> {
        self.close();
        Ok(())
      }
    })*
  };
}

members! { SerializeStruct, SerializeStructVariant }

impl ser::SerializeMap for Compound<'_> {
  type Ok = ();
  type Error = Refused;

  fn serialize_key<Key: ?Sized + Serialize>(&mut self, key: &Key) -> Result<(), Refused> {
    self.write(key)
  }

  fn serialize_value<Item: ?Sized + Serialize>(&mut self, item: &Item) -> Result<(), Refused> {
    item.serialize(Noting { log: self.log })
  }

  fn end(self) -> Result<(), Refused> {
    self.close();
    Ok(())
  }
}

/// What a write told the serializer, in the order that each struct, map and sequence began.
fn declared_by(write: impl FnOnce(Noting<'_>) -> Result<(), Refused>) -> Vec<Declaration> {
  let log = RefCell::new(Vec::new());
  write(Noting { log: &log }).unwrap();
  log.into_inner()
}

/// Every struct, map and sequence that declared a length wrote that many members or elements.
fn every_length_is_exact(declarations: &[Declaration]) -> bool {
  declarations
    .iter()
    .all(|declaration| declaration.length.is_none_or(|length| length == declaration.written))
}

/// A document whose one entry, of a newer outcome, keeps a payload of nested arrays and maps. It is text, so its
/// members keep the order written whichever order `serde_json::Value` gives its maps, sorted by name by default and
/// as inserted with `preserve_order`, as the second review of the third slice's second part found.
const SKIPPED: &str = r#"{"schema_version": "1.0", "generator": {"name": "catalog", "version": "1"}, "results": [
  {"index": 0, "input": "notes.txt", "outcome": "skipped",
    "skip_reason": {"kind": "excluded", "patterns": ["*.txt", {"case": [true, 1]}]}, "attempts": [[], {}]}]}"#;

/// The entry of a newer outcome of `SKIPPED`.
fn skipped() -> DocumentEntry {
  let read: Document = serde_json::from_str(SKIPPED).unwrap();
  read.results()[0].clone()
}

/// The entries of a document: a complete report with two streams, an identified detection, an error record that
/// carries a partial report, and the entry of a newer outcome of `SKIPPED`.
fn entries() -> Vec<DocumentEntry> {
  let streams = json!([stream(0, "video"), stream(1, "audio")]);
  let failed = with_partial(error_record("1.0"), partial_report("1.0"));
  let text = document(
    "1.0",
    json!([
      entry(0, "ok", "report", with(report("1.0"), "streams", streams)),
      entry(1, "ok", "detection", detection("1.0")),
      entry(2, "error", "error", failed),
    ]),
  );
  let read: Document = serde_json::from_value(text).unwrap();
  let mut entries = read.results().to_vec();
  entries.push(skipped());
  entries
}

#[test]
fn a_document_declares_its_members_exactly() {
  let generator = Generator::new("catalog", "1");
  let cases = [
    (Document::from_results(generator.clone(), entries()), 3),
    (Document::from_results(generator.clone(), Vec::new()), 3),
    (
      Document::from_usage_error(generator, UsageError::new("standard input (-) was named twice")),
      4,
    ),
  ];
  for (document, members) in cases {
    let declarations = declared_by(|serializer| document.serialize(serializer));
    let root = Declaration {
      name: Some("Document"),
      length: Some(members),
      written: members,
    };
    assert_eq!(declarations[0], root);
    assert!(every_length_is_exact(&declarations), "{declarations:?}");
    // The results follow the generator and the usage error, with their length given up front.
    let length = document.results().len();
    let results = Declaration {
      name: None,
      length: Some(length),
      written: length,
    };
    let position = if document.usage_error().is_some() { 3 } else { 2 };
    assert_eq!(declarations[position], results);
  }
}

#[test]
fn the_writer_declares_its_members_exactly_and_no_length_of_its_results() {
  let generator = Generator::new("catalog", "1");
  for entries in [entries(), Vec::new()] {
    let count = entries.len();
    let declarations = declared_by(|serializer| Document::serialize_results(&generator, entries, serializer));
    let root = Declaration {
      name: Some("Document"),
      length: Some(3),
      written: 3,
    };
    assert_eq!(declarations[0], root);
    let results = Declaration {
      name: None,
      length: None,
      written: count,
    };
    assert_eq!(declarations[2], results);
    assert!(every_length_is_exact(&declarations), "{declarations:?}");
  }
  // What the entries hold declares its lengths too, which the check above covers.
  let declarations = declared_by(|serializer| Document::serialize_results(&generator, entries(), serializer));
  for name in ["Generator", "MediaReport", "Stream", "Completeness"] {
    assert!(
      declarations.iter().any(|declaration| declaration.name == Some(name)),
      "{name}"
    );
  }
}

#[test]
fn the_kept_members_of_an_entry_declare_their_lengths() {
  // An entry is a map of no declared length, since an entry of a newer outcome writes its kept members after its own
  // three; each kept array and map declares its own, in the order read: `skip_reason`, its `patterns` and the map
  // and array inside them, then `attempts` with its empty array and map.
  let document = Document::from_results(Generator::new("catalog", "1"), vec![skipped()]);
  let declarations = declared_by(|serializer| document.serialize(serializer));
  let lengths: Vec<(Option<usize>, usize)> = declarations[3..]
    .iter()
    .map(|declaration| (declaration.length, declaration.written))
    .collect();
  assert_eq!(
    lengths,
    [
      (None, 5),
      (Some(2), 2),
      (Some(2), 2),
      (Some(1), 1),
      (Some(2), 2),
      (Some(2), 2),
      (Some(0), 0),
      (Some(0), 0)
    ]
  );
}
