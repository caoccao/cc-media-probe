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

//! A document read through the tests' own self-describing reader, which gives values that JSON does not carry as
//! ciborium gives them, and through `serde_json` with the private map of its `arbitrary_precision` feature written as
//! literal JSON (`json-output.md` section 2). Before the document's version each such value is held, so the version
//! error comes first; once the version is known each fails as malformed input. A record that an entry holds is
//! nested, so it reads strictly before its own version, in either order of the document's members, and then compares
//! its version with the document's before anything after it. A number at the root is no object, nested too.

use cc_media_probe::document::{Document, EntryOutcome};
use serde::Deserialize;
use serde_json::{Value, json};

use crate::fixtures::{document, entry, error_record, generator};
use crate::self_describing::{Tree, name};
use crate::version_first::{NESTED, OTHER_MAJOR};

const NOT_JSON: &str = "a value that JSON does not carry";
const PRIVATE_NAME: &str = "$serde_json::private::Number";

fn read(tree: Tree) -> Result<Document, String> {
  Document::deserialize(tree).map_err(|error| error.to_string())
}

/// A document of that version without entries, with one more member before its others or after them.
fn document_with(version: &str, member: (Tree, Tree), first: bool) -> Tree {
  Tree::of(&document(version, json!([]))).with(member.0, member.1, first)
}

/// A document of version 1.0 whose one entry holds that record under that member, with the document's `results`
/// before its version or after it.
fn holding(outcome: &str, member: &str, record: Tree, results_first: bool) -> Tree {
  let entry = Tree::of(&json!({ "index": 0, "input": "a", "outcome": outcome })).with(name(member), record, false);
  let document = Tree::of(&json!({ "generator": generator() }));
  let document = document.with(name("results"), Tree::Sequence(vec![entry]), results_first);
  document.with(name("schema_version"), name("1.0"), !results_first)
}

/// The values that JSON does not carry and that the reader gives whole.
fn not_json() -> Vec<Tree> {
  vec![
    Tree::Map(vec![(Tree::Integer(1), name("x"))]),
    Tree::Tagged(Box::new(Tree::Integer(0))),
    Tree::Wide(1 << 64),
    Tree::Bytes(vec![1, 2]),
    Tree::Number(f64::NAN),
  ]
}

#[test]
fn a_document_of_this_format_reads() {
  let read = read(Tree::of(&document(
    "1.0",
    json!([entry(0, "error", "error", error_record("1.0"))]),
  )))
  .unwrap();
  assert_eq!(read.results()[0].outcome(), &EntryOutcome::Error);
}

#[test]
fn such_a_value_before_the_documents_version_is_held_and_the_version_error_comes_first() {
  for value in not_json() {
    let message = read(document_with("2.0", (name("future"), value.clone()), true)).unwrap_err();
    assert!(message.starts_with(OTHER_MAJOR), "{value:?}: {message}");
    // An integer name of the document's own is held the same way.
    let message = read(document_with("2.0", (Tree::Integer(7), value), true)).unwrap_err();
    assert!(message.starts_with(OTHER_MAJOR), "{message}");
  }
}

#[test]
fn such_a_value_fails_once_the_documents_version_is_known_in_either_order() {
  for value in not_json() {
    for first in [true, false] {
      let message = read(document_with("1.0", (name("future"), value.clone()), first)).unwrap_err();
      assert!(message.starts_with(NOT_JSON), "{value:?} {first}: {message}");
    }
  }
}

#[test]
fn a_nested_record_refuses_such_a_value_before_its_own_version_in_either_order() {
  // The record that holds it reads strictly, so a value that JSON does not carry is malformed input before the
  // nested version, as a null is, whichever of the document's members comes first; a value that JSON carries there
  // gives the nested version error.
  let detection_with =
    |future: Tree, version: &str| Tree::Map(vec![(name("future"), future), (name("schema_version"), name(version))]);
  for results_first in [true, false] {
    for value in not_json() {
      let message = read(holding(
        "ok",
        "detection",
        detection_with(value.clone(), "2.0"),
        results_first,
      ))
      .unwrap_err();
      assert!(message.starts_with(NOT_JSON), "{value:?} {results_first}: {message}");
    }
    let message = read(holding(
      "ok",
      "detection",
      detection_with(Tree::Integer(1), "1.1"),
      results_first,
    ))
    .unwrap_err();
    assert!(message.starts_with(NESTED), "{results_first}: {message}");
  }
}

#[test]
fn a_nested_record_reads_its_version_before_a_later_integer_key_in_either_order() {
  let detection = Tree::Map(vec![
    (name("schema_version"), name("2.0")),
    (Tree::Integer(1), name("x")),
  ]);
  for results_first in [true, false] {
    let message = read(holding("ok", "detection", detection.clone(), results_first)).unwrap_err();
    assert!(message.starts_with(NESTED), "{results_first}: {message}");
  }
}

#[test]
fn a_wide_integer_in_an_entry_of_a_newer_outcome_fails_as_a_value_that_json_does_not_carry() {
  // An entry of a newer outcome keeps the members that it does not read, and 2^64 is none that JSON carries.
  for results_first in [true, false] {
    let message = read(holding("skipped", "ratio", Tree::Wide(1 << 64), results_first)).unwrap_err();
    assert!(message.starts_with(NOT_JSON), "{results_first}: {message}");
  }
}

#[test]
fn a_tag_around_a_document_or_an_outcome_read_alone_fails() {
  // The eighteenth review's cases: a typed request for the outermost value let ciborium drop the tag.
  let tagged = |tree: Tree| Tree::Tagged(Box::new(tree));
  let message = read(tagged(Tree::of(&document("1.0", json!([]))))).unwrap_err();
  assert!(message.starts_with(NOT_JSON), "{message}");
  let outcome = EntryOutcome::deserialize(tagged(name("ok")));
  assert!(outcome.map(|_| ()).unwrap_err().to_string().starts_with(NOT_JSON));
  assert_eq!(EntryOutcome::deserialize(name("ok")).ok(), Some(EntryOutcome::Ok));
}

/// A document of version 1.0 whose one entry holds a detection of version 2.0 with `future` before its version,
/// with the document's version before its `results` or after them.
fn nested_detection(future: &Value, version_first: bool) -> String {
  let detection = json!({ "future": future, "schema_version": "2.0" }).to_string();
  let results = format!(r#""results": [{{"index": 0, "input": "a", "outcome": "ok", "detection": {detection}}}]"#);
  let head = format!(r#""schema_version": "1.0", "generator": {}"#, generator());
  if version_first {
    format!("{{{head}, {results}}}")
  } else {
    format!("{{{results}, {head}}}")
  }
}

#[test]
fn a_nested_record_refuses_a_private_number_that_is_not_finite_before_its_version_in_either_order() {
  // The seventeenth review's case, with `arbitrary_precision` written out: `1e400` before the nested version is
  // malformed input, while `1.25` there is a value that JSON carries, so the nested version error comes first.
  for version_first in [true, false] {
    let message = |number: &str| {
      let text = nested_detection(&json!({ PRIVATE_NAME: number }), version_first);
      serde_json::from_str::<Document>(&text).unwrap_err().to_string()
    };
    assert!(message("1e400").starts_with(NOT_JSON), "{version_first}");
    assert!(message("1.25").starts_with(NESTED), "{version_first}");
  }
}

#[test]
fn a_member_of_the_private_name_before_a_nested_version_comes_after_the_version_error_in_either_order() {
  // The eighteenth review's case: only a type that is known refuses the name, and the nested record's type is known
  // once its version is, so the version error comes first.
  for version_first in [true, false] {
    let text = nested_detection(&json!("hi"), version_first).replace(r#""future""#, &json!(PRIVATE_NAME).to_string());
    let message = serde_json::from_str::<Document>(&text).unwrap_err().to_string();
    assert!(message.starts_with(NESTED), "{version_first}: {message}");
  }
}

#[test]
fn a_number_at_the_root_is_no_object_nested_too() {
  // The nineteenth review's case: `1.25`, and the private map that the feature gives for it, are a number at the
  // root of a document and of an entry's report, so neither is a root record without its version.
  let private = json!({ PRIVATE_NAME: "1.25" }).to_string();
  for text in ["1.25", &private] {
    let message = serde_json::from_str::<Document>(text).unwrap_err().to_string();
    assert!(!message.starts_with("the version error"), "{text}: {message}");
    let report: Value = serde_json::from_str(text).unwrap();
    let nested = document("1.0", json!([entry(0, "ok", "report", report)])).to_string();
    let message = serde_json::from_str::<Document>(&nested).unwrap_err().to_string();
    assert!(!message.starts_with("the version error"), "nested {text}: {message}");
  }
  // A map of that name that holds no number's text, or another member beside it, is an object without a version.
  for map in [
    json!({ PRIVATE_NAME: "hi" }),
    json!({ PRIVATE_NAME: "1.25", "other": 1 }),
  ] {
    let message = serde_json::from_value::<Document>(map.clone()).unwrap_err().to_string();
    assert!(
      message.starts_with("the version error: the root record has no schema_version"),
      "{map}: {message}"
    );
  }
}
